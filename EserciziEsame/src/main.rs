use std::rc::Rc;

mod Esame7Lug2023;

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
    
}
