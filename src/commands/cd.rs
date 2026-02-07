use std::{env, io::ErrorKind};

pub fn command_cd(path: Vec<String>) {
    if path.len() > 1 {
        eprintln!("cd: too many arguments");
        return;
    }
    if path[0] == "-" && path.len() == 1 {
        let oldpwd = match env::var("OLDPWD") {
            Ok(val) => val,
            Err(_) => {
                eprintln!("cd: OLDPWD not set");
                return;
            }
        };
        match env::set_current_dir(&oldpwd) {
            Ok(_) => {
                println!("{}", oldpwd);
            }
            Err(e) => match e.kind() {
                ErrorKind::NotFound => {
                    eprintln!("cd: No such file or directory: {}", oldpwd);
                }
                ErrorKind::PermissionDenied => {
                    eprintln!("cd: Permission denied: {}", oldpwd);
                }
                ErrorKind::NotADirectory => {
                    eprintln!("cd: Not a directory: {}", oldpwd);
                }
                _ => {
                    eprintln!("cd: error: {}", oldpwd);
                }
            },
        }
        return;
    }
    if path.is_empty() || path[0] == "~" {
        if let Some(home) = env::var_os("HOME") {
            match env::set_current_dir(&home) {
                Ok(_) => {}
                Err(e) => match e.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("cd: No such file or directory: {}", "home");
                    }
                    ErrorKind::PermissionDenied => {
                        eprintln!("cd: Permission denied: {}", "home");
                    }
                    ErrorKind::NotADirectory => {
                        eprintln!("cd: Not a directory: {}", "home");
                    }
                    _ => {
                        eprintln!("cd: error: {}", "home");
                    }
                },
            }
        } else {
            eprintln!("cd: HOME environment variable not set");
        }
        return;
    }
    let arg = path[0].clone();
    match env::set_current_dir(&arg) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                eprintln!("cd: No such file or directory: {}", arg);
            }
            ErrorKind::PermissionDenied => {
                eprintln!("cd: Permission denied: {}", arg);
            }
            ErrorKind::NotADirectory => {
                eprintln!("cd: Not a directory: {}", arg);
            }
            _ => {
                eprintln!("cd: error: {}", arg);
            }
        },
    }
}
