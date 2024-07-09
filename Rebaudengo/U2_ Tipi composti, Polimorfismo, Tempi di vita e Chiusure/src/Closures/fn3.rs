fn main() {
    let base_number = 10;

    // Definiamo una closure che non modifica variabili esterne
    let  square = |x: i32| {
        let result = x * x;
        println!("Il quadrato di {} è: {}", x, result);

    };

    square(base_number);
    square(5);

}