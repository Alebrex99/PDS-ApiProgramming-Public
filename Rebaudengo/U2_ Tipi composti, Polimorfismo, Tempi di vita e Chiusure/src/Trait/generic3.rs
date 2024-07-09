fn sum<T: std::ops::Add<Output = T> + Default + Copy> (items: &[T]) -> T {
    let mut result= T::default();;
    for item in items {
        result = result + *item;
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Somma: {}", sum(&numbers));

    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Somma: {}", sum(&floats));
}
