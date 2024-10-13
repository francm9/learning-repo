/**
 * Libreria de rust para comunicacion con st7735 utilizando arduino_hal
 * 
 * DOCS
 * - https://github.com/adafruit/Adafruit-ST7735-Library/blob/master/Adafruit_ST7735.cpp#L8
 * 
 */

use arduino_hal::{hal::port::{PB0, PB1, PB2, PB3, PB5}, port::{mode::Output, Pin}};
use arduino_hal::delay_ms;

pub struct ST7735
{
    dc:  Pin<Output, PB1>,
    sck: Pin<Output, PB5>,
    sda: Pin<Output, PB3>,
    rst: Pin<Output, PB0>,
    cs:  Pin<Output, PB2>,
}

impl ST7735
{   
    /**
     * Constructor de la clase st7735
     */
    pub fn new(dc: Pin<Output, PB1>, sck: Pin<Output, PB5>, sda: Pin<Output, PB3>, rst: Pin<Output, PB0>, cs: Pin<Output, PB2>) -> Self
    {
        ST7735 { dc, sck, sda, rst, cs }
    }

    fn reset(&mut self)
    {
        self.rst.set_low();
        delay_ms(50);
        self.rst.set_high();
        delay_ms(50);
    }

    pub fn init(&mut self)
    {
        // Inicializar la pantalla
        self.reset();
    }

}