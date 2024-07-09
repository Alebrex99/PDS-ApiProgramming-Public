fn main() {

    let x: u8 = 123; // u8 implements Copy
    let y = x;       // x can still be used

    println!("x={}, y={}", x, y);

    let v: Vec<u8> = vec![1, 2, 3]; // Vec<u8> implements Clone, but not Copy

    let w = v;             // This would move the value, rendering v unusable

    println!("w={:?}", w);

}