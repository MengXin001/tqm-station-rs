pub mod time;
pub mod error;

use std::time::Duration;
use log::{error, info};
use rsntp::{ProtocolError, SntpClient, SynchroniztationError};
use ::time::format_description::well_known::Rfc2822;
use ::time::UtcOffset;

pub fn sync_ntp_and_set_time(
    ntp_host: &str,
    timeout_secs: u16,
    retry: i32,
    print_time: bool,
    set_time: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut attempts = 0;
    let mut delay = Duration::from_millis(500);

    let result = loop {
        let mut client = SntpClient::new();
        client.set_timeout(Duration::from_secs(timeout_secs.into()));

        match client.synchronize(ntp_host) {
            Ok(res) => break res,
            Err(SynchroniztationError::ProtocolError(err)) => {
                if let ProtocolError::KissODeath(_) = err {
                    return Err(err.into());
                }
            }
            Err(err) => {
                if attempts < retry || retry < 0 {
                    error!(
                        "ntp sync error, retry in {} seconds: {}",
                        delay.as_secs(),
                        err
                    );
                    std::thread::sleep(delay);
                    delay *= 2;
                    attempts += 1;
                } else {
                    return Err(err.into());
                }
            }
        }
    };

    let utc_time = result.datetime().into_offset_date_time()?;
    let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    let local_time_str = utc_time.to_offset(offset).format(&Rfc2822)?;

    if print_time {
        info!("[{}] {}", ntp_host, local_time_str);
    }

    if set_time {
        match time::change_system_time(utc_time) {
            Ok(()) => {
                info!("系统时间设置为 {}", local_time_str);
                Ok(())
            }
            Err(errno) => Err(format!("设置系统时间失败 {}", errno).into()),
        }
    } else {
        Ok(())
    }
}
