
fn main() {
   let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(2);
    list.push_back(4);
    list.push_front(5);
    list.push_front(1);

    println!("Linked List: {:?}", list);

    // Inserimento di un elemento all'inizio della lista
    list.push_front(0);

    println!("Linked List after push_front: {:?}", list);

    // Rimozione dell'ultimo elemento dalla lista
    if let Some(last) = list.pop_back() {
        println!("Element removed from the back: {}", last);
    }

    println!("Linked List after pop_back: {:?}", list);

    // Rimozione del primo elemento dalla lista
    if let Some(first) = list.pop_front() {
        println!("Element removed from the front: {}", first);
    }
    // Stampa della lista dopo la rimozione dal primo
    println!("Linked List after pop_front: {:?}", list);
}
