use std::ops::Index;

struct MyVec{
    data: Vec<i32>,
    year: i32
}

impl MyVec {
    fn new(data: Vec<i32>) -> Self {
        MyVec{data: data, year: 2024}
    }
}

impl Index<std::ops::Range<usize>> for MyVec {
    type Output = [i32];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}

fn main() {
    let my_vec = MyVec::new(vec![1, 2, 3, 4, 5]);

    // Accesso tramite intervallo di indici
    let slice = &my_vec[1..4];
    println!("Slice: {:?}", slice); // Stampato: [2, 3, 4]
}