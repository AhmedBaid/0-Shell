use crate::commands::{
    cat::cat, cd::command_cd, cp::*, echo::*, exit::exit, ls::ls, mv::mv, pwd_state::PwdState,
    rm::rm,
};

use super::parser::*;

pub fn execute(cmd: CommandEnum, pwd_state: &mut PwdState) -> bool {
    let succes = match cmd {
        CommandEnum::Mv(c) => mv(c),
        CommandEnum::Ls(c) => ls(c),
        CommandEnum::Rm(c) => {
            if c.is_empty() {
                println!("rm: missing operand");
                return false;
            } else {
                return rm(c);
            }
        }
        CommandEnum::Cat(c) => cat(c),
        CommandEnum::Cp(c) => {
            return cp(c);
        }
        CommandEnum::Pwd => {
            let pwd = pwd_state.get_current_dir().replace("\n", "\\n");
            println!("{}", pwd);
            return true;
        }

        CommandEnum::Mkdir(dir, error_dir) => {
            if dir.is_empty() {
                println!("mkdir: missing operand");
                return false;
            }
            let mut count = 0;
            for d in dir {
                count += 1;
                if let Err(e) = std::fs::create_dir(&d) {
                    eprintln!(
                        "mkdir: cannot create directory '{}': {}",
                        error_dir[count - 1],
                        e
                    );
                    return false;
                }
            }
            return true;
        }

        CommandEnum::Cd(path, error_path) => {
            return command_cd(path, error_path, pwd_state);
        }

        CommandEnum::Echo(args) => {
            echo(args);
            return true;
        }

        CommandEnum::Exit => {
            exit();
            return true;
        }

        CommandEnum::Unknown(cmd) => {
            eprintln!("command not found: {}", cmd.replace("\n", "\\n"));
            return false;
        }
    };
    succes
}
