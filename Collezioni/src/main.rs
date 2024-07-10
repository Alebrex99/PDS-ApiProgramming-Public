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
    let mut vec_deque = VecDeque::<i32>::new();
    vec_deque.push_front(1);
    vec_deque.push_front(2);
    vec_deque.push_back(3);
    println!("{:?}", vec_deque);
    println!("{:?}", vec_deque.pop_back());
    println!("{:?}", vec_deque);

    let mut binary_heap = std::collections::BinaryHeap::<i32>::new();
    binary_heap.push(1);
    binary_heap.push(2);
    binary_heap.push(3);
    binary_heap.push(4);
    println!("{:?}", binary_heap);
    println!("{:?}", binary_heap.pop());


}
