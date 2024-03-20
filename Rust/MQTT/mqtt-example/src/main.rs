use std::{borrow::Borrow, error::Error, thread::sleep, time::Duration};
use mqtt::QOS_0;
use paho_mqtt as mqtt;
use rppal::gpio::{Gpio, OutputPin};

fn write_led(led: &mut OutputPin,value: u8){
   led.set_low(); 
}

fn main() -> Result<(), Box<dyn Error>> {
    // Inicializar los leds
    let mut led_1 = Gpio::new()?.get(14)?.into_output();
    let mut led_2 = Gpio::new()?.get(15)?.into_output();
    
    write_led(&mut led_1, 0);
    write_led(&mut led_2, 0);
    
    println!("LED1 is high? -> {} " , led_1.is_set_high());
    println!("LED2 is high? -> {} " , led_2.is_set_high());


    // Creación de opciones para el cliente MQTT
    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://192.168.1.148:1883")
        .client_id("rust-client")
        .finalize();

    // Configuración de opciones para la conexión
    let con_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    let client = paho_mqtt::Client::new(opts).unwrap();
    if let Err(_) = client.connect(con_opts) {
        print!("No se ha podido conectar");
    }

    // Publicar un mensaje
    /*let content = "Hello world";*/
    /*let msg = mqtt::Message::new("test/rust", content.clone(), QOS_0);*/
    /*let _tok = client.publish(msg);*/
    
    let test = client.start_consuming();

    // Suscribirse a dos topics
    if let Err(_) = client.subscribe("test/led1", QOS_0){
        println!("No se ha podido suscrbir al topic test/led1");
    }

    println!("Entra en el bucle");
    loop {
        for msg in test.iter(){
            if let Some(msg) = msg {
                println!("{}", msg)
            }
        }
        sleep(Duration::from_secs(1))
    }
}
