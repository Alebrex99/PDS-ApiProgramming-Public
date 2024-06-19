struct prova{
    x: i32,
    y: i32,
}

struct S(i32);
/*potrei creare oggetti come S(10) e S(20) e poi fare operazioni tra di essi
a tale struct associo un paio di metodi per far vedere come si possa fare*/
impl S {
    /*i metodi hanno un parametro (&self) che dice "tu verrai chiamato
    su un oggetto di tale tipo, SELF di classe S, e lo riceverai come riferimento non mutabile"*/
    fn display(&self) {
        println!("S({}) @{:p}", self.0, self); //stampa ciò che ho in S e l'indirizzo di S
    }
}
//DISTRUTTORE :
/*Aggiungiamo un distruttore a Struct S, che faccia vedere quando cessa di esistere; come in C++ in cui
se scrive TILDE nome_classe, qui cosi. DROP è un tratto*/
impl Drop for S {
    fn drop(&mut self) { //la funzione drop predefinita-> riceve sempre obj che sta per esser distrutto come mutabile e ci fa cose
        println!("Dropping S({}) @{:p}", self.0, self);
    }
}

fn f(i: usize)-> usize{
    return i+10;
}

//Con tupla
fn f1(tupla: (i32,i32,i32)){
    println!("{}", tupla.0);
}

//RITORNO DI VARIABILI LOCALI/RIFERIMENTI
fn f_local() ->  i32 {
    let i = 5; //
    return i
}
fn f_ref<'a>() -> &'a i32{
    let a =5;
    let r = &a;
    //return r; //non pui tornare un riferimento ad una variabile locale che muore a fine funzione
    return &0;
}

