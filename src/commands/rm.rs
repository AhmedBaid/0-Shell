use std::path::Path;

pub fn rm(args: Vec<String>) {
    let recursive = args.contains(&"-r".to_string());
    for arg in &args {
        if arg == "-r" {
            continue;
        }
        let file = Path::new(arg);
        if file.is_dir() && !recursive{
            println!("rm: cannot remove '{}': Is a directory", arg);
        } else if file.is_dir() {
            if let Err(e) = std::fs::remove_dir_all(file) {
                println!("rm: cannot remove '{}': {}", arg, e);
            }
        } else {
            if let Err(e) = std::fs::remove_file(file) {
                println!("rm: cannot remove '{}': {}", arg, e);
            }
        }
    }
}
