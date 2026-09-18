pub fn get_create_user_cmd(u: &str, p: &str) -> String {
    if cfg!(target_os = "windows") { format!(" net user {} {} /add\n", u, p) }
    else if cfg!(target_os = "macos") { format!(" sudo sysadminctl -addUser {} -password {}\n", u, p) }
    else { format!(" sudo useradd -m {} && echo '{}:{}' | sudo chpasswd\n", u, u, p) }
}
pub fn get_switch_user_cmd(u: &str, cmd: &str) -> String {
    if cfg!(target_os = "windows") { format!(" runas /user:{} \"{}\"\n", u, cmd) }
    else { format!(" su - {} -c '{}'\n", u, cmd.replace("'", "'\\''")) }
}
pub fn get_change_pass_cmd(u: &str, p: &str) -> String {
    if cfg!(target_os = "windows") { format!(" net user {} {}\n", u, p) }
    else if cfg!(target_os = "macos") { format!(" sudo sysadminctl -resetPasswordFor {} -newPassword {}\n", u, p) }
    else { format!(" echo '{}:{}' | sudo chpasswd\n", u, p) }
}
pub fn get_os_users() -> Vec<String> {
    let mut users = Vec::new();
    if cfg!(target_os = "windows") { /* TODO: Windows net user parsing */ }
    else {
        if let Ok(txt) = std::fs::read_to_string("/etc/passwd") {
            for line in txt.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 2 {
                    if let Ok(uid) = parts[2].parse::<u32>() {
                        if uid == 0 || uid >= 1000 { users.push(parts[0].to_string()); }
                    }
                }
            }
        }
    }
    users
}