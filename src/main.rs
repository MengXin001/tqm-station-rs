mod api;
mod config;
mod geolocation;
mod serial;
mod storage;
mod utils;

use log::{error, info};
use std::{env, f64::NAN, thread, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cfg = config::AppConfig::from_file("config.toml")?;
    let cli_args: Vec<String> = env::args().collect();
    let cli_interval = cli_args
        .iter()
        .position(|arg| arg == "-t")
        .and_then(|i| cli_args.get(i + 1))
        .and_then(|s| s.parse::<u64>().ok());

    let ntp_host = cfg.network.ntp_host.as_deref().unwrap_or("203.107.6.88");
    let _ = rsdate::sync_ntp_and_set_time(ntp_host, 5, 3, true, true)?;

    // config
    let sample_interval = cli_interval.or(cfg.station.interval).unwrap_or(60);
    let config_location = geolocation::GEOlocation::from_config(&cfg); // 预设坐标
    /// 本地存储
    let local_storage = cfg.storage.local_storage.unwrap_or(false);
    let (local_storage_tx, local_storage_rx) = tokio::sync::mpsc::channel(10);
    if local_storage {
        let flush_interval = cfg.storage.flush_interval.unwrap_or(5);
        let _ = storage::init_storage_task(local_storage_rx, flush_interval);
    }

    // placeholder
    let temperature = NAN;
    let humidity = NAN;
    let pressure = NAN;
    let gps_device = false;

    loop {
        match serial::query_wind_speed() { // TODO: serial -> trait
            Ok(wind_speed) => {
                let timestamp_now = chrono::Utc::now().timestamp();
                let geolocation = if gps_device {
                    // TODO: 从GPS硬件读取 blocking need
                    info!("获取GPS定位成功");
                    config_location.clone()
                } else {
                    config_location.clone()
                };
                let data_block = storage::DataBlock {
                    timestamp: timestamp_now,
                    temperature,
                    humidity,
                    pressure,
                    wind_speed,
                    lat: geolocation.lat,
                    lon: geolocation.lon,
                    height: geolocation.height,
                };
                if local_storage {
                    let _ = storage::enqueue_storage_data(&local_storage_tx, data_block.clone());
                }
                // disable in high frequency sampling
                if utils::network::is_connected(&cfg.network.check_host).await {
                    if let Err(e) = api::upload_data(&cfg.station.station_name, data_block).await {
                        error!("上传失败: {}", e);
                    }
                } else {
                    error!("网络连接失败");
                }
            }
            Err(e) => error!("RS485通信失败: {}", e),
        }

        thread::sleep(Duration::from_secs(sample_interval));
    }
}
