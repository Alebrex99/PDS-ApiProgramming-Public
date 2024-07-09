use std::time::Duration;
use std::thread::sleep;
fn main() {
    for i in 1..100 {
      sleep(Duration::from_secs(1));
        println!("{i}");
    }
}

