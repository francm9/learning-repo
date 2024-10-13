#![no_std]
#![no_main]

/**
 * DOCUMENTACION
 * 
 * Gestion de una pantalla led con arduino nano y rust
 * Url -> https://sp.arduino-france.site/pantalla-tft-spi/
 * 
 */

pub mod st7735;

use crate::st7735::st7735::ST7735;

 #[arduino_hal::entry]
fn main() ->  ! {
    // Inicializar los pines
    let dp = arduino_hal::Peripherals::take().unwrap();

    let pins = arduino_hal::pins!(dp);

    // Configurar la pantalla
    let dc = pins.d9.into_output();
    let sck = pins.d13.into_output();
    let sda = pins.d11.into_output();
    let rst = pins.d8.into_output();
    let cs = pins.d10.into_output();

    let mut st7735 = ST7735::new(dc, sck, sda, rst, cs);
    st7735.init();

    loop {
        
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
