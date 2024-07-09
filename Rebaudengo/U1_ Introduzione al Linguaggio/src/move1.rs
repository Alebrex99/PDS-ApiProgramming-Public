fn main() {
    let mut my_box = Box::new(1);

    my_box =  Box::new(2);
    println!("{:?}", my_box);
}