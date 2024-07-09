
use std::collections::LinkedList;
fn main() {
    let mut list = LinkedList::new();
    list.push_back("a".to_string());
    list.push_back("b".to_string());
    list.push_back("c".to_string());

    let mut tail = list.split_off(1);

    list.push_back("x".to_string());
    list.append(&mut tail);

    for element in list.iter() {
        println!("{}", element);
    }
    // Questo stamperà: a, x, b, c
}