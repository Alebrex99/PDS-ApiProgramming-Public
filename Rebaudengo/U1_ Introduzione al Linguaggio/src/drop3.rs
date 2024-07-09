struct S(i32);  // struct che contiene un unico campo di tipo i32
impl S {
    fn display (&self) {    // metodo chiamato su un oggetto di S
                            // ricevendolo come riferimento non mutabile
            println!("Sono S e contengo {} @ {:p}", self.0, self);
                            // mi dice che cosa contiene S e dove è memorizzato
    }
}
impl Drop for S  {
    fn drop(&mut self)      // ha la possibilità di manipolare il dato
                        {
                            println!("Dropping S({}) @ {:p}", self.0, self);
                        }
                }
fn main() {
    let s1 = S(1);
    s1.display();
    let mut s2 = s1;

    s2.display();

    s2.0 = 7;
    s2.display();
}
