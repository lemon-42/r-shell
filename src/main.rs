mod commands;
use commands::Commands;

use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin()
            .read_line(&mut user_input)
            .expect("Unable to read user input.");

        let trimmed_input = user_input.trim();
        match Commands::parse(trimmed_input) {
            Ok(command) => {
                let args: Vec<String> = trimmed_input
                    .split_whitespace()
                    .skip(1)
                    .map(String::from)
                    .collect();
                command.execute(&args);
            }
            Err(_) => {
                eprintln!("Error: Command not implemented yet.")
            }
        }
    }
}
