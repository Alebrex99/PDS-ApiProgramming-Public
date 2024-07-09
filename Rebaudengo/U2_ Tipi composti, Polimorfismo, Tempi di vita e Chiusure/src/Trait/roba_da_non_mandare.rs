use std::ops::Deref;

struct MyBox <String>(String);
impl MyBox<String> {
    fn new (x: String) -> MyBox<String> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn hello (name: &str)
{
    println!("Hello, {}!", name);
}

fn main() {

    let m = MyBox::new(String::from("Rust"));

    hello(&(*m)[..]);
}