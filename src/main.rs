#[allow(unused_imports)]
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::{exit, Command};

// const COMMANDS: &[&str] = &["exit", "echo", "type"];
const BUILTINS: &[&str] = &["exit", "echo", "type"];

fn build_commands_map(path: &str) -> HashMap<String, String> {
    let mut commands = HashMap::new();
    for path in path.split(':') {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                if let Some(command) = entry.file_name().to_str() {
                    commands
                        .entry(command.to_string())
                        .or_insert_with(|| path.to_string());
                }
            }
        }
    }
    commands
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_default();
    input
}

fn parse_input(input: &str) -> (&str, Vec<&str>) {
    let mut parts = input.split_whitespace();
    let program = parts.next().unwrap_or("");
    let arguments = parts.collect();
    (program, arguments)
}

fn execute_builtin(program: &str, arguments: &[&str]) {
    match program {
        "exit" => exit(arguments.get(0).and_then(|x| x.parse().ok()).unwrap_or(0)),
        "echo" => println!("{}", arguments.join(" ")),
        "type" => {
            if let Some(cmd) = arguments.get(0) {
                if BUILTINS.contains(cmd) {
                    println!("{} is a shell builtin", cmd);
                } else {
                    println!("{}: not found", cmd);
                }
            } else {
                println!("Usage: type [command]");
            }
        }
        _ => unreachable!(),
    }
}

fn execute_command(path: &str, program: &str, arguments: &[&str]) {
    let command_path = if program.contains('/') {
        program.to_string()
    } else {
        format!("{}/{}", path, program)
    };

    match Command::new(command_path).args(arguments).spawn() {
        Ok(mut child) => {
            child.wait().expect("failed to wait on child");
        }
        Err(e) => {
            println!("Error executing {}: {}", program, e);
        }
    }
}

fn main() {
    let path = env::var("PATH").unwrap_or_default();
    let mut commands = build_commands_map(&path);

    for builtin in BUILTINS {
        commands.insert(builtin.to_string(), "builtin".to_string());
    }

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let input = read_input().trim().to_string();
        if input.is_empty() {
            continue;
        }

        let (program, arguments) = parse_input(&input);
        match commands.get(program).map(String::as_str) {
            Some("builtin") => execute_builtin(program, &arguments),
            Some(path) => execute_command(path, program, &arguments),
            None => println!("{}: command not found", program),
        }
    }
}
