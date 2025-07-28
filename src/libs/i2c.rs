use sysfs_gpio::{Direction, Pin};
use std::{thread, time::Duration};

pub struct SoftI2C {
    sda: Pin,
    scl: Pin,
}

impl SoftI2C {
    pub fn new(sda_pin: u64, scl_pin: u64) -> Self {
        let sda = Pin::new(sda_pin);
        let scl = Pin::new(scl_pin);
        sda.export().ok();
        scl.export().ok();
        thread::sleep(Duration::from_micros(10));

        sda.set_direction(Direction::Out).unwrap();
        scl.set_direction(Direction::Out).unwrap();
        sda.set_value(1).unwrap();
        scl.set_value(1).unwrap();

        Self { sda, scl }
    }

    fn delay(&self) {
        thread::sleep(Duration::from_micros(5));
    }

    fn set_sda(&self, val: bool) {
        self.sda.set_direction(Direction::Out).unwrap();
        self.sda.set_value(val as u8).unwrap();
        self.delay();
    }

    fn set_scl(&self, val: bool) {
        self.scl.set_value(val as u8).unwrap();
        self.delay();
    }

    fn read_sda(&self) -> bool {
        self.sda.set_direction(Direction::In).unwrap();
        self.delay();
        self.sda.get_value().unwrap() != 0
    }

    fn start(&self) {
        self.set_sda(true);
        self.set_scl(true);
        self.set_sda(false);
        self.set_scl(false);
    }

    fn stop(&self) {
        self.set_sda(false);
        self.set_scl(true);
        self.set_sda(true);
    }

    fn write_byte(&self, byte: u8) -> bool {
        for i in 0..8 {
            self.set_sda((byte & (0x80 >> i)) != 0);
            self.set_scl(true);
            self.set_scl(false);
        }
        self.set_sda(true);
        self.set_scl(true);
        let ack = !self.read_sda();
        self.set_scl(false);
        ack
    }

    fn read_byte(&self, ack: bool) -> u8 {
        let mut byte = 0u8;
        self.sda.set_direction(Direction::In).unwrap();
        for _ in 0..8 {
            self.set_scl(true);
            byte <<= 1;
            if self.read_sda() {
                byte |= 1;
            }
            self.set_scl(false);
        }
        self.set_sda(!ack);
        self.set_scl(true);
        self.set_scl(false);
        self.set_sda(true);
        byte
    }

    pub fn read_register(&self, addr: u8, reg: u8) -> u8 {
        self.start();
        self.write_byte(addr << 1);
        self.write_byte(reg);
        self.start();
        self.write_byte((addr << 1) | 1);
        let val = self.read_byte(false);
        self.stop();
        val
    }

    pub fn read_registers(&self, addr: u8, reg: u8, buf: &mut [u8]) {
        self.start();
        self.write_byte(addr << 1);
        self.write_byte(reg);
        self.start();
        self.write_byte((addr << 1) | 1);
        for i in 0..buf.len() {
            buf[i] = self.read_byte(i + 1 != buf.len());
        }
        self.stop();
    }

    pub fn write_register(&self, addr: u8, reg: u8, val: u8) {
        self.start();
        self.write_byte(addr << 1);
        self.write_byte(reg);
        self.write_byte(val);
        self.stop();
    }
}