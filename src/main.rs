#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        // TODO: Uncomment the code below to pass the first stage
        print!("$ ");
        io::stdout().flush().unwrap();
        
        // Handling invalid commands
        //Captures the user's command in the command variable
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        //Prints the "<command> : command not found" message to the console

        command = command.trim().to_string();

        if command.starts_with("echo ") {
            let output = command[5..].trim();
            println!("{}", output);
        }
        // Exit the loop if the user types "exit"
        if command.trim() == "exit" {
            break;
        }
        println!("{}: command not found", command.trim());
    }
}
