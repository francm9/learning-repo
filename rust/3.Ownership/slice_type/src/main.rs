fn main() {
    let s = String::from("test cadena");
    let s2 = first_word(&s);    //Si se intentase hace un clear de la cadena de referencia,
                                //saltaría un error debido a que la cadena se ha modificado
    println!("{}", s2);

    let array = [1,2,3,4];
    let elems1_2 = &array[0..2];

    println!("{} {}", elems1_2[0], elems1_2[1]);
}

fn first_word(s: &String) -> &str {
    &s[0..4]
}

