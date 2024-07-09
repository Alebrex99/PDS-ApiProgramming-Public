use std::collections::LinkedList;
fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();
    list.push_back(3);
    list.push_back(1);
    list.push_back(5);
    list.push_back(2);

    // Convertire la LinkedList in un Vec
    let mut vec: Vec<_> = list.into_iter().collect();

    // Ordinare il Vec
    vec.sort();

    // Convertire il Vec ordinato in una LinkedList
    let sorted_list: LinkedList<_> = vec.into_iter().collect();

    // Stampa la lista ordinata
    for element in sorted_list.iter() {
        println!("{}", element);
    }
}