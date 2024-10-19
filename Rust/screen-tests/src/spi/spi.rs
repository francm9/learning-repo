/**
 * Libreria de rust para comunicacion con st7735 utilizando arduino_hal
 * 
 * DOCS
 * - https://github.com/adafruit/Adafruit-ST7735-Library/blob/master/Adafruit_ST7735.cpp#L8
 * 
 */

 use arduino_hal::{hal::port::{PB2, PB3, PB5}, port::{mode::Output, Pin}};
 
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

    pub fn write(&mut self, data: u8)
    {
        self.cs.set_low();
        //TODO: Revisar el for
        for i in 0..8 {
            if (data & (1 << (7 - i))) != 0 {
                self.sda.set_high();
            } else {
                self.sda.set_low();
            }
            self.sck.set_high();
            self.sck.set_low();
        }
        self.cs.set_high();
    } 
}