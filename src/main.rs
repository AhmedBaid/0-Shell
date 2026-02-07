use std::env;
use std::io::{self, Write, stdout};
pub mod commands;
pub mod helpers;
use commands::cd::*;
use helpers::parser::*;
use helpers::print_banner::*;

use crate::helpers::check_quotes::quotes_balanced;
use crossterm::cursor::{self, MoveToColumn};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode};

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    print_banner();
    enable_raw_mode()?;

    let mut history: Vec<String> = vec![];

    loop {
        let current_dir = env::current_dir().expect("Failed to get CWD");
        let dir_str = current_dir.display().to_string();

        let prompt_len = dir_str.len() + 2;
        let prompt_text = format!("{}{}$ {}", GREEN, dir_str, RESET);

        execute!(stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        print!("{}", prompt_text);
        io::stdout().flush()?;

        let mut input_buffer = String::new();
        let mut history_index = history.len();

        loop {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    let (current_x, current_y) = cursor::position().unwrap();
                    let cursor_idx = current_x as usize - prompt_len;
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
                            if cursor_idx >= input_buffer.len() {
                                input_buffer.push(c);
                            } else {
                                input_buffer.insert(cursor_idx, c);
                            }

                            execute!(
                                stdout(),
                                cursor::MoveToColumn(prompt_len as u16),
                                Clear(ClearType::UntilNewLine)
                            )?;
                            print!("{}", input_buffer);

                            execute!(
                                stdout(),
                                cursor::MoveTo((prompt_len + cursor_idx + 1) as u16, current_y)
                            )?;
                            io::stdout().flush()?;
                        }

                        KeyCode::Backspace => {
                            if !input_buffer.is_empty() {
                                if cursor_idx > 0 {
                                    input_buffer.remove(cursor_idx - 1);

                                    execute!(
                                        stdout(),
                                        cursor::MoveToColumn(prompt_len as u16),
                                        Clear(ClearType::UntilNewLine)
                                    )?;
                                    print!("{}", input_buffer);
                                    execute!(
                                        stdout(),
                                        cursor::MoveTo(
                                            (prompt_len + cursor_idx - 1) as u16,
                                            current_y
                                        )
                                    )?;
                                    io::stdout().flush()?;
                                }
                            }
                        }

                        KeyCode::Enter => {
                            if quotes_balanced(&input_buffer) {
                                print!("\r\n");
                                break;
                            } else {
                                input_buffer.push_str("\r\n");
                                print!("\r\ndequoted >");
                                io::stdout().flush()?;
                            }
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
                        KeyCode::Left => {
                            if cursor_idx > 0 {
                                execute!(stdout(), cursor::MoveTo(current_x - 1, current_y))
                                    .unwrap();
                            }
                        }
                        KeyCode::Right => {
                            if cursor_idx < input_buffer.len() {
                                execute!(stdout(), cursor::MoveTo(current_x + 1, current_y))
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if !input_buffer.is_empty() {
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
