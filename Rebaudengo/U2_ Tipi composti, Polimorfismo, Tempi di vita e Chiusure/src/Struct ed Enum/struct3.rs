#[derive(Copy, Clone)]
struct Empty;

fn main() {
   let p = Empty;

   let mut array: Vec<Empty> = Vec::new();

   for _ in 0..10
   {
      array.push(p);
   }

   println!("{}", array.len());
}