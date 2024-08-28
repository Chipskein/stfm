#![allow(unused)]
use std::{error::Error, io};
use ratatui::{
    backend::{Backend, CrosstermBackend}, crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    }, layout::Margin, widgets::{List, ListState}, Terminal
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
    let mut list_state = ListState::default();
    list_state.select_first();
    let res = run_app(&mut terminal, &mut app, &mut list_state);
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, list_state:&mut ListState) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app,list_state))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => {
                    match key.code {
                        KeyCode::Char('q') => {
                            break Ok(true);
                        }
                        KeyCode::Down => {
                            match app.index_selected {
                                Some(index) => {
                                    if index==app.files.len()-1{
                                        list_state.select_first();
                                        app.index_selected = list_state.selected();
                                        app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();

                                    } else {
                                        list_state.select_next();
                                        app.index_selected = list_state.selected();
                                        app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();
                                    }
                                    
                                }
                                _ => {
                                    list_state.select_first();
                                    app.index_selected = list_state.selected();
                                    app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();
                                }
                            }
                        }
                        KeyCode::Up => {
                            match app.index_selected {
                                Some(index)=>{
                                    if index==0{
                                        list_state.select_last();
                                        app.index_selected = list_state.selected();
                                        app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();
                                    } else {
                                        list_state.select_previous();
                                        app.index_selected = list_state.selected();
                                        app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();
                                    }
                                }
                                _ =>{
                                    list_state.select_last();
                                    app.index_selected = list_state.selected();
                                    app.selected_file = app.files.get(app.index_selected.unwrap_or(0)).cloned();
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}