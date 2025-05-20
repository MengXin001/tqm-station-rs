use chrono::Utc;
use log::info;
use serde_json::json;

use crate::geolocation::gps::GEOlocation;

pub async fn upload_data(station_name: &str, wind_speed: f64, gps: GEOlocation) -> Result<(), reqwest::Error> {
    let data = json!({
        "timestamp": Utc::now().timestamp(),
        "instrument": station_name,
        "observation": {
            "t": 0,
            "rh": 0,
            "p": 0,
            "dt": 0,
            "mslp": 0
        },
        "wind": {
            "spd": wind_speed,
            "dir": 0
        },
        "gps": {
            "lat": gps.lat,
            "long": gps.lon,
            "alt": gps.height,
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
