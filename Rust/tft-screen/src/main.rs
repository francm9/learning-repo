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

use crate::spi::spi::SPI;
use crate::st7735::st7735::ST7735S;

// Definición de algunos colores básicos
const BLACK: u16 = 0x0000;
const WHITE: u16 = 0xFFFF;
const RED: u16 = 0xF800;
const GREEN: u16 = 0x07E0;
const BLUE: u16 = 0x001F;
const YELLOW: u16 = 0xFFE0;

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

    display.init();
    display.clear(BLACK);

    display.draw_text(10, 10, "Eres", WHITE);
    display.draw_text(10, 20, "Super", WHITE);
    display.draw_text(10, 30, "Bonita", WHITE);
    display.draw_text(10, 40, "Piti", WHITE);

    loop {
        arduino_hal::delay_ms(1000);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
