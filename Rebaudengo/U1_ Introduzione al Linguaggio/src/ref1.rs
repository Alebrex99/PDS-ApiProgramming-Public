fn main() {
    let mut i = 32;

    let r = &i;
    println!("{}", *r);

    i = i+1;
    println!("{i}");
}