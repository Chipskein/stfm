#![allow(unused)]

use std::env::current_dir;
use::std::path::PathBuf;
use ratatui::{widgets::{ListState}};
use crate::files::*;
pub enum CurrentScreen {Main}
pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub files: Vec<StfmFile>, // A list of files in the current directory
    pub current_dir: PathBuf, // the current directory the user is in
    pub index_selected: Option<usize>, // the index of the file the user has selected
    pub selected_file: Option<StfmFile>, // the current file the user is selected
    pub list_state: ListState,
}

impl App{
    pub fn new() -> App {
        let current_dir = current_dir().unwrap();
        let files = list_files(&current_dir);
        let mut a=App {
            current_screen: CurrentScreen::Main,
            current_dir,
            files,
            selected_file: None,
            index_selected: None,
            list_state: ListState::default(),
        };
        a.list_state.select_first();
        a
    }
    
    pub fn cd(&mut self, dir_name: String) {
        self.current_dir.push(dir_name);
        change_dir(&self.current_dir);
        self.files = list_files(&self.current_dir);
    }

    pub fn down(&mut self) {
        match self.index_selected {
            Some(index) => {
                if index==self.files.len()-1{
                    self.list_state.select_first();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();

                } else {
                    self.list_state.select_next();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
                }
                
            }
            _ => {
                self.list_state.select_first();
                self.index_selected = self.list_state.selected();
                self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
            }
        }
    }

    pub fn up(&mut self) {
        match self.index_selected {
            Some(index)=>{
                if index==0{
                    self.list_state.select_last();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
                } else {
                    self.list_state.select_previous();
                    self.index_selected = self.list_state.selected();
                    self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
                }
            }
            _ =>{
                self.list_state.select_last();
                self.index_selected = self.list_state.selected();
                self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
            }
        }
    }
    pub fn handle_selected_file(&mut self) {
        if self.selected_file.is_some(){
            let file=self.selected_file.as_ref().unwrap();
            if file.is_dir {
                self.cd(file.name.clone());
            } else {
                //Handle File
                todo!();
            }
        }
    }
    pub fn previus_dir(&mut self) {
        self.current_dir.pop();
        self.files = list_files(&self.current_dir);
        self.index_selected=Some(0);
        self.selected_file = self.files.get(self.index_selected.unwrap_or(0)).cloned();
        self.list_state.select_first();
    }

    pub fn new_file(&mut self, file_name: &str) {
        todo!();
    }

    pub fn rm(&mut self, file_name: &str) {
        todo!();
    }

    pub fn search(&mut self, query: &str) {
        todo!()
    }

}