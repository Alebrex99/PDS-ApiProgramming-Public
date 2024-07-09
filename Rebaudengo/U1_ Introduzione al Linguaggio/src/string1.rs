fn main() {
    let s1 = String::from("hello");
    let mystr = s1.as_str();

    let len = calculate_length(mystr);

    println!("The length of '{}' is {}.", mystr, len);
}
fn calculate_length(s: &str) -> usize {
    let length = s.len(); 
    length
}
