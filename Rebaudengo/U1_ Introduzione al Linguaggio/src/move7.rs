fn main() {

    let x1 = 42;
    let y1 = Box::new(84);

        {
            let z = (x1, y1);
            println!("z: {:?}", z);
        }
    let x2 = x1;
    println!("x2: {}", x2);
}