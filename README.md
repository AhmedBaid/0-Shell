# 0-shell 🐚 (Mini Shell Project)

## 📌 Project Overview

**0-shell** is a minimalist Unix-like shell implemented in **Rust**.  
It is designed to demystify how a shell interacts with the Operating System. It parses user input—handling quotes, escape characters, and command chaining—and executes commands by communicating directly with the OS kernel.

## 🎯 Objectives
* **Build** a functional, interactive command-line interface (CLI).
* **Parse** complex input patterns like `bash` or `zsh` (e.g., `"double quotes"`, `'single quotes'`, `escape\ chars`).
* **Manage** internal state for navigation (`pwd`, `cd`).
* **Execute** processes and handle system calls.

---

## 🐚 Architecture & Concepts

### What is a Shell?
A shell is the user's interface to the Operating System. It operates in a continuous loop: **Read → Parse → Execute → Print** (often called a REPL).

### Low-Level Terminology

To understand 0-shell, you must understand the OS components it interacts with:

| Concept | Definition |
| :--- | :--- |
| **Program** | A static binary file stored on the disk (e.g., `/bin/ls`). |
| **Process** | A program *in execution* inside RAM. It has its own memory space and PID. |
| **Thread** | A unit of execution inside a process. Threads share the process's memory but have their own stack. |
| **Kernel** | The core of the OS. It manages hardware (CPU, RAM, Disk) and security. |
| **Syscall** | A request from a user program (like 0-shell) to the Kernel (e.g., `open()`, `fork()`, `execve()`). |

---

## ⚙️ Supported Features

| Category | Commands |
| :--- | :--- |
| **Navigation** | `cd`, `pwd`, `cd -` (Back to previous dir) |
| **File Ops** | `ls`, `mkdir`, `rm`, `cp`, `mv` |
| **I/O** | `echo`, `cat` |
| **System** | `clear`, `exit` |
| **Parsing** | `&&` (Chaining), `' '` (Literal), `" "` (Weak quote), `\` (Escape) |

---

## 🧪 Detailed Test Cases

Below are the verification scenarios to ensure the shell behaves like a standard POSIX shell. Each command has its own test suite to verify basic functionality and edge cases.

### 🧭 `pwd` (Print Working Directory)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Basic** | `pwd` | Prints the current absolute path. |
| **After Navigation**| `cd /tmp && pwd` | Prints `/tmp`. |
| **With arguments** | `pwd extra_args` | Ignores arguments and prints the current path. |

### 📂 `cd` (Change Directory)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Home Directory** | `cd` | Changes directory to the user's `$HOME`. |
| **Absolute Path** | `cd /var/log` | Changes directory directly to `/var/log`. |
| **Relative Path** | `cd src` | Moves into the `src` folder in the current directory. |
| **Parent Dir** | `cd ..` | Moves up one level in the directory tree. |
| **Current Dir** | `cd .` | Stays in the exact same directory. |
| **Previous Dir** | `cd -` | Toggles back to the previous directory (`OLDPWD`). |
| **Path with Spaces**| `cd "my folder"` | Successfully enters `my folder`. |
| **Missing Dir** | `cd does_not_exist` | Error: `cd: no such file or directory`. |
| **Not a Dir** | `cd main.rs` | Error: `cd: not a directory`. |

### 📋 `ls` (List Directory Contents)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Current Dir** | `ls` | Lists files and folders in the current directory. |
| **Specific Dir** | `ls /tmp` | Lists contents of the `/tmp` directory. |
| **Multiple Dirs** | `ls src /tmp` | Lists contents of `src`, then prints a header and lists `/tmp`. |
| **Missing Dir** | `ls fake_folder` | Error: `ls: cannot access 'fake_folder'`. |
| **Flags Support** | `ls -l` / `ls -a` | Outputs detailed list or includes hidden `.` files. |

### 📁 `mkdir` (Make Directory)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Single Dir** | `mkdir test_dir` | Creates a folder named `test_dir`. |
| **Multiple Dirs** | `mkdir dir1 dir2` | Creates both `dir1` and `dir2`. |
| **Quoted Space** | `mkdir "hello world"` | Creates *one* folder named `hello world`. |
| **Escaped Space** | `mkdir my\ folder` | Creates *one* folder named `my folder`. |
| **Already Exists** | `mkdir existing_dir`| Error: `mkdir: cannot create directory 'existing_dir': File exists`. |

### 🗑️ `rm` (Remove Files)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Single File** | `rm file.txt` | Deletes `file.txt`. |
| **Multiple Files** | `rm a.txt b.txt` | Deletes both `a.txt` and `b.txt`. |
| **Space in Name** | `rm "bad name.txt"` | Deletes the specific file containing a space. |
| **Missing File** | `rm ghost.txt` | Error: `rm: cannot remove 'ghost.txt': No such file or directory`. |
| **Is a Directory** | `rm my_folder` | Error: `rm: cannot remove 'my_folder': Is a directory`. |

### ✂️ `cp` (Copy Files)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Basic Copy** | `cp a.txt b.txt` | Creates a copy of `a.txt` named `b.txt`. |
| **Overwrite** | `cp a.txt existing.txt` | Overwrites `existing.txt` with the contents of `a.txt`. |
| **Copy to Dir** | `cp a.txt folder/` | Copies `a.txt` *into* the `folder` directory. |
| **Multiple to Dir** | `cp a.txt b.txt folder/`| Copies both files into the `folder` directory. |
| **Missing Source** | `cp fake.txt b.txt` | Error: `cp: cannot stat 'fake.txt': No such file`. |
| **Missing Dest Dir**| `cp a.txt fake_dir/` | Error: `cp: target 'fake_dir/' is not a directory`. |
| **Spaces in Names** | `cp "my file.txt" dest`| Successfully copies a file containing spaces. |

### 🚚 `mv` (Move / Rename Files)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Rename File** | `mv old.txt new.txt` | Renames `old.txt` to `new.txt` in the same directory. |
| **Move to Dir** | `mv file.txt folder/` | Moves `file.txt` *into* the `folder` directory. |
| **Multiple to Dir** | `mv a.txt b.txt folder/`| Moves both files into the `folder` directory. |
| **Overwrite** | `mv a.txt b.txt` | Renames `a.txt` to `b.txt`, deleting the original `b.txt`. |
| **Missing Source** | `mv fake.txt b.txt` | Error: `mv: cannot stat 'fake.txt': No such file`. |
| **Missing Dest Dir**| `mv a.txt fake_dir/` | Error: `mv: target 'fake_dir/' is not a directory`. |
| **Spaces in Names** | `mv "old name" new` | Successfully renames a file containing spaces. |

### 🗣️ `echo` (Print Text)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Basic** | `echo hello` | Prints `hello`. |
| **Multiple Words** | `echo hello world` | Prints `hello world`. |
| **Double Quotes** | `echo "hello world"` | Prints `hello world` (quotes are stripped). |
| **Single Quotes** | `echo 'hello world'` | Prints `hello world`. |
| **Escaped Quotes** | `echo \"hello\"` | Prints `"hello"`. |
| **No Arguments** | `echo` | Prints a blank newline. |

