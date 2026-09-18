use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::{env, io::{Read, Write}, thread, sync::{Arc, Mutex}};
use crate::parser::interceptor::CommandInterceptor;

pub fn run_pty() -> Result<(), Box<dyn std::error::Error>> {
    let pty_system = NativePtySystem::default();
    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let pair = pty_system.openpty(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 })?;
    
    let shell = if cfg!(target_os = "windows") { "cmd" } else { "bash" };
    let mut cmd = CommandBuilder::new(shell);
    cmd.env("SMART_TERM_ACTIVE", "1");
    if let Ok(d) = env::current_dir() { cmd.cwd(d); }
    let mut child = pair.slave.spawn_command(cmd)?;
    let mut reader = pair.master.try_clone_reader()?;
    
    let writer = Arc::new(Mutex::new(pair.master.take_writer()?));
    let w_clone = Arc::clone(&writer);

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_millis(150));
        let is_dev = env::var("CARGO_MANIFEST_DIR").is_ok();
        let msg = if is_dev { "\\e[1;35m[System] Smart Terminal (DEV MODE) Active\\e[0m" } else { "\\e[1;36m[System] Smart Terminal Mode Active\\e[0m" };
        if !cfg!(target_os = "windows") {
            // حذف export برای جلوگیری از ارث‌بری پرامپت در پروسه‌های فرزند و تمایز [dev] با [s]
            let p_tag = if is_dev { "\\[\\e[1;35m\\][dev]\\[\\e[0m\\]" } else { "\\[\\e[1;32m\\][s]\\[\\e[0m\\]" };
            let inject = format!("stty -echo; PS1=\"{} $PS1\"; clear; echo -e '\\r\\n{}'; stty echo\n", p_tag, msg);
            let _ = w_clone.lock().unwrap().write_all(inject.as_bytes());
        }
    });

    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            let _ = std::io::stdout().write_all(&buf[..n]);
            let _ = std::io::stdout().flush();
        }
    });

    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        let mut interceptor = CommandInterceptor::new();
        while let Ok(n) = std::io::stdin().read(&mut buf) {
            if n == 0 { break; }
            let out_bytes = interceptor.process_bytes(&buf[..n]);
            if let Ok(mut w) = writer.lock() {
                let _ = w.write_all(&out_bytes);
                let _ = w.flush();
            }
        }
    });

    child.wait()?;
    Ok(())
}