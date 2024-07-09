use std::rc::Rc;
fn main() {
    let five = Rc::new(5);
    let ten = Rc::clone(&five);
    
    let weak_five = Rc::downgrade(&five);
    
    println!("Conteggio referenze strong di five: {}", Rc::strong_count(&five));
    println!("Conteggio referenze weak di five: {}", Rc::weak_count(&five));
    
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    
    println!("Conteggio referenze strong di five: {}", Rc::strong_count(&five));
    println!("Conteggio referenze weak di five: {}", Rc::weak_count(&five));
    drop(strong_five);
    println!("Conteggio referenze strong di five: {}", Rc::strong_count(&five));
    println!("Conteggio referenze weak di five: {}", Rc::weak_count(&five));
    drop(ten);
    drop(weak_five);
    println!("Conteggio referenze strong di five: {}", Rc::strong_count(&five));
    println!("Conteggio referenze weak di five: {}", Rc::weak_count(&five));
}

