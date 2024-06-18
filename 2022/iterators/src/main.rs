

fn main() {
    let mut v = vec!["alfa".to_string(), "beta".to_string(), "gamma".to_string(), "delta".to_string()];
    //devo trasformarli in stringhe tali str con tostring, perche se no non sono manipolabili

    /*1) iter da reference a str*/
    for s in v.iter() {
        println!("{}", s);
    }

    /*2) iter da reference a str*/
    for s in v.iter_mut(){
        s.push_str("-1"); //paciocco ogni stringa aggiungendo il -1
    }
    println!("{:?}", v); //per vedere cosa c'è nel mio vettore modificato

    /*3) into_iter -> iteratore si mangia contenuto */
    /*
    for s in v.into_iter(){
        println!("{}", s);
        /*da errore perche into_iter operazione che si mangia elemnti , non ha la &,
          iter_mut invece si prende un & , reference su cui lavorare
          into_iter(self) , prende self e v non possiede piu ha ceduto ad uno ad uno tutti i
          suoi elementi e quindi s ha preso possesso del dato contenuto nel vettore poi buttato via, ultima riga print non sfruttabile*/
    }
    println!("{!?}", v);*/

    /**/
    for s in &v {
      /*se itero in reference voglio i reference in v*/
    }

    for s in &mut v{

    }

    for s in v{
        /*espande in into_iter*/
        println!("{}", s);
    }
}
