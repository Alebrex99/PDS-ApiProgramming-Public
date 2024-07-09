fn main() {
    use std::thread;

    let mut a = vec![1, 2, 3];
    let mut x = 0;

    let builder = thread::Builder::new()
        .name("t1".into())
        .stack_size(100_000);

    let handler = builder
            .spawn( move || {
                a.push(4);
                x = a.iter().sum();
                println!("x: {}", x);
        });
    handler.unwrap().join().unwrap();
    println!("Fine");
}
