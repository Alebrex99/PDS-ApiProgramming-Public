mod tratti;

use std::borrow::Cow;
use std::num::FpCategory::Nan;
use std::ops::{Deref, DerefMut};
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};
use tratti::Cilindro;
use tratti::Figura3D;
use tratti::CilindroGeneric;

//--------------------------GESTIONE TRATTO CUSTOM: LEZIONE--------------------------------------
/*tratti come figure geometriche: sto descrivendo la struttura di un possibile
comportamento; se ho una figura 2D posso calcolare perimetro /area.
&self : il perimetro /area lo calcolo a partire dall'obj che lo implementa. il &self di FIGURA2D sarà
il riferimento all'oggetto che implementa figura 2D*/
trait Figura2D{
    fn perimetro(&self) -> f64;
    fn area(&self) -> f64;
}
//struttuca cerchio può implementare il tratto Figura2D
#[derive(Debug)]
struct Cerchio{
    xc: f64,
    yc: f64,
    r: f64
}
//quando implemento il tratto per la struttura Cerchio, ciò cerchio implementa il tratto Figura2D
impl Figura2D for Cerchio{
    fn perimetro(&self) -> f64 {
        //perimetro di un cerchio
        2.0 * std::f64::consts::PI * self.r
    }
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
impl PartialEq for Cerchio{
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r
    }
}
impl Deref for Cerchio{
    type Target = f64; //dire a cosa mi usi come puntatore; quindi il cerchio può essere usato ora come un puntatore a un f64
    fn deref(&self) -> &Self::Target { //Responsabile di restituirmi un puntatore a qualcosa interno al mio cerchio:
        // "se tu cerchio ti usano come puntatore, allora restituisce il puntatore al tuo raggio
        &self.r
    }
}
impl DerefMut for Cerchio{
    fn deref_mut(&mut self) -> &mut Self::Target { //invecde di restituire un ref semplice, restituisce un ref mutabile
        &mut self.r
    }
}

/*Anche punto può implementare FIGURA2D*/
struct Punto{
    x: f64,
    y: f64
}
impl Figura2D for Punto{
    fn perimetro(&self) -> f64 {
        0.0
    }
    fn area(&self) -> f64 {
        0.0
    }
}

//--------------------COMPORTAMENTO POLIMORFICO: LEZIONE-----------------
//se ho una funzione che accetta (è chiamato su) un oggetto di TIPO FIGURA2D(come il riferimento ad interfaccia in java)
fn f1(f: &dyn Figura2D){
    //posso chiamare i metodi di Figura2D
    println!("l'area vale: {}", f.area());
    println!("il perimetro vale: {}", f.perimetro());
}




//------------------------------PROVE GENERICHE --------------------------------------
struct Test;
trait T3 {
    type AssociatedType;
    fn f(arg: Self::AssociatedType);
}
impl T3 for Test {
    type AssociatedType = f64;
    fn f(arg: f64) {
        println!("f: {}", arg);
    }
}

struct MyBox<T>(Box<T>);
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping Box<i32> @{:p}", self.0);
    }
}

#[derive(Debug)]
struct TestCopy(i32);
impl Clone for TestCopy {
    fn clone(&self) -> Self {
        *self
    }
}
impl Copy for TestCopy {}


