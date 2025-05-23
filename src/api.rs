use log::info;
use serde_json::json;

use crate::storage::DataBlock;
use crate::utils::calc;
pub async fn upload_data(station_name: &str, data: DataBlock) -> Result<(), reqwest::Error> {
    let dew_point = calc::dewpoint_fast(data.temperature, data.humidity);
    let mslp = calc::mslp(data.pressure, data.height, data.temperature);
    let data = json!({
        "timestamp": data.timestamp,
        "instrument": station_name,
        "observation": {
            "t": data.temperature,
            "rh": data.humidity,
            "p": data.pressure,
            "dt": dew_point,
            "mslp": mslp
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
    info!("sdkAPI上传结果: {}", res.status());
    Ok(())
}
