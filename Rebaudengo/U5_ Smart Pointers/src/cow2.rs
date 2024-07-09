use std::borrow::Cow;

fn modify_if_condition(content: Cow<str>) -> Cow<str> {
    if content.len() < 10 {
        Cow::Owned(content.into_owned().to_lowercase())
    } else {
        content
    }
}

fn main() {
    let long_string = String::from("This is a long string");
    let short_string = String::from("SHORT");

    let modified_long = modify_if_condition(Cow::from(&long_string));
    let modified_short = modify_if_condition(Cow::from(&short_string));

    println!("{}", modified_long); // Stampa "This is a long string"
    println!("{}", modified_short); // Stampa "short"
}


