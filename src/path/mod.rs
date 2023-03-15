use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

pub const INTERNAL_PATH: &str = "bin";
pub const USER_PRECOMMAND: &str = "$ ";

pub fn get_path() -> Vec<String> {
    let path = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => panic!("unable to get PATH"),
    };

    let mut paths: Vec<String> = Vec::new();

    for path in path.split(";").into_iter() {
        paths.push(String::from(path));
    }

    return paths;
}

pub fn get_user_current_dir() -> String {
    match env::current_dir() {
        Ok(res) => res.into_os_string().into_string().unwrap(),
        Err(_) => panic!("unable to get the current dir"),
    }
}

pub fn get_files_in_dir(dir: &str) {
    let path = Path::new(dir);
    if let Ok(folder) = fs::read_dir(path) {
        for path in folder {
            let file = match path {
                Ok(file_result) => file_result,
                Err(_) => continue,
            };

            // FIXME: for now putting [FOLDER_NAME]

            if file.file_type().unwrap().is_dir() {
                print!("[{}] ", file.file_name().to_str().unwrap());
            } else {
                print!("{} ", file.file_name().to_str().unwrap());
            }
        }
    }
    println!();
}
