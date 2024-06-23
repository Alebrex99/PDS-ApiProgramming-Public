use std::num::ParseIntError;

//---------------LIFETIME------------------------
fn lifetime<'b, 'c>(x: &'b S, y: &'c S) -> &'c u8{
    & y.0
}
fn f<'a, 'b>(i: &'a i32, v: &'b mut Vec<&'a i32>) {
    v.push(i);
}
struct S(u8);


//--------------LAMBDA FUNCTION E DI ORDINE SUPERIORE (GEN)------------------------
/*GENERATORE: ho funzione che cattura delle cose, viene generata una TUPLA alla vista di | | {}
CHE IMPL il TRATTO FUNZIONALE, quindi è chiamabile, ma essendo tupla ha uno stato e fa riferimento
a SELF in automatico, Quando vede codice -> i -> self.1 e BASE self.0 (della tupla) */
fn gen(base: String) -> impl FnMut()-> String{
    let mut i =0;
    return move ||{ //MOVE ci vuole perchè prenderebbe i come riferimento semplice, ma devi forzare per prendere possesso di i, lo consuma
        i+=1;
        format!("{}{}", base, i) //i diventa self.1 e base -> self.0
    };
}
/*fn gen2(base: String) -> impl FnMut()-> String{
    let mut i =0;
    fn f1(mut i:i32, base:String) -> String{
        i+=1;
        format!("{}{}", base, i)
        /*avrei problemi ad accedere a BASE perchè fn f1 è pezzo codice e basta,
        non posso perchè funzione ha scope isolato dal resto, non posso far permeare
        i e base in F1 , potrei metterli come parametri, ma  poi f1 ritorna e nulla.*/
    }
}*/


//--------------OPERAZIONI TRAMITE LE LAMBDA FUNCTIONS------------------------
/*cre una funzione specializzata nel costruire una funzione che quando sarà chiamata, creerà un vettore
della dimensione di N elementi, solo quando te lo chiedo*/
fn gen_vect(n : usize) -> impl FnOnce()-> Vec<i32>{
    return move || {
        println!("Sto per preparare il vettore con {n} elementi");
        let mut v = vec![];
        for i in 0..n{
            v.push(0);
        }
        println!("FATTO {:?}", v);
        return v;
    }
}



