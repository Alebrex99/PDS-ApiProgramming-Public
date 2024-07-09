fn compose<F, G>(f: F, g: G) -> impl Fn(i32) -> i32
    where
        F: Fn(i32) -> i32,
        G: Fn(i32) -> i32,
{
     move |x| g(f(x))
}

fn main() {
    let add_then_double = compose(|n| n + 1, |n| n * 2);
    println!("{}", add_then_double(5));
    println!("{}", add_then_double(10));
}
