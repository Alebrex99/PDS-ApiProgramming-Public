fn main() {
    let a = 2;

    let p = &mut 12;
    *p = *p + 1;

    let p1 = & (a + 2);

    let p2 = &(*p1 * 2);

    println!("{} {} {}", *p, *p1, *p2);
}