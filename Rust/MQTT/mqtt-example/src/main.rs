use std::time::Duration;
use mqtt::QOS_0;
use paho_mqtt as mqtt;

fn main() {
    // Creación de opciones para el cliente MQTT
    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
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
    let content = "Hello world";
    let msg = mqtt::Message::new("test/rust", content.clone(), QOS_0);
    let tok = client.publish(msg);
}
