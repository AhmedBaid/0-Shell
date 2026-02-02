use super::parser::*;
use std::env;

pub fn execute(cmd: Command) -> bool {
    match cmd {
        Command::Pwd => {
            if let Ok(dir) = env::current_dir() {
                println!("{}", dir.display());
            }
        }

        Command::Mkdir(dir) => {
            if let Err(e) = std::fs::create_dir(&dir) {
                println!("mkdir: {}", e);
            }
        }

        Command::Cd(path) => {
            if let Err(e) = env::set_current_dir(&path) {
                println!("cd: {}", e);
            }
        }

        Command::Echo(args) => {
            println!("{}", args.join(" "));
        }

        Command::Exit => return false,

        Command::Unknown(cmd) => {
            println!("command not found: {}", cmd);
        }

    }
    true
}
