

fn main() {
    let mut s = String::from("Esto es un String");
    println!("{}",s);

    //let mut s2 = "Hola"; --- No puede ser mutable
    //Haciendolo con String::from() sí que puede ser mutable

    s.push_str("\nEs otro string");
    println!("{}", s);
       
    //Si copiamos este string en otro, el anterior se expulsa del scope
    //y queda invalidado, por lo tanto para tener dos elementos
    //con el mismo contenido, se debe usar clone

    let s2 = s.clone();

    println!("s1 = {}\ns2 = {}", s, s2);

    let test = String::from("Test");
    take_ownership(test.clone());

    let test = changes(test.clone());

    println!("Test: {}", test);
}

fn take_ownership(str: String){
    println!("{}", str);
}

fn changes(mut str: String) -> String{
    str.push_str("Esto es añadido");
    str
}
