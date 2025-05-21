use chrono::Utc;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, Write};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::task;

#[repr(C)]
#[derive(Clone)]
pub struct DataBlock {
    pub timestamp: i64,
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub wind_speed: f64,
    pub lat: f64,
    pub lon: f64,
    pub height: f64,
}

impl DataBlock {
    pub fn to_bytes(&self) -> [u8; 62] {
        let mut data_buffer = [0u8; 62];
        data_buffer[..8].copy_from_slice(&self.timestamp.to_le_bytes());
        data_buffer[8..16].copy_from_slice(&self.temperature.to_le_bytes());
        data_buffer[16..24].copy_from_slice(&self.humidity.to_le_bytes());
        data_buffer[24..32].copy_from_slice(&self.pressure.to_le_bytes());
        data_buffer[32..40].copy_from_slice(&self.wind_speed.to_le_bytes());
        data_buffer[40..48].copy_from_slice(&self.lat.to_le_bytes());
        data_buffer[48..56].copy_from_slice(&self.lon.to_le_bytes());
        data_buffer[56..62].copy_from_slice(&self.height.to_le_bytes());
        data_buffer
    }
}

pub fn init_storage_task(mut local_storage_rx: Receiver<DataBlock>) {
    task::spawn_blocking(move || process_storage_queue(&mut local_storage_rx));
}

fn process_storage_queue(local_storage_rx: &mut Receiver<DataBlock>) -> io::Result<()> {
    create_dir_all("data")?;
    let filename = format!("data/Data_{}.bin", Utc::now().format("%Y%m%d"));
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&filename)?;
    let mut count = 0;

    while let Some(data_block) = local_storage_rx.blocking_recv() {
        file.write_all(&data_block.to_bytes())?;
        count += 1;
        if count == 5 {
            file.flush()?;
            count = 0;
        }
    }
    // TODO: 跨日处理
    file.flush()?; // 退出前flush
    Ok(())
}

pub fn enqueue_storage_data(local_storage_tx: &Sender<DataBlock>, data_block: DataBlock) {
    let _ = local_storage_tx.try_send(data_block);
}
