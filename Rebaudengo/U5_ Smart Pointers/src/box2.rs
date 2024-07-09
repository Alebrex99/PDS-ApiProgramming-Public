// Definizione di una struttura per un albero binario
#[derive(Debug)]
enum BinaryTree {
    Empty,
    Node(i32, Box<BinaryTree>, Box<BinaryTree>),
}
impl BinaryTree {
    // Metodo per creare un nuovo albero binario
    fn new() -> Self { BinaryTree::Empty }
    // Metodo per inserire un valore nell'albero binario
    fn insert(&mut self, value: i32) {
        match *self {
            BinaryTree::Empty => *self = BinaryTree::Node(value, Box::new(BinaryTree::Empty), Box::new(BinaryTree::Empty)),
            BinaryTree::Node(ref mut data, ref mut left, ref mut right) => {
                if value <= *data {
                    left.insert(value);
                } else {
                    right.insert(value);
                }
            }
        }
    }
}
fn main() {
    // Creazione di un nuovo albero binario
    let mut tree = BinaryTree::new();

    // Inserimento di alcuni valori nell'albero binario
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    println!("{:?}", tree);
}

