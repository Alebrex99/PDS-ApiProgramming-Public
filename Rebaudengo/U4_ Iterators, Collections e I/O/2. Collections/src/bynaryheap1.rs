use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from(vec![4, 10, 8, 3, 7]);
    heap.push(1);
    heap.push(15);

    // Stampa della BinaryHeap (senza un ordine particolare)
    println!("BinaryHeap: {:?}", heap);
    // Accesso al massimo elemento (senza rimuoverlo)
    if let Some(max) = heap.peek() {
        println!("Massimo elemento: {}", max);
    } else {
        println!("La BinaryHeap è vuota");
    }
    // Accesso al massimo elemento (senza rimuoverlo) per modificarlo
    if let Some(mut max) = heap.peek_mut() {
        println!("Cambio il massimo elemento: da {} a {}", *max, *max/2);
        *max = *max/2;
    } else {println!("La BinaryHeap è vuota"); }
    // Rimozione del massimo elemento
    if let Some(max) = heap.pop() {
        println!("Elemento rimosso: {}", max);
    } else {
        println!("La BinaryHeap è vuota");
    }
    println!("BinaryHeap dopo la rimozione: {:?}", heap);
}