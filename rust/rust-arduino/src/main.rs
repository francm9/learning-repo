#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);


    let mut red_rgb = pins.d9.into_output();
    let mut blue_rgb = pins.d10.into_output();
    let mut green_rgb = pins.d2.into_output();
    let mut green_led = pins.d7.into_output();
    let mut blue_led = pins.d5.into_output();
    let mut red_led = pins.d6.into_output();
    let mut trigger = pins.d3.into_output();
    let echo = pins.d4;                     //Por defecto es una entrada

    let timer1 = dp.TC1;


    loop {
        timer1.tcnt1.write(|w| w.bits(0));          //Reiniciamos el contador a 0
        red_rgb.toggle();
        blue_rgb.toggle();
        green_rgb.toggle();
        green_led.toggle();
        blue_led.toggle();
        red_led.toggle();
        

        trigger.set_high();
        arduino_hal::delay_us(10);              //hay que esperar 10us para que se complete
        trigger.set_low();
        
        while echo.is_low() {}                  //esperamos a que el pin se ponga en alto
        timer1.tccr1b.write(|w| w.cs1().prescale_64());

        while echo.is_high(){}
        timer1.tccr1b.write(|w| w.cs1().no_clock());

        let distance = timer1.tcnt1.read().bits().saturating_mul(4) / 58; 

        ufmt::uwriteln!(
            &mut serial, 
            "Distance: {} cm\n\r",
            distance
        ).void_unwrap(); 

        arduino_hal::delay_ms(1000);

    }
}
