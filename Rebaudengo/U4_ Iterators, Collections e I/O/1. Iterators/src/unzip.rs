fn main() {
    let data = vec![(1, 'a'), (2, 'b'), (3, 'c')];

    // Unzip della collezione di tuple in due iteratori separati
    let (numbers, characters): (Vec<_>, Vec<_>) = data.into_iter().unzip();

    println!("Numeri: {:?}", numbers);
    println!("Caratteri: {:?}", characters);
}
