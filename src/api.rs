use chrono::Utc;
use log::info;
use serde_json::json;

use crate::storage::DataBlock;
pub async fn upload_data(station_name: &str, data: DataBlock) -> Result<(), reqwest::Error> {
    let data = json!({
        "timestamp": data.timestamp,
        "instrument": station_name,
        "observation": {
            "t": data.temperature,
            "rh": data.humidity,
            "p": data.pressure,
            "dt": 0,
            "mslp": 0
        },
        "wind": {
            "spd": data.wind_speed,
            "dir": 0
        },
        "gps": {
            "lat": data.lat,
            "long": data.lon,
            "alt": data.height,
            "dir": 0,
            "speed": 0
        }
    });

    let client = reqwest::Client::new();
    let res = client
        .post("http://track.storm-chasers.cn/api/receive_meteo_data_json")
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await?;
    info!("上传结果: {}", res.status());
    Ok(())
}
