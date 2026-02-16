use std::{collections::HashSet, ffi::OsString, fs, path::Path};

pub fn cp(args: Vec<String>) -> bool {
    if args.is_empty() {
        eprintln!("cp: missing file operand");
        return false;
    }
    if args.len() < 2 {
        eprintln!(
            "cp: missing destination file operand after '{}'",
            args[0].replace("\n", "\\n")
        );
        return false;
    }

    let sources = &args[0..args.len() - 1];
    let destination_path: &Path = Path::new(args.last().unwrap());

    if args.len() > 2 {
        if !destination_path.is_dir() {
            eprintln!(
                "cp: target '{}' is not a directory",
                destination_path.display().to_string().replace("\n", "\\n")
            );
            return false;
        }

        let mut dest_seen: HashSet<OsString> = HashSet::new();
        for source in sources {
            let source_path = Path::new(source);

            if let Some(file_name) = source_path.file_name() {
                if !dest_seen.insert(file_name.to_os_string()) {
                    eprintln!(
                        "cp: warning: cannot copy '{}' to '{}': destination file already used by another argument",
                        source.replace("\n", "\\n"),
                        destination_path.join(file_name).display().to_string().replace("\n", "\\n")
                    );
                    continue;
                }
            }

            copy_file_logic(source_path, destination_path, true);
        }
    } else {
        let source_path = Path::new(&args[0]);
        copy_file_logic(source_path, destination_path, destination_path.is_dir());
    }
    true
}

fn copy_file_logic(source: &Path, destination: &Path, dest_is_dir: bool) {
    if !source.exists() {
        eprintln!(
            "cp: cannot stat '{}': No such file or directory",
            source.display().to_string().replace("\n", "\\n")
        );
        return;
    }
    if source.is_dir() {
        eprintln!(
            "cp: -r not specified; omitting directory '{}'",
            source.display().to_string().replace("\n", "\\n")
        );
        return;
    }

    let final_dest = if dest_is_dir {
        match source.file_name() {
            Some(name) => destination.join(name),
            None => {
                eprintln!(
                    "cp: cannot determine file name for '{}'",
                    source.display().to_string().replace("\n", "\\n")
                );
                return;
            }
        }
    } else {
        destination.to_path_buf()
    };

    if final_dest.exists() {
        if let (Ok(src_can), Ok(dst_can)) = (source.canonicalize(), final_dest.canonicalize()) {
            if src_can == dst_can {
                eprintln!(
                    "cp: '{}' and '{}' are the same file",
                    source.display().to_string().replace("\n", "\\n"),
                    final_dest.display().to_string().replace("\n", "\\n")
                );
                return;
            }
        }
    }

    if let Err(e) = fs::copy(source, &final_dest) {
        eprintln!(
            "cp: error copying to '{}': {}",
            final_dest.display().to_string().replace("\n", "\\n"),
            e
        );
    }
}
