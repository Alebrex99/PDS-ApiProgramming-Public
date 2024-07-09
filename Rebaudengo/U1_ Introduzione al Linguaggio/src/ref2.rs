fn main() {
    let mut i = 32;

    let r = &mut i;

    *r = *r+1;
    println!("{}", *r);

    println!("{}", i);
}
