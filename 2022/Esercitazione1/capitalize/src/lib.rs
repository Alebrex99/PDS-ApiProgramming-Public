//puoi mettere direttamente i capitalize qui dentro

pub fn capitalize(s: &str) -> String{
    let mut state =true; //indice per capire se prima lettera parola
    //codifica : creazione stringa nuova per sostituzione char maiuscolo, devo appendere i chars
    let mut s_finale = String::new();
    let mut s1 = String :: new();

    for ch in s.chars() {
        match ch{
            ' ' => {state = true ; s_finale.push(ch)}, //trovo uno spazio
            //' ' => s_finale.push(' '),
            ch => if state==true{
                //s_finale.push(' ');
                s1 = ch.to_uppercase().to_string();
                s_finale.push_str(&s1);
                state =false
            } //dopo ho char e converto
            else {s_finale.push(ch);},
            _ => state = false
        }

    }
    return s_finale.trim().to_string();
}
