use std::io::{Error, Read};
use std::fs::File;
use std::io::{self,Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;
extern crate chrono;
use chrono::offset::Utc;
use chrono::DateTime;
#[derive(Debug, Clone)]
pub struct StfmFile {
    pub full_path: String,
    pub name: String,
    pub extension: String,
    pub is_dir: bool,
    pub type_name: String,
    pub size: u64,
    pub modified: String,
}

#[cfg(windows)]
use std::os::windows::fs::MetadataExt; // for accessing Windows-specific metadata
pub fn is_hidden<P: AsRef<Path>>(path: P) -> std::io::Result<bool> {
    let path = path.as_ref();
    // Unix-like systems: Check if the file name starts with a dot
    #[cfg(unix)]
    {
        Ok(path
            .file_name()
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
                        let filename = match path.file_name() {
                            Some(name) => name.to_string_lossy().to_string(),
                            None => "UNKNOWN".to_string(),
                        };
                        let metadata = match entry.metadata() {
                            Ok(meta) => meta,
                            Err(_) => {
                                continue;
                            }
                        };
                        let ext = match path.extension() {
                            Some(ext) => {
                                if !metadata.is_dir() {
                                    ext.to_string_lossy().to_string()
                                } else {
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
                            type_name: match metadata.file_type().is_symlink() {
                                true => "Link".to_string(),
                                false => match metadata.is_dir() {
                                    true => "Directory".to_string(),
                                    false => "File".to_string(),
                                },
                            },
                            size: metadata.len(),
                            modified: match metadata.modified() {
                                Ok(time) => {
                                    let datetime: DateTime<Utc> = time.into();
                                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                                }
                                Err(_) => "Unknown".to_string(),
                            },
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
pub fn create_file(file_path: &PathBuf) -> Result<bool, Error> {
    match std::fs::File::create(file_path) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Delete a file
pub fn delete_file(file_name: &PathBuf) -> Result<bool, Error> {
    match std::fs::remove_file(file_name) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Read a file
pub fn read_file(file_name: &str) -> Result<String, Error> {
    match std::fs::File::open(file_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(_) => Ok(contents),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

/// Change the current directory
pub fn change_dir(dir_name: &PathBuf) -> Result<bool, Error> {
    match std::env::set_current_dir(dir_name) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Make a directory
pub fn make_dir(dir_name: &PathBuf) -> Result<bool, Error> {
    match std::fs::create_dir(dir_name) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Delete a directory
pub fn delete_dir(dir_name: &PathBuf) -> Result<bool, Error> {
    match std::fs::remove_dir_all(dir_name) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}

/// Rename a file
pub fn rename_file(old_name: &PathBuf, new_name: &PathBuf) -> Result<bool, Error> {
    match std::fs::rename(old_name, new_name) {
        Ok(_) => Ok(true),
        Err(e) => Err(e),
    }
}
/// Copy a file
pub fn copy_file(from_path:&PathBuf,to_path:&PathBuf,progress_sender: mpsc::Sender<u64>)->io::Result<()> {
    let mut from_file = match File::open(from_path){
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };
    let mut to_file = match File::create(to_path){
        Ok(file)=>file,
        Err(e)=>return Err(e),
    };
    //let mut buffer = [0; 1024];
    // Set buffer size to 128 KB
    let mut buffer = [0; 128 * 1024];
    let mut total_bytes=0;
    loop {
        let bytes_read = match from_file.read(&mut buffer){
            Ok(n)=>n,
            Err(e)=>return Err(e),
        };
        if bytes_read == 0 {
            break;
        }
        match to_file.write_all(&buffer[..bytes_read]) {
            Ok(_) => {
                total_bytes+=bytes_read;
                match progress_sender.send(total_bytes as u64){
                    Ok(_)=>{},
                    Err(_)=>{
                        return Err(io::Error::new(io::ErrorKind::Other,"Progress sender failed"));
                    },
                }
            }
            Err(e) => return Err(e),
        }
    }
    match to_file.flush(){
        Ok(_)=>{Ok(())},
        Err(e)=>return Err(e),
    }
}