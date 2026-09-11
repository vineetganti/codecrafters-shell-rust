#[allow(unused_imports)]
use std::io::{self, Write};

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
            } else {
                println!("{}:not found", cmd);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}

