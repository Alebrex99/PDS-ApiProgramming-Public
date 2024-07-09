
fn main() {
    let vec: Vec<i32> = vec![5, 10, 2, 8];
    let max = vec.into_iter().max();
    println!("Il valore massimo è {:?}.", max);
    println!("Il vettore è {:?}.", vec);
    }