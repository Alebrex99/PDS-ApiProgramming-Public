trait Biography{
    fn biography(&self) -> & str;
}

struct Person<'a> {
    name: &'a str,
    age: u32,
    bio: &'a  str,
}

impl Biography for Person<'_>  {
    fn biography(&self) -> & str {
        self.bio
    }
}

fn main() {
    let name = "Alice";
    let age = 30;
    let bio = format!("{} is {} years old.", name, age);

    let person = Person { name, age, bio: &bio };

    println!("Biography: {}", person.biography());
}