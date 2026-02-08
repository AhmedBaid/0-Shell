use std::{ fs::File, io::{ self, Read, Write }, path::Path };
use crossterm::{
    event::{ self, Event, KeyCode, KeyEventKind, KeyModifiers },
    terminal::{ disable_raw_mode, enable_raw_mode },
};

pub fn cat(args: Vec<String>) {
    if args.is_empty() {
        match enable_raw_mode() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to enable raw mode: {}", e);
                return;
            }
        }

        let mut input_buffer = String::new();

        loop {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                            if c == 'd' {
                               print!("\r\n");
                                break;
                            } else if c == 'c' {
                                print!("^C\r\n");
                                break;
                            }
                            io::stdout().flush().ok();
                        }

                        KeyCode::Char(c) => {
                            print!("{}", c); // Echo the character to screen
                            input_buffer.push(c); // Save to buffer
                            io::stdout().flush().ok();
                        }

                        KeyCode::Backspace => {
                            if !input_buffer.is_empty() {
                                input_buffer.pop();
                                print!("\x08 \x08");
                                io::stdout().flush().ok();
                            }
                        }

                        KeyCode::Enter => {
                            print!("\r\n");
                            print!("{}\r\n", input_buffer);
                            io::stdout().flush().ok();
                            input_buffer.clear();
                        }

                        _ => {}
                    }
                }
            }
        }
    } else {
        for file in args {
            let source_path = Path::new(&file);
            let file_open = File::open(source_path);
            match file_open {
                Ok(mut f) => {
                    let mut buf = [0u8; 8192];
                    loop {
                        match f.read(&mut buf) {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => {
                                io::stdout()
                                    .write_all(&buf[..n])
                                    .ok();
                            }
                            Err(e) => {
                                eprintln!("cat: {}: {}", file, e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => println!("{}", e),
            }
        }
    }
}
