
use std::rc::{Rc,Weak};
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()), }); // even the Weak pointer has an associated new() fn
    println!("leaf: strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf) );
    { // create a new scope within our main function
        let root = Rc::new(Node{ // create a branch where the previously created leaf is our child
            value:5,
            children: RefCell::new(vec![Rc::clone(&leaf)]),
            parent: RefCell::new(Weak::new()), });
        // 1. borrow a mutable reference of the leaf's parent
        // 2. dereference it
        // 3. set it equal to a weak reference to the root node
        *leaf.parent.borrow_mut() = Rc::downgrade(&root);

        println!("root: strong = {} weak = {}", Rc::strong_count(&root), Rc::weak_count(&root));
        println!("leaf: strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
        println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    } // parent goes out of scope even though the child has a reference to it
    // the parent now equals None! it's gone out of scope even though we had a reference to it!
    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade());
    println!("leaf: strong = {} weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
