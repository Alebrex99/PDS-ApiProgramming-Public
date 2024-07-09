fn max<T: PartialOrd>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        None
    } else {
        let mut max = &items[0];
        for item in items.iter() {
            if item > max { max = item; }
        }
        Some(max)
    }
}
fn my_max <T: std::fmt::Display> (value: Option<&T>)
{  match value {
        Some(max) => println!("Massimo: {}", max),
        None => println!("Sequenza vuota")
    }
}
fn main() {
    let numbers = vec![1, 3, 5, 2, 4];
    my_max(max(&numbers));
    let strings = vec!["apple", "banana", "orange", "grape"];
    my_max(max(&strings));
}
