#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

const COMMANDS: &[&str] = &["exit", "echo", "type"];

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
                "echo" => {
                    let to_print: String = arguments.join(" ");
                    println!("{}", to_print);
                }
                "type" => {
                    if !arguments.is_empty() {
                        let cmd = arguments[0];
                        if COMMANDS.contains(&cmd) {
                            println!("{} is a shell builtin", cmd);
                        } else {
                            println!("{} not found", cmd);
                        }
                    } else {
                        println!("Usage: type [command]");
                    }
                }
                _ => unreachable!(),
            }
        } else {
            println!("{}: command not found", program)
        }
    }
}

