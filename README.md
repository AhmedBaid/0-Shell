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
A shell is the user's interface to the Operating System. It operates in a continuous loop: **Read → Parse → Execute → Print**.



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
| **File Ops** | `ls`, `mkdir`, `rm`, `cp` |
| **I/O** | `echo`, `cat` |
| **System** | `clear`, `exit` |
| **Parsing** | `&&` (Chaining), `' '` (Literal), `" "` (Weak quote), `\` (Escape) |

---

## 🧪 Test Cases

Below are the verification scenarios to ensure the shell behaves like a standard POSIX shell.

### 📂 Navigation Tests (`pwd`, `cd`)

| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Basic PWD** | `pwd` | Prints current absolute path. |
| **Chain PWD** | `pwd && pwd` | Prints the path twice. |
| **Change Dir** | `mkdir test && cd test && pwd` | Output ends with `/test`. |
| **Parent Dir** | `cd .. && pwd` | Output moves up one level. |
| **Absolute Path** | `cd /tmp && pwd` | Current directory becomes `/tmp`. |
| **Home Dir** | `cd` | Changes directory to user `$HOME`. |
| **Previous Dir** | `cd -` | Toggles back to the previous directory (OLDPWD). |
| **Invalid Dir** | `cd notfound` | Error: "No such file or directory". |

### 🔨 File Manipulation (`mkdir`, `rm`, `cp`)

| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Single Dir** | `mkdir folder1` | Creates `folder1`. |
| **Multiple Dirs** | `mkdir a b c` | Creates folders `a`, `b`, and `c`. |
| **Space (Quote)** | `mkdir "hello world"` | Creates one folder named `hello world`. |
| **Space (Escape)** | `mkdir hello\ world` | Creates one folder named `hello world`. |
| **Backslash Name** | `mkdir hello\\` | Creates folder named `hello\`. |
| **Nested Quote** | `mkdir "22\"g"` | Creates folder named `22"g`. |
| **Copy File** | `echo hi > a.txt && cp a.txt b.txt` | `b.txt` is created containing "hi". |
| **Remove File** | `rm a.txt` | File `a.txt` is deleted. |
| **Remove Missing** | `rm missing.txt` | Error message displayed. |

### 📄 Output & Display (`echo`, `cat`, `ls`)

| Scenario | Input Command | Expected Behavior |
| :--- | :--- | :--- |
| **Basic Echo** | `echo hello` | Prints `hello`. |
| **Quoted Echo** | `echo "hello world"` | Prints `hello world` (quotes removed). |
| **Literal Echo** | `echo 'hello world'` | Prints `hello world`. |
| **Escaped Quote** | `echo \"hello\"` | Prints `"hello"`. |
| **Cat File** | `cat file.txt` | Displays content of `file.txt`. |
| **Cat Multiple** | `cat a.txt b.txt` | Prints content of A followed by B. |
| **List Dir** | `ls` | Lists current directory contents. |
| **List Specific** | `ls /tmp` | Lists contents of `/tmp`. |

### 🧠 Advanced Parsing Logic

These tests verify the core logic of the shell's parser.

| Scenario | Input Command | Result Interpretation |
| :--- | :--- | :--- |
| **Concatenation** | `mkdir "ab""cd"` | Creates directory `abcd`. |
| **Mixed Quotes** | `mkdir "hello"'world'` | Creates directory `helloworld`. |
| **Escaped Space** | `mkdir one\ two\ three` | Creates directory `one two three`. |
| **Literal Escape** | `echo 'hello\ world'` | Prints `hello\ world` (Backslash preserved inside single quotes). |
| **Chain in Quote** | `echo "hello && world"` | Prints `hello && world` (Does not execute `world`). |
| **Valid Chain** | `echo A && echo B` | Prints `A`, then prints `B`. |
| **Failed Chain** | `rm missing.txt && echo B` | Prints error for `rm`, **does not** print `B`. |

---

## 🏗️ Build & Run

Ensure you have Rust installed.

```bash
# 1. Clone the project
git clone [repo_link](https://github.com/AhmedBaid/0-Shell)
code 0-shell

# 3. Run the shell
cargo run