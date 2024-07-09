fn get_first_element(numbers: Vec<i32>) -> Option<i32> {
    let first = numbers.first()? ;
    Some(*first)
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let first_element = get_first_element(numbers.clone());

    match first_element {
        Some(n) => println!("Il primo elemento è: {}", n),
        None => println!("La lista è vuota!"),
    }

    let empty_numbers: Vec<i32> = vec![];
    let first_element_empty = get_first_element(empty_numbers.clone());

    match first_element_empty {
        Some(n) => println!("Il primo elemento è: {}", n),
        None => println!("La lista è vuota!"),
    }
}