fn main() {

    //----------------------------POLIMORFISMO IN RUST---------------------------------
    //MIGLIORAMENTI DI RUST RISPETTO A C++ : NON HO BISOGNO DI VTABLE
    //ho definito il comportamento di un oggetto che implementa il tratto Figura2D (di cerchio e punto)
    /*Ora posso crearmi degli oggetti*/
    let c = Cerchio{xc: 0.0, yc: 1.0, r: 10.0};
    let p = Punto{x: 3.0, y: 4.0};
    c.perimetro();//posso vedere chiare le caratteristiche di C e posso fare c.area ecc
    /*compilatore non ha bisogno di info particolari, sa che c è un
    tipo c è un tipo concreto e che impl una sua versione del metodo perimetro di figura2d. InASM
    avrei direttamente la chiamata alla sua versione di perimetro, senza dubbi
    (non ho bisogno di VTABLE, costa zero come qualunque altra chiamata*/

    //&DYN : USO DELLA VTABLE ANCHE IN RUST: POLIMORFISMO
    /*ora chiamando f1 non posso passare direttamente c e p, perchè sono grossi
    diversi (c è cerchio con 3 campi, p ha solo x e y), ma posso passare il riferimento*/
    f1(&c);
    f1(&p);
    /*Qua chiamo REF (&c/&p), ma sopra REF DYN , perchè l'altro deve capire che siccome FIGURA2D può
    essere l'uno o l'altro, lui nel costruire codice di F1 (sopra) non può sapere chi gli arriva.
    Viene compilato per i fatti suoi e non può il compilatore, quando viene creato il CODICE DI F1, sapere quale
    parametro specifico gli verrà dato. Quindi ogni volta che arriva un parametro a F1, quel parametro sarà un FAT POINTER,
    quindi deve avere il puntatore al DATO e la VTABLE ASSOCIATA IN MODO CHE SI POSSA RISOLVERE a caso punta davvero
    N.B.: verrà messa la VTABLE relativa a cosa quell'altro sta aspettando*/

    //-----------------FAT POINTER: PUNTATORE AL DATO + PUNTATORE ALLA VTABLE = 16 byte---------------
    /*i primi 8 byte puntano al cerchio o al punto, ma lui sa solo che puntano
    non sa cosa realmente sia, i secondi 8 puntano alla VTABLE di rust
    il comportamento è stato POLIMORFICO: ho definito una funzione che accetta una FIGURA 2D che viene
    per forza ricevuta come puntatore, altrimenti non potrei passare cose a volte piccole
    e a volte grosse. Ma visto che le implementazioni possono esser diverese (tra cerchio e punto)
    allora è necessario aggregare al puntatore alla struttura dati anche la VTABLE -> evidente se segno &dyn

    Se togli &dyn e passi solo &Figura2D il compilatore da errore, perchè non sa quanto è grande figura 2D.
    ciò rende consapevoli che quel parametro, ovvero passo una cosa grande 16 byte. Le chimate a F1() si portano dietro
    2 cose. Però quando faccio f1(&c) e f1(&p) -> io sto prendendo il puntaotre di C ,ma
    c so che è un cerchio, quindi il comilatore li mette il puntatore effettivo a C (che in questo momento
    è BORROWED finchè la funzione non torna) + mette il puntatore alla VTABLE , senza problemi perchè
    sa C è un cerchio e conosce quali siano i suoi metodi virtuali (stessa cosa con p)*/


    //-------------------------------DEFINIRE E USARE I TRATTI---------------------------------
    println!("------------DEFINIRE E USARE I TRATTI--------------");
    let zero :i32 = Default::default(); //0
    let zero = i32::default(); //stessa cosa
    println!("zero = {}", zero);

    //CILINDRO IN MODULO TRATTI.RS
    let cilindro = Cilindro{raggio: 10.0, altezza: 20.0};
    println!("il volume del cilindro è: {}", cilindro.volume()); //6283.185307179
    Cilindro::associated(10.0); //10.0
    Box::new(cilindro).prova(); //Prova

    //OGGETTO TRATTO &DYN + OGGETTO
    let cerchio = Cerchio{xc: 0.0, yc: 1.0, r: 10.0}; // grande 24 byte = 8 di xc, 8 di yc, 8 di r
    let oggetto_tratto: &dyn Figura2D = &cerchio;
    println!("VTABLE: {}", std::mem::size_of_val(oggetto_tratto)); //
    let oggetto_tratto_box :Box<&dyn Figura2D> = Box::new(&c);
    println!("VTABLE: {}", std::mem::size_of_val(*oggetto_tratto_box));


    //------------------DERIVE + IMPLEMENTAZIONE TRATTI: DEBUG, DISPLAY, EQ, PARTIALEQ------------------
    println!("------------DERIVE + IMPLEMENTAZIONE TRATTI: DEBUG, DISPLAY, EQ, PARTIALEQ--------------");
    //DERIVE DEBUG e DISPLAY: posso stampare c e p
    println!("il cerchio {:?} ha perimetro {}", c, c.perimetro()); //stampa una rapp interna {xc: 0.0, yc: 1.0, r: 10.0} ha perimetro 62.83185307179586

    //RELAZIONI D'ORDINE: FATTO CON [#DERIVE (PARTIALEQ)]
    let c2 = Cerchio{xc: 0.0, yc: 1.0, r: 10.0};
    /*CON [#DERIVE ]: l'uguaglianza viene fatta in automatico su tutti i valori del cerchio*/
    println!("{:?} == {:?} ? {}",c, c2, c==c2); //true, cambiando uno solo dei valori risponde false

    //RELAZIONE D'ORDINE: IMPLEMENTAZIONE TRATTO PARTIAL EQ
    let c3 = Cerchio{xc: 0.0, yc: 1.0, r: 20.0};
    println!("{:?} == {:?} ? {}",c, c3, c==c3); //false, perchè il raggio è diverso
    //Prova con NONE : NAN != NAN non sono mai uguali 2 NAN
    let c1_none = Cerchio{xc: 0.0, yc: 1.0, r: std::f64::NAN};
    let c2_none = Cerchio{xc: 0.0, yc: 1.0, r: std::f64::NAN};
    println!("{:?} == {:?} ? {}",c1_none, c2_none, c1_none==c2_none); //FALSE, perchè NAN != NAN

    //MIEI ESEMPI : PARTIAL EQ VS EQ
    let mut cilindro2 = Cilindro{raggio: 10.0, altezza: 20.0};
    println!("{:?} == {:?} ? {}",cilindro2, cilindro2, cilindro2==cilindro2); //true
    let cilindro_copy = cilindro2; //è una copia perchè hai derivato COPY + CLONE
    let cilindro_ref = &mut cilindro2;
    (*cilindro_ref).raggio = 20.0;

    //---------------------TRATTO DEREF -> PUOI LEGGERLO COME PUNTATORE------------------------
    /*Normalmente prendi riferimento a c, ma siccome impl il tratto DEREF , cosi c.deref() prende il
    puntatoore associato a c. DEREF ha fatto si che struttura cerchio possa comportarsi come un puntaotre,
    usata cosi in alcune situe si possono usare cose che pur non essendo puntatori si possono usare
    in tutte le situazioni in cui si usano*/
    let mut c4 = Cerchio{xc: 0.0, yc: 1.0, r: 10.0};
    println!("DEREF : il raggio del cerchio è {:p} -> {}", c4.deref(), *c4); //0x7ff7f3c1b1c8 -> 10
    println!("DEREF: il raggio del cerchio è {:p} -> {}", &c4, *c4); //definisco dove è c e poi dereferenzio, anche se c è un cerchio lo uso come tale; *c mi da la cosa a cui punta
    /*la struttura c che è un cerchio la uso proprio come fare a usare un puntatore, uso come puntatore una cosa che non lo è
    sarà utile quando ho bisogno di puntatare ad un dato non posseduto da uno solo ma da tanti e voglio che smetta di esistere quando più
    nessuno lo conosce, per permettere che dato accessibile a tanti devo fornire un puntatore, ma non posso fornire un puntatore
    vero perchè se lo lascio copiare ciò diventa difficile da gestire, RUST blocca.
    Quindi preparo una struttura con dentro il vero puntatore ma anche il conteggio di quanti lo conscoono. Ogni volta che qualcuno che mi duplica
    la struttura io incremento il contatore. Tutte le volte che qualcuno me la distrugge decremento. Quando contatore  è zero
    LIBERO MEMORIA. Gli altri lo usano come *PTR come puntatore, ma non lo è davvero, contiene un puntatore, ti ci fa accedere*/

    //TRATTO DEREFMUT -> PUOI LEGGERLO E SCRIVERLO COME PUNTATORE
    /*Se voglio modificare il raggio di c, posso farlo con il tratto DEREFMUT*/
    *c4 = 20.0; //siccome ho anche deref mut, *c mi da possibilità sicuro di leggere il raggio, ma anche di scriverlo
    println!("DEREF MUT: il raggio di c è : {:?}", c4);
    (*c4).abs(); //posso fare operazioni su c4 come se fosse un puntatore, ma non lo è, riconosce in automatico ch è un f64
    //oppure posso fare c4.abs() in rust

    //TRATTO COPY
    let test = TestCopy(10);
    let test2 = test.clone(); //test2 è una copia di test
    let test3 = test; //test3 è una copia di test
    println!("{:?}", test2.0); //TestCopy



    //------------------------------------TIPI GENRICI--------------------------------------
    println!("------------TIPI GENERICI--------------");
    let c_generic = CilindroGeneric{raggio: 10, altezza: 20};
    println!("il volume del cilindro generico è: {}", c_generic.volume()); //2000


    //-------------------------------------ESEMPI FAT-SMART POINTER----------------------------------------
    //BOX + RC / ARC : SIMILI MA ARC PER THREADS
    println!("------------BOX + RC / ARC--------------");
    let mybox = MyBox(Box::new(5));
    println!("{:?}", drop(mybox)); //Dropping Box<i32> @0x7ff7f3c1b1c8

    let mut b = Box::new(5);
    println!("b contiene l'indirizzo su heap a BOX, è : {:p}", b); //4
    println!("&b contiene l'indirizzo su stack del puntatore b a BOX, è : {:p}", &b); //4
    let rc = Rc::new(b); // b moved (consumato) in rc, su heap
    let rc_ref = &rc; // su stack, puntatore allo smart pointer Rc
    println!("rc = {:p}, rc_ref = {:p}", rc, rc_ref);
    let mut rc_clone = Rc::clone(&rc);
    //*rc_clone = Box::new(6); //error: cannot assign perchè RC è copia puntatore immutabile (non impl DEREF MUT, solo DEREF)
    let rc_clone2 = Rc::clone(&rc);
    let rc_clone3 = rc.clone(); //stessa cosa di Rc::clone(&rc)
    let rc_clone4 = rc_clone3.clone();
    println!("{}", *rc_clone); //5 -> RC impl DEREF, quindi posso dereferenziare
    println!("{}", *rc_clone2); //5
    println!("{}", *rc_clone3); //5
    println!("{}", *rc_clone4); //5
    let mut rc2 = Rc::new(5);
    println!("rc2.deref() = {} è uguale a dire *rc2 = {}", rc2.deref(), *rc2);

    //CELL E REFCELL
    println!("------------CELL + REFCELL--------------");
    //CELL
    let mut vec = vec![1,2,3];
    let vec_ref = &mut vec;
    vec_ref.push(4); //riferimento morto
    println!("{:?}", vec);
    //println!("{:?}", vec_ref); // riferimento torna in vita, error: value borrowed here after move

    let vec_imm = vec!['a', 'b', 'c']; //immutabile
    let cell = Cell::new(vec_imm); //ma ora in CELL
    cell.set(vec!['a', 'b', 'd']); //muta il contenuto di CELL
    println!("{:?}", cell.take()); //['a', 'b', 'd'] //estraggo il dato da CELL che non impl COPY

    //REFCELL
    let ptr = RefCell::new(5); {
        /*permette sia di avere un &mut che un & del valore di tipo T in RECELL, che
        di poter avere un REF (in lettura) ma anche capace di mutare il dato contenuto (come per CELL<T>)*/
        let mut m = ptr.borrow_mut(); //prendi un riferimento mut a 5 (il contenuto T di RefCell)
        println!("{}", *m);
        assert!(ptr.try_borrow().is_err());
        *m = 6;
    }{
        let m = ptr.borrow();
        assert!(ptr.try_borrow().is_ok());
        assert!(*m == 6);
    }

    //COW: CLONE ON WRITE
    let vett = vec![1,2,3,4,5];
    let cow = Cow::from(vett); //k è un riferimento a 5





}
