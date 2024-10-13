use st7735::color;
/**
 * DOCUMENTACION
 * 
 * Gestion de una pantalla led con arduino nano y rust
 * Url -> https://sp.arduino-france.site/pantalla-tft-spi/
 * 
 */
use st7735::ST7734;
use arduino_hal::Spi;
use arduino_hal::Pins;

fn main() {
    // Inicializar los pines
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Configurar la pantalla
    let dc = pins.d9;
    let sck = pins.d13;
    let sda = pins.d11;
    let rst = pins.d8;
    let cs = pins.d10;

    let spi = Spi::new(
        dp.SPI, 
        sclk, 
        sda, 
        pins.d12.into_pull_up_input().downgrade(), 
        cs,         
        Mode {
        polarity: spi::Polarity::IdleLow,
        phase: spi::Phase::CaptureOnFirstTransition,
    });
    let mut display = ST7734::new_with_spi(spi, dc);
    display.draw_line(0, 0, 10, 10, 0xFFFF00);
}
