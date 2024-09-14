#![no_std]
#![no_main]

use arduino_hal::Adc;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = Adc::new(dp.ADC, Default::default());
    let encoder_left = pins.a0.into_analog_input(&mut adc);
    let encoder_right = pins.a1.into_analog_input(&mut adc);
    let switch = pins.d2.into_pull_up_input();            
    
    // loop forever -- Es necesario por el uso de ! en la función main para que haya algo ejecutandose siempre\
    loop {
        let _ = ufmt::uwriteln!(
            &mut serial, 
            "Switch pressed -> {}\nEncoder LEFT -> {}\nEncoder RIGHT -> {}\n\r",
            switch.is_low(),
            encoder_left.analog_read(&mut adc),
            encoder_right.analog_read(&mut adc)
        );      
        arduino_hal::delay_ms(1000);
    }
}
