use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

    println!("Deque prima di retain: {:?}", deque);

    // Mantieni solo gli elementi che sono multipli di 3
    deque.retain(|&x| x % 3 == 0);

    println!("Deque dopo retain: {:?}", deque);
}
