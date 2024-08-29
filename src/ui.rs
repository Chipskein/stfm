#![allow(unused)]
use ratatui::{
    crossterm::ExecutableCommand,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    symbols::block,
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListDirection, ListItem, ListState, Paragraph, Scrollbar,
        ScrollbarOrientation, Wrap,
    },
    Frame,
};

use crate::{
    app::{App, CurrentScreen},
    files::StfmFile,
};

pub fn ui(frame: &mut Frame, app: &mut App) {
    // Create the layout sections.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());
    let title = Paragraph::new(Text::styled(
        app.current_dir.to_string_lossy(),
        Style::default().fg(Color::Green),
    ))
    .block(title_block);
    frame.render_widget(title, chunks[0]);

    match app.current_screen {
        CurrentScreen::Main => {
            let list_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let mut list_items = Vec::<ListItem>::new();
            app.files.iter().for_each(|file| {
                let widget_item = ListItem::new(Span::styled(
                    format!("[{}] {}", file.extension.to_uppercase(), file.name,),
                    Style::default(),
                ));
                list_items.push(widget_item);
            });
            let list = List::new(list_items)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom)
                .block(list_block);
            frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
        }
        CurrentScreen::Preview => {
            let chunk_main = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[1]);
            let list_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let mut list_items = Vec::<ListItem>::new();
            app.files.iter().for_each(|file| {
                let widget_item = ListItem::new(Span::styled(
                    format!("[{}] {}", file.extension.to_uppercase(), file.name,),
                    Style::default(),
                ));
                list_items.push(widget_item);
            });
            let list = List::new(list_items)
                .highlight_style(Style::default().bg(Color::White).fg(Color::Black))
                .highlight_symbol(">>")
                .repeat_highlight_symbol(true)
                .direction(ListDirection::TopToBottom)
                .block(list_block);
            frame.render_stateful_widget(list, chunk_main[0], &mut app.list_state);

            /*Preview Block */
            let preview_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let text = Paragraph::new(Text::from(app.preview_string.clone()))
                .block(preview_block)
                .scroll((app.vertical_scroll as u16, 0));
            frame.render_widget(text, chunk_main[1]);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                chunk_main[1],
                &mut app.preview_scroll_state,
            );
        }
        _ => {}
    }

    let file_info_text = format!("");
    let footer =
        Paragraph::new(Text::from(file_info_text)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, chunks[2]);

    /*
    if let Some(editing) = &app.currently_editing {
        let popup_block = Block::default()
            .title("Enter a new key-value pair")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));

        let area = centered_rect(60, 25, frame.area());
        frame.render_widget(popup_block, area);

        let popup_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        let mut key_block = Block::default().title("Key").borders(Borders::ALL);
        let mut value_block = Block::default().title("Value").borders(Borders::ALL);

        let active_style = Style::default().bg(Color::LightYellow).fg(Color::Black);

        //match editing {
            // CurrentlyEditing::Key => key_block = key_block.style(active_style),
            // CurrentlyEditing::Value => value_block = value_block.style(active_style),
        //};

        // let key_text = Paragraph::new(app.key_input.clone()).block(key_block);
        // frame.render_widget(key_text, popup_chunks[0]);

        // let value_text = Paragraph::new(app.value_input.clone()).block(value_block);
        // frame.render_widget(value_text, popup_chunks[1]);
    }
    */
    if let CurrentScreen::CreateNewFile = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let area = centered_rect(60, 25, frame.area());
        let chunks_pop_up=Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);
        let popup_block = Block::default()
            .title("Create a new file")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let desc_text = Text::styled(
            " Write Down the name of the new file and press Enter to create it or Esc to cancel",
            Style::default().fg(Color::Red),
        );
        let desc_paragraph = Paragraph::new(desc_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(desc_paragraph, chunks_pop_up[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .style(Style::default());
        let input = Paragraph::new(Text::styled(
            app.new_file.clone(),
            Style::default().fg(Color::Green),
        ))
        .block(input_block);
        frame.render_widget(input, chunks_pop_up[1]);
    }

    if let CurrentScreen::ConfirmDelete = app.current_screen {
        frame.render_widget(Clear, frame.area()); //this clears the entire screen and anything already drawn
        let area = centered_rect(60, 25, frame.area());
        let popup_block = Block::default()
            .title(format!("Delete file {}", app.selected_file.as_ref().unwrap().full_path))
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let desc_text = Text::styled(
            format!(" Are you sure you want to delete this file? [y/n]"),
            Style::default().fg(Color::Red),
        );
        let desc_paragraph = Paragraph::new(desc_text)
            .block(popup_block)
            .wrap(Wrap { trim: false });
        frame.render_widget(desc_paragraph, area);

    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
