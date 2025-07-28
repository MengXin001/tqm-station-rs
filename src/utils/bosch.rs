use crate::libs::i2c::SoftI2C;

pub struct BME280 {
    i2c: SoftI2C,
    address: u8,
    dig_t1: u16,
    dig_t2: i16,
    dig_t3: i16,
    dig_p1: u16,
    dig_p2: i16,
    dig_p3: i16,
    dig_p4: i16,
    dig_p5: i16,
    dig_p6: i16,
    dig_p7: i16,
    dig_p8: i16,
    dig_p9: i16,
    dig_h1: u8,
    dig_h2: i16,
    dig_h3: u8,
    dig_h4: i16,
    dig_h5: i16,
    dig_h6: i8,
    t_fine: i32,
}

impl BME280 {
    pub fn new(i2c: SoftI2C, address: u8) -> Self {
        Self {
            i2c,
            address,
            dig_t1: 0,
            dig_t2: 0,
            dig_t3: 0,
            dig_p1: 0,
            dig_p2: 0,
            dig_p3: 0,
            dig_p4: 0,
            dig_p5: 0,
            dig_p6: 0,
            dig_p7: 0,
            dig_p8: 0,
            dig_p9: 0,
            dig_h1: 0,
            dig_h2: 0,
            dig_h3: 0,
            dig_h4: 0,
            dig_h5: 0,
            dig_h6: 0,
            t_fine: 0,
        }
    }

    fn read_calibration(&mut self) {
        let mut data = [0u8; 24];
        self.i2c.read_registers(self.address, 0x88, &mut data);

        self.dig_t1 = ((data[1] as u16) << 8) | (data[0] as u16);
        self.dig_t2 = ((data[3] as i16) << 8) | (data[2] as i16);
        self.dig_t3 = ((data[5] as i16) << 8) | (data[4] as i16);

        self.dig_p1 = ((data[7] as u16) << 8) | (data[6] as u16);
        self.dig_p2 = ((data[9] as i16) << 8) | (data[8] as i16);
        self.dig_p3 = ((data[11] as i16) << 8) | (data[10] as i16);
        self.dig_p4 = ((data[13] as i16) << 8) | (data[12] as i16);
        self.dig_p5 = ((data[15] as i16) << 8) | (data[14] as i16);
        self.dig_p6 = ((data[17] as i16) << 8) | (data[16] as i16);
        self.dig_p7 = ((data[19] as i16) << 8) | (data[18] as i16);
        self.dig_p8 = ((data[21] as i16) << 8) | (data[20] as i16);
        self.dig_p9 = ((data[23] as i16) << 8) | (data[22] as i16);

        self.dig_h1 = self.i2c.read_register(self.address, 0xA1);

        let mut rhdata = [0u8; 7];
        self.i2c.read_registers(self.address, 0xE1, &mut rhdata);

        self.dig_h2 = ((rhdata[1] as i16) << 8) | (rhdata[0] as i16);
        self.dig_h3 = rhdata[2];
        self.dig_h4 = ((rhdata[3] as i16) << 4) | ((rhdata[4] as i16) & 0x0F);
        self.dig_h5 = ((rhdata[5] as i16) << 4) | ((rhdata[4] as i16) >> 4);
        self.dig_h6 = rhdata[6] as i8;
    }

    pub fn init(&mut self) {
        let id = self.i2c.read_register(self.address, 0xD0);
        if id == 0x60 {
            // BME280
        } else if id == 0x58 {
            // BMP280
        }
        self.read_calibration();

        self.i2c.write_register(self.address, 0xF2, 0x01); // 湿度
        self.i2c.write_register(self.address, 0xF4, 0x25); // 气温气压

    }

    pub fn read_data(&mut self) -> (f64, f64, f64) {
        let mut data = [0u8; 8];
        self.i2c.read_registers(self.address, 0xF7, &mut data);

        let press = (data[0] as u32) << 12 | (data[1] as u32) << 4 | (data[2] as u32) >> 4;
        let temp = (data[3] as u32) << 12 | (data[4] as u32) << 4 | (data[5] as u32) >> 4;
        let rh = (data[6] as u16) << 8 | data[7] as u16;

        // 温度补偿
        let v1 = ((temp as f64) / 16384.0 - (self.dig_t1 as f64) / 1024.0) * (self.dig_t2 as f64);
        let v2 = (((temp as f64) / 131072.0 - (self.dig_t1 as f64) / 8192.0)
            * ((temp as f64) / 131072.0 - (self.dig_t1 as f64) / 8192.0))
            * (self.dig_t3 as f64);
        self.t_fine = (v1 + v2) as i32;

        let temperature = self.t_fine as f64 / 5120.0;

        // 气压补偿
        let mut var1 = ((self.t_fine as f64) / 2.0) - 64000.0;
        let mut var2 = var1 * var1 * (self.dig_p6 as f64) / 32768.0;
        var2 += var1 * (self.dig_p5 as f64) * 2.0;
        var2 = (var2 / 4.0) + ((self.dig_p4 as f64) * 65536.0);
        var1 = ((self.dig_p3 as f64) * var1 * var1 / 524288.0 + (self.dig_p2 as f64) * var1)
            / 524288.0;
        var1 = (1.0 + var1 / 32768.0) * (self.dig_p1 as f64);
        let mut pressure = 1048576.0 - (press as f64);
        if var1 != 0.0 {
            pressure = (pressure - (var2 / 4096.0)) * 6250.0 / var1;
            var1 = (self.dig_p9 as f64) * pressure * pressure / 2147483648.0;
            var2 = pressure * (self.dig_p8 as f64) / 32768.0;
            pressure += (var1 + var2 + (self.dig_p7 as f64)) / 16.0;
        }

        // 湿度补偿
        let mut var3 = (self.t_fine as f64) - 76800.0;
        var3 = ((rh  as f64) - ((self.dig_h4 as f64)* 64.0 + (self.dig_h5 as f64) / 16384.0 * var3)) *
        ((self.dig_h2 as f64) / 65536.0 * (1.0 + (self.dig_h6 as f64) / 67108864.0 * var3 * 
        (1.0 + (self.dig_h3 as f64) / 67108864.0 * var3)));
        var3 = var3 * (1.0 - (self.dig_h1 as f64) * var3 / 524288.0);

        let humidity = if var3 > 100.0 {
            100.0
        } else if var3 < 0.0 {
            0.0
        } else {
            var3
        };

        (temperature, humidity, pressure)
    }
}
