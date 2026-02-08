use std::{env, io::ErrorKind, path::PathBuf};
use crate::commands::pwd_state::PwdState;

pub fn command_cd(args: Vec<String>, pwd_state: &mut PwdState) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    let target_dir = if args.is_empty() {
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0] == "-" {
        PathBuf::from(pwd_state.get_old_dir())
    } else if args[0] == "~" {
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else {
        PathBuf::from(&args[0])
    };

    let current_before_move = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cd: failed to get current directory: {}", e);
            return;
        }
    };

    match env::set_current_dir(&target_dir) {
        Ok(_) => {
            if let Ok(new_current) = env::current_dir() {
                pwd_state.set_states(
                    new_current.display().to_string(), 
                    current_before_move.display().to_string()
                );

                if !args.is_empty() && args[0] == "-" {
                    println!("{}", pwd_state.get_current_dir());
                }
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => eprintln!("cd: {}: No such file or directory", target_dir.display()),
            ErrorKind::PermissionDenied => eprintln!("cd: {}: Permission denied", target_dir.display()),
            ErrorKind::NotADirectory => eprintln!("cd: {}: Not a directory", target_dir.display()),
            _ => eprintln!("cd: {}: {}", target_dir.display(), e),
        },
    }
}