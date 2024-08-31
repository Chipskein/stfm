use std::io::{Error, Read};
use std::path::{PathBuf,Path};

#[derive(Debug, Clone)]
pub struct StfmFile {
    pub full_path: String,
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
}

#[cfg(windows)]
use std::os::windows::fs::MetadataExt; // for accessing Windows-specific metadata
pub fn is_hidden<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    let path = path.as_ref();
    // Unix-like systems: Check if the file name starts with a dot
    #[cfg(unix)]
    {
        Ok(path.file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name.starts_with('.')))
    }

    // Windows: Check the file attributes for the HIDDEN flag
    #[cfg(windows)]
    {
        let metadata = fs::metadata(path)?;
        let attributes = metadata.file_attributes();
        Ok(attributes & 0x2 != 0) // 0x2 is the FILE_ATTRIBUTE_HIDDEN flag
    }
}

/// List all files in the current directory
pub fn list_files(current_dir: &PathBuf, show_hidden: bool) -> Vec<StfmFile> {
    let mut files = Vec::new();
    match std::fs::read_dir(current_dir) {
        Ok(files_in_dir) => {
            for entry in files_in_dir {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if !show_hidden && is_hidden(&path).unwrap() {
                            continue;
                        }
                        let filename= match  path.file_name() {
                            Some(name) => name.to_string_lossy().to_string(),
                            None => "UNKNOWN".to_string()
                        };
                        let metadata = match  entry.metadata() {
                            Ok(meta) => {meta}
                            Err(_) => {continue;}
                        };
                        let ext= match path.extension() {
                            Some(ext) => {
                                if !metadata.is_dir() {
                                    ext.to_string_lossy().to_string()
                                } else{
                                    "DIR".to_string()
                                }
                            }
                            None => {
                                if metadata.file_type().is_symlink() {
                                    "LINK".to_string()
                                } else if metadata.is_dir() {
                                    "DIR".to_string()
                                } else if metadata.is_file() {
                                    "FILE".to_string()
                                } else {
                                    "UNKNOWN".to_string()
                                }
                            }
                        };
                        let file = StfmFile {
                            full_path: path.to_string_lossy().to_string(),
                            name: filename,
                            extension: ext,
                            is_dir: metadata.is_dir(),
                        };
                        files.push(file);
                    }
                    Err(_) => {
                        continue;
                    }
                }
            }
        }
        _ => {}
    }
    return files;
}

/// Create a file
pub fn create_file(file_path: &PathBuf)-> Result<bool, Error> {
    match std::fs::File::create(file_path){
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Delete a file
pub fn delete_file(file_name: &PathBuf)-> Result<bool, Error> {
    match std::fs::remove_file(file_name){
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }

}

/// Read a file
pub fn read_file(file_name: &str) -> Result<String,Error> {
    match std::fs::File::open(file_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => {
                    Ok(contents)
                }
                Err(e) => {
                    Err(e)
                }
            }
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Change the current directory
pub fn change_dir(dir_name: &PathBuf)-> Result<bool, Error> {
    match std::env::set_current_dir(dir_name) {
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Make a directory
pub fn make_dir(dir_name: &PathBuf)-> Result<bool, Error> {
    match std::fs::create_dir(dir_name){
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Delete a directory
pub fn delete_dir(dir_name: &PathBuf)-> Result<bool, Error> {
    match std::fs::remove_dir_all(dir_name){
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}

/// Rename a file
pub fn rename_file(old_name: &PathBuf, new_name: &PathBuf)-> Result<bool, Error> {
    match std::fs::rename(old_name, new_name){
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Err(e)
        }
    }
}