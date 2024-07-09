use std::ops::{Deref, DerefMut};

// Definiamo una struttura MyBox che contiene un valore
#[derive(Debug)]
struct MyBox<T>(T);

// Implementiamo Deref per MyBox
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Implementiamo DerefMut per MyBox
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut my_box = MyBox(5);

    // Usando deref coercion, Rust converte automaticamente &mut MyBox<i32> a &mut i32
    *my_box += 1;

    println!("Valore dopo l'incremento: {:?}", my_box);
}