### 📖 `cat` (Concatenate & Print Files)
| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Read File** | `cat file.txt` | Prints the contents of `file.txt` to the terminal. |
| **Multiple Files** | `cat a.txt b.txt` | Prints the contents of A, immediately followed by B. |
| **Missing File** | `cat fake.txt` | Error: `cat: fake.txt: No such file or directory`. |
| **Is a Directory** | `cat src/` | Error: `cat: src/: Is a directory`. |
| **Interactive Mode**| `cat` (no args) | Waits for user input, echoes it back until `Ctrl+C` or `Ctrl+D`. |

### 🧠 Core Parser & Syntax Logic
These tests verify that the shell correctly interprets special characters before sending them to the commands.

| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Valid Chain** | `echo A && echo B` | Prints `A`, then prints `B`. |
| **Failed Chain** | `cd fake && echo B` | Fails on `cd`. Does **not** print `B`. |
| **Concatenation** | `echo "ab""cd"` | Prints `abcd`. |
| **Mixed Quotes** | `echo "hello"'world'` | Prints `helloworld`. |
| **Escaped Space** | `mkdir one\ two` | Creates a single directory named `one two`. |
| **Literal Escape** | `echo 'hello\ world'` | Prints `hello\ world` (Backslash is literal inside single quotes). |
| **Chain in Quote** | `echo "hello && world"`| Prints `hello && world` (Treats `&&` as raw text, does not execute `world`). |

---

## 🏗️ Build & Run

Ensure you have Rust installed.

```bash
# 1. Clone the project
git clone [https://github.com/AhmedBaid/0-Shell](https://github.com/AhmedBaid/0-Shell)

# 2. Open the directory in your editor
code 0-Shell

# 3. Compile and run the shell
cargo run