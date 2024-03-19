fn main() {
    use paho_mqtt as paho_mqtt;

    // Creación de opciones para el cliente MQTT
    let opts = mqtt::CreateOptionsBuilder::new()
        .server_uri("tcp://localhost:1883")
        .client_id("rust-client")
        .finalize();

    // Configuración de opciones para la conexión
    let con_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20));

    let client = paho_mqtt::Client::new(opts);
    let conecction = paho_mqtt::Client::connect(&self, opt_opts);

}
