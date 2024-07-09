enum Operazioni {
    Add,
    Subtract,
}

impl Operazioni {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
let op1 = Operazioni::Add;
let op2 = Operazioni::Subtract;

    let res = op1.run(5,7);
    println!("{}", res);
    let res = op2.run(5,7);
    println!("{}", res);
}