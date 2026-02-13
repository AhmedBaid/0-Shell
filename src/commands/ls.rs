use chrono::{DateTime, Duration, Local};
use std::cmp::max;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::time::SystemTime;
use std::{fs, path::Path};
use users::{get_group_by_gid, get_user_by_uid};

#[derive(Debug, Clone, Copy)]
pub struct Flag {
    pub a: bool,
    pub l: bool,
    pub f: bool,
}
struct LongEntry {
    perms: String,
    links: String,
    user: String,
    group: String,
    size: String,
    date: String,
    name: String,
    blocks: u64,
}

pub fn ls(args: Vec<String>) -> bool {
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
                return false;
            }
            continue;
        }

        let path = Path::new(&arg);

        if path.exists() || fs::symlink_metadata(path).is_ok() {
            if path.is_dir() {
                dirs.push(arg);
            } else {
                files.push(arg);
            }
        } else {
            errors.push(arg);
        }
    }

    if files.is_empty() && dirs.is_empty() && errors.is_empty() {
        dirs.push(".".to_string());
    }

    if !l(files, dirs, errors.clone(), flag) || !errors.is_empty() {
        return false;
    }

    true
}

fn l(files: Vec<String>, dirs: Vec<String>, errors: Vec<String>, flag: Flag) -> bool {
    for err in &errors {
        println!("ls: cannot access '{}': No such file or directory", err);
    }

    if !files.is_empty() {
        let mut file_entries = Vec::new();
        for file_path in &files {
            let path = Path::new(file_path);
            if let Ok(m) = fs::symlink_metadata(path) {
                let name = file_path.clone();

                if flag.l {
                    file_entries.push(prepare_long_entry(name, &m, flag, path));
                } else {
                    let mut display_name = file_path.clone();
                    if flag.f {
                        display_name = append_indicator(display_name, &m);
                    }
                    println!("{}", display_name);
                }
            }
        }
        if flag.l && !file_entries.is_empty() {
            print!("{}", align_and_format(file_entries, false));
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
                if let Ok(r) = get_dir_content(path_str, false) {
                    let r = r.join(" ");
                    if !r.is_empty() {
                        println!("{r}");
                    }
                } else {
                    return false;
                }
            }
            (true, false, false) => {
                if let Ok(r) = get_dir_content(path_str, true) {
                    let r = r.join(" ");
                    if !r.is_empty() {
                        println!("{r}");
                    }
                } else {
                    return false;
                }
            }
            (false, true, _) | (true, true, _) => {
                print!("{}", run_ls_l(path_str, flag));
            }
            (false, false, true) => {
                if let Ok(r) = get_dir_content(path_str, false) {
                    println!("{}", add_symbols(r, path_str));
                } else {
                    return false;
                }
            }
            (true, false, true) => {
                if let Ok(r) = get_dir_content(path_str, true) {
                    println!("{}", add_symbols(r, path_str));
                } else {
                    return false;
                }
            }
        }
    }
    true
}

fn run_ls_l(path: &str, flag: Flag) -> String {
    let mut entries = Vec::new();
    if flag.a {
        if let Ok(metadata) = fs::metadata(path) {
            entries.push(prepare_long_entry(
                ".".to_string(),
                &metadata,
                flag,
                Path::new(path),
            ));
        }

        let parent_path = Path::new(path).join("..");
        if let Ok(metadata) = fs::metadata(&parent_path) {
            entries.push(prepare_long_entry(
                "..".to_string(),
                &metadata,
                flag,
                &parent_path,
            ));
        }
    }
    if let Ok(read_dir) = fs::read_dir(path) {
        let mut dir_items: Vec<_> = read_dir.filter_map(Result::ok).collect();
        dir_items.sort_by(|a, b| {
            let name_a = a.file_name().to_string_lossy().to_string();
            let name_b = b.file_name().to_string_lossy().to_string();
            let clean_key = |s: &str| -> String {
                s.chars()
                    .filter(|c| c.is_alphanumeric()) // Keep only Letters and Numbers
                    .collect::<String>()
                    .to_lowercase()
            };

            let key_a = clean_key(&name_a);
            let key_b = clean_key(&name_b);

            let order = key_a.cmp(&key_b);

            if order == std::cmp::Ordering::Equal {
                name_a.cmp(&name_b)
            } else {
                order
            }
        });
        for entry in dir_items {
            let name = entry.file_name().to_string_lossy().to_string();

            if !flag.a && name.starts_with('.') {
                continue;
            }

            if let Ok(metadata) = entry.metadata() {
                entries.push(prepare_long_entry(name, &metadata, flag, &entry.path()));
            }
        }
    }

    align_and_format(entries, true)
}

