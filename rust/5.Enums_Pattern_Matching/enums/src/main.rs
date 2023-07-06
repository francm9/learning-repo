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

    //Prueba de match
    let circulo: Figuras2;
    
    let test = 10;
    match test {
        1 => circulo = Figuras2::Circulo(1.0, 3.1415),
        2 => circulo = Figuras2::Circulo(2.0, 3.1415),
        _ => circulo = Figuras2::Circulo(10.0, 3.1415),
    }
    println!("El area es: {}", circulo.area());

    

}

enum Figuras2 {
	Circulo(f64, f64),
	Rectangulo(f64, f64),
}
impl Figuras2{
    fn area(&self) -> f64{  //Aquí se debe usar como referencia para no perder el ownership
	    match &self {       // A la hora de hacer la comparación se puede hacer con o sin &
		    Figuras2::Circulo(radio, pi) => {
	            radio * radio * pi	
		    }
		    Figuras2::Rectangulo(a, b) => {
		        a * b	
		    }
	    }
    }
}
