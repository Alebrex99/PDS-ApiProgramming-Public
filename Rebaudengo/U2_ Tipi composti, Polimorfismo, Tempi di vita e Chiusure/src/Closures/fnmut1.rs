fn main() {
    let mut counter = 0;

    // Definiamo una closure che può essere chiamata più volte
    let  mut increment_counter = || {
        counter += 1;
        println!("Il contatore è ora: {}", counter);
    };

    // Chiamiamo la closure più volte
    increment_counter();
    increment_counter();
    increment_counter();
}