use std::sync::mpsc::{channel, Receiver, Sender};
pub struct Exchanger<T: Send> {
    tx: Sender<T>,
    rx: Receiver<T>,
}
impl <T:Send> Exchanger<T> {
    /* quando viene invocato, si arresta senza consumare cicli di CPU fino a che non
    viene invocato il metodo corrispettivo sulla struttura ad essa accoppiata, dopodiché
    restituisce il valore che è stato passato come argomento al metodo gemello (che farà
    altrettanto), sotto forma di Some(t)*/
    pub fn exchange(&self, t:T) -> Option<T> {
        self.tx.send(t).ok()?;
        self.rx.recv().ok()
    }
}
pub fn make_exchangers<T:Send>() -> (Exchanger<T>, Exchanger<T>) {
    let (tx1, rx1) = channel::<T>();
    let (tx2, rx2) = channel::<T>();
    return (
        Exchanger { tx:tx1, rx: rx2 }, //ATTENDO : SCAMBIAMO
        Exchanger { tx:tx2, rx: rx1 }
    )
}
