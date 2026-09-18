pub fn should_log(cmd: &str) -> bool {
    !cmd.contains("-none")
}

pub fn clean_command(cmd: &str) -> String {
    cmd.replace("-none", "").trim().to_string()
}