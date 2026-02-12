use std::env;
use std::io::{self, stdout, Write};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

pub mod commands;
pub mod helpers;

use commands::pwd_state::*;
use crossterm::cursor::MoveToColumn;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use helpers::parser::{clear, execute_all, parse_input, ParseResult};
use helpers::welcome::*;

const NEON_BLUE: &str = "\x1b[38;2;0;180;255m";
const RESET: &str = "\x1b[0m";

fn main() -> io::Result<()> {
    clear();
    welcome();
    enable_raw_mode()?;

    let mut history: Vec<String> = vec![];
    let mut input_buffer = String::new();

    let mut cursor_char_index: usize = 0;

    let mut history_index = 0;
    let mut is_continuation = false;

    let start_dir = env::current_dir().expect("Failed to get current working directory");
    let mut pwd_state = PwdState::new(
        start_dir.display().to_string(),
        start_dir.display().to_string(),
    );

    loop {
        let current_display_dir = pwd_state.get_current_dir();

        let prompt_text = if !is_continuation {
            format!("{NEON_BLUE}{}$ {RESET}", current_display_dir)
        } else {
            "> ".to_string()
        };

        let prompt_visual_width = if !is_continuation {
            current_display_dir.width() + 2
        } else {
            2
        };

        execute!(stdout(), MoveToColumn(0), Clear(ClearType::CurrentLine))?;

        print!("{}{}", prompt_text, input_buffer);

        let buffer_visual_width: usize = input_buffer
            .chars()
            .take(cursor_char_index)
            .map(|c| c.width().unwrap_or(0))
            .sum();

        let final_cursor_x = (prompt_visual_width + buffer_visual_width) as u16;

        execute!(stdout(), MoveToColumn(final_cursor_x))?;
        io::stdout().flush()?;

        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char(c) => {
                        if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'd' {
                            print!("\r\n");
                            disable_raw_mode()?;
                            std::process::exit(0);
                        } else if key_event.modifiers.contains(KeyModifiers::CONTROL) && c == 'c' {
                            if !input_buffer.trim().is_empty() {
                                if history.last() != Some(&input_buffer) {
                                    history.push(input_buffer.clone());
                                }
                            }
                            history_index = history.len();
                            input_buffer.clear();
                            cursor_char_index = 0;
                            print!("\r\n");
                            is_continuation = false;
                            continue;
                        }

                        let byte_idx = input_buffer
                            .char_indices()
                            .map(|(i, _)| i)
                            .nth(cursor_char_index)
                            .unwrap_or(input_buffer.len());

                        input_buffer.insert(byte_idx, c);
                        cursor_char_index += 1;
                    }

                    KeyCode::Backspace => {
                        if cursor_char_index > 0 {
                            cursor_char_index -= 1;

                            let byte_idx = input_buffer
                                .char_indices()
                                .map(|(i, _)| i)
                                .nth(cursor_char_index)
                                .unwrap_or(input_buffer.len());

                            input_buffer.remove(byte_idx);
                        }
                    }

                    KeyCode::Delete => {
                        if cursor_char_index < input_buffer.chars().count() {
                            let byte_idx = input_buffer
                                .char_indices()
                                .map(|(i, _)| i)
                                .nth(cursor_char_index)
                                .unwrap_or(input_buffer.len());

                            input_buffer.remove(byte_idx);
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
                                let _ = execute_all(cmds, &mut pwd_state);
                                enable_raw_mode()?;

                                input_buffer.clear();
                                cursor_char_index = 0;
                                is_continuation = false;
                            }
                            ParseResult::Incomplete => {
                                input_buffer.push('\n');
                                cursor_char_index += 1;
                                is_continuation = true;
                            }
                        }
                    }

                    KeyCode::Up => {
                        if history_index > 0 {
                            history_index -= 1;
                            input_buffer = history[history_index].clone();
                            cursor_char_index = input_buffer.chars().count();
                        }
                    }

                    KeyCode::Down => {
                        if history_index < history.len() {
                            history_index += 1;
                            if history_index < history.len() {
                                input_buffer = history[history_index].clone();
                                cursor_char_index = input_buffer.chars().count();
                            } else {
                                input_buffer.clear();
                                cursor_char_index = 0;
                            }
                        }
                    }

                    KeyCode::Left => {
                        if cursor_char_index > 0 {
                            cursor_char_index -= 1;
                        }
                    }

                    KeyCode::Right => {
                        if cursor_char_index < input_buffer.chars().count() {
                            cursor_char_index += 1;
                        }
                    }

                    KeyCode::Home => cursor_char_index = 0,
                    KeyCode::End => cursor_char_index = input_buffer.chars().count(),

                    _ => {}
                }
            }
        }
    }
}
