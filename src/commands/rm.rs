use std::path::Path;

pub fn rm(args: Vec<String>) -> bool {
    let mut recursive = false;

    for arg in &args {
        if arg == "--recursive" {
            recursive = true;
            continue;
        }
        if arg.starts_with("-") {
            for c in arg[1..].chars() {
                if c != '-' && c != 'r' && c != 'R' {
                    println!("rm: invalid option -- '{}'", c);
                    return false;
                }
            }

            recursive = true;
        }
    }

    let targets: Vec<&String> = args.iter().filter(|a| !a.starts_with('-')).collect();

    if targets.is_empty() {
        println!("rm: missing operand");
        return false;
    }

    for arg in targets {
        let path = Path::new(arg);

        match std::fs::symlink_metadata(path) {
            Ok(meta) => {
                if meta.is_symlink() {
                    if let Err(e) = std::fs::remove_file(path) {
                        println!("rm: cannot remove symlink '{}': {}", arg, e);
                    }
                } else if meta.is_dir() {
                    if !recursive {
                        println!("rm: cannot remove '{}': Is a directory", arg);
                    } else {
                        if let Err(e) = std::fs::remove_dir_all(path) {
                            println!("rm: cannot remove '{}': {}", arg, e);
                        }
                    }
                } else {
                    if let Err(e) = std::fs::remove_file(path) {
                        println!("rm: cannot remove '{}': {}", arg, e);
                    }
                }
            }
            Err(e) => println!("rm: cannot remove '{}': {}", arg, e),
        }
    }
    true
}
