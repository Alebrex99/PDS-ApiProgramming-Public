fn generator(prefix: &str) -> impl FnMut() -> String {
    let mut i = 0;
    let b = prefix.to_string();
    return move || {i+=1; format!("{}{}",b,i)}
}

fn main() {
  let mut f = generator("id_");
  for _ in 1..5 {
      println!("{}",f());
  }
}
