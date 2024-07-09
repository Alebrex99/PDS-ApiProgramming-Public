struct Test (i32);

impl Drop for Test {
  fn drop (& mut self) {
    println!("Destroying Test ({}) at address {:p}", self.0, self);
    }
}

fn main() {
    let mut v =Vec::<Test>::with_capacity(4);
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

  for i in 0..4
    { v.push(Test(i)); }
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    v.push(Test(10));
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    for i in 20..25
    { v.push(Test(i));}

    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    v.pop();

    v.shrink_to_fit();
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());
}
