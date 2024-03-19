fn main() {
    use paho_mqtt as paho_mqtt;
    let opts = mqtt::CreateOptionsBuilder::new()
    .server_uri("tcp://localhost:1883")
    .client_id("client1")
    .finalize();

    let client = paho_mqtt::Client::new(opts);
    let conecction = paho_mqtt::Client::connect(&self, opt_opts);

}
