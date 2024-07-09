use std::collections::BinaryHeap;

fn main() {
    let mut max_heap = BinaryHeap::new();
    max_heap.push(4);
    max_heap.push(2);
    max_heap.push(5);
    max_heap.push(1);
    max_heap.push(7);
    max_heap.push(6);

    while let Some(value) = max_heap.pop() {
        println!("Value: {}", value);
    }
}