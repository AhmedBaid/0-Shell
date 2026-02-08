use crate::commands::{
    cat::cat, cd::command_cd, cp::*, echo::*, ls::ls, pwd_state::PwdState, rm::rm,
};

use super::parser::*;

pub fn execute(cmd: CommandEnum, pwd_state: &mut PwdState) -> bool {
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
            println!("hnaya drtha b pwd: {:?}", pwd_state);

            eprintln!("{}", pwd_state.get_current_dir());
        }

        CommandEnum::Mkdir(dir) => {
            for d in dir {
                if let Err(e) = std::fs::create_dir(&d) {
                    eprintln!("mkdir: cannot create directory '{}': {}", d, e);
                }
            }
        }

        CommandEnum::Cd(path) => {
            command_cd(path, pwd_state);
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
