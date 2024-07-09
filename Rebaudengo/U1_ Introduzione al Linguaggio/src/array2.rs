fn f (i: usize) -> usize
{
  return (i + 10)
}
fn main() {
   let v = [1,2,3,4];

    println!("{}", v[1]);
    println!("{}", v[f(0)]);
}
