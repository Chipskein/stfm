use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        self,
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use std::sync::mpsc::TryRecvError;
use std::{error::Error, io, time::Duration, time::Instant};
mod app;
mod files;
mod ui;
use crate::{
    app::{App, CurrentScreen},
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
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(1); // Adjust tick rate for UI refresh
    loop {
        terminal.draw(|f| ui(f, app))?;
        if let Some(ref receiver) = app.progress_receiver {
            match receiver.try_recv() {
                Ok(progress) => {
                    app.readed_bytes = progress;
                    match app.file_to_copy.clone() {
                        Some(file) => {
                            if app.readed_bytes == file.size {
                                app.file_to_copy = None;
                                app.readed_bytes = 0;
                                app.reset();
                            }                        
                        }
                        None => {}
                    }
                }
                Err(TryRecvError::Empty) => {}
                Err(TryRecvError::Disconnected) => {
                    app.file_to_copy = None;
                    app.readed_bytes = 0;
                    app.reset();
                }
            }
        }
        if crossterm::event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release {
                    continue;
                }
                match app.current_screen {

                    CurrentScreen::Main => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            break Ok(true);
                        }
                        
                        KeyCode::Char('n') => {
                            app.current_screen = CurrentScreen::IsNewFileADir;
                        }

                        KeyCode::Char('h') => {
                            app.current_screen = CurrentScreen::Help;
                        }

                        KeyCode::Char('d') => match app.selected_file.clone() {
                            Some(_) => {
                                app.current_screen = CurrentScreen::ConfirmDelete;
                            }
                            None => {
                                app.error_message = Some("No file selected".to_string());
                                app.current_screen = CurrentScreen::ErrorPopUp;

                            }
                        }
                        KeyCode::Char('c') => match app.selected_file.clone() {
                            Some(file) => {
                                if file.type_name == "File"{
                                    app.file_to_copy = Some(file);
                                }
                            }
                            None => {
                                app.error_message = Some("No file selected".to_string());
                                app.current_screen = CurrentScreen::ErrorPopUp;
                            }

                        }

                        KeyCode::Char('p') => match app.file_to_copy.clone() {
                            Some(_) => {
                                app.current_screen = CurrentScreen::ConfirmCopyingPopUp;
                            }
                            None => {
                                app.error_message = Some("No file selected".to_string());
                                app.current_screen = CurrentScreen::ErrorPopUp;
                            }
                        }

                        KeyCode::Char('r') => match app.selected_file.clone() {
                            Some(_) => {
                                app.current_screen = CurrentScreen::Rename;
                            }
                            None => {
                                app.error_message = Some("No file selected".to_string());
                                app.current_screen = CurrentScreen::ErrorPopUp;
                            }
                        }

                        KeyCode::Char('/') => {
                            app.search_input.clear();
                            app.current_screen = CurrentScreen::Search;
                        }

                        KeyCode::Char('.') => {
                            app.toggle_hidden();
                        }

                        KeyCode::PageDown => {
                            app.page_down();
                        }

                        KeyCode::PageUp => {
                            app.page_up();
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
                    },
                    CurrentScreen::Search => match key.code {
                        KeyCode::Esc => {
                            app.reset();
                        }
                        KeyCode::Backspace => {
                            app.search_input.pop();
                            app.search();
                        }
                        KeyCode::Enter => {
                            app.search();
                            app.current_screen = CurrentScreen::Main;
                        }
                        KeyCode::Delete => {
                            app.search_input.clear();
                            app.search();
                        }
                        _ => match key.code.to_string().chars().last() {
                            Some(c) => {
                                if c.is_alphanumeric() || c == '.' {
                                    app.search_input.push(c);
                                    app.search();
                                }
                            }
                            _ => {}
                        },
                    },
                    CurrentScreen::Preview => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }
                        
                        KeyCode::Down => {
                            app.scroll_down(10);
                        }

                        KeyCode::PageDown => {
                            app.scroll_down(50);
                        }
                        
                        KeyCode::Up => {
                            app.scroll_up(10);
                        }

                        KeyCode::PageUp => {
                            app.scroll_up(50);
                        }


                        
                        KeyCode::Left => {
                            app.scroll_left();
                        }
                        
                        KeyCode::Right => {
                            app.scroll_right();
                        }

                        _ => {}
                    },
                    CurrentScreen::IsNewFileADir => match key.code {
                        KeyCode::Char('d') => {
                            app.new_file_is_dir = true;
                            app.current_screen = CurrentScreen::CreateNewFile;
                        }

                        KeyCode::Char('f') => {
                            app.new_file_is_dir = false;
                            app.current_screen = CurrentScreen::CreateNewFile;
                        }

                        _ => {
                            app.new_file_is_dir = false;
                            app.current_screen = CurrentScreen::Main;
                        }
                    },
                    CurrentScreen::CreateNewFile => match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
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

                        _ => match key.code.to_string().chars().last() {
                            Some(c) => {
                                if c.is_alphanumeric() || c == '.' {
                                    match app.new_file.chars().last() {
                                        Some('.') => {
                                            if c != '.' {
                                                app.new_file.push(c);
                                            }
                                        }
                                        _ => {
                                            app.new_file.push(c);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        },
                    }

                    CurrentScreen::ConfirmDelete => match key.code {
                        KeyCode::Char('y') => {
                            app.rm();
                        }
                        _ => {
                            app.current_screen = CurrentScreen::Main;
                        }
                    }

                    CurrentScreen::ConfirmCopyingPopUp => match key.code {
                        KeyCode::Char('y') => {
                            app.current_screen = CurrentScreen::CopyingProgressBar;
                            app.copy();
                        }
                        _ => {
                            app.current_screen = CurrentScreen::Main;
                        }
                    }
                    
                    CurrentScreen::CopyingProgressBar => match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.file_to_copy = None;
                            app.readed_bytes = 0;
                            app.reset();
                        }
                            
                        _ => {}
                    }
                    
                    CurrentScreen::Rename => match key.code {
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                        }

                        KeyCode::Enter => {
                            if !app.new_file.trim().is_empty() {
                                app.rename(&app.new_file.clone());
                            }
                        }

                        KeyCode::Backspace => {
                            app.new_file.pop();
                        }

                        KeyCode::Delete => {
                            app.new_file.clear();
                        }

                        _ => match key.code.to_string().chars().last() {
                            Some(c) => {
                                if c.is_alphanumeric() || c == '.' {
                                    match app.new_file.chars().last() {
                                        Some('.') => {
                                            if c != '.' {
                                                app.new_file.push(c);
                                            }
                                        }
                                        _ => {
                                            app.new_file.push(c);
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                
                    CurrentScreen::ErrorPopUp => match key.code {
                        _ => {
                            app.error_message = None;
                            app.current_screen = CurrentScreen::Main;
                        }
                    }

                    CurrentScreen::Help => match key.code {
                        _ => {
                            app.current_screen = CurrentScreen::Main;
                        }
                    }

                }
            }
        }
        // Periodically tick the app to refresh UI without events
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            // Trigger a redraw by breaking the loop and letting the UI update on tick
        }
    }
}
