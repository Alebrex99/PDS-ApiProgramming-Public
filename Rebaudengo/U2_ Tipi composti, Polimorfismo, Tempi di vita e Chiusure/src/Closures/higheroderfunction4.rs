fn call_twice<F>(mut closure: F)
    where
        F: FnMut(),
{
    closure();
    closure()
}

fn main() {
    let mut i = 0;
    call_twice(|| i += 1 );
    println!("{}", i);
}
