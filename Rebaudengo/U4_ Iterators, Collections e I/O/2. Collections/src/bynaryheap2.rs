use std::collections::BinaryHeap;

fn main() {
    let mut max_heap = BinaryHeap::new();
    max_heap.push(4);
    max_heap.push(2);
    max_heap.push(5);
    max_heap.push(1);
    max_heap.push(7);
    max_heap.push(6);

    println!("Value: {:?}", max_heap);

    // Scandire il BinaryHeap senza rimuovere i valori
    for value in max_heap.iter() {
        println!("Value: {}", value);
    }
}