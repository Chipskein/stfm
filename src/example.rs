//mod files;
//use std::path::Path;
/*
    let current_dir=std::env::current_dir().unwrap();
    println!("{}",test.display());
    let path=test.to_str().unwrap();
    println!("{}",path);
    let t=files::file_exists(&path);
    println!("{}",t);
    let path_file=Path::new("test.txt").to_str().unwrap();
    let result=files::read_file(path_file);
    println!("{}",result);
*/
use clap::Parser;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser,Debug)]
struct Cli{
    /// The pattern to look for
    pattern:String,
    /// The path to the file to read
    path:std::path::PathBuf,
}

fn teste()  {
    let args = Cli::parse();
    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);
    let result = File::open(&args.path);
    let file= match result {
        Ok(file) => { file },
        Err(error) => { panic!("Can't deal with {}, just exit here", error);  }
    };
    let reader = BufReader::new(file);
    reader.lines().for_each(|line| {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    });

}
