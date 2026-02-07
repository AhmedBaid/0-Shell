use std::io::{ self, Write };

fn quotes_balanced(s: &str) -> bool {
    let dq = s
        .chars()
        .filter(|&c| c == '"')
        .count();
    let sq = s
        .chars()
        .filter(|&c| c == '\'')
        .count();
    dq % 2 == 0 && sq % 2 == 0
}

pub fn echo(args: Vec<String>) {
    let mut buffer = args.join(" ");
    let  mut  ctr = 0;
    while !quotes_balanced(&buffer) {
        ctr += 1;
        print!("> ");
        io::stdout().flush().ok();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() {
            break;
        }
        if ctr == 1 {
            buffer.push_str("\n");
        }

        buffer.push_str(&line);
    }
    let cleaned = buffer.replace('"', "").replace('\'', "").replace("\\n", "\n");

    println!("{}", cleaned);
}
