use std::sync::Mutex;
use std::thread;

fn main() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
               let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }

        	      println!("Alla fine del thread:{:?} n = {:?}", 
				thread::current().id(), guard);
            });
        }
    });

}
