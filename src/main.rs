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
            }
        }
    }
}