#[derive(Debug)]
struct S {
    i: i32,
}
//SOLUZIONE: ANCHE CON DERIVE
impl Clone for S{
    fn clone(&self) -> Self{
        S{i: self.i}
    }
}
impl Copy for S{}
pub fn driver_es1() {
    //Es1 Modificare la struct S, aggiungendo i tratti, affinchè compili il codice
    let mut a = Vec::<S>::new();
    let b = S { i: 1 };
    for i in 0..10 {
        a.push(b);
    }
}
