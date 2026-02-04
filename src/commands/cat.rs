use std::{
    fs::File,
    io::{self, BufReader, Write}, path::{self, Path},
};
use std::io::BufRead;

pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        loop {
            io::stdout().flush().ok();
            let mut line = String::new();
            if io::stdin().read_line(&mut line).is_err() {
                break;
            }
            println!("{}", line.replace("\n", "").replace("\"", ""));
        }
    } else {
        for file in args {
            println!("{file}");
                let source_path = Path::new(&file);

            let file_open = File::open(source_path);
        //    let reader;
            match file_open  {
                Ok(f) => {
                let     reader = BufReader::new(f);
                    for line in reader.lines() {
                        match line {
                            Ok(e) => println!("{}", e),
                            Err(e) => println!("{}", e),
                        }
                    }
                }
                Err(e) => println!("{}-", e),
            };
        }
    }
}
