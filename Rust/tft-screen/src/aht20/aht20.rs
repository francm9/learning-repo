/*
    https://github.com/sparkfun/SparkFun_Qwiic_Humidity_AHT20_Arduino_Library/blob/main/src/SparkFun_Qwiic_Humidity_AHT20.cpp
*/

use arduino_hal::i2c::I2c;
use arduino_hal::prelude::*;

const DEFAULT_ADDRESS: u8 = 0x38;

const AHT20_ADDRESS: u8 = 0x38;
const CMD_INITIALIZE: u8 = 0xBE;
const CMD_MEASURE: u8 = 0xAC;

pub struct AHT20 {
    i2c: I2c,
    address: u8,
}

impl AHT20
{
    pub fn new(i2c: I2c, address: Option<u8>) -> Self 
    {
        if address.is_some() {
            AHT20 {
                i2c,
                address: address.unwrap(),
            }
        } else {
            AHT20 {
                i2c,
                address: DEFAULT_ADDRESS,
            }
        }
    }

    pub fn begin(&mut self) -> bool
    {
        self.i2c.write(self.address, &[CMD_INITIALIZE]).is_ok()
    }

    pub fn read_data(&mut self) -> (i32, i32)
    {
        let mut data = [0u8; 6];
        if self.i2c.read(self.address, &mut data).is_err()
        {
            return (0, 0);
        }

        let humidity = ((data[1] as u16) << 12) | ((data[2] as u16) << 4) | ((data[3] as u16) >> 4);
        let temperature = ((data[3] as u32 & 0x0F) << 16) | ((data[4] as u32) << 8) | (data[5] as u32);

        let humidity = -6 + 125 as i32 * (humidity as i32) / 0x100000;
        let temperature = -47 + 176 * (temperature as i32) / 0x100000;

        (temperature, humidity)
    }
}