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
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
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

            let preview_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let text = Paragraph::new(Text::from(app.preview_string.clone()))
                .block(preview_block)
                .scroll((app.vertical_scroll as u16, app.horizontal_scroll as u16));

            frame.render_widget(text, chunk_main[1]);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                chunk_main[1],
                &mut app.v_preview_scroll_state,
            );
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::HorizontalBottom),
                chunk_main[1],
                &mut app.h_preview_scroll_state,
            );
        }
        _ => {}
    }
    match app.current_screen {
        CurrentScreen::IsNewFileADir => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(40, 20, frame.area());
            let mut title_pop_up = format!("Create new Entry",);
            let mut text = format!(" Do you want to create a directory [y/n]");
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default().fg(Color::Yellow));
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, area);
        }

        CurrentScreen::CreateNewFile => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(45, 25, frame.area());
            let chunks_pop_up = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(area);
            let popup_block = Block::default()
                .title("Create a new entry")
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(
                " Write Down the name of the new entry then press 'Enter' to create it or 'Esc' to cancel",
                Style::default().fg(Color::Yellow),
            );
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, chunks_pop_up[0]);
            let input_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White).fg(Color::Black));
            let input = Paragraph::new(Text::styled(
                app.new_file.clone(),
                Style::default().fg(Color::Black).bg(Color::White),
            ))
            .block(input_block);
            frame.render_widget(input, chunks_pop_up[1]);
        }

        CurrentScreen::ConfirmDelete => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(40, 20, frame.area());
            let mut title_pop_up = format!(
                "Delete file {}",
                app.selected_file.as_ref().unwrap().full_path
            );
            let mut text = format!(" Are you sure you want to delete this file? [y/n]");
            if app.selected_file.as_ref().unwrap().is_dir {
                title_pop_up = format!(
                    "Delete directory {}",
                    app.selected_file.as_ref().unwrap().full_path
                );
                text=format!(" Are you sure you want to delete this directory? All files inside will be deleted[y/n]");
            }
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default().fg(Color::Yellow));
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, area);
        }

        CurrentScreen::Rename => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(45, 25, frame.area());
            let chunks_pop_up = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(area);
            let popup_block = Block::default()
                .title(format!(
                    "Rename Entry {}",
                    app.selected_file.as_ref().unwrap().full_path
                ))
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(
                " Write Down the name of the new entry then press 'Enter' to change it or 'Esc' to cancel",
                Style::default().fg(Color::Yellow),
            );
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, chunks_pop_up[0]);

            let input_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().bg(Color::White).fg(Color::Black));
            let input = Paragraph::new(Text::styled(
                app.new_file.clone(),
                Style::default().fg(Color::Black).bg(Color::White),
            ))
            .block(input_block);
            frame.render_widget(input, chunks_pop_up[1]);
        }

        _ => {}
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
