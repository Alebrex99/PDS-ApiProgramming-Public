use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::ops::Deref;
use std::rc::Rc;
use crate::List2::{Cons, Nil};

enum List {
    Cons(i32, Box<List>),
    Nil,
}
// Definizione di una struttura per un albero binario
#[derive(Debug)]
enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}
impl BinaryTree{
    fn new() -> Self{
        BinaryTree::Empty
    }
    fn insert(&mut self, value:i32){
        match self{
            BinaryTree::Empty => *self = BinaryTree::Node( value, Box::new(BinaryTree::Empty), Box::new(BinaryTree::Empty)),
            BinaryTree::Node(data, left, right) => {
                if value <= *data{
                    left.insert(value)
                }
                else { right.insert(value) }
            }
        }
    }
}

//-------------------------SMART POINTER--------------------------------
#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<Node>>,
}

struct MyBox<T>(Box<T>);
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping Box<i32> @{:p}", self.0);
    }
}

#[derive(Debug)]
enum List2 {
    Cons(Rc<RefCell<i32>>, Rc<List2>),
    Nil,
}

fn abs_all<'a>(input: &'a mut Cow<'a, [i32]>)  -> Cow<'a, [i32]>{
    // Check if the input slice contains any negative values
    if input.iter().any(|&x| x < 0) {
        // If yes, create a new vector with the absolute values
        let mut output = Vec::new();
        for x in input.iter(){
            output.push(x.abs());
        }
        // Return the vector as an owned slice
        print!("Owned => ");
        Cow::Owned(output)
    } else {
        // If no, return the input slice as a borrowed slice
        print!("Borrowed => ");
        Cow::Borrowed(input)
    }
}



fn main() {
    //ALBERO BINARIO : BOX
    // Creazione di un nuovo albero binario
    let mut tree = BinaryTree::new();
    // Inserimento di alcuni valori nell'albero binario
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);
    println!("{:?}", tree);

    //RC : SHARED PTR
    let mut valore = Rc::new(5);
    {
        {
            println!("Valore: {:?}", valore);
            let copia = Rc::clone(&valore);
            match Rc::get_mut(&mut valore) {
                Some(v) => *v += 10, // Modifica il valore se è possibile.
                None => println!("Non è possibile ottenere un riferimento mutabile."),
            }
        }//RILASCIO della COPIA
        // Prova a ottenere un riferimento mutabile.
        match Rc::get_mut(&mut valore) {
            Some(v) => *v += 10, // Modifica il valore se è possibile.
            None => println!("Non è possibile ottenere un riferimento mutabile."),
        }
        println!("Valore: {:?}", valore);
    }
    //esempio famiglia
    let mut nipote1 = Rc::new(Node {
        value: 3,
        children: vec![],
    });
    let nipote2 = Rc::new(Node {
        value: 6,
        children: vec![],
    });
    let padre = Rc::new(Node {
        value: 9,
        children: vec![Rc::clone(&nipote1), Rc::clone(&nipote2)],
    });
    let nonno = Rc::new(Node {
        value: 27,
        children: vec![Rc::clone(&padre)],
    });
    println!("Riferimenti forti a nipote1 prima: {}", Rc::strong_count(&nipote1));
    match Rc::get_mut(&mut nipote1) {
        Some(v) => v.children.push(Rc::clone(&nonno)),
        None => println!("Non è possibile ottenere un riferimento mutabile."),
    }
    println!("NONNO : {:#?}", nonno);

    //-------------------------------------ESEMPI FAT-SMART POINTER----------------------------------------
    //BOX + RC / ARC : SIMILI MA ARC PER THREADS
    println!("------------BOX + RC / ARC--------------");
    let mybox = MyBox(Box::new(5));

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
    let vec_imm = vec!['a', 'b', 'c']; //immutabile
    let cell = Cell::new(vec_imm.clone()); //ma ora in CELL
    let r2 = &cell;
    let r3 = &cell;
    //let t = r2.get(); // NO perchè VEC non impl copy
    cell.set(vec!['a', 'b', 'c']); //muta il contenuto di CELL
    r2.set(vec!['d', 'e', 'f']);
    r3.set(vec!['g', 'h', 'i']);
    cell.set(vec!['l', 'm', 'n']); //muta il contenuto di CELL
    println!("{:?}", cell.take()); //['a', 'b', 'd'] //estraggo il dato da CELL che non impl COPY

    //REFCELL
    let mut ptr = RefCell::new(5);
    *ptr.get_mut() +=5;
    *ptr.borrow_mut() +=5;
    println!("{}", *ptr.borrow()); //11
    {
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

    //RC + REFCELL
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));
    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);
    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    //COW: CLONE ON WRITE
    let vett = vec![1,2,3,4,5];
    let cow = Cow::from(vett); //k è un riferimento a 5
    let cow2 = &cow;
    let cow3 = &cow;
    //1) se si cerca di modificare dato vett contenuto
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]); //BORROWED
    let mut output = abs_all(&mut input); // No clone occurs because input doesn't need to be mutated.

    println!("{:?}", output);

    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]); // Clone occurs because input needs to be mutated.
    output = abs_all(&mut input);
    println!("{:?}", output);
    //println!("{:?}", input); //ERRORE

    let mut input = Cow::from(vec![-1, 0, 1]); // No clone occurs because input is already owned.
    input.to_mut()[1] = 10; //cloned
    output = abs_all(&mut input);
    println!("{:?}", output);
}
