struct Test {
    a: i32,
    b: i32,
}

impl Test {
    fn change_a(&mut self, a: i32) {
        self.a = a;
    }

    fn change_b(&mut self, b: i32) {
        self.b = b; 
    }
}

fn main() {
    let mut t = Test { a: 1, b: 2,};
    println!("a:{} b:{}", t.a, t.b);
    t.change_a(3);
    t.change_b(4);
    println!("a:{} b:{}", t.a, t.b);
}
