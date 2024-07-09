trait Confrontabile<T> {
    fn confronta(&self, other: &T) -> bool;
}

// Implementazione del tratto Confrontabile per i tipi di dati numerici
impl<T> Confrontabile<T> for T
    where
        T: std::cmp::PartialOrd,
{
    fn confronta(&self, other: &T) -> bool {
        self >= other
    }
}

fn main() {    
 let x = 5;    
 let y = 10;    
 println!("x è maggiore o uguale a y? {}", x.confronta(&y));
}
