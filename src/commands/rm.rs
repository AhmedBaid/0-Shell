use std::path::Path;

pub fn rm(args: Vec<String>) -> bool {
    let recursive = args.iter().any(|a| a == "-r");
    let targets: Vec<&String> = args.iter().filter(|a| *a != "-r").collect();

    if targets.is_empty() {
        eprintln!("rm: missing operand");
        return false;
    }

    for arg in targets {
        let path = Path::new(arg);
        match std::fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() && !recursive {
                    eprintln!("rm: cannot remove '{}': Is a directory", arg);
                    return false;
                } else if meta.is_dir() {
                    if let Err(e) = std::fs::remove_dir_all(path) {
                        eprintln!("rm: cannot remove '{}': {}", arg, e);
                        return false;
                    }
                } else {
                    if let Err(e) = std::fs::remove_file(path) {
                        eprintln!("rm: cannot remove '{}': {}", arg, e);
                        return false;
                    }
                }
            }
            Err(e) => {
                eprintln!("rm: cannot remove '{}': {}", arg, e);
                return false;
            }
            
        }
    }
    true
}
