
fn main() {

    let mut x = Box::new(150);

    let mut z = &x;

    for i in 0..10
    {
        println!("{:?}", z);
        x = Box::new(i);
    }
    println!("{:?}", z);
}
