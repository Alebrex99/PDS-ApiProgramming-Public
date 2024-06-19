fn main() {
    let i = 5u8; //IMMUTABILE
    let mut j = 6; //MUTABILE

    let R1 = &i;
    let R2 = &j;
    let R3 = &mut j;

    //let R_mut = &mut j;
    let R_im = &j;
    println!("R_im = {}", *R_im);
    println!("j = {}", j);
    println!("R_im = {}", *R_im);

    //println!("R_mut = {}", *R_mut);
    j=10;
    //println!("R_mut = {}", *R_mut);
    //error : quando prendi in prestito un valore j mutabile, poi non puoi ne copiarlo


    //println!("R1 = {}", *R1);
    //*R1 = 5; //error : var i is immutable
    //*R2 = 6; //error : var j is mutable, but R2 is immutable
    /*R3 = 7; //ok : var j is mutable, R3 is mutable
    let R4 = &mut j;
    *R4 = 90;
    println!("R4 = {}", R4);
    j= 18;
    println!("J = {}", j);*/

    println!("Hello, world!");
    /* This is a comment */
}
