//use Esercitazione1::capitalize::capitalize;
use capitalize::capitalize;


#[cfg(test)]
mod tests {
    use capitalize::capitalize;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_capitalize() {
        let mut s1 = "stringa con più di una parola".to_string();
        let s2 = "solounaparola".to_string();
        let s3 = "ìnizia con caratteri àaccentati".to_string();
        let s4 = "  ".to_string();
        let s5 = "stringa    con più spazi  ".to_string();

        let res1 = capitalize(&s1);
        let res2 = capitalize(&s2);
        let res3 = capitalize(&s3);
        let res4 = capitalize(&s4);
        let res5 = capitalize(&s5);


        assert_eq!(res1, "Stringa Con Più Di Una Parola");
        assert_eq!(res2, "Solounaparola");
        assert_eq!(res3, "Ìnizia Con Caratteri Àaccentati");
        assert_eq!(res4, "  ");
        assert_eq!(res5, "Stringa    Con Più Spazi  ");
    }
}


