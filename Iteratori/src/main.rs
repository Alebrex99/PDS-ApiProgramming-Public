use std::ops::Mul;

struct P(i32);
impl Drop for P {
    fn drop(&mut self) {
        println!("Dropping P({})", self.0);
    }
}


fn main() {
    //-------------------------ITERATORI E POSSESSO : ITERATORI CONSUMANO O NO IL CONTENITORE---------------------------
    //PROVE PERSONALI
    println!("-----------------PROVE PERSONALI-----------------");
    let mut v4 = vec![1,2,3,4,5,6];
    //v4.next(); //non puoi perchè VEC impl solo INTO ITERATOR, ovvero prima devi trasformarlo in iteratore
    //let mut it = IntoIterator::into_iter(v4); //into_iter() -> restituisce un iteratore

    //USANDO LE FUNZIONI PREDISPOSTE DAL VEC PER TRASFORMARE IN ITERATOR
    let mut iteratore = v4.iter_mut(); //iter_mut() -> restituisce un iteratore mutabile
    let prossimo_dato = iteratore.next(); //next() -> restituisce un Option<&mut T>
    let dato = prossimo_dato.unwrap(); //unwrap() -> restituisce il valore Option<T> se Some(T)
    println!("{}", dato); //posso perchè iter_mut() -> restituisce un iteratore mutabile

    for x in v4.iter(){
        println!("{}", x);
    }

    let mut v5 = vec![1,2,3,4,5,6];
    v5 = v5.iter_mut(). //NON VIENE MODIFICATO L'ORIGINALE COSI
        map(|n| *n * 2).collect();
    println!("v5: {:?}", v5);
    //println!("{:?}", v_final);

    let mut v6 = vec![1,2,3,4,5,6];
    v6.iter_mut().for_each(|n| *n*=2); //SOLO COSI VIENE MODIFICATO ON PLACE L'ORIGINALE
    println!("v6: {:?}", v6);

    //CONSUMO COMPLETO CONTENITORE
    println!("-----------------ITERATOR - INTO ITERATOR-----------------");
    let v = vec![1, 2, 3, 4, 5, 6];
    for x in v { //sbriciola v e lo consuma
        println!("{}", x);
    }
    //println!("{:?}", v); // Errore: value borrowed here after move, V NON ESSITE PIU

    //NON CONSUMO CONTENITORE
    let mut  v1 = vec![1, 2, 3, 4, 5, 6];
    for x in &v1 { //usato un riferimento in lettura (l'iteratore non può modificare il contenitore)
        println!("{}", x);
    }
    println!("{:?}", v1);

    for x in &mut v1 { //V è un contenitore = implementazione di IntoIterator e chiamata a iter_mut()
        *x *=2; //mettiamo *x per accedere perchè il for con in è come se chiamasse iter_mut()
    }
    println!("{:?}", v1);// OK, perchè tale for

    //CONSUMO PARZIALE CONTENITORE
    for mut x in v1{
        x *=2;
        if x>4 {break;}
    }
    /*1 diventa 2, 2 ->4, 3 -> diventa 6 e poi la smetto. Ho parzialmente svuotato v1,
    quindi controlliamo cosa succede. Anche se non gli ho tirati fuori tutti da V1,
    siccome ho cominciato a prenderne, V non è più buono, è stato consumato*/
    //println!("{:?}", v1);// Errore: value borrowed here after move, V CONSUMATO PER UNA PARTE


    //USO DELLA STRUCT (TUPLA) : COSA SUCCEDE QUANDO CONSUMI PER UNA PARTE IL VETTORE
    let v2 = vec![P(1), P(2), P(3), P(4), P(5), P(6)];
    for x in v2{
        println!("Consuming P({})", x.0);
        if x.0 > 2 {break;}
    }
    println!("Done");
    //println!("{:?}", v2); //errore
    /*STAMPA :
Consuming P(1)
Dropping P(1)
Consuming P(2)
Dropping P(2)
Consuming P(3) //lo guardo -> troppo grande , quindi ho Break
Dropping P(3)  //apparentemente ho finito, ho tra le mani x che non mi serve quindi droppo P(3) ci sta
Dropping P(4)  //Droppo tutto il resto
Dropping P(5)
Dropping P(6)
Done
    guardo che inizio for, consumo P(1) e la stampa, poi in IF, non soddisfatto voglio il prossimo
    la X non mi serve più, tolto da vettore e lo butto -> <Dropping p(1) , poi
    consuming(P(2)) fino a P(3) che era troppo grande.
    Droppo tutto il resto -> poichè ho cominciato a smontarlo il vettore, RUST non permette di mollarlo li mezzo fatto
    lui lo smonta tutto e butta via tutto V
*/
     //SENZA CONSUMO :
    let v3 = vec![P(1), P(2), P(3), P(4), P(5), P(6)];
    for x in &v3{
        println!("Consuming P({})", x.0);
        if x.0 > 2 {break;}
    }
    println!("Done");
    /* non succede niente, estraggo ognuno, lo guardo, non accade nulla, mentre ci guardo dentro
    non si ha l'accesso al contenitore perchè ho un PRESTITO, inoltre, essendo &semplice non posso modificarlo
    il vettore è svuotato solo alla fine del MAIN
    Done
    Dropping P(1)
    Dropping P(2)
    Dropping P(3)
    Dropping P(4)
    Dropping P(5)
    Dropping P(6)*/

}
