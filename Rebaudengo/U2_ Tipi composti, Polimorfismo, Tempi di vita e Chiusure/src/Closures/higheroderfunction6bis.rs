fn function_generator<T>(v: T) -> impl Fn() -> T
    where
        T: Clone,
{
    return move || { v.clone() };
}

fn main() {
    // Crea una funzione che restituisce il numero 42
    let generate_42 = function_generator(42);
    // Crea una funzione che restituisce la stringa "hello"
    let generate_hello = function_generator("hello".to_string());

    // Esempio con un tipo più complesso come un vettore
    let vec = vec![1, 2, 3];
    let generate_vec = function_generator(vec);
    
    println!("Numero: {}", generate_42()); // Stampa "Numero: 42«
    println!("Numero: {}", generate_42()); // Stampa "Numero: 42"
    println!("Stringa: {}", generate_hello()); // Stampa "Stringa: hello"
    println!("Vettore: {:?}", generate_vec()); // Stampa "Vettore: [1, 2, 3]"
}
