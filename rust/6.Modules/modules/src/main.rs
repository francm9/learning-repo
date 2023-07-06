use crate::library::punto;

pub mod library;
fn main() {
    let punto = punto::Punto { x: 1, y:2, };
    println!("x = {}, y = {}", punto.x, punto.y);
}
