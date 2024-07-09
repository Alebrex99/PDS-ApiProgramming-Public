use crate::List::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let a = Cons(3, Box::new(Nil));
    let b = Cons(2, Box::new(a));
    let c = Cons(1, Box::new(b));
    let head = Cons(0, Box::new(c));

    // equivalente a
    // let list = Cons(0, Cons(1, Cons(2, Cons(3, Nil))));

    let mut current_node = &head;

    while let Cons(value, next) = current_node {
        println!("Value: {}", value);
        current_node = next;
    }
}

