use std::ffi::OsString;

fn main() {
    let stringa = String::from("Hello"); // Stringa di esempio

    // Converti la Stringa in un OsString utilizzando il metodo into()
    let os_string: OsString = stringa.into();

    println!("{:?}", os_string); // Stampa: "Hello"
}