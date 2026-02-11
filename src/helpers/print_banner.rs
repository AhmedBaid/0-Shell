use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

const PURPLE: &str = "\x1b[38;2;160;64;255m";
const RESET: &str = "\x1b[0m";

const BANNER: &str = "
    ██████╗       ███████╗██╗  ██╗███████╗██╗     ██╗     
    ██╔═████╗      ██╔════╝██║  ██║██╔════╝██║     ██║     
    ██║██╔██║█████╗███████╗███████║█████╗  ██║     ██║     
    ████╔╝██║╚════╝╚════██║██╔══██║██╔══╝  ██║     ██║     
    ╚██████╔╝      ███████║██║  ██║███████╗███████╗███████╗
    ╚═════╝       ╚══════╝╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝
";

pub fn welcome() {
    let delay = Duration::from_millis(2);

    print!("{PURPLE}");
    io::stdout().flush().ok();

    for ch in BANNER.chars() {
        print!("{ch}");
        io::stdout().flush().ok();
        sleep(delay);
    }

    println!("{RESET}");
}
