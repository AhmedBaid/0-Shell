use std::{ fs::File, io::{ self, Write }, path::{  Path } };
//use std::io::BufRead;
use std::io::{ Read };

pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        loop {
            io::stdout().flush().ok();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                break;
            }
            print!("{}", line);
        }
    } else {
        for file in args {
            let source_path = Path::new(&file);

            let file_open = File::open(source_path);
            //    let reader;
            match file_open {
                Ok(mut f) => {
                    /*    let reader = BufReader::new(f);
                    for line in reader.lines() {
                        match line {
                            Ok(e) => print!("{}", e),
                            Err(e) => println!("{}", e),
                        }
                    } */

                    let mut buf = [0u8; 8192];

                    loop {
                        let n = match f.read(&mut buf) {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("cat: {}: {}", file, e);
                                break;
                            }
                        };

                        io::stdout()
                            .write_all(&buf[..n])
                            .ok();
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
