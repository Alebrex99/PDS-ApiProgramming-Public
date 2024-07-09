fn get_first_element(arr: &[i32]) -> &i32 {
    if arr.is_empty() {
        panic!("Array is empty");
    }
    &arr[0]
}

fn main() {
    let array = [1, 2, 3, 4];
    println!("First element: {}", get_first_element(&array));
}
