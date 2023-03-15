use std::io::{self, Write};

mod built_in;
mod path;

fn main() {
    loop {
        let input = get_user_input(&path::get_user_current_dir());

        // NOTE: check for built_in command
        if input.eq("ls") {
            path::get_files_in_dir(&path::get_user_current_dir());
        } else {
            eprintln!("unknown command");
        }

        // NOTE: check for external PATH exe
    }
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
