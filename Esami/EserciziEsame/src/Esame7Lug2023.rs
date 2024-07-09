use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::{Condvar, Mutex};
use std::time::{Duration, Instant};


//VERSIONE 1
pub struct DelayedQueue<T:Send>{
    cond: Condvar, //tutte le volte che bisogna aspettare senza consumare CPU : serve convar
    data: Mutex<Vec<(Instant, T)>>, //poichè bisognerà accedere, inserire e fare cose in modo thread safe
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

    pub fn offer(&self, t: T, i:Instant){
        let mut queue = self.data.lock().unwrap();
        // se inserisco ordinatamente il prossimo elemento è sempre in fondo (take sarà piu facile)
        match queue.last() {
            None => queue.push((i, t)),
            Some(elem) => {
                if i < elem.0 {
                    //inserisco in coda
                    queue.push((i, t))
                } else {
                    for (idx, elem) in queue.iter().enumerate() {
                        if i > elem.0 {
                            queue.insert(idx, (i, t));
                            break;
                        }
                    }
                }
            }
        }
        self.cond.notify_all();
    }

}



//VERSIONE MALNATI-------

pub struct Item<T: Send>{
    i: Instant,
    t: T,
}
/*Item<T> deve implementare ORD per stare in una queue , non posso fare DERIVE, lui cercherebbe di ordinare
pure T , ma di T non conosco nulla e non potrei ordinarlo: DA FARE A MANO*/
impl<T: Send> Eq for Item<T> {}
impl<T: Send> PartialEq<Self> for Item<T> {
    fn eq(&self, other: &Self) -> bool {
        other.i == self.i
    }
}
impl<T: Send> PartialOrd<Self> for Item<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.i.partial_cmp(&self.i)
    }
}
impl<T: Send> Ord for Item<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        //self.i.cmp(&other.i) //non va bene cosi, vuoi che una priorità in modo che il più piccolo tempo sia il preferito
        other.i.cmp(&self.i)
    }
}

pub struct DelayedQueueMAL<T:Send>{
    cv: Condvar,
    data: Mutex<BinaryHeap<Item<T>>>
}

impl <T: Send> DelayedQueueMAL<T>{
    pub fn new() -> Self{
        DelayedQueueMAL{
            cv: Condvar::new(),
            data: Mutex::new(BinaryHeap::new()),
        }
    }
    pub fn size(&self) -> usize{
        self.data.lock().unwrap().len()
    }
    pub fn take(&self) -> Option<T>{
        let mut data = self.data.lock().unwrap();
        while(!data.is_empty()){
            let now = Instant::now();
            let greater = data.peek().unwrap();
            let i = greater.i;
            //il primo elemento è quello più grande, non voglio toglierlo, può non essere ancora scaduto
            if i<now{ //il dato è scaduto oltrepassata
                Some(data.pop().unwrap().t);
            }
            else {
                /*dovrò fare il time out della differenza. può darsi che quello in coda sia l'ultimo messo
                che non è ancora scaduto, nessuno aggiungerà più nulla (nessuno farà una notify), ma
                devo darmi una attesa per un tempo */
                let duration = now - i;
                data = self.cv.wait_timeout(data, duration).unwrap().0; //puoi uscire sia a causa di una notify / di tempo scaduto
            }
        }
        return None;
    }

    pub fn offer(&self, t: T, i:Instant){
        let mut queue = self.data.lock().unwrap(); //solo quando ho accesso al binary heap posso pushare
        let elem = Item{t, i};
        queue.push(elem);
        /*quando ho pushato il dato, devo notificare tutti gli altri thread che ho finito,
        perchè poteva esserci qualcuno in attesa e dopo che ho inserito a lui cambierà tutto*/
        self.cv.notify_all();
    }
}