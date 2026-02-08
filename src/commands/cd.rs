use std::{env, io::ErrorKind, path::PathBuf};
use crate::commands::pwd_state::PwdState;

pub fn command_cd(args: Vec<String>, pwd_state: &mut PwdState) {
    if args.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }

    // 1. Determine where we want to go
    let target_dir = if args.is_empty() {
        // Case: cd (go home)
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else if args[0] == "-" {
        // Case: cd - (go to old_dir)
        PathBuf::from(pwd_state.get_old_dir())
    } else if args[0] == "~" {
        // Case: cd ~ (go home)
        match env::var("HOME") {
            Ok(path) => PathBuf::from(path),
            Err(_) => {
                eprintln!("cd: HOME environment variable not set");
                return;
            }
        }
    } else {
        // Case: cd path
        PathBuf::from(&args[0])
    };

    // 2. Capture current directory BEFORE changing (to save as old_dir)
    let current_before_move = match env::current_dir() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("cd: failed to get current directory: {}", e);
            return;
        }
    };

    // 3. Attempt to change directory
    match env::set_current_dir(&target_dir) {
        Ok(_) => {
            // 4. Get the new canonical path (resolves .. and symlinks)
            if let Ok(new_current) = env::current_dir() {
                // UPDATE STATE:
                // Old -> Where we were (current_before_move)
                // New -> Where we are now (new_current)
                pwd_state.set_states(
                    new_current.display().to_string(), 
                    current_before_move.display().to_string()
                );

                // If we did 'cd -', print the directory we jumped to (standard shell behavior)
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