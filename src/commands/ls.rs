use chrono::{DateTime, Local};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::time::SystemTime;
use std::{fs, path::Path};

#[derive(Debug, Clone, Copy)]
pub struct Flag {
    pub a: bool,
    pub l: bool,
    pub f: bool,
}

pub fn ls(args: Vec<String>) {
    let mut flag = Flag {
        a: false,
        l: false,
        f: false,
    };

    let mut files = Vec::new();
    let mut dirs = Vec::new();
    let mut errors = Vec::new();

    let mut is_dir_marker = false;

    for arg in args {
        if arg == "--" {
            is_dir_marker = true;
            continue;
        }

        if arg.starts_with("-") && !is_dir_marker {
            if !is_flag(&arg, &mut flag) {
                println!("ls: unrecognized option '{arg}'");
                println!("Try 'ls --help' for more information.");
                return;
            }
            continue;
        }

        let path = Path::new(&arg);

        if path.exists() {
            if path.is_file() {
                files.push(arg);
            } else {
                dirs.push(arg);
            }
        } else {
            errors.push(arg);
        }
    }

    if files.is_empty() && dirs.is_empty() && errors.is_empty() {
        dirs.push(".".to_string());
    }
}

fn is_flag(arg: &String, flag: &mut Flag) -> bool {
    if arg.len() > 1 && arg[1..].chars().all(|c| "alF".contains(c)) {
        for c in arg[1..].chars() {
            match c {
                'a' => flag.a = true,
                'l' => flag.l = true,
                'F' => flag.f = true,
                _ => break,
            }
        }
        return true;
    }
    false
}
