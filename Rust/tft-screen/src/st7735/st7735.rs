use crate::spi::spi::SPI;
use arduino_hal::{hal::port::{PB0, PB1}, port::{mode::Output, Pin}};
use arduino_hal::delay_us;

pub struct ST7735S {
    spi: SPI,
    dc: Pin<Output, PB1>,
    rst: Pin<Output, PB0>,
}

impl ST7735S {
    pub fn new(spi: SPI, dc: Pin<Output, PB1>, rst: Pin<Output, PB0>) -> Self 
    {
        ST7735S { spi, dc, rst }
    }

    pub fn init(&mut self) 
    {
        // TODO
        // Init spi
        self.spi.begin();

        // Init display
        // self.rst.set_high();
    }

    pub fn write_command(&mut self, command: u8) 
    {
        self.dc.set_low();
        self.spi.write(command);
    }

    pub fn write_data(&mut self, data: u8) 
    {
        self.dc.set_high();
        self.spi.write(data);
    }

    pub fn reset(&mut self) 
    {
        self.dc.set_low();
        self.rst.set_high();
        delay_us(500);
        self.rst.set_low();
        delay_us(500);
        self.rst.set_high();
        delay_us(500);
    }
}