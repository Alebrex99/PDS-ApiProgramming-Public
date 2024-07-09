use std::time::{Duration, Instant};
use crossbeam_channel::tick;
fn main()
{
let start = Instant::now();
let ticker = tick(Duration::from_millis(100));

for _ in 0..5 {
ticker.recv().unwrap();
println!("Tempo trascorso: {:?}", start.elapsed());
    }
}
