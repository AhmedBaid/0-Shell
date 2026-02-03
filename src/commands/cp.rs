use std::{fs, io::ErrorKind, path::Path};
pub fn cp(args: Vec<String>) {
    let source_path = Path::new(&args[0]);
    let destination_path = Path::new(&args[1]);
    if !source_path.is_file() {
        println!("cp: cannot stat '{}': No such file", source_path.display());
        return;
    } else if source_path.is_file() && destination_path.is_dir() {
        let file_name = source_path.file_name().unwrap();
        let mut dest_file_path = destination_path.to_path_buf();
        dest_file_path.push(file_name);
        let res = fs::copy(source_path, dest_file_path);
        match res {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    println!("cp: permission denied: {}", destination_path.display());
                } else {
                    println!("cp: error copying file: {}", e);
                }
            }
        }
    } else {
        let res = fs::copy(source_path, destination_path);
        match res {
            Ok(_) => {}
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    println!("cp: permission denied: {}", destination_path.display());
                } else {
                    println!("cp: error copying file: {}", e);
                }
            }
        }
    }
}