static mut S_GLOBAL: S = S(10);
const S_CONST: S = S(10);
fn main() {
    //-----------------MEMORIA HEAP + STACK + DROP --------------------------
    //let s = S(3); //variabile locale
    //s.display(); //prendeva parametro &self, ce lo ha ma siccome è self lo ha prima con s. = &self
    /*in JAVA quando scrivo s.toString(), esso ha un this implicito; Qui si scrive esplictiamente SEFL
    perchè si può passare in tanti modi, qui lo passiamo NON MODIFICABILE = NON MUT, perchè display non cambia nulla*/

    //VARIABILI LOCALI + DINAMICHE (BOX)
    println!("------------VARIABILI LOCALI + DINAMICHE (BOX)-----------");
    let s1 = S(1); //variabile locale
    let s2 = Box::new(S(2)); //crei un BOX, quindi spazio su heap, e ci metti S(2)
    s1.display(); //stampa S(1) @0x7ffedf7f3f90 -> indirizzo su stack
    s2.display(); //stampa S(2) @0x55f7b3b1b2a0 -> indirizzo su heap

    //VARIABILI GLOBALI + COSTANTI
    unsafe {
        S_GLOBAL.display();
        S_GLOBAL.0 = 11;
        S_GLOBAL.display();
        println!("S_GLOBAL è una variabile globale, posta in zona {:p}", &S_GLOBAL);
    } //variabile GLOBALE in RUST = static

    S_CONST.display(); //è una costante
    println!("S_CONST è una costante, posta in zona {:p}", &S_CONST);



    //-----------------------TUPLE----------------------------------
    let tupla = (1, "ciao", true);
    tupla.0;


    //-----------------------ARRAY/SLICE/VEC----------------------------
    println!("---------------ARRAY-SLICES-VEC----------------------");
    //ARRAY
    let mut array: [i32;5] = [1,2,3,4,5];
    let arr_ref = &array; //riferimento a array, non slice

    //SLICE MUTABLE
    let mut a:[i32; 4] = [1,2,3,4]; //array di 4 elementi
    let slice = &mut a[1..3]; //prendi un riferimento a un sottoinsieme di a, da 1 a 3
    //a[2] = 3; //value borrowed da slice
    let terzo = slice.get(0);
    println!("SLICE : {:?}", terzo.unwrap());
    //println!("{:?}", slice); //stampa [2,3] -> error : durante la vita di slice, a[2] è stato modificato
    //println!("{}", a[f(10)]);//error : index out of bounds --> PANIC!!
    /*in C dove non c'è consapenvolezza lunghezza ti fa accedere, qui invece essendo ARRAY un tipo di dato
    che sa la sua lunghezza, se fai un errore ti blocca e ti dice dove è l'errore e perchè
    in C magari accede al dato, lo stampa e non sai cosa è successo*/

    //SLICE IMMUTABLE
    let slice_imm = &a[1..=3];
    let slice_copy = slice_imm;
    println!("{:?}", slice_imm);

    //VEC
    let mut vec = Vec::new();
    vec.push(1);
    let vec_ref = &mut vec; //riferimento a vec = diventa una slice in memoria
    vec_ref[0] = 10;
    vec[0] = 20;

    usize::MAX; //massimo valore di un usize
    let vec_withCap: Vec<i32> = Vec::with_capacity(10);
    let vec_macro = vec![1,2,3];


    //--------------------------STRINGHE---------------------------
    println!("-----------------STRINGHE---------------------");
    //STRINGHE &STR IMMUTBILI
    let s = "hello"; //Slice di byte = crea una stringa costante = area statica
    //come accedo ad un char della costante?
    let c = s.chars();
    println!("{:?}", c); //stampa h

    //STIRNGHE DINAMICHE MUTABILI
    let mut s1 = String::from("hello"); //crea una stringa modificabile
    //let s2 : str ; //Error : tipo primitivo str (non usabile direttamente, ma visto in realtà sempre come &str
    for (n, char) in s1.chars().enumerate(){
        println!("{} : {}", n, char);
    }
    let s1_ref:& str = &s1; //riferimento a s1
    for char in s1_ref.chars(){ //lunghezza in CHAR
        println!("{}", char);
    }


    //---------------------ESPRESSIONI E ISTRUZIONI---------------------
    println!("-----------------ESPRESSIONI E ISTRUZIONI------------------");
    let i ={
        println!("Hello");
        43
    };
    println!("{}", i);

    //MATCH
    let values = [1,2,3,4,5, 6,7,8,9,10,11,12];
    let slice_match = &values[..];
    match slice_match{
        &[0,..] => println!("0.."),
        &[1,2,.., finale @ 12] => println!("inizia con 1,2.. poi finisce con {}", finale),
        _ => println!("altro")
    }


    //------------------MOVIMENTO/PRESTITO -RIFERIMENTI-----------------------------
    /*let s1 = S(1); //variabile locale
    s1.display();
    let s2 = s1; //s1 è stato spostato in s2, quindi s1 non esiste più
    s2.display();
    s1.display();*/ //error : value borrowed here after move

    /*let mut s2 = s1;
    s2.0 = 7;
    s2.display();*/

    //RIFERIMENTO IMMUTABILE = PRESTITO IMMUTABILE
    let s = String::from("hello"); //crea una stringa vuota
    let r = &s; //crea un riferimento immutabile a s -> puoi solo READ s (no MUOVERLO, no WRITE)
    //1) movimento/copia di riferimenti : si può sempre
    let r_copy = r;
    println!("{}", r_copy); //stampa hello

    //2) movimento di riferimenti e modifica originale
    let s_moved = s; //sposta s in s_moved
    //println!("{}", s); //error : value borrowed here after move

    //3) ritorno da una funzione
    let r_function = f_local();

    //RIFERIMENTO MUTABILE = PRESTITO MUTABILE
    let mut string = String::from("hello_mutable");
    let r1 = &mut string;
    let string_move = string; //sposta string in string_move
    //println!("{}", string); //error : value borrowed here after move

    //TRATTO COPY
    let mut var_copy: i32 = 10;
    let mut var_copy2 = var_copy;
    var_copy = 3;
    println!("{}", var_copy); //stampa 3
    println!("{}", var_copy2); //stampa 10
}
