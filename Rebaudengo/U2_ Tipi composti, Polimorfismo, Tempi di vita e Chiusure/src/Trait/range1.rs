use std::ops::RangeBounds;
fn main() {
    let range = 1..; // Definizione di un range
    let end = range.end_bound(); // Ottenere il limite superiore del range

    match end {
        std::ops::Bound::Included(&upper) => println!("Il limite superiore è incluso: {}", upper),
        std::ops::Bound::Excluded(&upper) => println!("Il limite superiore è escluso: {}", upper),
        std::ops::Bound::Unbounded => println!("Il range non ha limite superiore"),
    }
}