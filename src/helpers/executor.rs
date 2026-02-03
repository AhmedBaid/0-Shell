use crate::{ command_cd, commands::cp::Cp };

use super::parser::*;
use std::env;

pub fn execute(cmd: Command) -> bool {
    match cmd {
        Command::Cp(c) => if c.len() != 2 {
            println!("cp: missing file operand");
        } else {
            Cp(c);
        }
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
            command_cd(path);
        }

        Command::Echo(args) => {
            println!("{}", args.join(" "));
        }

        Command::Exit => {
            return false;
        }

        Command::Unknown(cmd) => {
            println!("command not found: {}", cmd);
        }
    }
    true
}
