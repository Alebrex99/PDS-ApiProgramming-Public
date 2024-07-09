fn apply_function<F> (input: i32, function: F) -> i32
    where
        F: Fn(i32) -> i32,
{
    function(input)
}

fn main() {
    let doubled = apply_function(10, |n| n * 2);
    println!("{}", doubled);
}
