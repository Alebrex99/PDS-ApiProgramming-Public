fn f<'a>(s: &'a str, v: &'a mut Vec<&'a str>) {
    v.push(s);
}

fn main() {
    let mut v: Vec<&str> = Vec::new();
    {
        let s= String::from("abc");
        v.push(&s);
        println!("{:?}",v);
    }
}