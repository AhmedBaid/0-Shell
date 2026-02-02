use std::env;
use std::io;
pub mod commands;
pub mod helpers;
use commands::cd::*;
use helpers::parser::*;
use helpers::print_banner::*;

const CYAN: &str = "\x1b[36m";
const RESET: &str = "\x1b[0m";
fn main() {
    print_banner();

    loop {
        let mut prompt = String::new();

        let current_dir = env::current_dir().expect("Failed to get current working directory");
        let user = env::var("USER").unwrap_or("unknown".to_string());
        print!("{CYAN}{}:{}$ {RESET}", user, current_dir.display());

        use std::io::Write;
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut prompt) {
            Ok(0) => continue,
            Ok(_) => {
                let commands = parse_input(&prompt);
                if !execute_all(commands) {
                    break;
                }
            }
            Err(_) => break,
        }
    }
}
