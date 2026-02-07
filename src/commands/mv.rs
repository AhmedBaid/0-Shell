use std::path::Path;
pub fn mv(args: Vec<String>) {
    if args.len() < 2 {
        println!("mv: missing operand");
        return;
    }
    
    if args.len() == 2 {
        let src = &args[0];
        let dst = &args[1];

        match std::fs::rename(src, dst) {
            Ok(_) => (),
            Err(e) => println!("mv: cannot move '{}': {}", src, e),
        }
        return;
    }else if args.len() > 2 {
        let dst_dir = Path::new(args.last().unwrap());
        if !dst_dir.is_dir() {
            println!("mv: target '{}' is not a directory", dst_dir.display());
            return;
        }
        for src in &args[0..args.len() - 1] {
            let dst = dst_dir.join(src);
            match std::fs::rename(src, &dst) {
                Ok(_) => (),
                Err(e) => println!("mv: cannot move '{}': {}", src, e),
            }
        }
        return;
    }
 
}
