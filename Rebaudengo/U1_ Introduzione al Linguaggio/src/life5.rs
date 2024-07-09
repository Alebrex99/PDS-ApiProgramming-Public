fn main() {
    {
        let mut v = Vec::new();
        {
            let a = 1;
            v.push(&a);
        }
        println!("{:?}", v);
    }

}