/*
Una barriera è un costrutto di sincronizzazione usato per regolare l'avanzamento relativo della computazione di più thread.
All'atto della costruzione di questo oggetto, viene indicato il numero N di thread coinvolti.

Non è lecito creare una barriera che coinvolga meno di 2 thread.

La barriera offre un solo metodo, wait(), il cui scopo è bloccare temporaneamente l'esecuzione del thread che lo ha invocato,
non ritornando fino a che non sono giunte altre N-1 invocazioni dello stesso metodo da parte di altri thread:
- quando ciò succede, la barriera si sblocca e tutti tornano. Successive invocazioni del metodo wait() hanno lo stesso comportamento: la barriera è ciclica.

Attenzione a non mescolare le fasi di ingresso e di uscita!

Una RankingBarrier è una versione particolare della barriera in cui il metodo wait() restituisce un intero che rappresenta l'ordine di arrivo: il primo thread ad avere
invocato wait() otterrà 1 come valore di ritorno, il secondo thread 2, e così via. All'inizio di un nuovo ciclo, il conteggio ripartirà da 1.

Si implementi la struttura dati RankingBarrier a scelta nei linguaggi Rust o C++ '11 o successivi.
*/
use std::cmp::PartialEq;
use std::sync::{Condvar, Mutex};

#[derive(PartialEq)]
enum State{
    Progress,
    Closure
}
struct RankingBarrier{ //N.B. la barriera viene condivisa tra N threads, nel main sarà creato un ARC e dei cloni
    n_threads: usize, //threads coinvolti
    counter: Mutex<(usize, State)>, //threads aggiunti alla barriera, dopo che il primo ha fatto wait.-> uso MUTEX. visto che N possono fare wait in contemporanea
    cv: Condvar //serve per bloccare l'esecuzione del thread finchè non giungono altre N-1 invocazioni di wait
}


impl RankingBarrier{
    fn new(n_threads: usize) -> Result<Self, ()>{
        if n_threads< 2{
            Err(())
        }
        else{
            Ok(RankingBarrier {
                n_threads,
                counter: Mutex::new((0, State::Progress)),
                cv: Condvar::new(),
            })
        }
    }
    /*immagina , si crea nel main la ranking, essa viene messa in ARC, poi viene condivisa tra N threads.
    Possono chiamare wait a loro piacimento*/
    fn wait(&self) -> usize{
        let mut lock = self.counter.lock().unwrap();
        lock.0+=1;
        while lock.0 != self.n_threads && lock.1 == State::Progress{
            lock= self.cv.wait(lock).unwrap();
        }
        //appena esci dal wait, perchè la barrierà è chiusa. escono 1 alla volta
        lock.0-=1; //svuoto la barriera, decremento, 1 alla volta si svegliano
        // ho raggiunto N invocazioni
        if (lock).0==self.n_threads-1{ //se riempio tutta la barriera, perchè il conteggio degli wait è pari a n threads (-1 a causa del main)
            (lock).1=State::Closure; //chiudo la barriera
            self.cv.notify_all(); //sveglio i threads, perchè devono uscire dalla wait e tornare
        }

        //riapertura barriera, quando counter a 0
        if lock.0 == 0{
            lock.1 = State::Progress; //quando la barriera è riapert
            self.cv.notify_all()//per sicurezza
        }


        return lock.0;
    }

}