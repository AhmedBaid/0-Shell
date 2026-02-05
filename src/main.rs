use std::env;
use std::io::{self, Write};
pub mod commands;
pub mod helpers;
use commands::cd::*;
use helpers::parser::*;
use helpers::print_banner::*;

use crossterm::cursor::MoveToColumn;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    print_banner();
    enable_raw_mode()?;

    let mut history: Vec<String> = vec![];

    loop {
        let current_dir = env::current_dir().expect("Failed to get current working directory");
        let prompt_text = format!("{GREEN}{}$ {RESET}", current_dir.display());

        print!("{}", prompt_text);
        io::stdout().flush()?;

        let mut input_buffer = String::new();
        let mut history_index = history.len();

        loop {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'd' {
                                print!("\r\n");
                                disable_raw_mode()?;
                                std::process::exit(0);
                            } else if key_event.modifiers.contains(KeyModifiers::CONTROL)
                                && c == 'c'
                            {
                                print!("\r\n");
                                input_buffer.clear();
                                break;
                            }
                            input_buffer.push(c);
                            print!("{}", c);
                            io::stdout().flush()?;
                        }

                        KeyCode::Backspace => {
                            if !input_buffer.is_empty() {
                                input_buffer.pop();
                                print!("\x08 \x08");
                                io::stdout().flush()?;
                            }
                        }

                        KeyCode::Enter => {
                            print!("\r\n");
                            break;
                        }

                        KeyCode::Up => {
                            if history_index > 0 {
                                history_index -= 1;
                                input_buffer = history[history_index].clone();

                                execute!(
                                    io::stdout(),
                                    Clear(ClearType::CurrentLine),
                                    MoveToColumn(0)
                                )?;
                                print!("{}{}", prompt_text, input_buffer);
                                io::stdout().flush()?;
                            }
                        }

                        KeyCode::Down => {
                            if history_index < history.len() {
                                history_index += 1;

                                if history_index < history.len() {
                                    input_buffer = history[history_index].clone();
                                } else {
                                    input_buffer.clear();
                                }

                                execute!(
                                    io::stdout(),
                                    Clear(ClearType::CurrentLine),
                                    MoveToColumn(0)
                                )?;
                                print!("{}{}", prompt_text, input_buffer);
                                io::stdout().flush()?;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if !input_buffer.trim().is_empty() {
            if history.last() != Some(&input_buffer) {
                history.push(input_buffer.clone());
            }

            let commands = parse_input(&input_buffer);

            disable_raw_mode()?;
            let should_continue = execute_all(commands);
            enable_raw_mode()?;

            if !should_continue {
                break;
            }
        }
    }

    Ok(())
}
