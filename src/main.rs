#[allow(unused_imports)] // suppress warnings for imports only used on some code paths
use std::io::{self, Write}; // io for stdin/stdout, Write for flush()
use std::env; // for reading env vars (PATH) and splitting them
use std::fs; // for reading file metadata (to check executability)
use std::os::unix::fs::PermissionsExt; // adds .mode() to Permissions (Unix-only)
use std::os::unix::process::CommandExt; // adds .arg0() to Command (Unix-only)
use std::path::PathBuf; // owned, growable path type
use std::process::{Command, Stdio}; // for spawning external programs

// Searches each directory in $PATH for `cmd` and returns its full path
// if a matching, executable, regular file is found.
fn find_in_path(cmd: &str) -> Option<PathBuf> {
    let path_var = env::var("PATH").ok()?; // read PATH; bail out (None) if unset
    for dir in env::split_paths(&path_var) { // split PATH on the OS separator (':' on Unix)
        let candidate = dir.join(cmd); // build dir/cmd
        if let Ok(metadata) = fs::metadata(&candidate) { // stat the candidate path
            let is_file = metadata.is_file(); // must be a regular file, not a dir
            let is_executable = metadata.permissions().mode() & 0o111 != 0; // any exec bit set (owner/group/other)
            if is_file && is_executable {
                return Some(candidate); // found a usable match, stop searching
            }
        }
    }
    None // no directory in PATH had a matching executable
}

fn main() {
    loop { // read-eval-print loop: runs until "exit" is entered
        print!("$ "); // shell prompt, no trailing newline
        io::stdout().flush().unwrap(); // force the prompt to appear before we block on input


        let mut command = String::new(); // buffer to hold the raw input line
        io::stdin().read_line(&mut command).unwrap(); // blocks until user presses Enter
        let command = command.trim(); // drop the trailing newline (and surrounding whitespace)

        if command.is_empty() {
            continue; // user just hit Enter; re-prompt without doing anything
        }

        let tokens: Vec<&str> = command.split_whitespace().collect(); // naive whitespace tokenizer (no quoting support)
        let command = tokens[0]; // first token is the command name
        let args = &tokens[1..]; // remaining tokens are its arguments


        if command == "exit" {
            break; // exits the loop, ending the program
        } else if command == "echo" {
            println!("{}", args.join(" ")); // built-in: print args back out, space-separated
        } else if command == "type" {
            let target = args.get(0).copied().unwrap_or(""); // the command name `type` is being asked about
            if target == "echo" || target == "exit" || target == "type" || target == "pwd" {
                println!("{} is a shell builtin", target); // these are handled directly in this loop
            }
            else if let Some(path) = find_in_path(target) {
                println!("{} is {}", target, path.display()); // found as an external executable on PATH
            }
            else if command == "pwd" {
                match env::current_dir() { // built-in: print the current working directory
                    Ok(path) => println!("{}", path.display()), // display the path in a human-readable way
                    Err(e) => eprintln!("pwd: error retrieving current directory: {}", e), // e.g. permission denied, path not found
                }
            }
            else {
                println!("{}: not found", target); // not a builtin and not on PATH
            }
        } else if command == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: error retrieving current directory: {}", e),
            }
        }
         else if let Some(path) = find_in_path(command) {
            // command isn't a builtin, but an executable with this name exists on PATH
            let status = Command::new(&path)
                .arg0(command) // make argv[0] the typed name, not the full resolved path
                .args(args) // forward the rest of the user's arguments
                .stdin(Stdio::inherit()) // let the child share this process's stdin
                .stdout(Stdio::inherit()) // ...and stdout
                .stderr(Stdio::inherit()) // ...and stderr
                .status(); // spawn and block until the child exits
            if let Err(e) = status {
                eprintln!("{}: failed to execute: {}", command, e); // e.g. permission denied, exec format error
            }
        } else {
            println!("{}: command not found", command); // neither a builtin nor found on PATH
        }
    }
}
