use super::executor::*;
use crate::commands::pwd_state::*;

#[derive(Debug)]
pub enum CommandEnum {
    Rm(Vec<String>),
    Cp(Vec<String>),
    Mv(Vec<String>),
    Pwd,
    Cd(Vec<String>),
    Echo(Vec<String>),
    Mkdir(Vec<String>, Vec<String>),
    Exit,
    Unknown(String),
    Cat(Vec<String>),
    Ls(Vec<String>),
}

#[derive(Debug)]
pub enum ParseResult {
    Ok(Vec<CommandEnum>),
    Incomplete,
}

fn parse_tokens(input: &str) -> Result<Vec<Vec<String>>, String> {
    let mut commands: Vec<Vec<String>> = Vec::new();
    let mut current_args: Vec<String> = Vec::new();
    let mut current_token = String::new();

    #[derive(Clone, Copy, PartialEq)]
    enum Mode {
        Normal,
        Single,
        Double,
    }

    let mut mode = Mode::Normal;
    let mut escaped = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if escaped {
            match mode {
                Mode::Normal => {
                    current_token.push(c);
                }
                Mode::Double => {
                    if c == '"' || c == '\\' {
                        current_token.push(c);
                    } else {
                        current_token.push('\\');
                        current_token.push(c);
                    }
                }
                Mode::Single => {
                    current_token.push('\\');
                    current_token.push(c);
                }
            }
            escaped = false;
        } else {
            match mode {
                Mode::Normal => {
                    if c == '\\' {
                        escaped = true;
                    } else if c == '\'' {
                        mode = Mode::Single;
                    } else if c == '"' {
                        mode = Mode::Double;
                    } else if c == '&' {
                        if let Some(&next_c) = chars.peek() {
                            if next_c == '&' {
                                chars.next();

                                if !current_token.is_empty() {
                                    current_args.push(current_token.clone());
                                    current_token.clear();
                                }
                                if !current_args.is_empty() {
                                    commands.push(current_args.clone());
                                    current_args.clear();
                                }
                                continue;
                            }
                        }
                        current_token.push(c);
                    } else if c.is_whitespace() {
                        if !current_token.is_empty() {
                            current_args.push(current_token.clone());
                            current_token.clear();
                        }
                    } else {
                        current_token.push(c);
                    }
                }
                Mode::Single => {
                    if c == '\'' {
                        mode = Mode::Normal;
                    } else {
                        current_token.push(c);
                    }
                }
                Mode::Double => {
                    if c == '\\' {
                        escaped = true;
                    } else if c == '"' {
                        mode = Mode::Normal;
                    } else {
                        current_token.push(c);
                    }
                }
            }
        }
    }

    if escaped {
        return Err("Incomplete".to_string());
    }

    if mode != Mode::Normal {
        return Err("Incomplete".to_string());
    }

    if !current_token.is_empty() {
        current_args.push(current_token);
    }
    if !current_args.is_empty() {
        commands.push(current_args);
    }

    Ok(commands)
}

pub fn parse_input(input: &str) -> ParseResult {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return ParseResult::Ok(vec![]);
    }

    match parse_tokens(trimmed) {
        Ok(tokenized_commands) => {
            let mut cmds = Vec::new();

            for args in tokenized_commands {
                if args.is_empty() {
                    continue;
                }

                let cmd = args[0].as_str();
                let mut cmd_args = args[1..].to_vec();
                let uses_args = cmd_args.clone();
                cmd_args = cmd_args
                    .iter()
                    .map(|ele| ele.replace("\n", "\\n"))
                    .collect();

                let parsed = match cmd {
                    "ls" => CommandEnum::Ls(cmd_args),
                    "cat" => CommandEnum::Cat(cmd_args),
                    "cp" => CommandEnum::Cp(cmd_args),
                    "pwd" => CommandEnum::Pwd,
                    "cd" => CommandEnum::Cd(cmd_args),
                    "echo" => CommandEnum::Echo(uses_args),
                    "rm" => CommandEnum::Rm(cmd_args),
                    "mkdir" => CommandEnum::Mkdir(uses_args, cmd_args),
                    "exit" => CommandEnum::Exit,
                    "clear" => {
                        clear();
                        continue;
                    }
                    _ => CommandEnum::Unknown(args[0].clone()),
                };
                cmds.push(parsed);
            }
            ParseResult::Ok(cmds)
        }
        Err(_) => ParseResult::Incomplete,
    }
}

pub fn execute_all(cmds: Vec<CommandEnum>, pwd_state: &mut PwdState) -> bool {
    for cmd in cmds {
        let keep_running = execute(cmd, pwd_state);
        if !keep_running {
            return false;
        }
    }
    true
}


pub fn clear() {
    print!("\x1Bc");
}
