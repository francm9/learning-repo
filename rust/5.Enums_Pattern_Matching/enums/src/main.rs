enum Figuras{
    Circulo,
    Cuadrado,
}

struct Example{
    figura: Figuras,
}
fn main() {
    let _figura = Figuras::Circulo;
    let _example = Example{
        figura: Figuras::Cuadrado,
    };

    let mut option: Option<i32> = None;
   
    if option == None{
        println!("No hay valor");
    }
    option = Some(5);
    println!("El valor es: {}", option.unwrap());
    option = Some(10);
    println!("El valor es: {}", option.unwrap());
}
