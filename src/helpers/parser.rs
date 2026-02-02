use super::executor::*;
#[derive(Debug)]
pub enum Command {
    Pwd,
    Cd(String),
    Echo(Vec<String>),
    Mkdir(String),
    Exit,
    Unknown(String),
}

pub fn parse_input(input: &str) -> Vec<Command> {
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
            "pwd" => Command::Pwd,
            "cd" => Command::Cd(args.get(0).unwrap_or(&"/").to_string()),
            "echo" => Command::Echo(args.iter().map(|s| s.to_string()).collect()),
            "mkdir" => Command::Mkdir(args.get(0).unwrap_or(&"").to_string()),
            "exit" => Command::Exit,
            _ => Command::Unknown(cmd.to_string()),
        };

        cmds.push(parsed);
    }

    cmds
}
pub fn execute_all(cmds: Vec<Command>) -> bool {
    for cmd in cmds {
        let keep_running = execute(cmd);
        if !keep_running {
            return false;
        }
    }
    true
}
