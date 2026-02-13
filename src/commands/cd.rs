use crate::commands::pwd_state::PwdState;
use std::{env, io::ErrorKind, path::PathBuf};

pub fn command_cd(
    error_path: Vec<String>,
    mut args: Vec<String>,
    pwd_state: &mut PwdState,
) -> bool {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return false;
    }
    if args.len() == 1 {
        args[0] = args[0].replace("\\n", "\n");
    }
    let target_dir = if args.is_empty() {
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return false;
            }
        }
    } else if args[0] == "-" {
        PathBuf::from(pwd_state.get_old_dir())
    } else if args[0] == "~" {
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return false;
            }
        }
    } else {
        PathBuf::from(&args[0])
    };

    let current_before_move = pwd_state.get_current_dir();

    match env::set_current_dir(&target_dir) {
        Ok(_) => {
            if let Ok(new_current) = env::current_dir() {
                pwd_state.set_states(new_current.display().to_string(), current_before_move);

                if !args.is_empty() && args[0] == "-" {
                    println!("{}", pwd_state.get_current_dir());
                }
            } else {
                pwd_state.set_states(
                    PathBuf::from(".").display().to_string(),
                    current_before_move,
                );
            }
            return true;
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("cd:  No such file or directory : {}", error_path[0]);
                return false;
            }
            ErrorKind::PermissionDenied => {
                eprintln!("cd: Permission denied : {}", error_path[0]);
                return false;
            }
            ErrorKind::NotADirectory => {
                eprintln!("cd: Not a directory : {}", error_path[0]);
                return false;
            }
            _ => {
                eprintln!("cd: {}: {}", error_path[0], e);
                return false;
            }
        },
    }
}
