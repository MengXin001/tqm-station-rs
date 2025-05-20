use chrono::Utc;
use log::info;
use serde_json::json;

pub fn upload_data(station_name: &str, wind_speed: f64, gps_h: f64) -> Result<(), reqwest::Error> {
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
            "lat": 32.03377,
            "long": 118.84059,
            "alt": gps_h,
            "dir": 0,
            "speed": 0
        }
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://track.storm-chasers.cn/api/receive_meteo_data_json")
        .header("Content-Type", "application/json")
        .json(&data)
        .send()?;

    info!("上传结果: {}", res.status());
    Ok(())
}
