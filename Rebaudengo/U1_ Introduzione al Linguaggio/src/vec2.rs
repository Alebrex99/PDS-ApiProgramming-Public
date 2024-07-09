#[derive(Debug)]
struct Test (i32);
impl Drop for Test {
    fn drop (& mut self) {
        println!("Destroying Test ({}) at address {:p}", self.0, self);
    }
}

fn main() {
    let mut v =Vec::<Test>::new();
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    v.push(Test(0));

    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());
    println!("&v[0]: {:p}", &v[0]);

    v.push(Test(1));
    v.push(Test(2));
    v.push(Test(3));
    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    v.push(Test(4));

    println!("ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());

    v.shrink_to_fit();
    println!("Shrinking ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());


    let a = v.remove(1);
    println!("Removing ptr: {:p} {} {}", v.as_ptr(), v.capacity(), v.len());
    println!("Removed {:?} ", a);

    println!("Terminating");
}
