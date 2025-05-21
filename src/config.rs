use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NetworkConfig {
    pub check_host: String,
    pub ntp_host: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StationConfig {
    pub station_name: String,
    pub interval: Option<u64>,
    pub gps_h: Option<f64>,
    pub gps_lat: Option<f64>,
    pub gps_lon: Option<f64>,
}
#[derive(Debug, Deserialize)]
pub struct StorageConfig {
    pub local_storage: Option<bool>,
    pub local_storage_path: Option<String>,
    pub local_storage_interval: Option<u64>,
}
#[derive(Debug, Deserialize)]
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
