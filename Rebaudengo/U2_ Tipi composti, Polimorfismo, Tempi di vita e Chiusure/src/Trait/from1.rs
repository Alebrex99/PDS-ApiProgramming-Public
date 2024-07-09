#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Point {
    fn from((x, y): (i32, i32)) -> Self {
        Point { x, y }
    }
}

impl From<[i32; 2]> for Point {
    fn from([x, y]: [i32; 2]) -> Self {
        Point { x, y }
    }
}
fn main() {
    // from
    let p1 = Point::from((3, 1));
    let p2 = Point::from([5, 2]);

    // into
    let p3: Point = (1, 3).into();
    let p4: Point = [4, 0].into();

    println!("p1 {:?}\np2 {:?}\np3 {:?}\np4 {:?}", p1, p2, p3, p4);

    // ERRORE! non vale la simmetria
    //let a1 = <[i32; 2]>::from(point);
    //let a2: [i32; 2] = point.into();
    //let t1 = <(i32, i32)>::from(point);
    //let t2: (i32, i32) = point.into();
}