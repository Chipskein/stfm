use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{
        Block, Borders, Clear, List, ListDirection, ListItem, Paragraph, Scrollbar,
        ScrollbarOrientation, Wrap,
    },
    Frame,
};

use crate::app::{App, CurrentScreen};

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(frame.area());

    match app.current_screen {
        CurrentScreen::Main | CurrentScreen::Preview => {
            let chunk_top = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            let title_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            
            let mut title_str=app.current_dir.to_string_lossy().to_string();
            if !app.search_input.is_empty(){
                title_str=format!("{}\nSearch:{}",app.current_dir.to_string_lossy(),app.search_input);
            }

            let title = Paragraph::new(Text::styled(
                title_str,
                Style::default().fg(Color::Cyan),
            ))
            .block(title_block);
            frame.render_widget(title, chunk_top[0]);

            let mut file_text = format!(
                "Name:{}\nSize(bytes):{} Type:{}\nLast modified:{}",
                ' ', ' ', ' ', ' '
            );
            if let Some(file) = app.selected_file.clone() {
                file_text = format!(
                    "Name:{}\nSize(b):{} Type:{}\nLast modified:{}",
                    file.name, file.size, file.type_name, file.modified
                );
            }
            let file_info_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let file_info_text =
                Paragraph::new(Text::styled(file_text, Style::default())).block(file_info_block);
            frame.render_widget(file_info_text, chunk_top[1]);
        }
        CurrentScreen::Search => {
            let search_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let search = Paragraph::new(Text::styled(
                app.search_input.clone(),
                Style::default().fg(Color::Yellow),
            ))
            .block(search_block);
            frame.render_widget(search, chunks[0]);
        }
        _ => {}
    }

    match app.current_screen {
        CurrentScreen::Main | CurrentScreen::Search=> {
            let list_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let mut list_items = Vec::<ListItem>::new();

            app.files.iter().for_each(|file| {
                let mut style = Style::default().fg(Color::Green);
                if file.is_dir {
                    style = Style::default().fg(Color::Cyan);
                }
                let widget_item = ListItem::new(Span::styled(
                    format!("[{}] {}", file.extension.to_uppercase(), file.name,),
                    style,
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
                let mut style = Style::default().fg(Color::Green);
                if file.is_dir {
                    style = Style::default().fg(Color::Cyan);
                }
                let widget_item = ListItem::new(Span::styled(
                    format!("[{}] {}", file.extension.to_uppercase(), file.name,),
                    style,
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
            let title_pop_up = format!("Create new entry",);
            let text = format!(
                "For a new file press 'f'\nFor a new directory press 'd' \nAny other key to cancel"
            );
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default());
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
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);
            let popup_block = Block::default()
                .title("Create a new entry")
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(
                "Write down the name of the new entry then press 'Enter' to create it or 'Esc' to cancel",
                Style::default(),
            );
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, chunks_pop_up[0]);
            let input_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let input = Paragraph::new(Text::styled(app.new_file.clone(), Style::default()))
                .block(input_block);
            frame.render_widget(input, chunks_pop_up[1]);
        }

        CurrentScreen::ConfirmDelete => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(40, 20, frame.area());
            let file = match app.selected_file.clone() {
                Some(file) => file,
                None => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
            };
            let mut title_pop_up = format!("Delete file {}", file.full_path);
            let mut text = format!("Are you sure you want to delete this file? [y/n]");
            if file.is_dir {
                title_pop_up = format!("Delete directory {}", file.full_path);
                text=format!("Are you sure you want to delete this directory?\nAll files inside will be deleted [y/n]");
            }
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default());
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, area);
        }

        CurrentScreen::Rename => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(45, 25, frame.area());
            let file = match app.selected_file.clone() {
                Some(file) => file,
                None => {
                    app.current_screen = CurrentScreen::Main;
                    return;
                }
            };
            let chunks_pop_up = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(area);
            let popup_block = Block::default()
                .title(format!("Rename Entry {}", file.full_path))
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(
                " Write Down the name of the new entry then press 'Enter' to change it or 'Esc' to cancel",
                Style::default(),
            );
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, chunks_pop_up[0]);

            let input_block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default());
            let input = Paragraph::new(Text::styled(app.new_file.clone(), Style::default()))
                .block(input_block);
            frame.render_widget(input, chunks_pop_up[1]);
        }

        CurrentScreen::ErrorPopUp => {
            let msg = app.error_message.clone().unwrap_or(String::new());
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(40, 20, frame.area());
            let title_pop_up = format!("Error",);
            let text = format!("The following error occured :{}", msg);
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default().fg(Color::Red));
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, area);
        }

        CurrentScreen::Help => {
            frame.render_widget(Clear, frame.area());
            let area = centered_rect(60, 50, frame.area());
            let title_pop_up = format!("Help");
            let text = format!(
                r#"
                Welcome and thank you for using STFM! :3
                This is a simple file manager that allows you to navigate through your files and directories
                You can navigate through the files using the arrow keys
                You can open a file or directory by pressing 'Enter' or 'Right Arrow'
                You can go back to the previous directory by pressing 'Backspace' or 'Left' key
                You can see a preview of the file by selecting it
                With preview open you can scroll down by pressing 'Down' and scroll up by pressing 'Up'
                With preview open you can scroll right by pressing 'Right' and scroll left by pressing 'Left'
                With preview open you can go back to the main screen by pressing 'q' or 'Esc'
                You can create a new file/dir by pressing 'n'
                You can delete a file/dir by pressing 'd'
                You can rename a file/dir by pressing 'r'
                You can toggle hidden files by pressing '.'
                You can scroll down by pressing 'PageDown'
                You can scroll up by pressing 'PageUp'
                You can exit the application by pressing 'q' or 'Esc'
            "#
            );
            let popup_block = Block::default()
                .title(title_pop_up)
                .borders(Borders::ALL)
                .style(Style::default());
            let desc_text = Text::styled(text, Style::default());
            let desc_paragraph = Paragraph::new(desc_text)
                .block(popup_block)
                .wrap(Wrap { trim: false });
            frame.render_widget(desc_paragraph, area);
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
