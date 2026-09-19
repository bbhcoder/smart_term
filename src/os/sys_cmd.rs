use std::process::Command;

pub fn get_os_users() -> Vec<String> {
    let mut users = Vec::new();
    if cfg!(target_os = "windows") {
        if let Ok(out) = Command::new("powershell").arg("-Command").arg("Get-LocalUser | Select-Object -ExpandProperty Name").output() {
            let s = String::from_utf8_lossy(&out.stdout);
            for l in s.lines() { 
                let t = l.trim(); 
                if !t.is_empty() && t != "DefaultAccount" && t != "Guest" && t != "WDAGUtilityAccount" { 
                    users.push(t.to_string()); 
                } 
            }
        }
    } else {
        if let Ok(c) = std::fs::read_to_string("/etc/passwd") {
            for l in c.lines() {
                let p: Vec<&str> = l.split(':').collect();
                if p.len() > 2 && !p[0].starts_with('#') {
                    if let Ok(uid) = p[2].parse::<u32>() {
                        if (uid == 0 || uid >= 1000) && uid != 65534 { 
                            users.push(p[0].to_string()); 
                        }
                    }
                }
            }
        }
    }
    users
}

pub fn get_delete_user_cmd(u: &str) -> String {
    if cfg!(target_os = "windows") { format!(" net user {} /delete\n", u) }
    else if cfg!(target_os = "macos") { format!(" sudo dscl . -delete /Users/{}\n", u) }
    else { format!(" sudo userdel -r {}\n", u) }
}

pub fn get_create_user_cmd(u: &str, p: &str) -> String {
    if cfg!(target_os = "windows") { format!(" net user {} {} /add\n", u, p) }
    else if cfg!(target_os = "macos") { format!(" sudo dscl . -create /Users/{} && sudo dscl . -passwd /Users/{} {}\n", u, u, p) }
    else { format!(" sudo useradd -m {} && echo '{}:{}' | sudo chpasswd\n", u, u, p) }
}

pub fn get_change_pass_cmd(u: &str, p: &str) -> String {
    if cfg!(target_os = "windows") { format!(" net user {} {}\n", u, p) }
    else if cfg!(target_os = "macos") { format!(" sudo dscl . -passwd /Users/{} {}\n", u, p) }
    else { format!(" echo '{}:{}' | sudo chpasswd\n", u, p) }
}