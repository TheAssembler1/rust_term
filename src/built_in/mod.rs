use super::path;
use std::{env, ops::Add};

pub fn cd(current_dir: String, args: Vec<String>) {
    // NOTE: should be path.. FUCK
    let path = args.get(1).expect("unable to get second arg");
    let dest_path = current_dir.add("\\").add(path);
    env::set_current_dir(dest_path).unwrap();
}

pub fn ls(current_dir: &str) {
    let entries = path::get_files_in_dir(current_dir);
    for entry in entries {
       if entry.metadata().unwrap().is_dir() {
           println!("[{}]", entry.file_name().to_str().unwrap());
       } else {
           println!("{}", entry.file_name().to_str().unwrap()); 
       }
    } 
}
