use crate::command_cd;

use super::parser::*;
use std::env;

pub fn execute(cmd: CommandEnum) -> bool {
    match cmd {
        CommandEnum::Pwd => {
            if let Ok(dir) = env::current_dir() {
                println!("{}", dir.display());
            }
        }

        CommandEnum::Mkdir(dir) => {
            if let Err(e) = std::fs::create_dir(&dir) {
                println!("mkdir: {}", e);
            }
        }

        CommandEnum::Cd(path) =>{
            command_cd(path);
        }

        CommandEnum::Echo(args) => {
            println!("{}", args.join(" "));
        }

        CommandEnum::Exit => return false,

        CommandEnum::Unknown(cmd) => {
            println!("command not found: {}", cmd);
        }
    }
    true
}
