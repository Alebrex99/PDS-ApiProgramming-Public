struct Example {
    data: i32,
}

impl Example {
    fn get_data_ref(&self) -> &i32 {
        &self.data
    }
}

fn main() {
    let ex = Example { data: 42 };
    let data_ref = ex.get_data_ref();
    println!("Data reference: {}", data_ref);
}
