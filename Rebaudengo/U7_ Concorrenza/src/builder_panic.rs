fn main() {
    use std::thread;

    let a = vec![1, 2, 3];

    let builder = thread::Builder::new()
        .name("t1".into())
        .stack_size(100_000);

    let handler = builder
        .spawn( move || {
            println!("{}", a[4]); // panica. Viene fornito il nome del thread.
        });
    handler.unwrap().join().unwrap();
    println!("Fine");
}