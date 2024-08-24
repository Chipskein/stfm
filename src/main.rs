mod files;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

/// Implement simple CLI to manage simple file operations
#[derive(Parser,Debug)]
#[clap(name = "stfm")]
struct App{
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug, Subcommand)]
enum Command {
    /// Read a file usage :`stfm read <path>`
    Read {
        /// The path to read from
        path: PathBuf,
    },
    /// List all files in a directory usage: `stfm list <path>`
    List {
        /// The path to list all files
        path: PathBuf,
    },
    /// Create a file usage: `stfm create <path>`
    Create {
        /// The path to create a file
        path: PathBuf,
    },
    /// Rename a file usage: `stfm rename <old_path> <new_path>`
    Rename {
        /// The path to rename
        old_path: PathBuf,
        /// The path to new name
        new_path: PathBuf,
    },
    /// Move a file usage: `stfm move <old_path> <new_path>`
    Move {
        /// The path to move
        old_path: PathBuf,
        /// The path to new location
        new_path: PathBuf,
    },
    /// Copy a file usage: `stfm copy <old_path> <new_path>`
    Copy {
        /// The path to copy
        old_path: PathBuf,
        /// The path to new location
        new_path: PathBuf,
    },
    /// Delete a file usage: `stfm delete <path>`
    Delete {
        /// The path to delete
        path: PathBuf,
        /// Delete recursively if the path is a directory
        #[clap(short, long, default_value = "false")]
        recursive: bool,
    },
}


fn main (){
    let args = App::parse();
    match args.command {
        Command::Read { path } => {
            let result = files::read_file(&path.to_str().unwrap());
            println!("{}", result);
        }
        Command::List { path } => {
            let result = files::list_files(&path.to_str().unwrap());
            println!("{:?}", result);
        }
        Command::Create { path } => {
            files::create_file(&path.to_str().unwrap());
        }
        Command::Rename { old_path, new_path } => {
            files::rename_file(&old_path.to_str().unwrap(), &new_path.to_str().unwrap());
        }
        Command::Move { old_path, new_path } => {
            files::rename_file(&old_path.to_str().unwrap(), &new_path.to_str().unwrap());
        }
        Command::Copy { old_path, new_path } => {
            files::copy_file(&old_path.to_str().unwrap(), &new_path.to_str().unwrap());
        }
        Command::Delete { path, recursive } => {
            if recursive {
                files::delete_dir(&path.to_str().unwrap());
            } else {
                files::delete_file(&path.to_str().unwrap());
            }
        }
        
    }
}
