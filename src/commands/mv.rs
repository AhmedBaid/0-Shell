use std::fs;
use std::path::Path;

pub fn mv(args: Vec<String>) -> bool {
    if args.len() < 2 {
        println!("mv: missing operand");
        return false;
    }

    if args.len() == 2 {
        let src = &args[0];
        let dst = &args[1];
        let src_path = Path::new(src);
        let dst_path = Path::new(dst);

        let final_dst = if dst_path.is_dir() {
            match src_path.file_name() {
                Some(name) => dst_path.join(name),
                None => dst_path.to_path_buf(),
            }
        } else {
            dst_path.to_path_buf()
        };

        match fs::rename(src_path, &final_dst) {
            Ok(_) => (),
            Err(e) => println!("mv: cannot move '{}': {}", src, e),
        }
        return true;
    } else if args.len() > 2 {
        let dst_dir = Path::new(args.last().unwrap());

        if !dst_dir.is_dir() {
            println!("mv: target '{}' is not a directory", dst_dir.display());
            return false;
        }

        for src in &args[0..args.len() - 1] {
            let src_path = Path::new(src);

            if let Some(file_name) = src_path.file_name() {
                let dst = dst_dir.join(file_name);
                match fs::rename(src_path, &dst) {
                    Ok(_) => (),
                    Err(e) => println!("mv: cannot move '{}': {}", src, e),
                }
            }
        }
        return true;
    }
    false
}
