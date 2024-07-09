fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn main() {
    let bau = String::from("bau");
    let pair = duplicate(bau);
    println!("{:?}", pair);
}
