
use std::rc::Rc;
fn main() {
    let five = Rc::new(5);

    let weak_five = Rc::downgrade(&five);

    println!("{:?}", five);
    drop(five);

    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    println!("{:?}", strong_five);
}