fn get_dir_content(path: &str, show_hidden: bool) -> Result<Vec<String>, bool> {
    let mut filenames = Vec::new();
    if show_hidden {
        if let Ok(_) = fs::metadata(path) {
            filenames.push(".".to_string());
        }

        let parent_path = Path::new(path).join("..");
        if let Ok(_) = fs::metadata(&parent_path) {
            filenames.push("..".to_string());
        }
    }
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
            return Err(false);
        }
    }
    filenames.sort_by(|a, b| {
        let a_is_special = a == "." || a == "..";
        let b_is_special = b == "." || b == "..";

        if a_is_special && !b_is_special {
            return std::cmp::Ordering::Less;
        }
        if !a_is_special && b_is_special {
            return std::cmp::Ordering::Greater;
        }
        if a_is_special && b_is_special {
            return a.cmp(b);
        }

        let clean_a = a.trim_start_matches('.');
        let clean_b = b.trim_start_matches('.');

        clean_a.to_lowercase().cmp(&clean_b.to_lowercase())
    });
    Ok(filenames)
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

fn format_permissions(metadata: &fs::Metadata, file_path: &Path) -> String {
    let mode = metadata.permissions().mode();
    let mut s = String::with_capacity(11);

    if metadata.is_dir() {
        s.push('d');
    } else if metadata.is_symlink() {
        s.push('l');
    } else if metadata.file_type().is_char_device() {
        s.push('c');
    } else if metadata.file_type().is_block_device() {
        s.push('b');
    } else if metadata.file_type().is_fifo() {
        s.push('p');
    } else if metadata.file_type().is_socket() {
        s.push('s');
    } else {
        s.push('-');
    }

    s.push(if (mode & 0o400) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o200) != 0 { 'w' } else { '-' });
    if (mode & 0o4000) != 0 {
        s.push(if (mode & 0o100) != 0 { 's' } else { 'S' });
    } else {
        s.push(if (mode & 0o100) != 0 { 'x' } else { '-' });
    }

    s.push(if (mode & 0o040) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o020) != 0 { 'w' } else { '-' });
    if (mode & 0o2000) != 0 {
        s.push(if (mode & 0o010) != 0 { 's' } else { 'S' });
    } else {
        s.push(if (mode & 0o010) != 0 { 'x' } else { '-' });
    }

    s.push(if (mode & 0o004) != 0 { 'r' } else { '-' });
    s.push(if (mode & 0o002) != 0 { 'w' } else { '-' });
    if (mode & 0o1000) != 0 {
        s.push(if (mode & 0o001) != 0 { 't' } else { 'T' });
    } else {
        s.push(if (mode & 0o001) != 0 { 'x' } else { '-' });
    }

    let has_xattr = xattr::list(file_path)
        .map(|mut i| i.next().is_some())
        .unwrap_or(false);

    if has_xattr {
        s.push('+');
    } else {
        s.push(' ');
    }
    s
}

fn format_date(modified: SystemTime) -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Local> = modified.into();
    let datetime = datetime + Duration::hours(1);
    let six_months = std::time::Duration::from_secs(180 * 24 * 60 * 60);

    let is_old_or_future = match now.duration_since(modified) {
        Ok(d) => d > six_months,
        Err(_) => true,
    };

    if is_old_or_future {
        datetime.format("%b %d  %Y").to_string()
    } else {
        datetime.format("%b %d %H:%M").to_string()
    }
}

fn append_indicator(mut name: String, metadata: &fs::Metadata) -> String {
    if metadata.is_dir() {
        name.push('/');
    } else if metadata.is_symlink() {
        name.push('@');
    } else if metadata.file_type().is_fifo() {
        name.push('|');
    } else if metadata.file_type().is_socket() {
        name.push('=');
    } else if (metadata.permissions().mode() & 0o111) != 0 {
        name.push('*');
    }
    name
}

