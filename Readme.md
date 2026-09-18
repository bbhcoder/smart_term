# 🚀 SmartTerm

**A Context-Aware, Intelligent Terminal Wrapper with OS-Level Isolation**

SmartTerm is a next-generation terminal session manager written in Rust. It supercharges your command-line experience by providing project-based history isolation, namespace sandboxing, seamless OS-level user context switching, and smart output copying.

> 💡 **Always with you:** SmartTerm can integrate into your `.bashrc`, `.zshrc`, or PowerShell `$PROFILE` during installation to automatically launch in every new terminal window, keeping you in your favorite environment.

---

## ✨ Core Features

- **🧠 Context-Aware & Deduplicated History** — Commands are saved based on your current directory, project, or namespace. When you press the up-arrow, you'll never see repetitive commands (like multiple `clear`s) again!
- **📋 Smart Copy (`ccopy`)** — Run any command and instantly copy its output to your system clipboard just by appending `ccopy` (e.g., `docker logs api ccopy`).
- **🔐 OS-Level User Isolation** — Bind specific directories to specific OS users. SmartTerm automatically wraps your commands in `su` or `runas` when you enter a bound directory.
- **🪄 Transparent Proxy Mode** — Secure, invisible password masking. SmartTerm safely handles interactive OS prompts (like `sudo` or user creation) without leaking them into your command history.
- **📁 Project & Namespace Sandboxing** — Group related tasks into isolated workspaces (`namespace use <name>`).
- **🛠 Tool-Specific Filtering** — Instantly filter your terminal history for a specific tool using `restore [tool]` (e.g., `restore docker`).
- **♻️ Self-Updating** — A single built-in command (`core update` / `installer get`) fetches and natively installs the latest package for your OS, falling back smartly between `curl`, `wget`, or `python3`.
- **🌍 Cross-Platform** — Native support and automated installers for Linux (`.rpm`, `.deb`), macOS (`.pkg`), and Windows (`.msi`).

---

## 📦 Installation

The easiest way to install **SmartTerm** is via our cross-platform automated scripts. These scripts automatically detect your OS, install the correct package natively, and optionally set SmartTerm as your default shell.

### Linux & macOS

```bash
curl -fsSL https://raw.githubusercontent.com/bbhcoder/smart_term/main/install.sh | bash
```

*(If you don't have `curl`, you can use `wget`: `wget -qO- https://raw.githubusercontent.com/bbhcoder/smart_term/main/install.sh | bash`)*

### Windows (PowerShell)

Open PowerShell **as Administrator** and run:

```powershell
irm https://raw.githubusercontent.com/bbhcoder/smart_term/main/install.ps1 | iex
```

### Manual Download & Build

You can download the pre-compiled native installers directly from the [Releases](../../releases) page, or build from source:

```bash
git clone https://github.com/bbhcoder/smart_term.git
cd smart_term
cargo build --release
```

---

## 🚀 Quick Start & Command Reference

SmartTerm acts as a transparent layer over your default shell. Just type `smart` from anywhere (it preserves your current directory!), or set it as default during installation. 

Type `help` or `smart help` at any time to see this list in your terminal.

### 1. History & Copy Magic

| Command | Description |
| :--- | :--- |
| `restore <tool>` | Fills the up-arrow history ONLY with previous commands starting with `<tool>` (e.g., `restore docker`). |
| `cexit` | Exits the filtered state and returns to normal history. |
| `<cmd> ccopy` | Executes `<cmd>`, prints the output, and **instantly copies it to your clipboard**. |
| `namespace use <nm>`| Enters an isolated history namespace (perfect for specific, complex tasks). |
| `namespace exit` | Leaves the current namespace and returns to global history. |

### 2. Users & OS Context

| Command | Description |
| :--- | :--- |
| `user list` | Displays all OS users and SmartTerm registered users. |
| `user info` | Shows the currently active user context. |
| `user add <nm>` | Creates a new OS-level user interactively. |
| `user remove <nm>` | Deletes an OS user and interactively asks to safely reassign their history. |
| `user <nm> pass change`| Updates the OS password for the specified user safely. |
| `login user <nm>` | Switches your current SmartTerm session to another user. |
| `user logout` | Reverts to the default user session. |
| `user set default <nm>`| Sets a specific user as the default for new sessions. |

### 3. Directory Binding (Auto-Switching)

Bind a directory so that all commands executed inside it automatically run as a specific OS user.

| Command | Description |
| :--- | :--- |
| `bind user <nm>` | Binds the current directory to `<nm>`. Every command run here will automatically be wrapped in `su` or `runas`. |
| `unbind user` | Removes the binding from the current directory. |

### 4. System & Updates

| Command | Description |
| :--- | :--- |
| `core update` | Safely detects your OS and updates SmartTerm to the latest release natively. |
| `installer get` | Alias for `core update`. |
| `help` | Displays the built-in quick reference guide. |

---

## 🏗 Architecture

- **Interceptor Engine** — Parses raw PTY byte streams to intercept special commands before they reach the shell.
- **State Machine** — Employs a multi-stage authentication state machine to securely handle OS password prompts and interactive history reassignment without leakage.
- **Local Storage** — Uses SQLite (`rusqlite`) in `~/.smart_term_history.sqlite` for blazing-fast, relational history, CWD tracking, and binding lookups.
- **Smart Dispatcher** — Uses native OS fallback chains (`curl` -> `wget` -> `python3` -> `dpkg`/`rpm`/`installer`/`msiexec`) to ensure features like `core update` work on minimal distributions.

---

## 🔄 Releasing (CI/CD)

To publish a new release, tag your commit. This triggers the GitHub Actions pipeline to automatically build the `.rpm`, `.msi`, and `.pkg` installers in the cloud:

```bash
git add .
git commit -m "🚀 V1.0 Release"
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

---

## 📄 License

This project is licensed under the **MIT License**.

**Author:** Arsalan Shoaei Ahmad Abad