pub async fn is_connected(check_host: &str) -> bool {
    for _ in 0..2 {
        let alive = tokio::process::Command::new("ping")
            .arg("-c1")
            .arg("-W5")
            .arg(check_host)
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false);
        if alive {
            return true;
        }
    }
    false
}