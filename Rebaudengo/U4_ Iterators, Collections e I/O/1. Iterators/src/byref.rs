fn main()
{
    let mut numeri = vec![1, 2, 3].into_iter();

    // Utilizziamo by_ref per prendere i primi due elementi 
    // senza consumare completamente l'iteratore
    let primi_due = numeri.by_ref().take(2);

    for n in primi_due {
        println!("Primi 2: {}", n);
    }
    for n in numeri {
        println!("Tutti i numeri: {}", n);
    }
}
