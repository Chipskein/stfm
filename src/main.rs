use std::{error::Error, io};
use ratatui::{
    backend::{Backend, CrosstermBackend}, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, Terminal
};
mod files;
mod app;
mod ui;
use crate::{
    app::{App,CurrentScreen},
    ui::ui,
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;
    // create app and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);
    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("{err:?}");
    }
    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => {
                    match key.code {
                        KeyCode::Char('q')|KeyCode::Esc => {
                            break Ok(true);
                        }
                        KeyCode::Char('n') => {
                            app.current_screen=CurrentScreen::IsNewFileADir;
                            //app.current_screen=CurrentScreen::CreateNewFile;
                        }
                        KeyCode::Char('d') => {
                            match  app.selected_file.clone() {
                                Some(_) => {
                                    app.current_screen=CurrentScreen::ConfirmDelete;
                                }
                                None => {}
                            }
                        }
                        KeyCode::Down => {
                            app.down();
                        }
                        KeyCode::Up => {
                            app.up();
                        }
                        KeyCode::Enter | KeyCode::Right => {
                            app.handle_selected_file();
                        }
                        KeyCode::Backspace | KeyCode::Left => {
                            app.previus_dir();
                        }

                        _ => {}
                    }
                }
                CurrentScreen::Preview => {
                    match key.code {
                        KeyCode::Char('q')|KeyCode::Esc | KeyCode::Left=> {
                            app.current_screen=CurrentScreen::Main;
                        }
                        KeyCode::Down => {
                            app.scroll_down();
                        }
                        KeyCode::Up => {
                            app.scroll_up();
                        }
                        _ => {}
                    }
                }
                CurrentScreen::IsNewFileADir => {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.new_file_is_dir=true;
                            app.current_screen=CurrentScreen::CreateNewFile;
                        }
                        KeyCode::Char('n') => {
                            app.new_file_is_dir=false;
                            app.current_screen=CurrentScreen::CreateNewFile;
                        }
                        _ => {
                            app.new_file_is_dir=false;
                            app.current_screen=CurrentScreen::Main;
                        }
                    }
                }
                CurrentScreen::CreateNewFile => {
                    match key.code {
                        KeyCode::Esc => {
                            app.current_screen=CurrentScreen::Main;
                        }
                        KeyCode::Enter => {
                            if !app.new_file.trim().is_empty() {
                                app.new_file(&app.new_file.clone());
                            } 
                        }
                        KeyCode::Backspace => {
                            app.new_file.pop();
                        }

                        KeyCode::Delete => {
                            app.new_file.clear();
                        }

                        _ => {
                            match key.code.to_string().chars().last() {
                                Some(c) =>{
                                    app.new_file.push(c);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                CurrentScreen::ConfirmDelete => {
                    match key.code {
                        KeyCode::Char('y') => {
                            app.rm();
                        }
                        _ => {
                            app.current_screen=CurrentScreen::Main;
                        }
                    }
                }
            }
        }
    }
}