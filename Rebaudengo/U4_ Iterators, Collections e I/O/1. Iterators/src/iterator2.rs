struct MyRange {
    count: usize,
}

impl Iterator for MyRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            self.count -= 1;
            Some(self.count)
        }
    }
}

fn main() {
    let range_iter = MyRange {count: 20};
    
    for n in range_iter {
        println!("Next number: {}", n);
    }
}

