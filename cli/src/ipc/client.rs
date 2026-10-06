use smartcore::abi::messages::{ClientRequest, DaemonResponse};
use smartcore::abi::protocol::{deserialize_response, serialize_request};
use std::env;
use std::io::{Read, Write};

fn get_sock_path() -> String {
    let base_dir = env::var("SMART_BASE_DIR").unwrap_or_else(|_| {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/tmp"));
        format!("{}/.smart_dev", home)
    });
    format!("{}/smart_term.sock", base_dir)
}

#[cfg(unix)]
pub fn send_request(req: ClientRequest) -> Result<DaemonResponse, String> {
    use std::os::unix::net::UnixStream;
    let mut stream = UnixStream::connect(get_sock_path()).map_err(|e| e.to_string())?;
    let req_str = serialize_request(&req).map_err(|e| e.to_string())?;
    stream.write_all(req_str.as_bytes()).map_err(|e| e.to_string())?;
    let mut buf = Vec::new();
    stream.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    let res_str = String::from_utf8(buf).map_err(|e| e.to_string())?;
    deserialize_response(&res_str).map_err(|e| e.to_string())
}

#[cfg(unix)]
pub fn send_fire_and_forget(req: ClientRequest) {
    use std::os::unix::net::UnixStream;
    if let Ok(mut stream) = UnixStream::connect(get_sock_path()) {
        if let Ok(req_str) = serialize_request(&req) {
            let _ = stream.write_all(req_str.as_bytes());
        }
    }
}

#[cfg(windows)]
pub fn send_request(req: ClientRequest) -> Result<DaemonResponse, String> {
    let req_str = serialize_request(&req).map_err(|e| e.to_string())?;
    let rt = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
    rt.block_on(async {
        use tokio::net::windows::named_pipe::ClientOptions;
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut client = ClientOptions::new().open(r"\\.\pipe\smart_term_ipc").map_err(|e| e.to_string())?;
        client.write_all(req_str.as_bytes()).await.map_err(|e| e.to_string())?;
        let mut buf = vec![0u8; 8192];
        let n = client.read(&mut buf).await.map_err(|e| e.to_string())?;
        let res_str = String::from_utf8(buf[..n].to_vec()).map_err(|e| e.to_string())?;
        deserialize_response(&res_str).map_err(|e| e.to_string())
    })
}

#[cfg(windows)]
pub fn send_fire_and_forget(req: ClientRequest) {
    if let Ok(req_str) = serialize_request(&req) {
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            let _ = rt.block_on(async {
                use tokio::net::windows::named_pipe::ClientOptions;
                use tokio::io::AsyncWriteExt;
                if let Ok(mut client) = ClientOptions::new().open(r"\\.\pipe\smart_term_ipc") {
                    let _ = client.write_all(req_str.as_bytes()).await;
                }
            });
        }
    }
}
