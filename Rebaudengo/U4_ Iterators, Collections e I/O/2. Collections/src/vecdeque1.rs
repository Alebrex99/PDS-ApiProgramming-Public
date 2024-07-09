use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back(1);
    queue.push_back(2);

    queue.push_front(3);
    queue.push_front(4);

    println!("Queue: {:?}", queue);

    // Rimozione di un elemento dalla testa della coda
    if let Some(front) = queue.pop_front() {
        println!("Element removed from the front: {}", front);
    }

    println!("Queue after removal: {:?}", queue);

    // Rimozione di un elemento dalla fine della coda
    if let Some(back) = queue.pop_back() {
        println!("Element removed from the back: {}", back);
    }

    println!("Queue after removal from back: {:?}", queue);
    println!("Is the queue empty? {}", queue.is_empty());
}