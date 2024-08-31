#[allow(dead_code)]
use crate::files::*;
use ::std::path::PathBuf;
use ratatui::widgets::{ListState, ScrollbarState};
use std::env::current_dir;
#[derive(Debug)]
pub enum CurrentScreen {
    Search,
    Main,
    Preview,
    CreateNewFile,
    ConfirmDelete,
    IsNewFileADir,
    Rename,
    ErrorPopUp,
    Help,
}
#[derive(Debug)]
pub struct App {
    /*MAIN*/
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub files: Vec<StfmFile>,          // A list of files in the current directory
    pub current_dir: PathBuf,          // the current directory the user is in
    pub index_selected: Option<usize>, // the index of the file the user has selected
    pub selected_file: Option<StfmFile>, // the current file the user is selected
    pub list_state: ListState,         // the state of the list widget

    /*PREVIEW */
    pub preview_string: String, // the string to be displayed in the preview block
    pub v_preview_scroll_state: ScrollbarState, // the state of the vertical scrollbar at preview
    pub vertical_scroll: usize, // the vertical scroll position of the preview block
    pub h_preview_scroll_state: ScrollbarState, // the state of the vertical scrollbar at preview
    pub horizontal_scroll: usize, // the vertical scroll position of the preview block

    /*CreateNewFile */
    pub new_file: String,      // the name of the new file to be created
    pub new_file_is_dir: bool, // if the new file is a directory
    pub error_message: Option<String>,
    pub show_hidden: bool, // if hidden files should be shown
    pub search_input: String,
}

impl App {
    pub fn new() -> App {
        let current_dir = current_dir().unwrap();
        let files = list_files(&current_dir, true);
        let mut a = App {
            current_screen: CurrentScreen::Main,
            current_dir,
            files,
            selected_file: None,
            index_selected: None,
            list_state: ListState::default(),
            preview_string: String::new(),
            v_preview_scroll_state: ScrollbarState::default(),
            h_preview_scroll_state: ScrollbarState::default(),
            vertical_scroll: 0,
            horizontal_scroll: 0,
            new_file: String::new(),
            new_file_is_dir: false,
            show_hidden: true,
            error_message: None,
            search_input: String::new(),
        };
        a.list_state.select_first();
        a.index_selected = a.list_state.selected();
        a.selected_file = a.files.get(a.index_selected.unwrap_or(0)).cloned();
        a
    }

    pub fn toggle_hidden(&mut self) {
        self.show_hidden = !self.show_hidden;
        self.reset();
    }

    pub fn cd(&mut self, dir_name: String) {
        let mut new_path = self.current_dir.clone();
        new_path.push(dir_name);
        match change_dir(&new_path) {
            Ok(_) => {}
            Err(e) => {
                self.error_message = Some(e.to_string());
                self.current_screen = CurrentScreen::ErrorPopUp;
                return;
            }
        };
        self.current_dir = new_path;
        self.reset();            
    }

    pub fn down(&mut self) {
        match self.index_selected {
            Some(index) => {
                if index == self.files.len() - 1 || index == usize::MAX {
                    self.list_state.select_first();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
                } else {
                    self.list_state.select_next();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
                }
            }
            _ => {}
        }
    }

    pub fn up(&mut self) {
        match self.index_selected {
            Some(index) => {
                if index == 0 {
                    self.list_state.select_last();
                    self.index_selected = Some(self.files.len() - 1);
                    self.selected_file = self.files.get(self.files.len() - 1).cloned();
                } else {
                    self.list_state.select_previous();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self
                        .files
                        .get(self.index_selected.unwrap_or(self.files.len() - 1))
                        .cloned();
                }
            }
            _ => {}
        }
    }

    pub fn page_up(&mut self) {
        self.list_state.scroll_up_by(5);
        self.index_selected = self.list_state.selected();
        self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
    }

    pub fn page_down(&mut self) {
        self.list_state.scroll_down_by(5);
        self.index_selected = self.list_state.selected();
        self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
    }

    pub fn handle_selected_file(&mut self) {
        match self.selected_file.clone() {
            Some(file) => {
                if file.is_dir {
                    self.cd(file.name.clone());
                } else {
                    self.current_screen = CurrentScreen::Preview;
                    self.preview_string = match read_file(&file.full_path) {
                        Ok(content) => content,
                        Err(e) => {
                            self.error_message = Some(e.to_string());
                            self.current_screen = CurrentScreen::ErrorPopUp;
                            return;
                        }
                    };
                    self.v_preview_scroll_state = self
                        .v_preview_scroll_state
                        .content_length(self.preview_string.len());
                    self.vertical_scroll = 0;
                    self.h_preview_scroll_state = self
                        .h_preview_scroll_state
                        .content_length(self.preview_string.len());
                    self.horizontal_scroll = 0;
                }
            }
            None => {}
        }
    }

