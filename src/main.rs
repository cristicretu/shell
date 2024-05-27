#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

const COMMANDS: &[&str] = &["exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        let program = input.split_whitespace().next().unwrap();
        let arguments = input.split_whitespace().skip(1).collect::<Vec<&str>>();

        if COMMANDS.contains(&program) {
            match program {
                "exit" => exit(arguments.get(0).and_then(|x| x.parse().ok()).unwrap_or(0)),
                _ => unreachable!(),
            }
        } else {
            println!("{}: command not found", program)
        }
    }
}

