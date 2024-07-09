struct ContatoreFibonacci {
    a: usize,
    b: usize,
    max: usize,
}
impl Iterator for ContatoreFibonacci {
    type Item = usize;
    // Definiamo il metodo next per ottenere il prossimo elemento
    fn next(&mut self) -> Option<Self::Item> {
        if self.a < self.max {
            let valore = self.a;
            let nuovo_valore = self.a + self.b;
            self.a = self.b;
            self.b = nuovo_valore;
            Some(valore)
        } else {
            None
        }
    }
}
fn main() {
    let fibonacci_iteratore = ContatoreFibonacci { a: 0, b: 1, max:1000 };
    for numero in fibonacci_iteratore {
        println!("Fibonacci: {}", numero);
    }
}

