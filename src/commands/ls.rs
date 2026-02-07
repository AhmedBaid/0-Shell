use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::time::SystemTime;
use std::{fs, path::Path};
use chrono::{DateTime, Local};

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

    l(files, dirs, errors, flag);
}

fn l(files: Vec<String>, dirs: Vec<String>, errors: Vec<String>, flag: Flag) {
    for err in &errors {
        println!("ls: cannot access '{}': No such file or directory", err);
    }

    for file_path in &files {
        let path = Path::new(file_path);
        if flag.l {
            if let Ok(m) = fs::symlink_metadata(path) {
                let name = path.file_name().unwrap().to_string_lossy().to_string();
                print!("{}", format_long_item(name, &m, flag));
            }
        } else {
            let mut name = file_path.clone();
            if flag.f {
                if let Ok(m) = fs::symlink_metadata(path) {
                   if m.permissions().mode() & 0o111 != 0 { name.push('*'); }
                }
            }
            println!("{}", name);
        }
    }

    let show_headers = !files.is_empty() || dirs.len() > 1 || !errors.is_empty();

    for (i, path_str) in dirs.iter().enumerate() {
        if i > 0 || !files.is_empty() {
            println!();
        }

        if show_headers {
            println!("{}:", path_str);
        }

        match (flag.a, flag.l, flag.f) {
            (false, false, false) => {
                let r = get_dir_content(path_str, false).join(" ");
                if !r.is_empty() { println!("{r}"); }
            }
            (true, false, false) => {
                let r = get_dir_content(path_str, true).join(" ");
                if !r.is_empty() { println!("{r}"); }
            }
            (false, true, _) | (true, true, _) => {
                print!("{}", run_ls_l(path_str, flag));
            }
            (false, false, true) => {
                let r = get_dir_content(path_str, false);
                println!("{}", add_symbols(r, path_str));
            }
            (true, false, true) => {
                let r = get_dir_content(path_str, true);
                println!("{}", add_symbols(r, path_str));
            }
        }
    }
}


fn run_ls_l(path: &str, flag: Flag) -> String {
    let mut s = String::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let name = entry.file_name().to_string_lossy().to_string();

                if !flag.a && name.starts_with('.') {
                    continue;
                }

                if let Ok(metadata) = entry.metadata() {
                    s.push_str(&format_long_item(name, &metadata, flag));
                }
            }
        }
    }
    s
}

fn format_long_item(mut name: String, metadata: &fs::Metadata, flag: Flag) -> String {
    if flag.f {
        if metadata.is_dir() { name.push('/'); }
        else if metadata.is_symlink() { name.push('@'); }
        else if metadata.file_type().is_fifo() { name.push('|'); }
        else if metadata.file_type().is_socket() { name.push('='); }
        else if (metadata.permissions().mode() & 0o111) != 0 { name.push('*'); }
    }

    let type_char = if metadata.is_dir() { 'd' } 
                    else if metadata.is_symlink() { 'l' } 
                    else { '-' };

    let mode = metadata.permissions().mode();
    let perms = format_permissions(mode);
    let nlink = metadata.nlink();
    let uid = metadata.uid();
    let gid = metadata.gid();
    let size = metadata.len();
    let modified = metadata.modified().unwrap_or(SystemTime::now());
    let date_str = format_date(modified);

    format!("{}{} {:>3} {:>5} {:>5} {:>8} {} {}\n",
        type_char, perms, nlink, uid, gid, size, date_str, name
    )
}

fn get_dir_content(path: &str, show_hidden: bool) -> Vec<String> {
    let mut filenames = Vec::new();
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name();
                    if let Ok(name_str) = name.into_string() {
                        if !show_hidden && name_str.starts_with('.') {
                            continue;
                        }
                        filenames.push(name_str);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("ls: cannot access '{}': {}", path, e);
        }
    }
    filenames.sort();
    filenames
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

fn add_symbols(paths: Vec<String>, base: &str) -> String {
    let mut result = Vec::new();
    for mut path in paths {
        let full_path = std::path::Path::new(base).join(&path);

        if let Ok(metadata) = fs::symlink_metadata(&full_path) {
            let file_type = metadata.file_type();

            if file_type.is_dir() {
                path.push('/');
            } else if file_type.is_symlink() {
                path.push('@');
            } else if file_type.is_fifo() {
                path.push('|');
            } else if file_type.is_socket() {
                path.push('=');
            } else if (metadata.permissions().mode() & 0o111) != 0 {
                path.push('*');
            }
        }
        result.push(path);
    }
    result.join(" ")
}

fn format_permissions(mode: u32) -> String {
    let mut s = String::new();
    s.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o100) != 0 { 'x' } else { '-' });
    s.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o010) != 0 { 'x' } else { '-' });
    s.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    s.push(if (mode & 0o001) != 0 { 'x' } else { '-' });
    s
}

fn format_date(time: SystemTime) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format("%b %d %H:%M").to_string()
}