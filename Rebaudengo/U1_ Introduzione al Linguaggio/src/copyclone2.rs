fn main() {

    let mut v: Vec<u8> = vec![1, 2, 3]; // Vec<u8> implements Clone, but not Copy

    let mut w = v.clone();    // This would move the value, rendering v unusable

    v[0] = 10;
    w[1] = 50;

    println!("v={:?} w={:?}", v, w);
}