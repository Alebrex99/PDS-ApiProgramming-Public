
mod Module1 {
#[derive(Debug)]
pub struct Test{
     a: i32,
     b: bool
}
pub fn f (i: i32, boo: bool) -> Test
{
    Test {a: i, b: boo}
}

}
use Module1::Test;
use Module1::f;
fn main() {
    let t: Test = f(100, true);
    println!("{:?}", t);
}

