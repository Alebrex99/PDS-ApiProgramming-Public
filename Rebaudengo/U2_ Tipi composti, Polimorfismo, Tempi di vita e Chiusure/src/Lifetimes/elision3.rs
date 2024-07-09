fn get_element<T>(data: & [T], index: usize) -> Option<&T> {
    data.get(index)
}

fn main() {
    let data = &[1, 2, 3, 4, 5];
    let index = 2;

    match get_element(data, index) {
        Some(element) => println!("Element at index {}: {}", index, element),
        None => println!("Index out of bounds"),
    }
}
