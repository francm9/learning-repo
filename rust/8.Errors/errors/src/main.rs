use std::fs::File;
fn main() {
    //panic!("Te expulso porque me sale de la polla");
    let ok = File::open("test.txt");
    let file = match ok {
        Ok(result) => result,
        Err(error) => panic!("Error abriendo el archivo. Error: {}", error),
    };
}
