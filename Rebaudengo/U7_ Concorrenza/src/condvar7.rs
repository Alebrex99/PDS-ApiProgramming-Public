use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut pending = lock.lock().unwrap();
        *pending = true;
        thread::sleep(Duration::from_secs(1));
// We notify the condvar that the value has changed.
        cvar.notify_one();
    });

// wait for the thread to start up
    let (lock, cvar) = &*pair;
    let result = cvar.wait_timeout_while(
        lock.lock().unwrap(),
        Duration::from_millis(100),
        |&mut pending| !pending,
    ).unwrap();
    if result.1.timed_out() {
              // timed-out without the condition ever evaluating to false.
        println!("Time out with {}", *result.0);
    }
    else {
        println!("Not time out with {}", *result.0);
    }

}
