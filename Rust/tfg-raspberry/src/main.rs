//use reqwest;
use std::{process::Command, ops::Index};
use execute::Execute;

// #[tokio::main]
fn main(){
    let path = "./src/scripts/get_temperature.sh";
    let mut temperature_command = Command::new(path);
    let temperature = temperature_command.output().expect("Fallo al ejecutar el comando");
    println!("Temperatura: {}", String::from_utf8_lossy(&temperature.stdout));
}
