use std::ops::{Index, IndexMut};

#[derive(Debug)]
struct StringSet {
    data: Vec<String>,
}

impl StringSet {
    fn new() -> Self {
        StringSet { data: Vec::new() }
    }

    fn add(&mut self, s: String) {
            self.data.push(s)
    }
}

impl Index<usize> for StringSet {
    type Output = String;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for StringSet {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

fn main() {
    let mut string_set = StringSet::new();
    string_set.add("apple".to_string());
    string_set.add("banana".to_string());
    string_set.add("orange".to_string());
    println!("Original StringSet: {:?}", string_set);

    string_set[1] = "grape".to_string();
    println!("Modified StringSet: {:?}", string_set);
}