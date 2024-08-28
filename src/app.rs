use std::env::current_dir;
use::std::path::PathBuf;
use crate::files::list_files;
pub enum CurrentScreen {Main}
pub struct App {
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub files: Vec<PathBuf>, // A list of files in the current directory
    pub current_dir: PathBuf, // the current directory the user is in
}

impl App {
    pub fn new() -> App {
        let current_dir = current_dir().unwrap();
        let files = list_files(&current_dir);
        App {
            current_screen: CurrentScreen::Main,
            current_dir,
            files,
        }
    }
    
}