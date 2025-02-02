#![no_std]
#![no_main]

/**
 * DOCUMENTACION
 * 
 * Gestion de una pantalla led con arduino nano y rust
 * Url -> https://sp.arduino-france.site/pantalla-tft-spi/
 * Proyecto de arduino que ha funcionado con el arduino uno -> https://docs.arduino.cc/retired/library-examples/tft-library/TFTDisplayText/
 * Controlador -> ST7735S
 */

pub mod st7735;
pub mod spi;

use arduino_hal::delay_ms;

use crate::spi::spi::SPI;
use crate::st7735::st7735::ST7735S;

 #[arduino_hal::entry]
fn main() ->  ! 
{
    // Inicializar los pines
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Configurar la pantalla
    let dc = pins.d9.into_output();
    let sck = pins.d13.into_output();
    let sda = pins.d11.into_output();   // mosi
    let rst = pins.d8.into_output();
    let cs = pins.d10.into_output();

    let spi = SPI::new(sck, sda, cs);
    let mut display = ST7735S::new(spi, dc, rst);

    // display.reset();
    display.write_command(0x28);
    delay_ms(5000);
    display.write_command(0x29);

    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
