mod Module1 {

pub struct Test{
    a: i32,
    b: bool
}

    fn f ()
    {
        let t = Test { a: 1, b: false};
        println!("Hello, main ");
    }
}

use Module1::Test;

fn main() {
    let t = Module1::Test { a: 12, b: false};
    println!("Hello, world!");
}