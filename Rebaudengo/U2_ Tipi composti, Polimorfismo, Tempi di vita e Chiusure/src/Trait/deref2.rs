struct Selector {
    elements: Vec<String>,
    current: usize
}

use std::ops::Deref;

impl Deref for Selector {
    type Target = String;
    fn deref(&self) -> &String { & self.elements[ self.current ] }
}

fn main() {


let mut s = Selector{elements: vec!["a".to_string(), "b".to_string()], current:0};
assert_eq!(*s, "a");

s.current = 1;
assert_eq!(*s, "b");
}