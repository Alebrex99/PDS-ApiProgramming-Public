use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len()); 
            // riferimento a variabile locale che non deve essere catturata con move
        });

        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}
