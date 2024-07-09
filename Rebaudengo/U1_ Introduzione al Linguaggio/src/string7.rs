use std::str;

fn main() {
    let bytes_array: [u8; 5] = [104, 101, 108, 108, 111];
    // Array di byte ASCII per "hello"

    let stringa = str::from_utf8(&bytes_array) ;

    println!("{:?}", stringa);

}