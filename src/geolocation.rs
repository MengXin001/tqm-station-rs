use crate::config::AppConfig;

#[derive(Clone)]
pub struct GEOlocation {
    pub lat: f64,
    pub lon: f64,
    pub height: f64
}

impl GEOlocation {
    pub fn from_config(cfg: &AppConfig) -> Self {
        GEOlocation {
            lat: cfg.station.gps_lat.unwrap_or(0.0),
            lon: cfg.station.gps_lon.unwrap_or(0.0),
            height: cfg.station.gps_h.unwrap_or(0.0),
        }
    }
}