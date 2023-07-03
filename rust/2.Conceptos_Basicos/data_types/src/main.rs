fn main() {
    //Statics

    let guess: u32 = "42".parse().expect("Not a Number");
    println!("El valor es: {}", guess);

    let test = 42_123_000;
    println!("El valor es: {}", test);

    let test_op = 43.2f32 + 25.2f32;
    println!("El valor es: {}", test_op);

    //Compunds
    
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("El valor de x es: {}", x);
    println!("El valor de y es: {}", y);
    println!("El valor de z es: {}", z);

    let array: [i32; 5];
    let array2 = [1, 2, 3, 4, 5];
    let array3 = [3;5];
}
