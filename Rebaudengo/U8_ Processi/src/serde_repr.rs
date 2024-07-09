use crate::SmallPrime::*;
use serde_repr::*;

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum SmallPrime {
    Two = 2,
    Three,
    Five = 5,
    Seven = 7,
    Eight
}
fn main() {
    let nums: Vec<SmallPrime> = vec![Two, Three, Five, Eight, Seven];

    // Prints [2,3,5,8,7]
    println!("{}", serde_json::to_string(&nums).unwrap());
}


