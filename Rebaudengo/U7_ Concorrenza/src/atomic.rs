use std::time::Duration;

use std::sync::atomic::Ordering::Relaxed;
use std::thread;
use std::sync::atomic::AtomicUsize;
fn main() {
    let num_done = AtomicUsize::new(0);
    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                println!("{}", i);
                num_done.store(i + 1, Relaxed);
            }
        });
        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 { break; }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_nanos(10));
        }
    });

    println!("Done!");
}
