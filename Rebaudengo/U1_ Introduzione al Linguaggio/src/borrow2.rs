fn main() {

    let mut b = Box::new(84);
    let r = & mut b;

    *r = Box::new(100);

    println!("{:?}", r);
    println!("{:?}", b);

}