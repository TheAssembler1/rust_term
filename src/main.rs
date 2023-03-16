use std::io::{self, Write};
use std::process::Command;

mod built_in;
mod path;

fn main() {
    let commands_map = path::load_commands();

    loop {
        io::stdout().flush().unwrap();
        let input = get_user_input(&path::get_user_current_dir());
        let args = parse_arguments(input);
        // NOTE: remove commands to just have args
        let command = args.get(0).unwrap();

        match command.as_str() {
           "ls" => built_in::ls(&path::get_user_current_dir()),
           "cd" => built_in::cd(path::get_user_current_dir(), args),
            command => {
                 if !commands_map.contains_key(command) {
                     eprintln!("unknown command");
                 } else {
                     let command_path = commands_map.get(command).unwrap();
                     Command::new(command_path).args(args.iter().skip(1))
                         .spawn()
                         .expect("Failed")
                         .wait()
                         .unwrap();
                 }
            },
        }
    }
}

fn parse_arguments(args: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for arg in args.split(" ").into_iter() {
        result.push(String::from(arg));
    }
    result
}

fn get_user_input(current_dir: &str) -> String {
    loop {
        print!("{}{}", current_dir, path::USER_PRECOMMAND);
        io::stdout().flush().unwrap();

        let mut input: String = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(res) => res,
            Err(_) => continue,
        };
        input.pop();
        input.pop();

        return input;
    }
}
