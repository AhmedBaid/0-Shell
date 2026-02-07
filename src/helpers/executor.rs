use crate::commands::{cat::cat, cd::command_cd, cp::*, echo::*, ls::ls, rm::rm};

use super::parser::*;
use std::env;

pub fn execute(cmd: CommandEnum) -> bool {
    match cmd {
        CommandEnum::Ls(c) => ls(c),
        CommandEnum::Rm(c) => {
            if c.is_empty() {
                println!("rm: missing operand");
            } else {
                rm(c);
            }
        }
        CommandEnum::Cat(c) => cat(c),
        CommandEnum::Cp(c) => {
            if c.len() != 2 {
                println!("cp: missing file operand");
            } else {
                cp(c);
            }
        }
        CommandEnum::Pwd => {
            if let Ok(dir) = env::current_dir() {
                eprintln!("{}", dir.display());
            }
        }

        CommandEnum::Mkdir(dir) => {
            for d in dir {
                if let Err(e) = std::fs::create_dir(&d) {
                    eprintln!("mkdir: cannot create directory '{}': {}", d, e);
                }
            }
        }

        CommandEnum::Cd(path) => {
            command_cd(path);
        }

        CommandEnum::Echo(args) => {
            echo(args);
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
