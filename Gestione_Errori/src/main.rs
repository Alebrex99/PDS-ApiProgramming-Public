use thiserror::Error;

//modelliamo una funzione che può fallire
#[derive(Error, Debug)] //con derive FileError -> impl il tratto ERROR e il tratto DEBUG
#[error("Problema con il FILE : {0}")] //con questo metodo FileError -> impl il tratto DISPLAY
struct FileError(&'static str);
/*accetta una stringa costante come descrizione, affinchè sia usabile e comprensibile
ad altri deve implementare il tratto ERROR, che farlo a mano sbatti. quindi con THIS ERROR conviene
perchè aggiunge in automatico la possibilità di usare [DERIVE]
FileError è una struct di tipo tupla (campi non hanno nomi) e quindi hanno un indice;
il messagio che descrive cosa si è verificato è campo 0 dentro cui ho descrizione.
La funzione leggi file la possiamo implementare.
*/

#[derive(Error, Debug)] //poichè content error e gli altri anche non sono noti essere errori, THISERROR come libreria utile
#[error("Contenuto '{content}' non adatto : {code}")]
struct ContentError{
    content : String,
    code: i32
}

#[derive(Error, Debug)]
#[error("Errore di elaborazione")]
enum ProcessingError{
    /*caro rust se ti passa tra mani un FILE ERROR devi creare un PROCESSING ERROR, usa FROM
    quel file error diventa un processing error che ha come valore di ENUM il campo file.
    se hai bisogno processing error partendo da un content error -> stessa cosa
    FROM -> implementazione automatica del Trato From che converte */
    File( #[from] FileError),
    Content(  #[from] ContentError)
}

fn leggi_file(filename:&str) -> Result<String, FileError>{ //il tipo file error inizialmente non essite
    if filename.len() <3 { return Err(FileError("Nome file troppo corto"))}
    else if filename.len()>5 {return Err(FileError("Nome file troppo lungo"))}
    else {return Ok("contenuto del FILE".to_string())}
}

fn elabora_contenuto(content: String) -> Result<i32, ContentError>{
    if content.len() >3 {
        return Err(ContentError{content: content.clone(), code: 1});
    }
   return Ok(1);
}

/*torna un RESULT: legge file -> se va bene quello che lo ha letto lo passa a elabora contenuto , se
va bene alla fine da intero; che tipo di ERRORE PUO SUCCEDERE? N errori,
1) se non sono riuscito a leggere il file -> ERRORE su file error
2) se invece ho letto il file, ma il contenuto aveva problemi -> ERRROE su CONTENT ERROR
quindi ho bisogno di una classe ulteriore per rappresentare questi possibili problemi -> ENUM
in questo ENUM inserisco entrambi gli errori con i #FROM e introduci dei nomi tuoi il cui tipo
sono gli ENUM degli errori specifici con la clausula FROM, cosi implementano in auto il tratto from*/
fn combina_azioni(filename: &str) -> Result<i32, ProcessingError>{
    let c = leggi_file(filename)?; //puo andare male -> restituirebbe un FILE ERROR se ho errore
    let r = elabora_contenuto(c)?;  // può andare male -> restituirebbe un CONTENT ERROR se ho errore
    Ok(r) // se tutto va bene torno r
}

fn main() {
    //verifico che leggi file funzioni o no
    match leggi_file("abcd"){
        Ok(content) => println!("il file contiene: {}", content),
        Err(e) => println!("c'è stato un problema: {}", e)
    }

    match leggi_file("abcd"){
        Ok(content) => println!("il file contiene: {}", content),
        Err(e) => println!("c'è stato un problema: {}", e)
    }

    match elabora_contenuto("qwerty".to_string()){
        Ok(val) => println!("il contenuto è stato elaborato con successo: {}", val),
        Err(e) => println!("c'è stato un problema: {}", e)
    }

    match combina_azioni("abcdef"){
        //in questo modo posso combinare 2 possibili errori diversi
        Ok(content) => println!("il file contiene: {}", content),
        Err(e) => println!("c'è stato un problema: {}", e),
    }
}
