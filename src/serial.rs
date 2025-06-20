use std::io::{self, Read, Write};
use std::time::Duration;
use log::info;

const WIND_SPEED_QUERY: [u8; 8] = [0x02, 0x03, 0x00, 0x00, 0x00, 0x01, 0x84, 0x39];

pub fn query_wind_speed() -> io::Result<f64> {
    let port_name = "/dev/ttyS3";
    let baud_rate = 9600;
    let timeout = Duration::from_millis(1500);

    let mut serialport = serialport::new(port_name, baud_rate)
        .timeout(timeout)
        .open()?;

    serialport.write_all(&WIND_SPEED_QUERY)?;

    let mut buffer = [0u8; 8];
    let len = serialport.read(&mut buffer)?;

    if len == 7 && buffer[0] == 0x02 && buffer[1] == 0x03 {
        let ws10 = (buffer[3] as u16) << 8 | (buffer[4] as u16);
        let ws = ws10 as f64 / 10.0;
        info!("风速: {} m/s", ws);
        Ok(ws)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "风速读取失败"))
    }
}