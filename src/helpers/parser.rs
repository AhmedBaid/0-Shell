use super::executor::*;
use std::process::Command;
#[derive(Debug)]
pub enum CommandEnum {
    Cp(Vec<String>),
    Pwd,
    Cd(Vec<String>),
    Echo(Vec<String>),
    Mkdir(String),
    Exit,
    Unknown(String),
}

pub fn parse_input(input: &str) -> Vec<CommandEnum> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return vec![];
    }

    let chunks: Vec<&str> = trimmed
        .split("&&")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    let mut cmds = Vec::new();

    for chunk in chunks {
        let parts: Vec<&str> = chunk.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let cmd = parts[0];
        let args = &parts[1..];
        let parsed = match cmd {
            "cp" => CommandEnum::Cp(args.iter().map(|r| r.to_string()).collect()),
            "pwd" => CommandEnum::Pwd,
            "cd" => CommandEnum::Cd(args.to_vec().iter().map(|s| s.to_string()).collect()),
            "echo" => CommandEnum::Echo(args.iter().map(|s| s.to_string()).collect()),
            "mkdir" => CommandEnum::Mkdir(args.get(0).unwrap_or(&"").to_string()),
            "exit" => CommandEnum::Exit,
            "clear" => {
                execute_clear();
                continue;
            }
            _ => CommandEnum::Unknown(cmd.to_string()),
        };

        cmds.push(parsed);
    }

    cmds
}
pub fn execute_all(cmds: Vec<CommandEnum>) -> bool {
    for cmd in cmds {
        let keep_running = execute(cmd);
        if !keep_running {
            return false;
        }
    }
    true
}
pub fn execute_clear() {
    Command::new("clear")
        .status()
        .expect("Failed to execute clear command");
}