    pub fn previus_dir(&mut self) {
        self.current_dir.pop();
        self.reset();
    }

    pub fn scroll_up(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_sub(10);
        self.v_preview_scroll_state = self.v_preview_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_down(&mut self) {
        self.vertical_scroll = self.vertical_scroll.saturating_add(10);
        self.v_preview_scroll_state = self.v_preview_scroll_state.position(self.vertical_scroll);
    }

    pub fn scroll_left(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_sub(10);
        self.h_preview_scroll_state = self.h_preview_scroll_state.position(self.horizontal_scroll);
    }

    pub fn scroll_right(&mut self) {
        self.horizontal_scroll = self.horizontal_scroll.saturating_add(10);
        self.h_preview_scroll_state = self.h_preview_scroll_state.position(self.horizontal_scroll);
    }

    pub fn new_file(&mut self, file_name: &str) {
        if !self.new_file_is_dir {
            let full_new_path = PathBuf::from(&self.current_dir).join(file_name);
            match create_file(&full_new_path) {
                Ok(_) => {}
                Err(e) => {
                    self.new_file.clear();
                    self.new_file_is_dir = false;
                    self.error_message = Some(e.to_string());
                    self.current_screen = CurrentScreen::ErrorPopUp;
                    return;
                }
            };
            self.files = list_files(&self.current_dir, self.show_hidden);
            self.index_selected = Some(0);
            self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
            self.current_screen = CurrentScreen::Main;
            self.new_file.clear();
            self.new_file_is_dir = false;
        } else {
            let full_new_path = PathBuf::from(&self.current_dir).join(file_name);
            match make_dir(&full_new_path) {
                Ok(_) => {}
                Err(e) => {
                    self.new_file.clear();
                    self.new_file_is_dir = false;
                    self.error_message = Some(e.to_string());
                    self.current_screen = CurrentScreen::ErrorPopUp;
                    return;
                }
            };
            self.reset();
        }
    }

    pub fn rm(&mut self) {
        let file = self.selected_file.as_ref().unwrap();
        if file.is_dir {
            match delete_dir(&PathBuf::from(file.full_path.clone())) {
                Ok(_) => {}
                Err(e) => {
                    self.error_message = Some(e.to_string());
                    self.current_screen = CurrentScreen::ErrorPopUp;
                    return;
                }
            }
        } else {
            match delete_file(&PathBuf::from(file.full_path.clone())) {
                Ok(_) => {}
                Err(e) => {
                    self.error_message = Some(e.to_string());
                    self.current_screen = CurrentScreen::ErrorPopUp;
                    return;
                }
            }
        }
        self.reset();
    }

    pub fn rename(&mut self, new_name: &str) {
        match self.selected_file.clone() {
            Some(file) => {
                let old_path = PathBuf::from(&file.full_path);
                let parent_dir = PathBuf::from(&self.current_dir);
                let new_path = parent_dir.join(new_name);
                match rename_file(&old_path, &new_path) {
                    Ok(_) => {}
                    Err(e) => {
                        self.error_message = Some(e.to_string());
                        self.current_screen = CurrentScreen::ErrorPopUp;
                        return;
                    }
                }
                self.reset();
            }
            _ => {}
        }
    }

    pub fn search(&mut self) {
        let query = self.search_input.clone();
        let mut new_files = Vec::new();
        let files= list_files(&self.current_dir, self.show_hidden);
        for file in files.iter() {
            if file.name.contains(&query) {
                new_files.push(file.clone());
            }
        }
        self.files = new_files;
        self.list_state.select_first();
        self.index_selected = Some(0);
        self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
    }

    pub fn reset(&mut self) {
        self.search_input.clear();
        self.files = list_files(&self.current_dir, self.show_hidden);
        self.list_state.select_first();
        self.index_selected = Some(0);
        self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
        self.new_file.clear();
        self.new_file_is_dir = false;
        self.error_message = None;
        self.vertical_scroll = 0;
        self.horizontal_scroll = 0;
        self.preview_string.clear();
        self.current_screen = CurrentScreen::Main;
    }

}
