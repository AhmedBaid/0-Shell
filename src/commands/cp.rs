use std::{fs, io::ErrorKind, path::Path};
pub fn cp(args: Vec<String>) -> bool {
    let source_path = Path::new(&args[0]);
    let destination_path = Path::new(&args[1]);
    if !source_path.is_file() {
        eprintln!("cp: cannot stat '{}': No such file", source_path.display());
        return false;
    } else if source_path.is_file() && destination_path.is_dir() {
        let file_name =  match  source_path.file_name(){
            Some(name) => name,
            None => {
                eprintln!("cp: error getting file name from source path");
                return false;
            }
        };
        let mut dest_file_path = destination_path.to_path_buf();
        dest_file_path.push(file_name);
        let res = fs::copy(source_path, dest_file_path);
        match res {
            Ok(_) => {return true;},
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("cp: permission denied: {}", destination_path.display());
                    return false;
                } else {
                    eprintln!("cp: error copying file: {}", e);
                    return false;
                }
            }
        }
    } else {
        let res = fs::copy(source_path, destination_path);
        match res {
            Ok(_) => return true,
            Err(e) => {
                if e.kind() == ErrorKind::PermissionDenied {
                    eprintln!("cp: permission denied: {}", destination_path.display());
                    return false;
                } else {
                    eprintln!("cp: error copying file: {}", e);
                    return false;
                }
            }
        }
    }
}
