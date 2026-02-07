pub fn echo(args: Vec<String>) {
    let buffer = args.join(" ");
    let cleaned = buffer.replace('"', "").replace('\'', "");

    println!("{}", cleaned);
}
