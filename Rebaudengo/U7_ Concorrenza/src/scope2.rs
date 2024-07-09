use std::thread;
fn main()
{
    let mut v = vec![1, 2, 3];
let mut x = 0;
thread::scope(|s| {
s.spawn(|| { // è lecito creare un riferimento a v
println!("length: {}", v.len());
});
s.spawn(|| { // anche qui viene catturato &v
for n in &v {println!("{n}"); }
x += v[0]+v[2]; // x è catturata come &mut
});
});

    // Solo quando entrambi i thread saranno terminati si proseguirà
    v.push(4); // non ci sono più riferimenti, si può modificare
    println!("Vettore: {:?}", v);
    println!("x: {:?}", x);
}
