use std::fmt::{Debug, Display};
use std::mem::size_of;
use std::ops::{Add, Deref, Index, IndexMut, Range};

//------------------------TIPI COMPOSTI---------------------
//STRUCT
#[derive(Copy, Clone)]
struct Empty;
#[derive(Debug)]
struct Test {
    a: i32,
    b: i32,
    c: Vec<String>,
}
impl Test {
    fn new(a: i32, b: i32, x: Vec<String>) -> Test {
        Test { a, b, c: x }
    }
    fn length(&self) -> usize {
        //conta il numero di elementi in una struct
        size_of::<Test>()
    }
}

//ENUM
enum Color {
    Struttura { r: i32, g: i32, b: i32 },
    Green,
    Blue,
}
impl Color {
    fn get_r(&self) -> i32 {
        match self {
            Color::Struttura { r, g, b } => *r,
            _ => 0,
        }
    }
}

#[derive(Debug)]
enum Shape {
    Circle { r: f64 },
    Rectangle { w: f64, h: f64 },
    Square { s: f64 },
    Test { name: String, age: i32 },
}
impl Shape {
    fn compute_area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rectangle { w, h } => w * h,
            Shape::Square { s } => s * s,
            Shape::Test { name, age } => name.len() as f64,
        }
    }
}
fn compute_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle { w, h } => w * h,
        Shape::Circle { r } => std::f64::consts::PI * r * r,
        Shape::Square { s } => s * s,
        Shape::Test { name, age } => name.len() as f64,
    }
}
fn process(shape: Shape) {
    if let Shape::Test { name, age } = shape {
        println!("Name: {}", name);
    }
}

//-------------------------------TRATTI E GENERICS------------------------------
#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}
impl Default for Person {
    fn default() -> Self {
        Person {
            name: "Default".to_string(),
            age: 0,
        }
    }
}
impl Person {
    fn length(&self) -> usize {
        size_of::<Person>()
    }
}

//SUPER TRATTO E SOTTO TRATTO
trait Supertrait {
    fn Sup(&self) {
        println!("In super");
    }
    fn g(&self) {}
}
trait Subtrait: Supertrait {
    fn Sub(&self) {
        println!("In sub");
    }
    fn h(&self);
}
impl Supertrait for Person {}
impl Subtrait for Person {
    fn h(&self) {
        println!("In h");
    }
}
impl Default for Test {
    fn default() -> Self {
        Test {
            a: 0,
            b: 0,
            c: vec![],
        }
    }
}

//GENERICS
fn sum<T: Add<Output = T> + Default + Copy>(items: &[T]) -> T {
    let mut result = T::default();
    //se usassi solo questo non ti serve T: COPY, ma solo T:DEBUG
    /*for item in items{
        println!("Item: {:?}", item);
    }*/
    for item in items { //into_iter()
        result = result + *item;
    }
    result
}
fn sum2<T: Add<Output = T> + std::ops::AddAssign + Default + Copy + Debug >(items: &mut [T]) -> T {
    let mut result = T::default();
    //se usassi solo questo non ti serve T: COPY, ma solo T:DEBUG
    let mut v = &mut  *items;
    for item in  v{
        println!("Item: {:?}", item);
    }
    /*for item in items.iter(){
        println!("Item: {:?}", item)
    }*/
    for item in items.iter_mut(){
        result = result + *item;
        *item += T::default(); //T::default() di i32 è 0
    }
    result
}

fn max<T: PartialOrd + Clone>(items: &[T]) -> Option<&T> {
    if items.is_empty() {
        None
    } else {
        let mut max = &items[0];
        //oppure impl CLONE
        let mut max2 = items[0].clone();
        for item in items {
            if item > max { max = item; }
        }
        Some(max)
    }
}
/*
struct Point<T> {
    x: T,
    y: T,
}*/

trait Tratto<T, AssociatedType=i32>{
    type AssociatedType;
    fn f_generic(&self, x: T) -> T;
    fn confronta(&self, other: &T) -> bool;
}
impl<T: PartialOrd> Tratto<T> for T{
    type AssociatedType = i32;
    fn f_generic(&self, x: T) -> T {
        x
    }
    fn confronta(&self, other: &T) -> bool {
        self >= other
    }
}
impl Tratto<i32> for Person{
    type AssociatedType = Self;
    fn f_generic(&self, x: i32) -> i32 {
        let r = &x;
        let p = Self::AssociatedType{age: 10, name: "ciao".to_string()};
        x
    }
    fn confronta(&self, other: &i32) -> bool {
        todo!()
    }
}

//TRATTI DI LIBRERIA
#[derive(Debug)]
struct CustomStruct{
    number: i32,
    boxed_value: Box<i32>,
}
impl Deref for CustomStruct {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.number
    }
}

