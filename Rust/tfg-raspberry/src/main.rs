mod sensors;
use rppal::gpio::{Gpio,Mode, self, Level};
// use sensors::dht11;
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn measure_dht11(pin: u8) -> Result<(), Box<dyn Error>>{
    let mut gpio_pin = Gpio::new()?.get(pin)?.into_output_low();
    let mut count = 0;

    // Iniciar el envío con un pulso bajo de 20ms
    // gpio_pin.set_high();
    // sleep(Duration::from_millis(25));

    // Se pone a alto y se espera a que conteste el sensor
    gpio_pin.write(Level::Low);
    println!("Estado: {}", gpio_pin.is_set_low());
    // sleep(Duration::from_micros(40));
    while (true) {}
    // Se cambia a modo de entrada
    //gpio_pin.set_mode(Mode::Input);
    // println!("Modo de entrada: {:?}", gpio_pin.mode());

    // while gpio_pin.read() == Level::High {
    //     count += 1;
    //     //println!("count: {}", count);
    // }
    
    Ok(())
}

fn main(){
    measure_dht11(4);
}
