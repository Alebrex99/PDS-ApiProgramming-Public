use std::fmt::{Display, Formatter, write};

/*implementazione in automatico del tratto DEBUG
   - #[derive(Debug,)]
   - #[derive(Debug, PartialEq)]   implementazione in auto
   display NON E' DERIVABILE*/
#[derive(Debug,)]

struct Point{
    x: i32,
    y: i32
}


impl Point{
    fn new(x: i32, y: i32) -> Self{
        //Point{x,y}
        let p = Point {x,y};
        println!("Creating Point ad address {:p}", &p); //&p : lo da sull ostack, poi lo passa e ritorna e assegna da un altra parte
        p
    }

    fn swap(self) -> Self{
        Point{x:self.y, y:self.x}
    }
   /* si mangia SELF, non sara piu usabile poi torna un SELF*/
}

//------------------IMPLEMENTO TRATTI DROP----------------------
impl Drop for Point{
    fn drop (&mut self){
        println!("Dropping point at address {:p}", self);
        /*:p : scrivibile per stampare puntatore di quella cosa li*/
    }
}



impl PartialEq<Point> for Point{
    /*Struct point puo esssere cofronotata ad un altro punto
        quando trovo questo , usero l'implementazione qua presente.
        i membri chiesti da PARTIAL EQ:
        - eq
        - NE : not equal , torna true se diverso
        basta impementarne 1 , perche se è equal lu no lo sa, ma se vede un confrono diverso, fa NOT = e cosi va benne
    self.x = other.x && self.y==other.y
    */
    fn eq(&self, other: &Point)-> bool {
        return self.x==other.x && self.y==other.y
    }

}

impl PartialEq<i32> for Point{
    fn eq (&self, other:&i32)-> bool{
        return self.x+self.y==*other
        /*cosi so poter passarlo per reference senza consumarlo*/
    }
}

impl PartialEq<Point> for i32{
    fn eq(&self, other:&Point) -> bool{
        return other == self
    }
}

/*
mod m {
    impl PartialEq<Point> for i32{ //non lo trova !
        fn eq(&self, other: &Point) -> bool{
            return other == self
        }
    }
}*/



//------------------IMPLEMENTO TRATTI DISPLAY E DEBUG----------------------
impl Display for Point{
    fn fmt(&self, f:&mut Formatter<'_>)-> std::fmt::Result{
        todo!();
        /*usiamo la macro WRITE che gia permette con un formato simile
        a quello della PRINTLN , consente di scrivere in modo agevole, come vogliamo
        i dati in struttura vengano fatti VEDERE*/
        write!(f, "({}, {})", self.x, self.y);
    }
}






fn main() {
    println!("Hello, world!");
    /*punti creati fino a fine programma*/
    let p1 = Point::new(1, 2);
    //let p2 = Point::new(2, 1);
    let mut _p2 = Point::new(2,1);
    /* se scrivessi 3 == p1-> casino ,
    perche def cercata in def di sinistra,
    perche i32 no nimplementa partial eq con POIN ma il contrario si
    */

    /*se assegno a p2, p1? devi ovvio commetnare p1== p2 , gli if dopo
    prende p1 contenuto e sovrascrivere P2, allora cio che era in P2, viene stampato
    perche viene chiamato il TRATTO DROP, poi fa assegnazione, in p2, mette 1, 2
    poi arriva a fine graffa e butta via tutto.
    quando implementazione tratto DROP:
    - metodo chiamato se var raggiunge fine esistenza (})
    - oppure quando a variabile assegno un valore
    p2 , raggiunge fine , viene buttato via, grave se avessi numero creazioni che non matchano numero drop
    se ho piu drop di creazioni -> DOUBLE FREE*/
    _p2 = p1;


    //FUNZIONE DROP DI MOVIMENTO GLOBALE
    drop(_p2);
    /*prende per ovimento il punto e lo butta via
    permette di liberarci di risorse perima che naturalemtne finiscano*/

    /*if p1 == p2{
        println!("sono uguali");
        /*se voglio stampare contenuto del mio PUNTO:*/
        println!("sono uguali {} {}", 3, p1);
        /*struttura POINT non implementa il DISPLAY tratto, serve per visualizzare
        umanamanete la cose : come una stringa bella*/
    }else{
        println!("sono diversi");
    }*/

}
