use std::rc::Rc;
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}
fn main() {
    let nipote1 = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    let nipote2 = Rc::new(Node {
        value: 6,
        children: vec![],
    });

    let padre = Rc::new(Node {
        value: 9,
        children: vec![Rc::clone(&nipote1), Rc::clone(&nipote2)],
    });

    let nonno = Rc::new(Node {
        value: 27,
        children: vec![Rc::clone(&padre)],
    });
    println!("{:#?}", nonno);

}
