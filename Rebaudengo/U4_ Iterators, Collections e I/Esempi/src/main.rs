use std::collections::{BTreeMap, HashMap, LinkedList};
use std::fs::File;
use std::io::Read;
use std::iter::Peekable;

struct Contatore{
    count: usize,
    max: usize
}
struct Test{
    contatore: Contatore,
}
impl Iterator for Contatore {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            let val = self.count;
            self.count += 1;
            Some(val)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Pixel{
    r: i8,
    g: i8,
    b: i8
}
impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = std::array::IntoIter<i8, 3>;
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.r, self.g, self.b])
    }
}
fn read_file(file_name: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    //-----------------------ITERATORI------------------------
    println!("-----------------ITERATORI-----------------");
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[1..3];
    for i in slice.into_iter() {
        println!("{}", i);
    }
    println!("slice: {:?}", slice);
    // Ottiene il primo elemento dall'iteratore
    if let Some(first_num) = numbers.iter_mut().next() {
    *first_num += 1;
    } else {
    println!("Nessun elemento trovato");
    }
    let iteratore = numbers.iter();
    for num in iteratore{
        println!("{}", num);
    }

    //INTO ITERATOR
    let p = Pixel{r:10, g:11, b:12};
    for i in p{
        println!("elem pixel : {:?}", i);
    }
    let list = LinkedList::<i32>::new();


    //MAP - FILTER - FILTER MAP
    let mut vec = vec!["ciao".to_string(), "come".to_string(), "stai".to_string()];
    let iter = vec.iter();
    let iter_map = iter.map(|x| {
        let i = 10;
        let r = x.len() + i;
        r
    });

    let numbers = vec![1, 2, 3, 4, 5];
    // Creiamo un iteratore dal vettore + applichiamo il metodo map per raddoppiare ogni elemento
    let iter = numbers.iter();
    let doubled_iter = iter.clone().map(|&x| x * 2);
    println!("{:?}", iter);
    // Stampiamo i risultati
    for doubled in doubled_iter {
        println!("{}", doubled);
    }

    //FILTER
    let mut iter2 = vec.iter();
    let iter_filter = iter2.filter(|x| x.len() > 4);

    //FILTER MAP
    let iter3 = vec.iter_mut();
    let iter_filter_map = iter3.filter_map(|x| {
        if x.chars().count() > 0 {
            x.push_str("!");
            Some(x)
        } else {
            None
        }
    });
    println!("VEC : {:?}", vec);

    //FLATTEN
    let nested_numbers = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    // Appiattisce la struttura nested di vettori in un singolo vettore
    let flattened_numbers = nested_numbers.iter().flatten();
    for num in flattened_numbers {
        println!("{}", num);
    }

    //FLAT MAP
    let numbers = vec![1, 2, 3, 4];
    let new_numbers = numbers.iter()
        .flat_map(|x| vec![*x, (*x) * (*x), (*x) * (*x) * (*x)]);
    for n in new_numbers {
        println!("{:?}", n);
    } // Output: [1, 1, 1, 2, 4, 8, 3, 9, 27, 4, 16, 64]

    //TAKE
    let numbers = vec![10, 20, 30, 40, 50];
    // Prende i primi due numeri maggiori di 20 dall'inizio del vettore
    let first_two_over_twenty = numbers.iter()
        .filter(|num| **num > 20)
        .take(2);
    for n in first_two_over_twenty {
        println!("Primi due numeri maggiori di 20: {:?}", n);
    }

    //PEEKABLE
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter();
    let mut peekable_numbers = iter.peekable();
    if let Some(&next) = peekable_numbers.peek() {
        println!("Prossimo elemento: {:?}", next);
    }
    let range = 1..10;
    let reverse = range.clone().rev();

    //CHAIN
    let numbers1 = vec![1, 2, 3];
    let numbers2 = vec![4, 5, 6, 7];
    // Concateniamo i due vettori utilizzando il metodo chain
    let chained_numbers = numbers1
        .iter()
        .chain(numbers2.iter());
    for n in chained_numbers {
        println!("Numeri concatenati: {:?}", n);
    }

    let strings = vec!["hello", "world"];
    let numbers = vec![1, 2, 3];
    // Concateniamo il vettore di stringhe con il vettore di numeri
    let concatenated = strings.iter()
        .map(|s| s.to_string())
        .chain(numbers.iter().map(|n| n.to_string()));
    for s in concatenated {
        println!("elemento: {:?}", s);
    }

    //CHARS
    let words = vec!["hello", "world"];
    let chars = "123";
    // Concateniamo l'iteratore delle parole con l'iteratore dei caratteri
    let chained_sequence = words.iter()
        .flat_map(|word| word.chars())
        .chain(chars.chars());
    for n in chained_sequence{
        println!("Carattere: {:?}", n);
    }

    //ZIP
    let numbers = vec![1, 2, 3];
    let letters = vec!['a', 'b', 'c'];
    // Otteniamo un iteratore che produce tuple (numero, lettera)
    let mut iter = numbers.iter().zip(&letters);
    println!("{:?}", letters);

    //Svuota iterator, Iteriamo attraverso ogni coppia (numero, lettera)
    while let Some((number, letter)) = iter.next() {
        println!("Numero: {}, Lettera: {}", number, letter);
    }

    let (n,l): (Vec<i32>, Vec<char>) = iter.unzip();
    println!("{:?}", n);


    //CYCLE
    let numbers = vec![1, 2, 3];
    // Otteniamo un iteratore che cicla ripetutamente attraverso i numeri
    let mut cycle_iter = numbers.iter().cycle();
    // Stampiamo i primi 5 numeri del ciclo
    for _ in 0..5 {
        if let Some(num) = cycle_iter.next() {
            println!("Numero: {:?}", num);
        }
    }

    //COPIED - CLONED
    let tuple_vec = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    // Otteniamo un iteratore che produce copie delle tuple originali
    let copied_iter = tuple_vec.iter().copied();
    // Stampiamo ogni tupla nel nuovo iteratore
    for tuple in copied_iter.into_iter() { //into iter aggunto in automatico
        println!("Tuple: {:?}", tuple);
    }
    for tuple in tuple_vec{
        println!("Tuple: {:?}", tuple);
    }


    //CONSUMATORI
    let v = vec![1, 2, 3, 4, 5];
    let v2 = v.iter().collect::<Vec<&i32>>();

    println!("{:?}", read_file("file.txt"));

    //FOR EACH
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.iter_mut().for_each(|x| *x += 5);
    for n in numbers {
        println!("Numero: {:?}", n);
    }

    //FOLD
    let mut numbers = vec![ 1, 2, 3, 4, 5];
    // Somma tutti gli elementi della collezione
    let sum = numbers.iter().clone().fold(10, |accumulatore, x| accumulatore + x);
    println!("La somma di tutti gli elementi è: {}", sum);

    let product = numbers.iter_mut().try_fold(1, |acc,  x| {
        if *x != 0 {
            *x+=1;
            Ok(acc * *x)
        } else {
            Err("ho uno Zero nella sequenza")
        }
    });
    match product {
        Ok(result) => println!("Il prodotto di tutti gli elementi è: {}", result),
        Err(err) => println!("Errore: {}", err),
    }
    println!("{:?}", numbers);

    //POSITION + NTH
    let numbers = vec![1, 2, 3, 4, 5];
    // Find the index of the first even number
    let index = numbers.iter().position(|&x| x % 2 == 0);
    match index {
        Some(i) => println!("The first even number is at index {}", i),
        None => println!("No even number found"),
    }
    let i = numbers.iter().nth(index.unwrap()).unwrap();

    //PARTITION
    let people = vec![
        ("Alice", 30),
        ("Bob", 25),
        ("Charlie", 35),
        ("David", 40),
    ];
    let (over_thirty, under_thirty): (Vec<_>, Vec<_>) = people.into_iter().partition(|(_, age)| *age > 30);


    //--------------------------------COLLEZIONI--------------------------------------
    //HASH MAP
    let mut mappa_hash = HashMap::new();
    mappa_hash.insert("chiave", "valore");
    let r= mappa_hash.get("chiave");
    let elem = mappa_hash.keys().find(|&&x| x == "chiave");
    println!("{:?}", elem.unwrap());

    let mut scores = HashMap::new();
    scores.insert("Team Blue", 10);
    scores.insert("Team Red", 20);
    // Incrementa il punteggio del team "Team Blue" se esiste,
    // altrimenti inserisci un nuovo punteggio
    scores.entry("Team Blue_MODIFIED").and_modify(|score| *score += 5).or_insert(15);
    println!("{:?}", scores);

    //BTREE MAP
    let mut scores = BTreeMap::new();
    scores.insert("Alice", 42);
    scores.insert("Bob", 69);
    println!("{:?}", scores.entry("Alice"));
}

