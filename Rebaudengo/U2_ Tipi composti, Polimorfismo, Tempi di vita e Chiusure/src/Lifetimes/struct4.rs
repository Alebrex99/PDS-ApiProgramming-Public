
#[derive(Debug)]
struct User<'a> {
    id: u32,
    name: &'a  str,
}

struct Data<'a> {
    user: User<'a>,
    password: String,
}

fn main() {
    let user = User { id: 1, name: "Alice" };
    let data = Data { user, password: String::from("password123") };

    println!("Data: {:?}, {:?}", data.user, data.password);
}