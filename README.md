# 🚀 SmartTerm

A blazing-fast, daemon-based terminal context and history manager built in Rust. SmartTerm seamlessly integrates with your favorite shell (Bash, Zsh, PowerShell) to track command history, manage execution contexts, and persist session states using a sub-millisecond IPC architecture.

## ✨ Core Features

*   **Native Shell Hooks:** Zero-latency `preexec` and `precmd` hooks for Bash, Zsh, and PowerShell. No noticeable delay in your daily workflow.
*   **Daemon Architecture:** A background `smartd` service handles heavy database lifting and state synchronization without blocking your terminal UI.
*   **Ultra-Fast IPC:** Uses Unix Domain Sockets (Linux/macOS) and Named Pipes (Windows) for instantaneous communication.
*   **Hierarchical Auto-Context (Bindings):** Automatically switch your active Namespace, Project, or User just by `cd`-ing into a directory. SmartTerm scans directory ancestors, so you stay in context even deep inside subdirectories!
*   **Context Isolation:** Strictly separate your command history based on your current Namespace, Project, or User.
*   **Smart History Management:** Clean up your tracks surgically with context-aware history resets.

---

## 📦 Installation

### Linux & macOS (Bash / Zsh)
Install the binaries and inject the shell hook in one command:
```bash
curl -sSL https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.sh | bash
```

### Windows (PowerShell)
Install the binaries and add the key-handlers to your PowerShell profile (Automatically configures Windows Defender exclusions):
```powershell
irm https://raw.githubusercontent.com/bbhcoder/smart_term2/main/install.ps1 | iex
```

---

## 🚀 Getting Started

1.  **Start the Daemon:** 
    The daemon must be running in the background to process hooks and manage the SQLite database.
    ```bash
    smartd &
    ```
2.  **Restart your Terminal:** 
    Open a new terminal window to ensure the shell hooks are loaded.
3.  **Start Typing!**
    Every command you execute is now securely synced to your local SQLite database with its associated context. Use the **Up Arrow** to navigate your completely isolated history.

---

## 🛠️ Command Reference

SmartTerm provides an intuitive CLI to manage your current terminal context.

### 🔗 Directory Bindings (Auto-Context)
Lock a specific directory (and all its subdirectories) to a project or user.
*   `smart bind project <name>` - Bind current directory to a project.
*   `smart bind user <username>` - Bind current directory to a user.
*   `smart unbind` - Remove the binding from the current directory.

### 🏢 Context Management
Manually set or clear your active session state.
*   **Namespaces:**
    *   `smart namespace use <name>` - Switch to a namespace (prompts for project).
    *   `smart namespace list` - View available namespaces.
*   **Projects:**
    *   `smart project set <name>` - Change the active project.
    *   `smart project clear` - Clear the active project.
*   **Users:**
    *   `smart user login <username>` - Switch the active user.
    *   `smart user list` - View available users.
*   **Quick Clear:**
    *   `smart unset [all|project|namespace|user]` - Quickly clear specific states from your prompt.

### 📜 History & Data Management
*   `smart reset` - Delete all command history for your *current isolated context and path*.
*   `smart reset-all` - Permanently wipe the entire history database.
*   `smart history` - View the history of the current context.

### 🔄 Updates
*   `smart --update` (or `-up`) - Check for updates and automatically recompile/download the latest version.

---

## 🏗️ Workspace Architecture

SmartTerm is designed as a modular Rust workspace for maximum maintainability:

*   **`cli` (smart):** The frontend command-line interface. Routes interactive commands and invisible shell hooks to the daemon.
*   **`smartd`:** The background Tokio async daemon. Hosts the IPC server and manages the shared in-memory state cache.
*   **`smartcore`:** The brain. Contains ABI protocol definitions (serde), the `preexec`/`precmd` execution engine, and the hierarchical path scanning logic.
*   **`db`:** Handles all SQLite interactions (WAL-mode), schema migrations, and queries for history, bindings, and states.
*   **`shell`:** Generates native, non-intrusive shell scripts for Bash, Zsh, and PowerShell to safely hook into the terminal lifecycle.

## 🗄️ Database
All data is stored locally in `~/.smart_dev/smart_term.db`. You can query it directly using `sqlite3`.

## 📄 License
MIT License