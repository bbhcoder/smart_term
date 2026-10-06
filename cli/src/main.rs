use clap::{Parser, Subcommand};
use commands::{hook, interactive, update, history, prompt, bind};
use std::{env, fs, process::{Command, Stdio}, thread, time::Duration};

pub mod commands; pub mod ipc; pub mod deploy;

#[derive(Parser)] pub struct Cli { #[command(subcommand)] pub command: Option<Commands> }

#[derive(Subcommand)]
pub enum Commands {
    Hook { #[command(subcommand)] action: HookCommands },
    Namespace { #[command(subcommand)] action: NamespaceCommands },
    User { #[command(subcommand)] action: UserCommands },
    Project { #[command(subcommand)] action: ProjectCommands },
    History { limit: Option<u32>, #[arg(short, long)] all: bool },
    HistNav { index: usize }, Init { shell_name: String },
    Prompt, Update, Org, Exit, Rollback, Deploy, Reset, ResetAll,
    Bind { #[command(subcommand)] action: BindCommands },
    Unset { target: Option<String> }, Unbind,
}

#[derive(Subcommand)] pub enum HookCommands { PreExec { command: String }, PreCmd { exit_code: i32 } }
#[derive(Subcommand)] pub enum NamespaceCommands { Use { name: String }, Exit, List }
#[derive(Subcommand)] pub enum UserCommands { Login { username: String }, Logout, List }
#[derive(Subcommand)] pub enum ProjectCommands { Set { name: String }, Clear }
#[derive(Subcommand)] pub enum BindCommands { Project { name: String }, User { username: String }, Clear }

fn enter_dev_subshell() {
    let exe = env::current_exe().unwrap();
    let smartd_path = exe.parent().unwrap().join("smartd");
    let base = format!("{}/.smart_dev", env::var("HOME").unwrap_or_else(|_| "/tmp".into()));
    let _ = fs::create_dir_all(&base);
    let _ = fs::write(format!("{}/source_path", base), env::current_dir().unwrap().to_string_lossy().as_ref());
    #[cfg(unix)] let _ = Command::new("pkill").arg("-f").arg(smartd_path.to_str().unwrap()).status();
    let mut d = Command::new(&smartd_path).env("SMART_BASE_DIR", &base).stdout(Stdio::null()).stderr(Stdio::null()).spawn().unwrap();
    thread::sleep(Duration::from_millis(500));
    println!("\x1b[34m[SmartTerm Dev Sandbox]\x1b[0m Native Subshell Active. Type 'exit' to leave.");
    let shell = env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
    if shell.contains("zsh") {
        let rc = format!("{}/.zshrc", base);
        let _ = fs::write(&rc, format!("source ~/.zshrc\nexport HISTFILE=\"{}/.zsh_history\"\nsmart() {{ \"{}\" \"$@\"; }}\neval \"$(\"{}\" init zsh)\"\n", base, exe.display(), exe.display()));
        let _ = Command::new(&shell).env("ZDOTDIR", &base).env("SMART_BASE_DIR", &base).status();
    } else {
        let rc = format!("{}/.bashrc", base);
        let _ = fs::write(&rc, format!("source ~/.bashrc\nexport HISTFILE=\"{}/.bash_history\"\nsmart() {{ \"{}\" \"$@\"; }}\neval \"$(\"{}\" init bash)\"\n", base, exe.display(), exe.display()));
        let _ = Command::new(&shell).arg("--rcfile").arg(&rc).env("SMART_BASE_DIR", &base).status();
    }
    let _ = d.kill();
    println!("\x1b[33m[System] Dev Sandbox Closed.\x1b[0m");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_dev = env::current_exe().map(|p| p.to_string_lossy().contains("target")).unwrap_or(false);
    if args.len() == 2 {
        let a = args[1].as_str();
        if ["-v", "--v", "--version", "version"].contains(&a) { println!("SmartTerm v{}", env!("CARGO_PKG_VERSION")); return; }
        if ["-up", "--up", "--update", "update"].contains(&a) { update::execute(); return; }
    }
    if args.len() == 1 { if is_dev { enter_dev_subshell(); } else { println!("\x1b[34m[SmartTerm]\x1b[0m v{} Active.", env!("CARGO_PKG_VERSION")); } return; }
    match Cli::try_parse() {
        Ok(cli) => if let Some(cmd) = cli.command { match cmd {
            Commands::Hook { action } => hook::handle(action), Commands::Namespace { action } => interactive::handle_namespace(action),
            Commands::User { action } => interactive::handle_user(action), Commands::Project { action } => interactive::handle_project(action),
            Commands::History { limit, all } => history::handle(limit.unwrap_or(50), all), Commands::HistNav { index } => history::handle_nav(index),
            Commands::Init { shell_name } => print!("{}", match shell_name.as_str() { "zsh" => shell::generators::zsh::generate(), "bash" => shell::generators::bash::generate(), "powershell" => shell::generators::powershell::generate(), _ => String::new() }),
            Commands::Prompt => prompt::handle(), Commands::Update => update::execute(), Commands::Org => deploy::enable_org(),
            Commands::Exit => deploy::disable_org(), Commands::Rollback => deploy::rollback(), Commands::Deploy => deploy::deploy(&env::current_dir().unwrap().to_string_lossy()),
            Commands::Reset => history::handle_reset(), Commands::ResetAll => history::handle_reset_all(), Commands::Bind { action } => bind::handle(action),
            Commands::Unset { target } => interactive::handle_unset(target), Commands::Unbind => bind::handle_unbind(),
        } } Err(e) => { let _ = e.print(); }
    }
}
