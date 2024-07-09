//Es 2
pub fn driver_es_2() {
    //- Cosa fa iter(),filter() e zip()?
    //- Se si rimuovere .clone(), il codice compila lo stesso?
    //- Cosa viene stampato?
    let c = vec![1, 2, 4, 5, 8];
    let a = c.iter().filter(|&x| x % 2 == 0).zip('a'..'z');
    let b = a.clone().map(|(x, y)| format!("{y}{x}")).last();
    println!("{:?}", b);
    println!("{:?}", a.count());
}
