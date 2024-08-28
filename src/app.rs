#![allow(unused)]

use std::env::current_dir;
use::std::path::PathBuf;
use crate::files::*;
pub enum CurrentScreen {Main}
pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub files: Vec<StfmFile>, // A list of files in the current directory
    pub current_dir: PathBuf, // the current directory the user is in
    pub index_selected: Option<usize>, // the index of the file the user has selected
    pub selected_file: Option<StfmFile>, // the current file the user is selected
}

impl App {
    pub fn new() -> App {
        let current_dir = current_dir().unwrap();
        let files = list_files(&current_dir);
        App {
            current_screen: CurrentScreen::Main,
            current_dir,
            files,
            selected_file: None,
            index_selected: None,
        }
    }
    
    pub fn cd(&mut self, dir_name: &str) {
        self.current_dir.push(dir_name);
        change_dir(&self.current_dir);
        self.files = list_files(&self.current_dir);
    }

    pub fn go_back(&mut self) {
        self.current_dir.pop();
        self.files = list_files(&self.current_dir);
    }

    pub fn go_forward(&mut self, dir_name: &str) {
        self.current_dir.push(dir_name);
        self.files = list_files(&self.current_dir);
    }

    pub fn touch(&mut self, file_name: &str) {
        let full_path = self.current_dir.join(file_name);
        create_file(&full_path);
        self.files = list_files(&self.current_dir);
    }

    pub fn rm(&mut self, file_name: &str) {
        let full_path = self.current_dir.join(file_name);
        delete_file(&full_path);
        self.files = list_files(&self.current_dir);
    }

    pub fn search(&mut self, query: &str) {
        todo!()
    }

}