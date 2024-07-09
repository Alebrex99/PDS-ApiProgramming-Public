use std::collections::VecDeque;

fn main() {
    let mut deque = VecDeque::from(vec![1, 2, 3, 4, 5]);

    // Accesso all'elemento con l'indice 2
    if let Some(element) = deque.get(2) {
        println!("Elemento all'indice 2: {}", element);
    } else {
        println!("Indice non valido");
    }

    // Modifica dell'elemento con l'indice 3
    if let Some(element) = deque.get_mut(3) {
        *element = 10;
        println!("Elemento modificato: {:?}", deque);
    } else {
        println!("Indice non valido");
    }

    for i in 0..5 {
        deque[i] = i;
    }
    println!("{:?}", deque);
}