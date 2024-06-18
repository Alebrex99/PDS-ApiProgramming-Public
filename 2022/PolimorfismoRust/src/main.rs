use std::thread::sleep;
use std::time::Duration;
use crate::timer::Timer;

mod timer{
    use std::time::{Instant}; //gia dati
    pub struct Timer{
    /*creo obj Label per avere visione comprensibile, noto che struttura pubblica,
    dal codice principale la posso usare, ma CAMPI NON PUBBLICI ,non posso leggerli, devo usare emtodi*/
        label:String,
        start: Instant,
    }

    impl Timer{
        /*creo metodo, con funzione new() che accetta label e torna self
        */
        pub fn new(label:String) -> Self{
            Timer {label, start: Instant::now()} //tonra rapp presa da clock
        }
    }
    impl Drop for Timer {
        fn drop(&mut self) {
            //quando butto via timer, voglio capire quanto tempo passato e stamparlo
            println!("Elapsed timer for {}: {:?}", self.label, Instant::now()-self.start);

        }
    }
}

fn main() {
    println!("Hello, world!");
    //creo obj timer, popolato obj, poi obj distrutto e main finisce.
    /*non ho detto struttura ha bisogno trattamento paticlare, perche timer non implementa tratto drop, solo cosi non
    vedo nulla ma parte*/
    let _t = Timer :: new("l1".to_string());
    //sleep(Duration::new(3, 0)); //crea obj timer , sta li 3 sec, poi butta via, ma quando butta via stampa quanto ci ha messo

    /*for i in 1..5{
        let t2 = Timer::new("l2".to_strin());
        /*programma prima crea primo time, inixia secondo timer, inziai e tiene li
        L2 : durera un tot
        poi il for continua, ricrea obj t2, fa ancora sleep , dice quanto durato ecc.
        quando for fintio : distrutto T obj e avremo tempo complessivo, levar t1 e t2, le dichiaro e basta,
        perche con loro presenza sapendo che semplicemente se distrutte fanno qualcosa e ci basta*/
        sleep(Duration::new(3, 0));
        println!("Ciclo #{}", i);
    }*/

    /*aspetto di RAII : distruzione avviene smepre anche se le cose vanno male:
    */
    for i in 1..5{
        let t2 = Timer::new("l2".to_string());
        /*programma prima crea primo time, inixia secondo timer, inziai e tiene li
        L2 : durera un tot
        poi il for continua, ricrea obj t2, fa ancora sleep , dice quanto durato ecc.
        quando for fintio : distrutto T obj e avremo tempo complessivo, levar t1 e t2, le dichiaro e basta,
        perche con loro presenza sapendo che semplicemente se distrutte fanno qualcosa e ci basta*/
        sleep(Duration::new(3, 0));
        println!("Ciclo #{}", i);
        if i==3 {panic!("Errore grave!");} //istruzione che manda in crash programma
        /*tratto RAII : avvengono sempre e comunque , obj comunuqe distrutto
        possiamo cablare nel costruttore un operazione inziiale e nel distruttore op finale, mentre nei posti dove ci
        interessa basta dichiarere var locale che dura nel blocco {} possiamo usarla
        garantirci anche se succede qualcosa */
    }
}
