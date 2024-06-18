pub fn is_valid_isbn(isbn: &str) ->bool {
    let mut t=0;
    let mut w = 10;
    for ch in isbn.chars(){
        match ch{
            '-' => continue,
            //quella X puo stare solo nell'ultima posizione, la prima avrebbe peso 10, ultima peso 1. la x la considero solo se w è 1
            'X' if w==1 => t+=10,
            /*se compreso tra 0 e 9, trasformo carattere in digit, accettando solo le cifre da 0 a 10, altrimenti se metto 16 : codice exa
            non restituiac eper forza, ma restituisce un OPTION; lo sbusto
            - se è un meno -> lasscio li
            - se è una x -> sommo a tot
            - se char da 0 a 9 lo sommo al resto
            se uso :
            - '0'..='9'=> ch.to_digit(10) .unwrap()*w    puo dare erore, devo esser sicuro di poterla fare , se qualcuno da stringa e noi
            continiamo a ciclRCI SOPRA , non puo andare a negativi perche è unsigned, ma andrebbe a 4 miliardi ecc*/
            '0'..='9'  => ch.to_digit(10).unwrap()*w,
            _ => return false
        }
        if w==0 {return false} //se qualcuno mi ha messo stringa troppo lunga , non rischio di far traboccare decremnto di w
        w -= 1;
    }
    t%11 ==0 && w==0;
    /*devo tornare una condizione bool che dipende dal mio risultato
    se costuisco stringa solo con 0 , trasfgorma in digit , moltiplica per 10, somma a tot , non trova altri chars esce da for , veriffca
    0%11 -> si fa 0, ma non è isbn valido, non basta allora che sia 0, ma deve essere anche w==0
    w parte da 10, se ho 10 chars, ad ogni cilco abbasso di 1, finisce a 0; ma se ne avessi di piu w non vale 0, potrebbe venire somma pesata
    ma non sarebbe giusto.
    stringa 0 con tale formula da somma peesata che da 0*/
}
