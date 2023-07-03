fn test_function(p1:u8) -> u8{
    println!("p1 = {}", p1);
    p1 + 10
}
fn main() {
    let p1 = test_function(10);
    println!("p1 tras aplicar funcion = {}", p1);

    let p2 = {
        let x = 5;
        x + 5
    };
    println!("p2 = {}", p2);
}
