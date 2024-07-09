
use core::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
fn main() {
    let pair = Arc::new((Mutex::new(Vec::<i32>::new()),Condvar::new()));
    let pair2 = pair.clone();
    let t = thread::spawn(move || {
        let (m, cv) = &*pair2;
        for i in 0..100 {
            let mut v = m.lock().unwrap();
            v.push(i);
            cv.notify_all();
        }
    });
    let (m, cv) = &*pair;
    let mut round = 0;
    while round != 100 {
        let mut v = m.lock().unwrap();
        while round == v.len()
            {
                v = cv.wait(v).unwrap();
            }
        println!("Mentre dormivo sono stati prodotti {} elementi ", v.len() - round);

        for i in round .. v.len()
        {
            println!("{}", v[i]);
        }
   round = v.len();
    }
    t.join().unwrap();
}
