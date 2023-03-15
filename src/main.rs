use std::io::{self, Write};
use std::env;

const INTERNAL_PATH: &str = "bin";
const USER_PRECOMMAND: &str = "$ ";

fn main() {
    built_in::test::test();
    let paths = get_path();

    for path in paths {
        println!("{}", path);
    }

    loop {
        get_user_input(&get_user_current_dir());
    }
}

fn get_path() -> Vec<String> {
    let path = match env::var("PATH") {
        Ok(val) => val,
        Err(_) => panic!("unable to get PATH") 
    };

    let mut paths: Vec<String> = Vec::new();

    for path in path.split(";").into_iter() {
        paths.push(String::from(path));
    }

    return paths;
}

fn get_user_current_dir() -> String {
    match env::current_dir() {
        Ok(res) => res.into_os_string().into_string().unwrap(),
        Err(_) => panic!("unable to get the current dir"),
    }
}

fn get_user_input(current_dir: &str) -> String {
    loop {
        print!("{}{}", current_dir, USER_PRECOMMAND); 
        io::stdout().flush().unwrap();

        let mut input: String = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(res) => res,
            Err(_) => continue,
        };
        input.pop();

        return input;
    }
}
