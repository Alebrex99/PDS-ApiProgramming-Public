fn main() {
    let vec = vec![1, 2, 3, 4, 5];

    // Ottenere una slice che include tutto il vettore
    let slice1: &[i32] = &vec[..];

    // Ottenere una slice che include solo una parte del vettore
    let slice2: &[i32] = &vec[1..3]; // Dal secondo all'indice 3 (escluso)

    println!("{:?}", slice1); // Stampa: [1, 2, 3, 4, 5]
    println!("{:?}", slice2); // Stampa: [2, 3]
}