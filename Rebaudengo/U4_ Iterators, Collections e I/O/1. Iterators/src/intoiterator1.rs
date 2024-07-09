struct Pixel {
    r: i8,
    g: i8,
    b: i8,
}
impl IntoIterator for Pixel {
    type Item = i8;
    type IntoIter = std::array::IntoIter<i8, 3>;
    fn into_iter(self) -> Self::IntoIter {
        std::array::IntoIter::new([self.r, self.g, self.b])
    }
}
fn main() {
    let pixel = Pixel { r: 54, g: 23, b: 74 };
    let mut iter = pixel.into_iter();
    // Chiamare next restituirà il prossimo elemento dell'iteratore, se presente
    if let Some(component) = iter.next() {
        println!("Il primo componente è: {}", component);
    }
    // Puoi continuare a chiamare next per ottenere i successivi elementi
    if let Some(component) = iter.next() {
        println!("Il secondo componente è: {}", component);
    }
    if let Some(component) = iter.next() {
        println!("Il terzo componente è: {}", component);
    }
    // Se chiami next dopo che tutti gli elementi sono stati consumati, otterrai None
    if let Some(component) = iter.next() {
        println!("Questo non verrà stampato perché non ci sono più elementi.");
    } else {
        println!("Non ci sono più componenti.");
    }
}


