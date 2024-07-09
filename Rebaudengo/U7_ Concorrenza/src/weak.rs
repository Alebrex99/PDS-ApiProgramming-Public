use std::sync::{Arc, Weak};
fn main() {
    let data = Arc::new("hello".to_owned());
    let weak_ref = Arc::downgrade(&data); // Create a weak reference

    // Try to upgrade the weak reference
    if let Some(upgraded) = weak_ref.upgrade() {
        println!("Weak reference still alive: {}", upgraded.to_uppercase());
    } else {
        println!("Weak reference is no longer valid.");
    }

    // Dropping the strong reference
    drop(data);

    // Attempt to upgrade again (should return None)
    if let Some(upgraded) = weak_ref.upgrade() {
        println!("Weak reference still alive: {}", upgraded);
    } else {
        println!("Weak reference is no longer valid.");
    }
}
