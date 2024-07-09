struct Example {
    data1: i32,
    data2: i32,
}

impl Example {
    
    fn get_data_ref<'a>(&'a self, other: &'a i32) -> &'a i32 {
        if self.data1 > *other {
            &self.data1
        } else {
            &other
        }
    }
}

fn main() {
    let ex = Example { data1: 42, data2: 20 };
    let other_data = 30;
    let data_ref = ex.get_data_ref(&other_data);
    println!("Data reference: {}", data_ref);
}