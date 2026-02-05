use crate::{ command_cd, commands::cp::* };

use super::parser::*;
use std::env;

pub fn execute(cmd: CommandEnum) -> bool {
    match cmd {
        CommandEnum::Cp(c) => if c.len() != 2 {
            eprintln!("cp: missing file operand");
        } else {
            cp(c);
        }
        CommandEnum::Pwd => {
            if let Ok(dir) = env::current_dir() {
                eprintln!("{}", dir.display());
            }
        }

        CommandEnum::Mkdir(dir) => {
            if let Err(e) = std::fs::create_dir(&dir) {
                eprintln!("mkdir: {}", e);
            }
        }

        CommandEnum::Cd(path) => {
            command_cd(path);
        }

        CommandEnum::Echo(args) => {
            eprintln!("{}", args.join(" "));
        }

        CommandEnum::Exit => {
            return false;
        }

        CommandEnum::Unknown(cmd) => {
            eprintln!("command not found: {}", cmd);
        }
    }
    true
}
