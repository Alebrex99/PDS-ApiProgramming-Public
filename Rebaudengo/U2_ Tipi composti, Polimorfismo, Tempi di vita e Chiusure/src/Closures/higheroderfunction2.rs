fn call_with_ten<F>(func: F) -> i32
    where
        F: Fn(i32) -> i32,
{
    func(10)
}

fn main() {
    let venti = |x| x * 2;
    let trenta = |x| x * 3;
    println!("10 x 2 = {}", call_with_ten(venti));
    println!("10 x 3 = {}", call_with_ten(trenta));
}
