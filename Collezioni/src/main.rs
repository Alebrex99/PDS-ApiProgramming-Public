use std::collections::{LinkedList, VecDeque};

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    //VEC
    let vec = vec![1,2,3,4,5,6];
    let row = vec![ //vettore di enum con valori non omogenei
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //VEC DEQUE
    let mut vec_deque = VecDeque::new();

    let mut lista: LinkedList<i32> = LinkedList::new();

}
