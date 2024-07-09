fn main() {
    let mut counter = 0;
    let mut sum = 0;

    let result = loop {
        counter += 1;
        sum += counter;

        if counter == 10 {
            break sum * 2;
        }
    };

    println!("The result is {result}");
}