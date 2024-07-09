use std::ops::Index;


struct MyVector<T> {
    data: Vec<T>,
    matricola: i32
}

impl<T> MyVector<T> {
    fn new(matr: i32) -> MyVector<T> {
        MyVector { data: Vec::new(),
                   matricola: matr}
    }

    fn push(&mut self, item: T) {
        self.data.push(item);
    }
}

impl<T> Index<usize> for MyVector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        print!("Accesso at index {} => ", index);
        &self.data[index]
    }
}

fn main() {
    let mut my_vector = MyVector::new(250_000);
    my_vector.push(23);
    my_vector.push(25);


    println!("{}", my_vector[0]);
    println!("{}", my_vector[1]);

}