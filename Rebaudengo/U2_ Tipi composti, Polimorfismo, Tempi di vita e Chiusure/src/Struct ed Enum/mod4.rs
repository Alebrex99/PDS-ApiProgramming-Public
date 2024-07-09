mod Module1 {

    pub struct Test{
        pub a: i32,
        pub b: bool
    }
}

use Module1::Test;
fn main() {
    let t = Test { a: 12, b: false};
    println!("Hello, world!");
}