fn cambia (myref: & mut Box<i32>) -> &mut Box<i32>
{
    *myref = Box::new(200);
    myref
}

fn main() {

    let mut mybox = Box::new(150);

    let mut z = &mut mybox;

    *z = Box::new(100);

    z = cambia(z);

    let newref = & *z;
    let secondref = newref;

    println!("{:?}", newref);
    println!("{:?}", secondref);

    println!("{:?}", mybox);
}
