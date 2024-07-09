fn main() {

    for index in 0 .. 10 {
        println!("Index:{}", index);
        let s: &str = match index {
            0 ..= 4 => { "\tI'm in the first half" },
            5 => { "\tI'm in the middle" },
            _ => { "\tI'm in the second half..." }
        };
        println!("{}", s);
    }
}