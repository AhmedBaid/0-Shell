# 0-Shell

Minimal Unix-like shell written in Rust.

## Status (half-scope)
This implementation targets roughly half of the project requirements:

Implemented commands:
- `echo`
- `pwd`
- `cd`
- `ls` (supports `-a` and `-F`)
- `exit`

Not yet implemented (examples):
- `cat`, `cp`, `rm`, `mv`, `mkdir`
- `ls -l`

## Usage
```bash
cargo run
```

Example session:
```text
$ pwd
/workspace/0-Shell
$ ls -a -F
./ ../ .git/ Cargo.toml README.md src/
$ echo "Hello There"
Hello There
$ exit
```