impl Add for CustomStruct {
    type Output = i32;
    fn add(self, rhs: CustomStruct) -> Self::Output {
        self.number + rhs.number
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
    z: String
}
impl Display for Point{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point: x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}
impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.x != other.x {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}
impl Clone for Point {
    /*fn clone(&self) -> Self {
        Point {x: self.x, y: self.y, z: self.z.clone()}
    }*/
    fn clone(&self) -> Self {
        Point{..(*self).clone()}
    }
}
impl From<[i32;2]> for Point {
    fn from(array: [i32;2])-> Self{
        Point{x: array[0], y: array[1], z: "default".to_string()}
    }
}
//generato in automatico
impl Into<Point> for (i32,i32){
    fn into(self) -> Point{
        Point{x: self.0, y: self.1, z: "default".to_string()}
    }
}

//TRATTI GENERICI
struct MyVector<T>{
    data: Vec<T>,
    len: usize
}

impl<T> Index<usize> for MyVector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> Index<Range<usize>> for MyVector<T>{
    type Output = [T];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> IndexMut<usize> for MyVector<T>{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
impl<T> From<Vec<T>> for MyVector<T>{
    fn from(other: Vec<T>) -> Self {
        MyVector{data: Vec::new(), len: other.len()}
    }
}



//----------------------------TEMPI DI VITA-------------------------------

fn f2(a: & i32, b: &i32) -> i32{
    *a
}

fn g<'a>(a: &'a i32, b: &'a i32) -> &'a i32{
    a
}
fn merge_slices<'a, T>(first: &'a [T], second: &'a [T]) -> Vec<&'a T>  {
    let mut merged: Vec<&'a T> = Vec::new();
    let chained = first.iter().chain(second.iter());
    for item in chained {
        merged.push(item);
    }
    merged
}
struct S(u8);
fn f<'a>(x: &S, y: &'a S) -> &'a u8 {
    &y.0
}
fn print_byte(byte: &u8) {
    println!("Byte: {}", byte);
}
fn crea_string() -> &'static str{
    let s = "ciao";
    s
}
struct User {
    id: u32,
    name: &'static str
}
struct User2<'a> {
    id: u32,
    name: &'a str
}
struct Data<'a> {
    user: User2<'a>,
    password: String
}
impl Data<'_> {
    fn metodo<'a, 'b>(&'a self, b:&'b i32) -> &'a str{
        self.user.name
    }
}

//----------------------------CHIUSURE-------------------------
//FUNZIONI COME PUNTATORI
fn fun_sup(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}
fn f1 (x: i32) -> i32 {
    x + 1
}

//FUNZIONI DI ORDINE SUPERIORE
fn higher_order_fun<F, T, U> (mut f: F, arg: T)-> F where F: FnMut(T) -> U{
    f(arg); //quando la chiami la chiusura fa un BORROW MUT del dato
    return f;
}
fn higher_order_fun2<T> (arg: T)-> impl FnMut(T) -> i32{
    let mut i = 0;
    return move |arg| {i+=1; i};
}

fn consume_closure<F>(f: F) where F: FnOnce() -> String {
    println!("La closure dice: {}", f());
}
fn gen(base: String) -> impl FnMut()-> String{
    let mut i =0;
    return move ||{ //MOVE ci vuole perchè prenderebbe i come riferimento semplice, ma devi forzare per prendere possesso di i, lo consuma
        i+=1;
        format!("{}{}", base, i) //i diventa self.1 e base -> self.0
    };
}
fn gen_vect(n : usize) -> impl FnOnce()-> Vec<i32>{
    return move || {
        println!("Sto per preparare il vettore con {n} elementi");
        let mut v = vec![];
        for i in 0..n{
            v.push(0);
        }
        println!("FATTO {:?}", v);
        return v;
    }
}

