use std::sync::mpsc::{channel, sync_channel};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

fn main() {

    //-------------------------------------CONCORRENZA: STATO CONDIVISO----------------------------------
    //----------------------------------THREADS + ARC + MUTEX ------------------------------------
    println!("------------THREADS + ARC + MUTEX--------------");
    //MUTEX NON E' COPY, VIENE MOSSO
    let mutex = Mutex::new(0); //non condivisibile cosi, non implementa il tratto COPY
    let mutex_moved = mutex; //questo lo muove solamente, per copiare il riferimento dobbiamo usare ARC
    //println!("{:?}", mutex); //error: value borrowed here after move
    let mutex_moved = Arc::new(mutex_moved); //condivisibile
    let mutex_copy = mutex_moved.clone(); //crea un riferimento solo condivisibile
    let mut lock = mutex_moved.lock().unwrap(); //prendi un riferimento mutabile al contenuto del MUTEX
    println!("{}", *lock); //0

    //RIFERIMENTO A MUTEX: BORROW
    let mutex2 = Mutex::new(0);
    let a = &mutex2; //borrowed possibile solo in lettura
    println!("{:?}", *a); //Mutex { data: 0 }

    //THREAD : SYNC E SEND
    /*let i=0; //impl SYNC
    let arc = Arc::new(&i);
    let t3 = thread::spawn(move || {
        println!("{}", *arc);
    });
    t3.join().unwrap();*/


    //ESEMPIO DI CONDIVISIONE DI DATI TRA THREADS
    let a: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let a1: Arc<Mutex<i32>> = a.clone();
    let a2: Arc<Mutex<i32>> = a.clone();
    let a3 = Arc::clone(&a);

    let t1 = thread::spawn(move || {
        for _ in 0..10_000_000 {
            let mut l = a1.lock().unwrap();
            *l += 1;

        }
    });

    let t2= thread::spawn(move || {
        for _ in 0..10_000_000 {
            let mut l = a2.lock().unwrap();
            *l += 1;
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    println!("{}", a.lock().unwrap());


    //THREADS SHARED DATA
    let shared_data = Arc::new(Mutex::new(Vec::new()));
    let mut threads = vec![];
    for (i) in (0..10)
    {
        //let data_copy = shared_data; //NO : non impl COPY
        let data = shared_data.clone(); //duplicazione del possesso
        threads.push(thread::spawn(move || { //data è ceduto al thread
            let mut v = data.lock().unwrap(); //v è di tipo MutexGuard<T>
            v.push(i); //quando v esce dall scope, il lock viene rilasciato
        }));
        //println!("{:?}", data);
    }
    for t in threads { t.join().unwrap(); }
    //println!("{:?}", threads);
    //come stampo threads?
    let v = shared_data.lock().unwrap();
    println!("{:?}", *v); //stampa [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]


    //-------------------------------ATOMIC--------------------------------
    println!("--------------------ATOMIC----------------------------");

    //-----------------------------CONVAR--------------------------------
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    // Inside of our lock, spawn a new thread, and then wait for it to start.
    thread::spawn(move|| {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // We notify the condvar that the value has changed.
        cvar.notify_one();
    });
    // Wait for the thread to start up.
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap();
    }



    //------------------------------------CONCORRENZA: MESSAGGI CHANNEL ---------------------------
    println!("--------------------CONCORRENZA : MESSAGGI CHANNEL--------------------");
    //codice con 1 sender e 1 recevier

    /*let (tx, rx) = channel::<String>();  //funz channel crea un canale e restituisce una tupla(sender, receiver)
    let t1 = thread::spawn(move || {  //passo lambda con "move" perchè voglio che il thread si prenda la responsabilità di possedere quello che usa
        for i in 0..3 {
            println!("Sending {}", i);
            if tx.send(format!("Message {}", i)).is_err() { //l'if serve a dire che per qualche motivo il ricevitore dovesse morire, il sender si ferma e non manda neanche il dato
                println!("Aborting...");
                break;
            }
        }
        println!("Sender done!");
    });
    let t2 = thread::spawn(move || {  //passo lambda con "move" perchè voglio che il thread si prenda la responsabilità di possedere quello che usa
        while let Ok(v) = rx.recv() {
            println!("Got {}", v);
        }
        println!("Receiver done!");
    });
    t1.join().unwrap();
    t2.join().unwrap();*/

/*
sender prepara i suoi dati, li manda (se il ricevitore non è morto)
receiver, finchè il sender è vivo, riceve dati
quando il thread principale finisce, tutti i thread secondari muoiono. quindi devo aspettare i thread altrimneti non riesco (devo fare t.join.unwrap)

console:
il mittente parte per primo e invia tutti e tre i dati anche se il receiver non sta ricevendo (questo perchè il canale è unbounded quindi può contenere tanti messaggi).
finisce di inviare i dati (message: sender done!)
quando il recevier riesce a partire, inizia a ricevere i dati nell'ordine in cui sono stati inviati finchè non finisce.
*/


    //codice con n senders e 1 recevier

    let (tx, rx) = channel::<String>(); //crei il TX originale
    let mut handles = vec![];
    for t in 1..4 {
        let tx = tx.clone(); //per ogni thread ho fatto una copia. (N+1 TX) creo una tx qui dentro che sta nello stack ed è clone dell'altra tx
        handles.push(thread::spawn(move || { //avendo scritto move, il primo si mangia la tx, ma il secondo poi non ce l'ha più quindi devo clonare
            for i in 0..3 { //scegli numero mex di invio
                println!("Thread {t} sending {}", i);
                if tx.send(format!("Message {}.{}", t, i)).is_err() { //l'if serve a dire che per qualche motivo il ricevitore dovesse morire, il sender si ferma e non manda neanche il dato
                    println!("Aborting...");
                    break;
                }
            }
            println!("Sender {t} done!");
        }));
    }
    drop(tx); //l'originale sarebbe ancora vivo altrimenti, siccome esisterebbe potrebbe inviare cose.

    handles.push(thread::spawn(move || {
        while let Ok(v) = rx.recv() {
            println!("Got {}", v);
        }
        println!("Receiver done!");
    }));
    for h in handles {
        h.join().unwrap();
    }

/*
al termine del for ho 4 senders (il primo creato alla riga 42, gli altri 3 creati con il for).
quindi ho n+1 senders. Quando i thread finiscono e muoiono mi rimane solo una copia -> l'originale.
devo però distruggerla altrimenti il receiver non finirà mai. quindi aggiungo un drop(tx) fuori dal for
con h.join().unwrap();
Quando i 3 THREAD TX finiscono di inviare cose, buttano via copia -> conteggio scende da 4 ad 1 di TX, quindi il thread receiver
starebbe li ad aspettare i messaggi inviati dal TX originale.
FOR H IN HANDLES : Aspetto che muoiano tutti quanti, senders e receiver, quindi preparo un vettore per comodità.
Quando mando in esecuzione vedo che i thread iniziano a inviare messaggi in ordine anche se possono essere alternati l'uno con l'altro
es: t1 sends 0, t2 sends 0, t2 sends 1, t2 sends 2, t1 sends 1
stessa cosa il receiver può iniziare a ricevere anche se il thread senders non ha finito di inviare, ma riceverà comunque i messaggi in ordine

RIMOZIONE DEL DROP (TX) : PROBLEMA :
se rimuovo la drop(tx) il programma compila ma si pianta perchè non riesce a terminare perchè il receiver ha ancora un sender attaccato.
nessuno sta usando questo sender, ma il receiver non può saperlo.
Il sender aspetta che il receiver finisca, ma non può finire perchè sta ancora aspettando che il sender mandi qualcosa. Ho una situazione di deadlock
*/



    //esempio sync_channel
    /*
    let (tx, rx) = sync_channel::<String>(1);  //buffer può contenere solo un messaggio
    let mut handles = vec![];
    for t in 1..4 {
        let tx = tx.clone(); //creo una tx qui dentro che sta nello stack ed è clone dell'altra tx
        handles.push(thread::spawn(move || { //avendo scritto move, il primo si mangia la tx, ma il secondo poi non ce l'ha più quindi devo clonare
            for i in 0..3 {
                println!("Thread {t} send {}", i);
                if tx.send(format!("Message {}.{}", t, i)).is_err() { //l'if serve a dire che per qualche motivo il ricevitore dovesse morire, il sender si ferma e non manda neanche il dato
                    println!("Aborting...");
                    break;
                }
            }
            println!("Sender {t} done!");
        }));
    }
    drop(tx);

    handles.push(thread::spawn(move || {
        while let Ok(v) = rx.recv() {
            println!("Got {}", v);
        }
        println!("Receiver done!");
    }));
    for h in handles {
        h.join().unwrap();
    }*/

/*
se ho un buffer di 0 deve succedere che sender e receiver devono per forza incontrarsi (randevoux), devono scambiarsi direttamente il messaggio
finchè il recever non c'è, il sender non può mandare.
tutti provano ad inviare ma finchè non arriva il receiver non si riesce
*/
}

