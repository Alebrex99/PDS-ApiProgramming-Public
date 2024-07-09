fn main() {
     let a = [10, 20, 30, 40, 50];
     for element in a {
        println!("the value is: {element}");
    }

    for n in 1..10 { 			// Stampa i numeri da 1 a 9
        println!(“{}”, n); 
    }

    let names = ["Bob", "Frank", "Ferris"];
    for name in names.iter() {  	           // Stampa i tre nomi
        println!("{}", name);
    }
    
    for name in &names[ ..=1 ] {  	     // Stampa i primi due nomi
        println!("{}", name);
    }
    for number in (1..=10).rev() {   // Stampa una sequenza al contrario
        println!("{number}!");
    }
    
    for (i,n) in names.iter().enumerate() { //stampa indici e nomi
        println!("names[{}]: {}", i, n);
    }
}
