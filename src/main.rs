use std::env;
use std::io::{self, stdout, Write};

pub mod commands;
pub mod helpers;

use commands::pwd_state::*;
use crossterm::cursor::MoveToColumn;
use crossterm::cursor::{self};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use helpers::parser::{execute_all, parse_input, ParseResult};
use helpers::print_banner::welcome as print_banner;

const PURPLE: &str = "\x1b[38;2;160;64;255m";
const RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    print_banner();
    enable_raw_mode()?;

    let mut history: Vec<String> = vec![];
    let mut input_buffer = String::new();
    let mut history_index = 0;
    let mut input_purline = String::new();
    let mut is_continuation = false;

    let start_dir = env::current_dir().expect("Failed to get current working directory");
    let mut pwd_state = PwdState::new(
        start_dir.display().to_string(),
        start_dir.display().to_string(),
    );

    loop {
        let current_display_dir = pwd_state.get_current_dir();

        let prompt_len = if is_continuation {
            2
        } else {
            current_display_dir.len() + 2
        };

        execute!(stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;

        let prompt_text = if !is_continuation {
            format!("{PURPLE}{}$ {RESET}", current_display_dir)
        } else {
            "> ".to_string()
        };

        print!("{}", prompt_text);
        io::stdout().flush()?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    let (current_x, current_y) = cursor::position().unwrap();

                    let cursor_idx = (current_x as usize).saturating_sub(prompt_len);

                    match key_event.code {
                        KeyCode::Char(c) => {
                            if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'd' {
                                print!("\r\n");
                                disable_raw_mode()?;
                                std::process::exit(0);
                            } else if key_event.modifiers.contains(KeyModifiers::CONTROL)
                                && c == 'c'
                            {
                                if !input_buffer.trim().is_empty() && !input_buffer.contains('\n'){
                                    if history.last() != Some(&input_buffer) {
                                        history.push(input_buffer.clone());
                                    }
                                }
                                history_index = history.len();
                                input_buffer.clear();
                                print!("\r\n");
                                is_continuation = false;
                                break;
                            }

                            if cursor_idx >= input_purline.len() {
                                input_buffer.push(c);
                                input_purline.push(c);
                            } else {
                                input_buffer.insert(cursor_idx, c);
                                input_purline.insert(cursor_idx, c);
                            }

                            execute!(
                                stdout(),
                                cursor::MoveToColumn(prompt_len as u16),
                                Clear(ClearType::UntilNewLine)
                            )?;
                            print!("{}", input_purline);

                            execute!(
                                stdout(),
                                cursor::MoveTo((prompt_len + cursor_idx + 1) as u16, current_y)
                            )?;
                            io::stdout().flush()?;
                        }

                        KeyCode::Backspace => {
                            if !input_purline.is_empty() {
                                if cursor_idx > 0 && cursor_idx <= input_purline.len() {
                                    input_buffer.remove(cursor_idx - 1);
                                    input_purline.remove(cursor_idx - 1);

                                    // Redraw line
                                    execute!(
                                        stdout(),
                                        cursor::MoveToColumn(prompt_len as u16),
                                        Clear(ClearType::UntilNewLine)
                                    )?;
                                    print!("{}", input_purline);

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
                            print!("\r\n");
                            io::stdout().flush()?;

                            input_purline.clear();
                            match parse_input(&input_buffer) {
                                ParseResult::Ok(cmds) => {
                                    if !input_buffer.trim().is_empty() && !input_buffer.contains('\n') {
                                        if history.last() != Some(&input_buffer) {
                                            history.push(input_buffer.clone());
                                        }
                                    }
                                    history_index = history.len();

                                    disable_raw_mode()?;
                                    let _ = execute_all(cmds, &mut pwd_state);
                                    enable_raw_mode()?;

                                    input_buffer.clear();
                                    is_continuation = false;
                                    break;
                                }
                                ParseResult::Incomplete => {
                                    input_buffer.push('\n');
                                    is_continuation = true;
                                    break;
                                }
                            }
                        }

                        KeyCode::Up => {
                            if history_index > 0 {
                                history_index -= 1;
                                input_buffer = history[history_index].clone();
                                input_purline = history[history_index].clone();
                                execute!(
                                    io::stdout(),
                                    Clear(ClearType::CurrentLine),
                                    MoveToColumn(0)
                                )?;
                                print!("{}{}", prompt_text, input_purline.replace("\n", "\r\n"));
                                io::stdout().flush()?;
                            }
                        }

                        KeyCode::Down => {
                            if history_index < history.len() {
                                history_index += 1;
                                if history_index < history.len() {
                                    input_buffer = history[history_index].clone();
                                    input_purline = history[history_index].clone();
                                } else {
                                    input_buffer.clear();
                                    input_purline.clear();
                                }

                                execute!(
                                    io::stdout(),
                                    Clear(ClearType::CurrentLine),
                                    MoveToColumn(0)
                                )?;
                                print!("{}{}", prompt_text, input_purline.replace("\n", "\r\n"));
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
