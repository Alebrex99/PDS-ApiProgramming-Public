use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};

pub struct DelayedQueue<T:Send>{
    cond: Condvar,
    mutex: Mutex<Vec<(Instant, T)>>,
}

impl<T:Send> DelayedQueue<T> {
    pub fn new() -> Self{
        DelayedQueue{
            cond: Condvar::new(),
            mutex: Mutex::new(vec![]),
        }
    }
    fn take(&self) -> Option<T>{
        let mut queue = self.mutex.lock().unwrap();
        // queue : &mut T -> &mut vec ->
        loop{
            let to_wait;
            match queue.last(){
                None => return None,
                Some(min) => {
                    if min.0.duration_since(Instant::now()) >= Duration::from_nanos(0){
                        let res = queue.pop().unwrap().1;
                        self.cond.notify_all(); //notifico gli altri che ho finito
                        return Some(res)
                    }
                    else{ to_wait = min.0.duration_since(Instant::now()); }
                }
            }
            let res = self.cond.wait_timeout(queue, to_wait).unwrap();
            queue = res.0;
        }
    }
}

fn main() {


}