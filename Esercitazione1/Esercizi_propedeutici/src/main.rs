use std::fmt::Display;
use std::time::SystemTime;

enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String)
}

fn print_error(e: Error) {
    match e {
        Error::Simple(time) => {
            let time_elapsed = time.elapsed().unwrap();
            println!("Simple error at {:?}", time_elapsed)
        },
        Error::Complex(time, message) => println!("Complex error at {:?} with message {}", time, message)
    }
}

pub enum MulErr{
    Overflow,
    NegativeNumber,
}
pub fn mul (a: i32, b:i32) -> Result<u32, MulErr>{
    if(a<0 && b>0 || a>0 && b<0){
        return Err(MulErr::NegativeNumber)
    }
    let result = if((a as u32*b as u32) >= u32::MAX) {Err(MulErr::Overflow)}
        else { Ok((a*b) as u32)};
    result
}

struct Node_heap{
    name: String,
    size: u32,
    count: u32,
}
impl Node_heap{
    pub fn new(name: String) -> Box<Self>{
        Box::new(Node_heap{name, size: 0, count: 0})
    }
}
impl Drop for Node_heap{
    fn drop(&mut self){
        println!("Dropping Node_heap {}", self.name);
    }
}

//VERSIONE NODE INEFFICIENTE : CREAZIONE DI 3 STRUCT
struct Node {
    name: String,
    size: u32,
    count: u32,
}
impl Node {
    //constructor
    pub fn new(name: String) -> Self{ Node::from_size_count(name, 0, 0) }
    pub fn from_size (name: String, size: u32) -> Self{
        Node {name, size, count:0}
    }
    pub fn from_size_count (name: String, size: u32, count: u32) -> Self{
        Node {name, size, count}
    }
    pub fn size(self: &Self, size: u32) -> Self{
        Node::from_size(self.name.clone(), size) //perchè parlando di String altrimenti lo consumi
    }
    fn count(&self, count: u32) -> Self{
        Node::from_size_count(self.name.clone(), self.size, count)
    }
    fn toString(&self)-> String{
        format!("Chiamata a toString : {}", self.name)
    }
}
impl Display for Node{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "Formattazione (toString) di Node {} size {} count {}", self.name, self.size, self.count)
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node {}", self.name);
    }
}

fn main() {
    let max = i32::MAX;
    println!("Max value of i32 is {}", max);
    println!("Max value of u32 is {}", u32::MAX);
    let res = mul(2,2);
    match res {
        Ok(value) => println!("Result is {}", value),
        Err(e) => match e {
            MulErr::Overflow => println!("Overflow"),
            MulErr::NegativeNumber => println!("Negative number")
        }
    }

    let node = Node::new(String::from("node")).size(10).count(5);
    //droppi le prime due strutture NODE perchè ad ongi chiamata ne crei una e riassegni la vecchia variabile node a quella nuova droppando la vecchia
    println!("Node size is {} and count is {}", node.size, node.count);
    println!("{}", node);
    println!("{}", node.toString());

}
