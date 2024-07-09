use std::thread;
use std::rc::Rc;

fn main() {
    let data1 = Rc::new(1);
    let data2 = data1.clone();
    println!("t0: {}", *data1);

    let jh = thread::spawn(move || {
        println!("t1: {}", *data2);
    });
    jh.join().unwrap();
}