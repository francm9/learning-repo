fn main() {
    let mut number = 3;
    if number < 3 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    'count_to_10: loop {
        number += 1;
        if number == 10 { break 'count_to_10; }
    }

    println!("The value of number is: {}", number);

    for elem in 0..10 {
        println!("{}", elem);
    }
}
