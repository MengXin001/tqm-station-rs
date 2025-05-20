mod api;
mod config;
mod serial;
mod storage;
mod wifi;

use crate::config::AppConfig;
use log::{error, info};
use std::{env, thread, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cfg = AppConfig::from_file("config.toml")?;
    let cli_args: Vec<String> = env::args().collect();
    let cli_interval = cli_args
        .iter()
        .position(|arg| arg == "-t")
        .and_then(|i| cli_args.get(i + 1))
        .and_then(|s| s.parse::<u64>().ok());
    let ntp_host = cfg.network.ntp_host.as_deref().unwrap_or("203.107.6.88");
    rsdate::sync_ntp_and_set_time(ntp_host, 5, 3, true, true)?;
    let interval = cli_interval.or(cfg.station.interval).unwrap_or(60);
    loop {
        match serial::query_wind_speed() {
            Ok(wind_speed) => {
                let gps_h = 0.0;
                if let Err(e) = storage::save_to_tf(wind_speed) {
                    error!("存储失败: {}", e);
                }

                if wifi::is_connected(&cfg.network.check_host) {
                    if let Err(e) = api::upload_data(&cfg.station.station_name, wind_speed, gps_h) {
                        error!("上传失败: {}", e);
                    }
                } else {
                    error!("网络连接失败");
                }
            }
            Err(e) => error!("RS485通信失败: {}", e),
        }

        thread::sleep(Duration::from_secs(interval));
    }
}
