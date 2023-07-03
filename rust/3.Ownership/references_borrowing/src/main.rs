fn main() {
    let mut s = String::from("Esto es un String");
    test_fn(&mut s);
    println!("{}", s);
}

fn test_fn(s: &mut String){
    s.push_str(" concatenado");
}
