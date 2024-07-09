mod module1 {

    #[derive(Debug)]
    pub struct Test{
        a: i32,
        b: bool
    }
    impl Test {
       pub fn new() -> Test{
          Test {a: 0, b: false}
      }
    }
    pub fn f (i: i32, boo: bool) -> Test
    {
        Test {a: i, b: boo}
    }

}
use module1::Test;
use module1::f;

fn main() {
    let t: Test = f(100, true);
    println!("{:?}", t);
    let t:Test = Test::new();
    println!("{:?}", t);
}