use std::thread;
use std::time::Duration;

fn run(msg: &str) {
    for i in 0..10 {
        println!("{}{}",msg,i);
        thread::sleep(Duration::from_nanos(1));
    }
}

fn main() {
    let t1 = thread::spawn(|| {run( "aaaa"); });
    let t2 = thread::spawn(|| {run(" bbbb"); });
    t1.join().unwrap();
    t2.join().unwrap();
}