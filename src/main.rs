#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?;
    for dir in env::split_paths(&path_var) {
        let candidate = dir.join(cmd);
        if let Ok(metadata) = fs::metadata(&candidate) {
            let is_file = metadata.is_file();
            let is_executable = metadata.permissions().mode() & 0o111 != 0;
            if is_file && is_executable {
                return Some(candidate);
            }
        }
    }
    None
}

fn main() {
    loop {
        // TODO: Uncomment the code below to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();
        

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        //Prints the "<command> : command not found" message to the console
        let command = command.trim();

        if command.is_empty() {
            continue;
        }

        let tokens: Vec<&str> = command.split_whitespace().collect();
        let command = tokens[0];
        let args = &tokens[1..];

        
        if command == "exit" {
            break;
        } else if command.starts_with("echo ") {
            println!("{}", args.join(" "));
        } else if command.starts_with("type ") {
            let target = args.get(0).copied().unwrap_or("");
            if target == "echo" || target == "exit" || target == "type" {
                println!("{} is a shell builtin", target);
            } 
            else if let Some(path) = find_in_path(target) {
                println!("{} is {}", target, path.display());
            }
            else {
                println!("{}: not found", target);
            }
        } else if let Some(path) = find_in_path(command) {
            let status = Command::new(&path)
                .arg0(command)
                .args(args)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();
            if let Err(e) = status {
                eprintln!("{}: failed to execute: {}", command, e);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}

