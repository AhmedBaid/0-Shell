use std::{env, io::ErrorKind};

pub fn command_cd(path: String) {
    match env::set_current_dir(&path) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                println!("cd: No such file or directory: {}", path);
            }
            ErrorKind::PermissionDenied => {
                println!("cd: Permission denied: {}", path);
            }
            ErrorKind::NotADirectory => {
                println!("cd: Not a directory: {}", path);
            }
            _ => {
                println!("cd: error: {}", path);
            }
        },
    }
}
