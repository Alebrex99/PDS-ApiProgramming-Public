use std::rc::Rc;
use std::time::Duration;
use crate::Esame4Set2023::{Cache, Entry};


mod Esame22Gen2024;
mod Esame4Set2023;
mod Esame7Lug2023;
mod Esame20Giu2023;
mod Esame16Gen2023;
mod Esame25Giu2024;

fn make_box<T>(i : T) -> Box<T>{
    let b = Box::new(i);
    return b
}


struct Point {
      x: i16,
      y: i16,
}

enum PathCommand {
      Move(Point),
      Line(Point),
      Close,
}

pub struct Data{
    Element :AsVector,
    next: Rc<Data>
}

enum AsVector{
    AsVector(Box::<Rc<i32>>),
    None
}

fn main() {
    //ESERCIZIO 1
    println!("Hello, world!");
    let c = make_box(27);

    //esercizio 2
    let mut v = Vec::<PathCommand>::new();
    v.push(PathCommand::Move(Point{x:1,y:1}));
    v.push(PathCommand::Line(Point{x:10, y:20}));
    v.push(PathCommand::Close);
    let slice = &v[..];

    let c = Cache::new();
    c.put(1, 2, Duration::from_secs(1));
    
}
