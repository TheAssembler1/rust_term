use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use is_executable::IsExecutable;
use std::collections::HashMap;

pub const INTERNAL_PATH: &str = "bin";
pub const USER_PRECOMMAND: &str = "$ ";

pub fn get_path_from_file(dir: &str, file: &str) -> String {
    let folder = String::from(dir) + "\\";
    folder + file
}

pub fn load_commands() -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    get_path().iter().for_each(|path| {
        get_files_in_dir(path).iter().for_each(|file| {
            let file_name = String::from(file.file_name().to_str().unwrap());
            let file_path = Path::new(&file_name);
            let path_ref = &get_path_from_file(path, &file_name);
            let full_path = Path::new(path_ref);
            if full_path.is_executable() {
                let command_name = file_path.file_stem().unwrap().to_str().unwrap();
                let command_path = full_path.to_str().unwrap();
                //FIXME: ensure no duplicate commands
                map.insert(String::from(command_name), String::from(command_path));
            }
        });
    });

    map
}

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

pub fn get_files_in_dir(dir: &str) -> Vec<DirEntry> {
    let mut result: Vec<DirEntry> = Vec::new();
    let path = Path::new(dir);
    if let Ok(folder) = fs::read_dir(path) {
        for path in folder {
            let file = match path {
                Ok(file_result) => file_result,
                Err(_) => continue,
            };

            result.push(file);
        }
    }

    result
}
