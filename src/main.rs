#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

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

        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type ") {
            let cmd = &command[5..];
            if cmd == "echo" || cmd == "exit" || cmd == "type" {
                println!("{} is a shell builtin", cmd);
            } 
            else if let Some(path) = find_in_path(cmd) {
                println!("{} is {}", cmd, path.display());
            }
            else {
                println!("{}: not found", cmd);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}

