use crate::{
    command_cd,
    commands::{cat::cat, cp::*, echo::*},
};

use super::parser::*;
use std::env;

pub fn execute(cmd: CommandEnum) -> bool {
    match cmd {
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
                println!("{}", dir.display());
            }
        }

        CommandEnum::Mkdir(dir) => {
            if let Err(e) = std::fs::create_dir(&dir) {
                println!("mkdir: {}", e);
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
            println!("command not found: {}", cmd);
        }
    }
    true
}
