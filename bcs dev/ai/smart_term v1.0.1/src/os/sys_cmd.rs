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