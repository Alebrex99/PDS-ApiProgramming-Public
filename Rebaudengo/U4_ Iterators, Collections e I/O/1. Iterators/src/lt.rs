fn main() {
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6];

    // Confronto dei due iteratori di numeri interi
    let result = numbers1.iter().lt(numbers2.iter());

    if result {
        println!("Il primo iteratore è minore del secondo");
    } else {
        println!("Il primo iteratore non è minore del secondo");
    }
}
