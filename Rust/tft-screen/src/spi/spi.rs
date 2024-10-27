/**
 * Libreria de rust para comunicacion con st7735 utilizando arduino_hal
 * - Modo 0 y frecuencia de 2 MHz
 * 
 * DOCS
 * - https://github.com/adafruit/Adafruit-ST7735-Library/blob/master/Adafruit_ST7735.cpp#L8
 * - https://github.com/adafruit/Adafruit_BusIO/blob/master/Adafruit_SPIDevice.cpp
 * - https://www.es-ebyte.com/news/470
 */

 use arduino_hal::{delay_us, hal::port::{PB2, PB3, PB5}, port::{mode::Output, Pin}};
 
 pub struct SPI
 {
     sck: Pin<Output, PB5>,
     sda: Pin<Output, PB3>,
     cs:  Pin<Output, PB2>,
 }
 
 impl SPI
 {   
    /**
     * Constructor de la clase SPI
    */
    pub fn new(sck: Pin<Output, PB5>, sda: Pin<Output, PB3>, cs: Pin<Output, PB2>) -> Self
    {
        SPI{sck, sda,cs}
    }

    pub fn begin(&mut self)
    {
        self.cs.set_high();

        // Configuracion de momento en MODO 0
        self.sck.set_low();

        self.sda.set_high();
    }

    pub fn write(&mut self, data: &[u8]) {
        self.cs.set_low();
        
        for &byte in data {
            for i in 0..8 {
                if (byte & (1 << (7 - i))) != 0 
                {
                    self.sda.set_high();
                } 
                else 
                {
                    self.sda.set_low();
                }
                self.sck.set_high();
                delay_us(2);
                self.sck.set_low();
                delay_us(2);
            }
        }

        self.cs.set_high();
        delay_us(10);
    }
}