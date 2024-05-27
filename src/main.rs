#[allow(unused_imports)]
use std::io::{self, Write};

const COMMANDS: &[&str] = &["help", "exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        if COMMANDS.contains(&input) {
            continue;
        } else {
            println!("{}: command not found", input);
        }
    }
}

