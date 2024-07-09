use std::ops::Deref;
#[derive(Debug)]
struct CustomStruct {
    number: i32,
    boxed_value: Box<i32>,
}

impl Deref for CustomStruct {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.number
    }
}

fn main() {
    let boxed_value = Box::new(42);
    let custom_struct = CustomStruct { number: 10, boxed_value };

    let mut custom_struct_ref = &custom_struct;

    println!("Dereferencing custom_struct: {:?}", *custom_struct_ref);  // Prints: Dereferencing custom_struct: 10
    println!("Dereferencing boxed_value: {:?}", *custom_struct_ref.boxed_value);  // Prints: Dereferencing boxed_value: 42
}

