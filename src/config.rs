use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub check_host: String,
    pub ntp_host: Option<String>,
}

#[derive(Deserialize)]
pub struct StationConfig {
    pub station_name: String,
    pub interval: Option<f32>,
    pub log_level: Option<String>,
    pub gps_h: Option<f64>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
}
#[derive(Deserialize)]
pub struct StorageConfig {
    pub local_storage: Option<bool>,
    pub flush_interval: Option<u64>,
    pub cloud_storage: Option<bool>,
    pub upload_interval: Option<u64>,
}
#[derive(Deserialize)]
pub struct AppConfig {
    pub network: NetworkConfig,
    pub station: StationConfig,
    pub storage: StorageConfig,
}

impl AppConfig {
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let cfg = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        cfg.try_deserialize()
    }
}
