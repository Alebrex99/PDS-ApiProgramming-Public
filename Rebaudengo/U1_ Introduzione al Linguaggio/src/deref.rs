fn main() {
    let v = 32;
    let p = &v;
    let pp = &p;
    let ppp = &pp;

    let str = ppp.to_string();

    println!("{str}");
}