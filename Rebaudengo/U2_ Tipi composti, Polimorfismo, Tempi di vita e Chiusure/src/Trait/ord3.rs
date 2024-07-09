fn main() {

    let min = 50;
    let max = 100;

    let num1 = 200;
    let num2 = 25;

    let clamped_number = num1.clamp(min, max);
    println!("Clamped number: {}", clamped_number); // Stampa: Clamped number: 100

    let clamped_number = num2.clamp(min, max);
    println!("Clamped number: {}", clamped_number); // Stampa: Clamped number: 50

}