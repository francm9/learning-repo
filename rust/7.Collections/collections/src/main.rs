fn main() {
    let mut v = vec![1, 2, 3, 4];
    v.push(5);
    for elem in &mut v {
        *elem = *elem + 1;
    }
    for elem in &v{         //Se puede prestar con & o utilizar v.clone()
        print!("{} ", elem);
    }

    let mut index = 0;
    //let elem2 = v.get(index);     //Se uhabria podido continuar usando ya que tiene el Copy trait
    index = index + 1;
    let elem3 = v.get(index);

    if let Some(elem) = elem3 {
        println!("El valor es: {}", elem);
    }

    use std::collections::HashMap;
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("Blue".to_string(), 1);
    map.insert(String::from("Green"), 5);

    let key = String::from("Green");
    
    let elem1 = map.get(&key).copied();
    println!("El elem es: {}", elem1.unwrap());

    for key in map.keys(){
        println!("{}", key);
    }

    for value in map.values(){
        println!("{}", value);
    }
    

    //Dereferenciar un valor --- solo se hace cuando devuelve un valor mutable referenciado
    let value = &mut 20;
    *value = 21;

}
