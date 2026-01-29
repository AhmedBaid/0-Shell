use std::env;
use std::io::{self, Write};
use std::path::Path;

fn list_directory(path: &Path, show_all: bool, classify: bool) -> io::Result<Vec<String>> {
    let mut entries = Vec::new();
    for entry in path.read_dir()? {
        let entry = entry?;
        let name = match entry.file_name().to_str() {
            Some(name) => name,
            None => continue,
        };
        if !show_all && name.starts_with('.') {
            continue;
        }
        let mut display_name = name.to_string();
        if classify {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                display_name.push('/');
            }
        }
        entries.push(display_name);
    }
    entries.sort();
    Ok(entries)
}

fn run_command(args: &[String]) -> bool {
    if args.is_empty() {
        return true;
    }

    match args[0].as_str() {
        "exit" => return false,
        "echo" => {
            let output = args[1..].join(" ");
            println!("{output}");
        }
        "cd" => {
            let target = args.get(1).cloned().or_else(|| env::var("HOME").ok());
            match target {
                Some(path) => {
                    if let Err(err) = env::set_current_dir(&path) {
                        eprintln!("cd: {err}");
                    }
                }
                None => eprintln!("cd: missing path and HOME not set"),
            }
        }
        "pwd" => match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(err) => eprintln!("pwd: {err}"),
        },
        "ls" => {
            let mut show_all = false;
            let mut classify = false;
            let mut target = ".";
            for arg in &args[1..] {
                if arg.starts_with('-') {
                    show_all |= arg.contains('a');
                    classify |= arg.contains('F');
                } else {
                    target = arg;
                }
            }
            match list_directory(Path::new(target), show_all, classify) {
                Ok(entries) => {
                    println!("{}", entries.join(" "));
                }
                Err(err) => eprintln!("ls: {err}"),
            }
        }
        _ => {
            eprintln!("Command '{}' not found", args[0]);
        }
    }

    true
}

fn parse_input(line: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_single = false;
    let mut in_double = false;

    for ch in line.chars() {
        match ch {
            '\'' if !in_double => {
                in_single = !in_single;
            }
            '"' if !in_single => {
                in_double = !in_double;
            }
            c if c.is_whitespace() && !in_single && !in_double => {
                if !current.is_empty() {
                    args.push(current);
                    current = String::new();
                }
            }
            _ => current.push(ch),
        }
    }

    if !current.is_empty() {
        args.push(current);
    }

    args
}

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        if io::stdout().flush().is_err() {
            break;
        }

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => {
                println!();
                break;
            }
            Ok(_) => {
                let trimmed = line.trim_end();
                if trimmed.is_empty() {
                    continue;
                }
                let args = parse_input(trimmed);
                if !run_command(&args) {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
