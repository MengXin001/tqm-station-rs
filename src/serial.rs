use crate::libs::i2c::SoftI2C;
use crate::utils::bosch::BME280;
use std::io::{self, Read, Write};
use std::time::Duration;

const WIND_SPEED_QUERY: [u8; 8] = [0x02, 0x03, 0x00, 0x00, 0x00, 0x01, 0x84, 0x39];
const SDA_PIN_NUM: u64 = 11; // GPIO0_B3_d => GPIO 11
const SCL_PIN_NUM: u64 = 12; // GPIO0_B4_d => GPIO 12

pub fn query_wind_speed() -> io::Result<f64> {
    let port_name = "/dev/ttyS3";
    let baud_rate = 9600;
    let timeout = Duration::from_millis(50);

    let mut serialport = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()?;

    serialport.write_all(&WIND_SPEED_QUERY)?;

    let mut buffer = [0u8; 8];
    let len = serialport.read(&mut buffer)?;

    if len == 7 && buffer[0] == 0x02 && buffer[1] == 0x03 {
        let ws10 = (buffer[3] as u16) << 8 | (buffer[4] as u16);
        let ws = ws10 as f64 / 10.0;
        Ok(ws)
    } else {
        let ws = f64::NAN;
        Ok(ws)
    }
}

pub fn query_bme280() -> io::Result<(f64, f64, f64)> {
    let soft_i2c = SoftI2C::new(SDA_PIN_NUM, SCL_PIN_NUM);
    let mut bme280 = BME280::new(soft_i2c, 0x76);
    let _ = bme280.init();

    let (temperature, humidity, pressure) = bme280.read_data();

    let temperature = if temperature > 85.0 || temperature < -45.0 {
        f64::NAN
    } else {
        (temperature * 100.0).round() / 100.0
    };
    let pressure = if pressure > 110000.0 || pressure < 30000.0 {
        f64::NAN
    } else {
        (pressure * 10.0).round() / 1000.0 - 2.0
    };
    let humidity = (humidity * 100.0).round() / 100.0;

    Ok((temperature, humidity, pressure))
}