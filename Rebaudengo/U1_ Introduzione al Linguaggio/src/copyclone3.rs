#[derive(Debug, Clone, Copy)]
pub struct PointCloneAndCopy {
      pub x: f64
}
#[derive(Debug, Clone)]
pub struct PointCloneOnly {
    pub x: f64
}
fn test_copy_and_clone() {
    let p1 = PointCloneAndCopy { x: 0. };
    let p2 = p1; // because type has Copy, it gets copied automatically.
    println!("{:?} {:?}", p1, p2);
}
fn test_clone_only() {
    let p1 = PointCloneOnly { x: 0. };
    let p2 = p1; // because type has no Copy, this is a move instead.
    println!("{:?} {:?}", p1, p2);
}
pub fn main() {
    test_copy_and_clone();
    test_clone_only();
}
