use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;
use std::process::Command;

// const COMMANDS: &[&str] = &["exit", "echo", "type"];
const BUILTINS: &[&str] = &["exit", "echo", "type"];

fn main() {
    let path = env::var("PATH").unwrap();
    let mut commands: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    for path in path.split(':') {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    if let Some(command) = entry.file_name().to_str() {
                        // commands.insert(command.to_string(), path.to_string()); // make sure to keep only the first one
                        commands
                            .entry(command.to_string())
                            .or_insert(path.to_string());
                    }
                }
            }
        }
    }

    for builtin in BUILTINS {
        commands.insert(builtin.to_string(), "builtin".to_string());
    }
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let input = input.trim();

        let program = input.split_whitespace().next().unwrap();
        let arguments = input.split_whitespace().skip(1).collect::<Vec<&str>>();

        if commands.contains_key(program)
            || BUILTINS.contains(&program)
            || program.contains("/")
            || program.contains(".")
        {
            // if it has a slash, it's a path
            if program.contains("/") {
                let mut child = Command::new(program)
                    .args(arguments)
                    .spawn()
                    .expect("failed to exec child");

                child.wait().expect("failed to wait on child");
                continue;
            }

            match program {
                "exit" => exit(arguments.get(0).and_then(|x| x.parse().ok()).unwrap_or(0)),
                "echo" => {
                    let to_print: String = arguments.join(" ");
                    println!("{}", to_print);
                }
                "type" => {
                    if !arguments.is_empty() {
                        let cmd = arguments[0];
                        if BUILTINS.contains(&cmd) {
                            println!("{} is a shell builtin", cmd);
                        } else if let Some(path) = commands.get(cmd) {
                            println!("{}/{}", path, cmd);
                        } else {
                            println!("{}: not found", cmd);
                        }
                    } else {
                        println!("Usage: type [command]");
                    }
                }
                _ => {
                    // println!("{}/{}", commands[program], program);
                    // println!("{:?}", arguments);

                    let child_args = if arguments.len() > 0 {
                        arguments
                    } else {
                        vec![]
                    };
                    let mut child = Command::new(format!("{}/{}", commands[program], program))
                        .args(child_args)
                        .spawn()
                        .expect("failed to exec child");

                    child.wait().expect("failed to wait on child");
                }
            }
        } else {
            println!("{}: command not found", program);
        }
    }
}

