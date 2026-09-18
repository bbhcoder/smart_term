# 🚀 SmartTerm

**A Context-Aware, Intelligent Terminal Wrapper with OS-Level Isolation**

SmartTerm is a next-generation terminal session manager written in Rust. It supercharges your command-line experience by providing project-based history isolation, namespace sandboxing, and seamless, transparent OS-level user context switching.

---

## ✨ Core Features

- **🧠 Context-Aware History** — Commands are saved based on your current directory, project, or namespace. No more cluttered global history!
- **🔐 OS-Level User Isolation** — Bind specific directories to specific OS users. SmartTerm automatically wraps your commands in `su` or `runas` when you enter a bound directory.
- **🪄 Transparent Proxy Mode** — Secure, invisible password masking. SmartTerm safely handles interactive OS prompts (like `sudo` or `su` passwords) without leaking them into your command history.
- **📁 Project & Namespace Sandboxing** — Group related tasks into isolated workspaces (`namespace use <name>`).
- **🛠 Tool-Specific Filtering** — Instantly filter your terminal history for a specific tool using `restore [tool]` (e.g., `restore docker`).
- **🌍 Cross-Platform** — Native support and automated installers for Linux (`.rpm`, `.deb`), macOS (`.pkg`), and Windows (`.msi`).

---

## 📦 Installation

You can download the pre-compiled native installers from the [Releases](../../releases) page.

Alternatively, to build from source:

```bash
cargo build --release
```

---

## 🚀 Quick Start & Usage

SmartTerm acts as a transparent layer over your default shell. Once launched, try these powerful internal commands:

### 1. Context & Tool Filtering

```bash
restore docker     # Fills the up-arrow history ONLY with previous docker commands
cexit               # Exits the filtered state and returns to normal history
```

### 2. OS User Binding (Auto Context-Switching)

Bind a directory so that all commands executed inside it run as a specific OS user.

```bash
bind user postgres  # Binds the current directory to the 'postgres' user
whoami               # SmartTerm automatically runs: su - postgres -c 'whoami'
cd ~                 # Leaving the directory automatically returns you to your default user
```

### 3. Namespace Isolation

Create isolated environments for complex tasks.

```bash
namespace use infra_setup   # Enter an isolated history namespace
# ... run commands ...
namespace exit               # Return to the global history
```

### 4. User Management (Cross-Platform)

SmartTerm integrates directly with your OS user management (`useradd`, `sysadminctl`, `net user`).

```bash
user list
user add new_dev
user new_dev pass change
```

---

## 🏗 Architecture

- **Interceptor Engine** — Parses raw PTY byte streams to intercept special commands before they reach the shell.
- **State Machine** — Employs a multi-stage authentication state machine to securely handle OS password prompts and prevent history leakage.
- **Local Storage** — Uses SQLite (`rusqlite`) for blazing-fast, relational history and binding lookups.

---

## 🔄 Releasing (CI/CD)

Once you're ready to publish a new release, save this file as `README.md` in your project root, then push your code and tag the release. This triggers the GitHub Actions pipeline that automatically builds the `.rpm`, `.msi`, and `.pkg` installers in the cloud:

```bash
git add .
git commit -m "🚀 Initial release: Core engine, OS integration, and CI/CD pipelines"
git tag v1.0.0
git push origin main
git push origin v1.0.0
```

---

## 📄 License

This project is licensed under the **MIT License**.

**Author:** Arsalan Shoaei Ahmad Abad