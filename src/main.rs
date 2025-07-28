mod api;
mod config;
mod geolocation;
mod serial;
mod storage;
mod utils;
mod libs;

use log::{error, info, debug};
use std::{f64::NAN, time::{Duration, Instant}, env};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli_args: Vec<String> = env::args().collect();
    let config_path = cli_args
        .iter()
        .position(|arg| arg == "-c")
        .and_then(|idx| cli_args.get(idx + 1))
        .map(|s| s.as_str())
        .unwrap_or("config.toml");

    let cfg = config::AppConfig::from_file(config_path)?;
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(cfg.station.log_level.clone().unwrap_or("warn".to_string()))).init();
    let ntp_host = cfg.network.ntp_host.as_deref().unwrap_or("203.107.6.88");
    let _ = rsdate::sync_ntp_and_set_time(ntp_host, 5, 3, true, true).unwrap();

    // config
    let sample_interval = cfg.station.interval.unwrap_or(60.0);
    let config_location = geolocation::GEOlocation::from_config(&cfg); // 预设坐标
    /// 云端存储
    let cloud_storage = cfg.storage.cloud_storage.unwrap_or(false);
    let upload_interval = cfg.storage.upload_interval.unwrap_or(5);
    /// 本地存储
    let local_storage = cfg.storage.local_storage.unwrap_or(false);
    let (local_storage_tx, local_storage_rx) = tokio::sync::mpsc::channel(10);
    if local_storage {
        let flush_interval = cfg.storage.flush_interval.unwrap_or(5);
        let _ = storage::init_storage_task(local_storage_rx, flush_interval);
    }

    // placeholder
    let gps_device = false;
    let mut read_count = 0;

    loop {
        read_count += 1;
        let timestamp_now = chrono::Utc::now().timestamp();
        let (wind_speed, temperature, humidity, pressure) = tokio::task::spawn_blocking(|| {
            let wind_speed = serial::query_wind_speed().unwrap_or(NAN);
            let (temperature, humidity, pressure) = serial::query_bme280().unwrap_or((NAN, NAN, NAN));
                (wind_speed, temperature, humidity, pressure)
            })
        .await
        .unwrap();
        debug!("风速: { } m/s, 温度: { } °C, 湿度: { } %, 气压: { } hPa", wind_speed, temperature, humidity, pressure);
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
        if cloud_storage && read_count == upload_interval {
            let station_name = cfg.station.station_name.clone();
            let check_host = cfg.network.check_host.clone();
            tokio::spawn(async move {
                if utils::network::is_connected(&check_host).await {
                    if let Err(e) = api::upload_data(&station_name, data_block).await {
                        error!("上传失败: {}", e);
                    }
                } else {
                    error!("网络连接失败");
                }
            });
            read_count = 0;
        }
        let _ = tokio::time::sleep(Duration::from_secs_f32(sample_interval)).await;
    }
}
