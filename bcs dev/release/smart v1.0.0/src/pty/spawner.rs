use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::thread;
use crate::parser::interceptor::CommandInterceptor;

pub fn run_pty() -> Result<(), Box<dyn std::error::Error>> {
    let pty_system = NativePtySystem::default();
    
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    let shell = if cfg!(target_os = "windows") { "cmd" } else { "bash" };
    let cmd = CommandBuilder::new(shell);
    let mut child = pair.slave.spawn_command(cmd)?;

    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;

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
            let _ = writer.write_all(&out_bytes);
            let _ = writer.flush();
        }
    });

    child.wait()?;
    Ok(())
}