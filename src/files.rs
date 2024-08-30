use std::io::Read;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct StfmFile {
    pub full_path: String,
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
}


/// List all files in the current directory
pub fn list_files(current_dir: &PathBuf) -> Vec<StfmFile> {
    let mut files = Vec::new();
    match std::fs::read_dir(current_dir) {
        Ok(files_in_dir) => {
            for entry in files_in_dir {
                let entry = entry.unwrap();
                let path = entry.path();
                let file=StfmFile {
                    full_path: path.to_string_lossy().to_string(),
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    extension: match path.extension() {
                        Some(ext) => ext.to_string_lossy().to_string(),
                        None => {
                            if entry.metadata().unwrap().is_dir(){
                                "DIR".to_string()
                            } else{
                                "FILE".to_string()
                            }
                        },
                    },
                    is_dir: entry.metadata().unwrap().is_dir(),
                };
                files.push(file);
            }
        },
        _ => {},
    }
    return files;
    
}

/// Create a file
pub fn create_file(file_path: &PathBuf) {
    std::fs::File::create(file_path).unwrap();
}

/// Delete a file
pub fn delete_file(file_name: &PathBuf) {
    std::fs::remove_file(file_name).unwrap();
}

/// Read a file
pub fn read_file(file_name: &str) -> String {
    let mut file = std::fs::File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

/// Change the current directory
pub fn change_dir(dir_name: &PathBuf) {
    match std::env::set_current_dir(dir_name) {
        Ok(_) => {},
        Err(_) => {},
    }
}

/// Make a directory
pub fn make_dir(dir_name: &PathBuf) {
    std::fs::create_dir(dir_name).unwrap();
}

/// Delete a directory
pub fn delete_dir(dir_name: &PathBuf) {
    std::fs::remove_dir_all(dir_name).unwrap();
}



/// Rename a file
pub fn rename_file(old_name: &PathBuf, new_name: &PathBuf) {
    std::fs::rename(old_name, new_name).unwrap();
}

/*
    /// Copy a file
    pub fn copy_file(from: &PathBuf, to: &PathBuf) {
        std::fs::copy(from, to).unwrap();
    }
*/



