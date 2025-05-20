pub fn is_connected(check_host: &str) -> bool {
    for _ in 0..2 {
        let success = std::process::Command::new("ping")
            .arg("-c1")
            .arg("-W5")
            .arg(check_host)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if success {
            return true;
        }
    }
    false
}