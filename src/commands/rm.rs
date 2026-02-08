use std::path::Path;

pub fn rm(args: Vec<String>) {
    let recursive = args.iter().any(|a| a == "-r");
    let targets: Vec<&String> = args.iter().filter(|a| *a != "-r").collect();

    if targets.is_empty() {
        println!("rm: missing operand");
        return;
    }

    for arg in targets {
        let path = Path::new(arg);
        match std::fs::metadata(path) {
            Ok(meta) => {
                if meta.is_dir() && !recursive {
                    println!("rm: cannot remove '{}': Is a directory", arg);
                } else if meta.is_dir() {
                    if let Err(e) = std::fs::remove_dir_all(path) {
                        println!("rm: cannot remove '{}': {}", arg, e);
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
}
