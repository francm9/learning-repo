use std::{error::Error, thread::sleep, time::Duration};
use mqtt::{topic, QOS_0};
use paho_mqtt as mqtt;
use rppal::gpio::{Gpio, OutputPin};

fn write_led(led: &mut OutputPin,value: u8){
   if (value == 0){
        led.set_low();
   } else if (value == 1){
        led.set_high();
   } else {
        println!("Valor incorrecto");
   }
}

fn publish_msg(client: &mqtt::Client, topic: &str, payload: &str){
    let msg = mqtt::Message::new(topic, payload.to_string(), QOS_0);
    if let Err(_) = client.publish(msg){
        println!("No se ha podido publicar el mensaje");
    }
}

fn manage_msg(topic: &str, payload: &str, led_1: &mut OutputPin, led_2: &mut OutputPin){
    if topic == "test/led1"{
        let value = payload.parse::<u8>().unwrap();
        write_led(led_1, value);
    } else if topic == "test/led2"{
        let value = payload.parse::<u8>().unwrap();
        write_led(led_2, value);
    } else {
        println!("Topic no reconocido");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Inicializar los leds
    let mut led_1 = Gpio::new()?.get(14)?.into_output();
    let mut led_2 = Gpio::new()?.get(15)?.into_output();

    // Creación de opciones para el cliente MQTT
    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://192.168.1.137:1883")
        .client_id("raspberry")
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
    
    let test = client.start_consuming();

    // Suscribirse a dos topics
    if let Err(_) = client.subscribe("test/led1", QOS_0){
        println!("No se ha podido suscrbir al topic test/led1");
    }

    if let Err(_) = client.subscribe("test/led2", QOS_0){
        println!("No se ha podido suscrbir al topic test/led2");
    }

    loop {
        for msg in test.iter(){
            if let Some(msg) = msg {
                println!("Topic: {} \nPayload: {}", msg.topic(), msg.payload_str());
                manage_msg(msg.topic(), msg.payload_str().as_ref(), &mut led_1, &mut led_2)
            }
        }
        sleep(Duration::from_secs(1))
    }
}
