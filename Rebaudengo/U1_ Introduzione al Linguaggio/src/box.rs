fn makeBox(a: i32) -> Box<(i32,i32)> {
    let r = Box::new( (a, 1) );
    return r;
}

fn main() {
    let mut b = makeBox(5);
    b.0 = b.0 * 2;
    let c  = b.0 + b.1;

    println!("{c}");
}