fn main() {
    //------------------------------------------LIFETIME--------------------------------------------
    /*Qui ho funzione F, prende 2 ref con tempo 'B e 'C e ne restituisce uno con tempo vita 'C,
    siccome restituisce un riferimento con tempo vita 'C, ha 2 livelli implicazione: 1 sull' accesso
    a riferimento in quanto tale -> non posso dereferenziarlo, leggere il contenuto di quel
    riferimento R DOPO il periodo della sua esistenza, ma anche al contrario -> 2  siccome esso è un
    pezzo del parametro Y ricevuto, il valore attribuito a parametro Y (quindi V2) non può esser cambiato
    fintanto che io uso quel riferimento, se no cambia carte in tavola. */
    let v1 = S(1);
    let mut v2 = S(2);
    let r = lifetime(&v1, &v2);
    let v2 = v1;
    println!("{}",r);

    let stringa = "ciao";
    let n = 42;
    let mut vettore: Vec<&i32> = Vec::new();
    f(&n, &mut vettore);
    println!("{:?}", vettore);


    //---------------------------FUNZIONI LAMBDA E DI ORDINE SUPERIORE (GEN)------------------------
    println!("---------------------FUNZIONE LAMBDA SEMPLICE---------------------");
    //FUNZIONE LAMBDA-CHIUSURA -TRATTI
    let i =5;
    let mut _vec: Vec<i32> = Vec::with_capacity(10);
    let lambda = move || {_vec};
    //_vec.push(10); //errore

    //FUNZIONE LAMBDA SEMPLICE : GENERATORE DI NOMI
    for i in 2..5{
        let f = |v| v*i; /*variabile f è una funzione lambda
        usabile come funzione ch eprende un numero e restituisce un numero che dipende da quanto vale
        i*/
        //Preparo via via delle f diverse
        println!("i : {i} -> f(1): {}", f(1));
        println!("i : {i} -> f(2): {}", f(2));
        println!("=====================");
        /*Quando i vale 2 la funzione moltiplica per 2, poi quando ivale 3 ho 3 e 6 ecc.
         Quindi lamia funzione cattura dall'ambiente in cui era i, lo ha tenuto e riocrdato.
         potrei memorizzare tali funzioni in un ARRAY e si ricordano il valore di i di ogni ciclo.
         Riescono a rirodarselo perchè RUST dietro genera una TUPLA CON SOLO CAMPO (I) in campo0
         se avessi usato più pezzi avrei fatto cosa piu complessa: */
    }

    //USO DI N PEZZI
    /*F1 è tupla con 2 campi -> dentro F1 ho una string con 24 byte e un INT 32 con 4 byte
    Quando la invoco, parte codice specificato sopra, prende campo i dentro TUPLA, somma 1, prende parte BASE
    */
    let mut f1 = gen("a".to_string());
    println!("{}", f1());
    println!("{}", f1());
    println!("{}", f1());
    println!("{}", f1());

    let mut f2  = gen("b".to_string()); //completamente indipendnete va avanti con i suoi conteggi diversi daf1
    println!("{}", f2());
    println!("{}", f2());
    println!("{}", f2());
    println!("{}", f2());
    println!("{}", f1()); //mettendo f1 finale -> mi accorgo che f1 e f2 sono indipendenti
    //Oppure: replico comportamento di GEN, la chiusura prende la variabile libera definita nel contesto del MAIN la j
    let mut j:i32 = 0;
    let mut stringa_moved = "ciao".to_string();
    let mut f3 = move |base: String| {
        j+=1 ;
        stringa_moved = "ciao_modified".to_string();
        format!("{}{}", base, j)}; //f2 è funzione lambda che prende i e restituisce stringa
    println!("{}", f3("c".to_string()));
    println!("{}", f3("c".to_string()));
    println!("{}", f3("c".to_string()));
    println!("Variabile consumata, ma è int con tratto Copy {}", j);//j è variabile libera consumata da f3 ma incrementa il tratto COPY(no problem)
    //println!("{} ", stringa_moved); //error perchè stringa_moved è stata consumata da f3

    let y = Box::new(10);
    let f_ex = move |x: Box<i32>| {y};
    //viene generata una tupla in STACK con 1 campo (y) -> muovi il puntatore a BOX allo heap dentro la tupla


    //----------------------OPERAZIONI TRAMITE LE LAMBDA FUNCTIONS----------------------------------
    println!("---------------------OPERAZIONI TRAMITE LE LAMBDA FUNCTIONS---------------------");
    /*spesso abbiamo bisogno di fare cose sopra vettori di valori:
    tutti i valori di V vengono passati ad 1 a uno nella chiusura passata. LA CHIUSURA in realtà non
    cattura nulla e si limita solo ad operare sul mio parametro facendo cose*/

    //TRASFORMAZIONE DI UN VETTORE
    let v = vec![1,2,3,4,5,6,7,8,9,0]; //vorrei trasformarlo in una serie di stringhe
    let v1: Vec<String> = v
        .iter() //MAP non puoi applicarlo direttamente su vettore, ma devo dire di darmi elementi uno ad uno
        .map(|n|format!("n_{}", n))  //MAP -> per dire di applicare a tutti gli elementi di v la funzione che scrivo
        .collect(); //collect() -> per trasformare in un vettore di stringhe
    println!("V1 semplice: {:?}", v1);

    //FILTRO DI UN VETTORE
    let v2: Vec<String> = v
        .iter() //MAP non puoi applicarlo direttamente su vettore, ma devo dire di darmi elementi uno ad uno, fonrisce i &i32, riferimenti agli elementi del vettore
        .filter(|n|**n<5)//**n -> dereferenzia 2 volte perchè n è un riferimento &i32 a un riferimento &i32 emesso da iter
        .map(|n|format!("n_{}", n))  //MAP -> per dire di applicare a tutti gli elementi di v la funzione che scrivo
        .collect(); //collect() -> per trasformare in un vettore di stringhe
    println!("V2 con filter: {:?}", v2);

    //SOFISTICATO: FILTER + CLOSURE
    /*direi che il concetto del filtro invece di essere sempre <5 è MINORE DI i, ovvero ottengo
    una CLOSURE catturando il valore di i ->
    - ad una prima iterazione prenderò solo 1,2,0;
    - alla seconda 1,2,3,0
    - cosi via fino a 1,2,3, ... ,5,0*/
    for i in 3..7{
        let v3: Vec<String> = v
            .iter() //MAP non puoi applicarlo direttamente su vettore, ma devo dire di darmi elementi uno ad uno, fonrisce i &i32, riferimenti agli elementi del vettore
            .filter(move |n|**n<i)//**n -> dereferenzia 2 volte perchè n è un riferimento &i32 a un riferimento &i32 emesso da iter
            .map(|n|format!("n_{}", n))  //MAP -> per dire di applicare a tutti gli elementi di v la funzione che scrivo
            .collect(); //collect() -> per trasformare in un vettore di stringhe
        println!("V3 con filter + closure: {:?}", v3);
    }

    //PREPARAZIONE DI OGGETTI UTILI IN FUTURO (GEN VEC)
    /*in pratica io non mi sono ancora preparato il vettore, ma ho creato le condizioni (la funzione gen_vect)
    in modo da potercelo avere quando lo voglio*/
    let f = gen_vect(4); //preparo le CONDIZIONI (una funzione) per creare in futuro il vettore
    println!("Preparata la funzione");
    let v = f();
    //creo il vettore, quando mi serve davvero allora lo costruisco con la funzione, non dovrò ricordarmi quanti erano gli element, lo sa lui
}
