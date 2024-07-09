fn main() {
    let range = 1..10;
    let f = || range.count();
    let n1 = f();
    let n2 = f();
}
