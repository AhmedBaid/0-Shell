use std::{ fs::File, io::{ self, Read, Write }, path::Path };
use crossterm::{
    event::{ self, Event, KeyCode, KeyEventKind, KeyModifiers },
    terminal::{ enable_raw_mode },
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
        match enable_raw_mode() {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to enable raw mode: {}", e);
                return;
            }
        }
        for file in args {
            let source_path = Path::new(&file);
            match File::open(source_path) {
                Ok(mut f) => {
                    let mut buffer = [0; 1024];
                    loop {
                        if event::poll(std::time::Duration::from_millis(0)).unwrap_or(false) {
                            if let Event::Key(key_event) = event::read().unwrap() {
                                if
                                    key_event.kind == KeyEventKind::Press &&
                                    key_event.modifiers.contains(KeyModifiers::CONTROL) &&
                                    key_event.code == KeyCode::Char('c')
                                {
                                    print!("^C\r\n"); // Print explicit break indicator
                                    break; // Stop reading this file
                                }
                            }
                        }
                        match f.read(&mut buffer) {
                            Ok(0) => {
                                break;
                            }
                            Ok(n) => {
                                let chunk = &buffer[..n];
                                let s = String::from_utf8_lossy(chunk);
                                let formatted = s.replace("\n", "\r\n");
                                print!("{}", formatted);
                                io::stdout().flush().ok();
                            }
                            Err(e) => {
                                eprintln!("Error reading file: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => println!("cat: {}: {}", file, e),
            }
        }
        crossterm::terminal::disable_raw_mode().ok();
    }
}
