# 🚀 SmartTerm

**A Context-Aware, Intelligent Terminal Wrapper with OS-Level Isolation**

SmartTerm is a next-generation terminal session manager written in Rust. It supercharges your command-line experience by providing project-based history isolation, namespace sandboxing, and seamless, transparent OS-level user context switching.

> 💡 **Anywhere, anytime:** Just type `smart` in your standard terminal, and we'll launch right there with you, keeping your current directory intact!

---

## ✨ Core Features

- **🧠 Context-Aware History** — Commands are saved based on your current directory, project, or namespace.
- **🎯 Zero-Repetition History** — Arrow-up through your history without ever seeing the same `clear` or `build` command repeated. 
- **📋 Smart Copy (`ccopy`)** — Run any command and instantly copy its output to your clipboard just by appending `ccopy`.
- **🔐 OS-Level User Isolation** — Bind specific directories to specific OS users. SmartTerm automatically wraps your commands in `su` or `runas`.
- **🪄 Transparent Proxy Mode** — Secure, invisible password masking. SmartTerm safely handles interactive OS prompts without leaking them into your history.
- **📁 Project & Namespace Sandboxing** — Group related tasks into isolated workspaces (`namespace use <name>`).
- **🛠 Tool-Specific Filtering** — Instantly filter your terminal history for a specific tool using `restore [tool]`.
- **♻️ Self-Updating** — Built-in commands (`core update` / `installer get`) fetch and install the latest native package for your OS without external dependencies.
- **🌍 Cross-Platform** — Native support and automated installers for Linux (`.rpm`, `.deb`), macOS (`.pkg`), and Windows (`.msi`).

---

## 📦 Installation

The easiest way to install **SmartTerm** is via our cross-platform automated scripts. These scripts automatically detect your OS and install the correct package natively.

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

You can download pre-compiled native installers directly from the [Releases](../../releases) page, or build from source:

```bash
git clone https://github.com/bbhcoder/smart_term.git
cd smart_term
cargo build --release
```

---

## 📚 Command Reference

Once launched by typing `smart`, you have access to a powerful suite of internal commands. You can view this list at any time inside the app by typing `help` or `smart help`.

### 1. History & Copy Magic
| Command | Description |
| :--- | :--- |
| `restore <tool>` | Fills the up-arrow history ONLY with previous commands starting with `<tool>` (e.g., `restore docker`). |
| `cexit` | Exits the filtered state and returns to normal history. |
| `<cmd> ccopy` | Executes `<cmd>`, displays the output, and **instantly copies it to your clipboard**. (e.g., `ls -la ccopy`). |
| `namespace use <nm>` | Enters an isolated history namespace (perfect for specific, complex tasks). |
| `namespace exit` | Leaves the current namespace and returns to global history. |

### 2. Users & OS Context
| Command | Description |
| :--- | :--- |
| `user list` | Displays all OS users and SmartTerm registered users. |
| `user info` | Shows the currently active user context. |
| `user add <nm>` | Creates a new OS-level user interactively. |
| `user remove <nm>` | Deletes an OS user. Interactively asks if you want to keep and reassign their history. |
| `user <nm> pass change` | Updates the OS password for the specified user safely. |
| `login user <nm>` | Switches your current SmartTerm session to another user. |
| `user logout` | Reverts to the default user session. |
| `user set default <nm>` | Sets a specific user as the default for new sessions. |

### 3. Directory Binding (Auto-Switching)
Bind a directory so that all commands executed inside it automatically run as a specific OS user.
| Command | Description |
| :--- | :--- |
| `bind user <nm>` | Binds the current directory (or active project) to `<nm>`. Every command run here will automatically be wrapped in `su` or `runas`. |
| `unbind user` | Removes the binding from the current directory. |

### 4. System & Updates
| Command | Description |
| :--- | :--- |
| `core update` | Automatically detects your OS, falls back through native downloaders (`curl`, `wget`, `python`), and updates SmartTerm to the latest release. |
| `installer get` | Alias for `core update`. |
| `help` | Displays the built-in quick reference guide. |

---

## 🏗 Architecture

- **Interceptor Engine** — Parses raw PTY byte streams to intercept special commands before they reach the shell.
- **State Machine** — Employs a multi-stage authentication state machine to securely handle OS password prompts and interactive history reassignment.
- **Local Storage** — Uses SQLite (`rusqlite`) for blazing-fast, relational history and binding lookups.
- **Smart Dispatcher** — Uses native OS fallback chains to ensure features like `ccopy` and `core update` work even on minimal distributions.

---

## 🔄 Releasing (CI/CD)

To publish a new release, tag your commit. This triggers the GitHub Actions pipeline to automatically build the `.rpm`, `.msi`, and `.pkg` installers:

```bash
git add .
git commit -m "🚀 V1.0 Release: History isolation, Smart Copy, and Native Updaters"
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

---

## 📄 License

This project is licensed under the **MIT License**.

**Author:** Arsalan Shoaei Ahmad Abad