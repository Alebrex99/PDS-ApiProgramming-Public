use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};


//VERSIONE 1
pub struct DelayedQueue<T:Send>{
    cond: Condvar, //tutte le volte che bisogna aspettare senza consumare CPU : serve convar
    data: Mutex<Vec<(Instant, T)>>, //poichè bisognerà accedere, inserire e fare cose in modo thread safe
}

pub struct Item<T: Send>{
    i: Instant,
    t: T,
}

pub struct DelayedQueueMAL<T:Send>{
    cond: Condvar,
    data: Mutex<BinaryHeap<Item<T>>>
}

impl<T:Send> DelayedQueue<T> {
    pub fn new() -> Self{ //devo poter inserire delle cose in modo thread safe e estrarle in modo thread safe
        DelayedQueue{
            cond: Condvar::new(),
            data: Mutex::new(vec![]),
        }
    }
    pub fn size(&self) -> usize{
        self.data.lock().unwrap().len()
    }
    pub fn take(&self) -> Option<T>{
        let mut queue = self.data.lock().unwrap(); //prima di tutto estraggo i dati, ovvero piglio lock per accedere alla coda
        // queue : &mut T -> &mut vec ->
        loop{ //ora cicliamo
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

    pub fn take_malnati()-> Option<T>{
        Some(1)
    }
    pub fn offer<T>(&self, t: T, i:Instant){
        let mut data = self.data.lock().unwrap(); //estraggo i dati dalla coda
        //inserisci un elemento che non potrà essere estratto prima dell'istante di scadenza i
        data.push((t, i )); //io ho inserito ma poteva esserci qualcuno che aspettava, quindi la cosa inserita può modificare le carte in tavola
        self.cond.notify_all(); //ecco che bisogna notificare a tutti
    }

}

fn main() {


}