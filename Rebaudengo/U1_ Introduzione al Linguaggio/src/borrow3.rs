use rand::Rng;
fn main() {

    let mut b = Box::new(84);
    let r = & mut b;

    *r = Box::new(100);

    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..10);

    if n > 5 {
    println!("{:?}", b);
    }
    else {
        println!("{:?}", r);
    }
}