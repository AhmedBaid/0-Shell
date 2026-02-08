use std::env;
use std::io::{self, stdout, Write};
use std::io::{self, Write};

pub mod commands;
pub mod helpers;

use crate::helpers::check_quotes::quotes_balanced;
use commands::pwd_state::*;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::{self, MoveToColumn};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use helpers::parser::{execute_all, parse_input, ParseResult};
use helpers::print_banner::print_banner;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
fn main() -> io::Result<()> {
    print_banner();
    enable_raw_mode()?;

    let mut history: Vec<String> = vec![];
    let mut input_buffer = String::new();
    let mut history_index = 0;

    let mut is_continuation = false;

    let current_dir = env::current_dir().expect("Failed to get current working directory");
    let mut pwd_state = PwdState::new(
        current_dir.display().to_string(),
        current_dir.display().to_string(),
    );
    loop {
        let dir_str = current_dir.display().to_string();

        let prompt_len = dir_str.len() + 2;

        execute!(stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;
        let prompt_text = format!("{GREEN}{}$ {RESET}", pwd_state.get_current_dir());
        if !is_continuation {
            print!("{}", prompt_text);
        } else {
            print!("> ");
        }
        io::stdout().flush()?;

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
                                    if !input_buffer.ends_with('\n') {
                                        input_buffer.pop();
                                        print!("\x08 \x08");
                                        io::stdout().flush()?;
                                    }
                                }
                            }
                        }

                        KeyCode::Enter => {
                            print!("\r\n");
                            io::stdout().flush()?;

                            match parse_input(&input_buffer) {
                                ParseResult::Ok(cmds) => {
                                    if !input_buffer.trim().is_empty() {
                                        if history.last() != Some(&input_buffer) {
                                            history.push(input_buffer.clone());
                                        }
                                    }
                                    history_index = history.len();

                                    disable_raw_mode()?;
                                    let keep_running = execute_all(cmds, &mut pwd_state);
                                    enable_raw_mode()?;

                                    if !keep_running {
                                        disable_raw_mode()?;
                                        std::process::exit(0);
                                    }

                                    input_buffer.clear();
                                    is_continuation = false;
                                    break;
                                }
                                ParseResult::Incomplete => {
                                    input_buffer.push('\n');
                                    is_continuation = true;
                                    break;
                                }
                                ParseResult::Err(e) => {
                                    println!("Error: {}\r", e);
                                    input_buffer.clear();
                                    is_continuation = false;
                                    break;
                                }
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
    }
}
