mod sensors;
use rppal::gpio::{Gpio,Mode};
// use sensors::dht11;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn measure_dht11(pin: u8) -> Result<(), Box<dyn Error>>{
    let mut gpio_pin = Gpio::new()?.get(pin)?.into_io(Mode::Output);
    let mut count = 0;

    // Iniciar el envío con un pulso de 20ms
    gpio_pin.set_low();
    sleep(Duration::from_millis(20));

    // Cambiar el pin a modo lectura
    gpio_pin.set_mode(Mode::Input);
    while gpio_pin.is_low() {
        // Esperar a que el pin se ponga en alto
        count += 1;
        println!("Count: {}", count);
        sleep(Duration::from_micros(1));
    }
    Ok(())

}

fn main() -> Result<(), Box<dyn Error>>{
    // let gpio = Gpio::new()?.get(23)?.into_io(Mode::Output);
    // let mut dht11 = Dht11::new(23);
    // let reading = dht11.get_reading();

    // println!("Temperature: {}°C", reading.temperature);
    // println!("Humidity: {}%", reading.humidity);
    measure_dht11(23);
    Ok(())
}