fn format_shell_name(name: &str) -> String {
    if name.contains('\n') {
        let mut res = String::from("'");
        for c in name.chars() {
            if c == '\n' {
                res.push_str("'$'\\n''");
            } else {
                res.push(c);
            }
        }
        res.push('\'');
        if res.ends_with("''") {
            res.truncate(res.len() - 2);
        }
        res
    } else if name.contains('\'') {
        format!("\"{}\"", name)
    } else if name.contains('"') || name.contains(' ') {
        format!("'{}'", name)
    } else {
        name.to_string()
    }
}

fn align_and_format(entries: Vec<LongEntry>, show_total: bool) -> String {
    if entries.is_empty() {
        return String::new();
    }

    let mut w_links = 0;
    let mut w_user = 0;
    let mut w_group = 0;
    let mut w_size = 0;
    let mut w_date = 0;
    let mut total_blocks = 0;

    for e in &entries {
        w_links = max(w_links, e.links.len());
        w_user = max(w_user, e.user.len());
        w_group = max(w_group, e.group.len());
        w_size = max(w_size, e.size.len());
        w_date = max(w_date, e.date.len());
        total_blocks += e.blocks;
    }

    let mut out = String::new();

    if show_total {
        out.push_str(&format!("total {}\n", total_blocks / 2));
    }

    for e in entries {
        let display_name = if let Some((link_name, target_name)) = e.name.split_once(" -> ") {
            format!(
                "{} -> {}",
                format_shell_name(link_name),
                format_shell_name(target_name)
            )
        } else {
            format_shell_name(&e.name)
        };

        out.push_str(&format!(
            "{} {:>lw$} {:<uw$} {:<gw$} {:>sw$} {:>dw$} {}\n",
            e.perms,
            e.links,
            e.user,
            e.group,
            e.size,
            e.date,
            display_name,
            lw = w_links,
            uw = w_user,
            gw = w_group,
            sw = w_size,
            dw = w_date
        ));
    }
    out
}

fn prepare_long_entry(
    mut name: String,
    metadata: &fs::Metadata,
    flag: Flag,
    full_path: &Path,
) -> LongEntry {
    if flag.f && !metadata.is_symlink() {
        name = append_indicator(name, metadata);
    }

    if metadata.file_type().is_symlink() {
        if let Ok(target_path_buf) = fs::read_link(full_path) {
            let mut target_str = target_path_buf.to_string_lossy().to_string();
            if flag.f {
                let resolved_target = if target_path_buf.is_absolute() {
                    target_path_buf.clone()
                } else {
                    full_path
                        .parent()
                        .unwrap_or(Path::new("."))
                        .join(&target_path_buf)
                };

                if let Ok(target_meta) = fs::metadata(&resolved_target) {
                    target_str = append_indicator(target_str, &target_meta);
                }
            }
            name.push_str(" -> ");
            name.push_str(&target_str);
        }
    }

    let perms = format_permissions(metadata, full_path);

    let links = metadata.nlink().to_string();

    let uid = metadata.uid();
    let user = get_user_by_uid(uid)
        .map(|u| u.name().to_string_lossy().to_string())
        .unwrap_or_else(|| uid.to_string());

    let gid = metadata.gid();
    let group = get_group_by_gid(gid)
        .map(|g| g.name().to_string_lossy().to_string())
        .unwrap_or_else(|| gid.to_string());

    let size = if metadata.file_type().is_block_device() || metadata.file_type().is_char_device() {
        let rdev = metadata.rdev();
        let major = (rdev >> 8) & 0xfff;
        let minor = (rdev & 0xff) | ((rdev >> 12) & 0xfff00);
        format!("{:>3}, {:>3}", major, minor)
    } else {
        metadata.len().to_string()
    };

    let date = format_date(metadata.modified().unwrap_or(SystemTime::now()));

    LongEntry {
        perms,
        links,
        user,
        group,
        size,
        date,
        name,
        blocks: metadata.blocks(),
    }
}