fn main() {
    //------------------------TIPI COMPOSTI---------------------
    let t = Test::new(1, 2, vec!["ciao".to_string(), "mondo".to_string()]);
    let r = &t;
    let d = r.length();
    println!("size of Test: {}", t.length());

    //ENUM
    //implementazione enum
    let cerchio = Shape::Circle { r: 10f64 };
    let rettangolo = Shape::Rectangle { w: 10f64, h: 20f64 };
    let area = cerchio.compute_area();
    println!("Area of circle: {}", area);

    //funzione qualunque
    let rettangolo = Shape::Rectangle { w: 10f64, h: 20f64 };
    let area = compute_area(rettangolo); //consuma rettangolo, a meno che rettangolo non impl COPY
    println!("Area of rectangle: {}", area);
    //println!("Rettangolo consumato: {:?}", rettangolo); //errore, rettangolo è stato consumato

    //match + destrutturazione
    let test = Shape::Test {
        name: "NOME".to_string(),
        age: 64,
    };
    let Shape::Test { name, age } = test else {
        panic!("Errore")
    };
    println!("Name: {:?}, Age: {:?}", name, age);

    let mut test = Test::new(10, 11, vec!["ciao".to_string(), "mondo".to_string()]);
    let Test { a, b, ref mut c } = test; //MOVIMENTO DI VEC : movimento parziale di Test. devi usare REF MUT
    println!("Test : {:?}", test);

    //---------------------------TRATTI E GENERICS----------------------
    println!("-----------------TRATTI E GENERICS-----------------");
    let p = Person::default();
    let t = Test::default();
    println!("Person: {:?}", p);
    p.Sub();

    //OGGETTO TRATTO
    let oggtrait: &dyn Subtrait = &p;
    oggtrait.Sup();

    //GENERICS:
    println!("GENERICS:");
    //funzione ADD
    let numbers = vec![1, 2, 3, 4, 5];
    println!("Somma: {}", sum(&numbers));
    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    println!("Somma: {}", sum(&floats));
    let mut numbers_mut = vec![1, 2, 3, 4, 5];
    println!("Somma: {}", sum2(&mut numbers_mut));
    //tratti generici
    let x = 5;
    let y = 10;
    let vec = vec![1, 2, 3, 4, 5];
    let vec2 = vec![1,2,3,4,5];
    println!("{:?}",vec.confronta(&vec2));
    println!("{:?}",vec.f_generic(vec2));
    //FROM generico : partendo da un VEC<T> -> MyVector<T>
    let vec_norm: Vec<i32> = Vec::new();
    let vec_int = vec![1,2,3,4,5];
    let vec_float = vec![1.1,2.2,3.3,4.4,5.5];
    let my_vec_int = MyVector::from(vec_int);
    let my_vec_float = MyVector::from(vec_float);

    //TRATTI DI LIBRERIA
    let boxed_value = Box::new(42);
    let custom_struct = CustomStruct { number: 10, boxed_value };
    println!("Dereferencing custom_struct: {:?}, uso deref: {:?}", *custom_struct, custom_struct.deref());
    //deref coercion
    let custom_struct_ref: &i32 = &custom_struct; //&CustomStruct -> &i32 : &Self -> &Self::Target
    println!("Dereferencing custom struct: {:?}", *custom_struct_ref);
    let custom_struct_ref2 = &custom_struct;
    println!("Dereferencing boxed_value: {:?}", custom_struct_ref2.boxed_value);
    //from-into
    let point = Point::from([1,2]);
    let point_auto: Point = [1,2].into();
    let point_into: Point = (1,2).into();
    //let point_from = Point::from((1,2)); //non implementato in auto FROM da INTO


    //------------------------------TEMPI DI VITA------------------------------------
    println!("-----------------TEMPI DI VITA-----------------");
    let mut vec = vec!["ciao", "mondo"];
    {
        let i = "hello".to_string();
        //let r = &i;
        //vec.push(r);
    } //ERROR: i esce dallo scope, ma r è ancora in vec
    println!("{:?}", vec);
    let first_slice = &[1, 2, 3, 4];
    let second_slice = &[5, 6, 7, 8];
    let slice = &["ciao".to_string(), "mondo".to_string()];
    let slice2 =  &["come".to_string(), "va".to_string()];
    let merged_result = merge_slices(slice, slice2);
    println!("{:?}", merged_result);
    //PROBLEMA DEL BORROW
    let v1 = S(1);
    let v2 = S(2);
    let r = f(&v1, &v2);
    //v2 = v1; //ERROR
    print_byte(r);

    let s = crea_string();

    //------------------------------------CHIUSURE-----------------------------------------
    println!("------------------------CHIUSURE-------------------");
    //1) FN MUT
    let ptr: fn(i32) -> i32 = f1;
    let mut count = 0;
    let mut increment = move || {
        count += 1; // Incrementiamo la variabile catturata
        println!("Il conteggio è: {}", count);
    };
    increment();
    increment();
    println!("Hello, {}", count);

    //2) FN MUT
    let mut count = 0;
    let mut increment_n =  |n| {
        count += n; // Incrementiamo la variabile catturata
        println!("Il conteggio è: {}", count);
    };
    increment_n(10);
    //println!("{}", count); //ERROR LEGGI l'originale mentre stai usando un &MUT
    increment_n(5);
    println!("{}", count);

    //3 FN ONCE
    let mut range = 1..10;
    let mut f =  || range.count();
    let n1 = f(); //MOVED di RANGE : chiama la chiusura che consuma range a causa di count()
    //let n2  = f(); //errore : range è stato consumato


    //4 FN
    let mut data = vec![1, 2, 3, 4, 5];
    // Definiamo una closure che può essere chiamata solo una volta
    let process_data = move || {
        println!("Elaborazione dei dati in corso...");
        let sum: i32 = data.iter().sum();
        println!("La somma dei dati è: {}", sum);
    };
    // Chiamiamo la closure
    process_data();
    process_data();
    process_data();

    //ESEMPIO PER ECCELLENZA
    let mut vec = vec![1, 2, 3, 4, 5];
    let f = || vec.contains(&3);
    f();
    //vec.push(10); //errore : vec è mutable borrowed
    f();

    let mut vec = vec![1, 2, 3, 4, 5];
    let f = || vec;
    //println!("{:?}", vec);

    let mut s = "ciao".to_string();
    let r = &mut s;
    let mut closure = || {
        let k = r;
        println!("{}", k);
    };


    //FUNZIONI DI ORDINE SUPERIORE
    let mut text = "Hello, world!".to_string();
    let mut printer = |arg: String| {
        text.push_str(&arg);
        println!("{}", text);
    };
    printer(" printer".to_string());
    higher_order_fun(printer, " higher order".to_string());

}