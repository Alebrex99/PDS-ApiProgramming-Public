use std::ffi::CString;


fn main() {
    let ss = "Hello".to_string();
    let s = CString::new(ss).unwrap();
    println!("{:?}", s);
}