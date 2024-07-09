fn main() {
    let numbers = [1, 2, 3, 4, 5];

    // Itera attraverso ciascun elemento dell'array e stampalo

    for num in numbers.iter() {
        println!("{}", num);
    }

    for num in &numbers {
        println!("{}", num);
    }
}