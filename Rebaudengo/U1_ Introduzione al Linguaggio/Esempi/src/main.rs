use std::collections::HashMap;
use std::io;
use std::ops::Mul;
use std::time::{Duration, Instant};
use rand::Rng;

//----------------------------INTRODUZIONE AL LINGUAGGIO----------------------------
//COPY VS CLONE
#[derive(Debug, Clone, Copy)]
pub struct PointCloneAndCopy {
    pub x: f64
}
#[derive(Debug, Clone)]
pub struct PointCloneOnly {
    pub x: f64
}
fn test_copy_and_clone() {
    let p1 = PointCloneAndCopy { x: 0. };
    let p2 = p1; // because type has Copy, it gets copied automatically.
    println!("{:?} {:?}", p1, p2);
}
fn test_clone_only() {
    let p1 = PointCloneOnly { x: 0. };
    let p2 = p1; // because type has no Copy, this is a move instead.
    //println!("{:?} {:?}", p1, p2); //ERROR
}

//VEC
#[derive(Debug)]
struct Test (i32);
impl Drop for Test {
    fn drop (& mut self) {
        println!("Destroying Test ({}) at address {:p}", self.0, self);
    }
}

//STRING
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

//FIRST WORD TEXT
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

//-----------------------------POSSESSO----------------------------
fn takes_ownership(some_string: String) {
    let s = some_string.to_uppercase();
    println!("{}", s);
}

fn cambia (myref: & mut Box<i32>) -> &mut Box<i32>
{
    *myref = Box::new(200);
    myref
}

fn main() {
    //----------------------------INTRODUZIONE AL LINGUAGGIO----------------------------
    println!("-----------------INTRODUZIONE AL LINGUAGGIO: ------------------");
    //SHADOWING
    let x = 5; //shadow
    let x = x + 1;
    println!("{}", x); //stampa 6
    let mut y = 5;

    //BORROWING
    let a = 2;
    let p = &mut 12;
    *p = *p + 1; //13
    let p1 = & (a + 2); //& 4
    let p2 = &(*p1 * 2); //& 8 = 4*2
    println!("{} {} {}", *p, *p1, *p2); //13, 4, 8

    //BOX
    let mut bv = Box::new(vec![1, 2, 3]);
    bv.push(19);

    //COPY VS CLONE
    test_copy_and_clone();
    test_clone_only();

    //VEC
    let mut v =Vec::<Test>::with_capacity(4);
    v.push(Test(1));
    v.push(Test(2));
    v.push(Test(3));
    v.push(Test(4));
    v.push(Test(5));
    println!("VEC : ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());
    v.pop();
    v.shrink_to_fit();
    println!("VEC : ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    //STRINGHE
    let s = "hello";
    for c in s.chars() {
        println!("{}", c);
    }
    let s1 = String::from("hello");
    let name1 = "Matteo";
    let name2 = "Giovanni".to_string();
    greet(name1);
    greet(&name2);

    //ESPRESSIONI
    let condition = true;
    let mut esp = if condition {5} else {6};
    println!("{:?}", esp);
    let mut counter = 0;
    let mut sum = 0;

    let result = loop {
        counter += 1;
        sum += counter;

        if counter == 10 {
            break sum * 2;
        }
    };
    println!("The result is {result}");

    //WHILE
    let mut counter = 0;
    let time_limit = Duration::new(1,0); // Crea una durata di 1 secondo
    let start = Instant::now(); // Determina l’ora attuale
    while (Instant::now() - start) < time_limit { // Finché non è passato 1 s...
        counter += 1; // ...incrementa il contatore
    }
    println!("{}", counter);

    //FOR
    let names = ["Bob", "Frank", "Ferris"];
    for name in names.iter() {  	           // Stampa i tre nomi
        println!("{}", name);
    }
    for name in &names[ ..=1 ] {  	     // Stampa i primi due nomi
        println!("{}", name);
    }
    for number in (1..=10).rev() {   // Stampa una sequenza al contrario
        println!("{number}!");
    }
    for (i,n) in names.iter().enumerate() { //stampa indici e nomi
        println!("names[{}]: {}", i, n);
    }

    //CHARS IN RUST
    let y = "y̆";
    let mut chars = y.chars();
    let c = chars.next().unwrap();
    println!("{:?}", c);

    //IO DA TASTIERA
    let mut s = String::new();
    let mut result = match io::stdin().read_line(&mut s).unwrap() {
        0 => {println!("No input"); "No input"},
        _ => {
            let bytes = s.as_bytes();
            let mut index=s.len();
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    index = i;
                }
            }
            let sliceStr = &s[..index];
            sliceStr
        }
    };
    println!("the first word is: {}", result);

    let mut s2 = String::new();
    let mut result2 = match io::stdin().read_line(&mut s2).unwrap() {
        0 => {println!("No input"); "No input"},
        _ => {
            let mut index=s2.len();
            for (i, item) in s2.chars().enumerate() {
                if item == ' ' {
                    index = i;
                }
            }
            &s2[..index]
        }
    };
    println!("the first word is: {}", result2);


    //--------------------------------POSSESSO-------------------------------------
    println!("-----------------POSSESSO------------------");
    //RIASSEGNAZIONE = RILASCIO DEL VECCHIO VALORE
    let mut t = Test(6);
    println!("{:?}@{:p}", t, &t);
    t = Test(7);
    println!("{:?}@{:p}", t, &t);

    //MOVIMENTO
    let mut t = Test(8);
    let t2 = t;

    //POSSESSO E RIFERIMENTI
    let mut mybox = Box::new(150);
    let mut z = &mut mybox;
    *z = Box::new(100);
    z = cambia(z); //il BOX a 200
    println!("{:?}", z);

    let newref = & *z;
    println!("{:?}", newref);
    //println!("{:?}", mybox); //errore
    println!("{:?}", newref);


    let mut v = vec![1, 2, 3];
    let mut b = Box::new([1,2,3,4]);
    (*b)[0] = 10;
    println!("{:?}", b);

    println!("DROP TESTS FINE SCOPE MAIN");
}

