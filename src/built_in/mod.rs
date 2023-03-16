use super::path;
use std::{env, ops::Add};

pub fn cd(current_dir: String, args: Vec<String>) {
    let dest_path;
    // NOTE: should be path.. FUCK
    if args.len() > 1 {
        let path = match args.get(1) {
            Some(arg) => arg,
            _ => {
                eprintln!("invalid path");
                return;
            }
        };

        dest_path = current_dir.add("\\").add(path);
    } else {
        dest_path = String::from(env::home_dir().unwrap().to_str().unwrap());
    }
    if let Err(_) = env::set_current_dir(dest_path) {
        eprintln!("invalid path");
    }
}

pub fn ls(current_dir: &str) {
    let entries = path::get_files_in_dir(current_dir);
    for entry in entries {
       print!("{:?}", entry.metadata().unwrap().permissions().);
       if entry.metadata().unwrap().is_dir() {
           println!("[{}]", entry.file_name().to_str().unwrap());
       } else {
           println!("{}", entry.file_name().to_str().unwrap()); 
       }
    } 
}
