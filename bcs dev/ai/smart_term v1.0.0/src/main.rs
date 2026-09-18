mod pty;
mod parser;
mod history;
mod auth;

fn main() {
    if let Err(e) = history::db::init_db() {
        eprintln!("Database Init Error: {}", e);
        return;
    }

    let _raw_mode = pty::io_handler::RawModeGuard::new().unwrap();
    if let Err(e) = pty::spawner::run_pty() {
        eprintln!("Error: {}", e);
    }
}