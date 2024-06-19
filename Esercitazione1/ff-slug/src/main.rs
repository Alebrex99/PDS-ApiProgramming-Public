use clap::Parser;

const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";


// with ieratators
fn conv_alt(c: char) -> char {
    match c {
        'a'..='z' | '0'..='9' => c,
        _ => match SUBS_I.chars().position(|x|x == c ) {
            Some(i) => SUBS_O.chars().nth(i).unwrap(),
            None => '-',
        }
    }
}

// with for loop
fn  conv(c: char) -> char {
    match c {
        // use match syntax to match a range of characters
        'a'..='z' | '0'..='9' => c,
        _ => {
            for (i, x) in SUBS_I.chars().enumerate() { //ogni chars in SUBS_I può N byte UTF-8
                if x == c {
                    return SUBS_O.as_bytes()[i] as char; //sono sicuro che in SUBS0 ogni char è 1 byte (ASCII)
                }
            }
            '-'
        }
    }
}

fn slugify(s: &str) -> String {
    let mut slug = String::new();
    for c in s.chars() {
        // converting a char to lowercase may lead to one or more chars
        for c in c.to_lowercase() {
            let c = conv(c);
            if c != '-' {
                slug.push(c);
            } else if !slug.ends_with('-') {
                slug.push('-');
            }
        }
    }
    if slug.ends_with('-') && slug.len() > 1 {
        slug.pop();
    }
    slug
}

///Programma che prende in input una stringa e la trasforma in uno slug
#[derive(Debug, Parser)]
struct Args {
    input: String,

    #[arg(short, long, default_value="false")]
    verbose: bool,

    #[arg(short, long)]
    repeat: Option<u32>
}


fn main() {

    let args: Args = Args::parse();

    let res =  slugify(&args.input);
    
    match args.repeat {
        Some(n) => {
            for i in 0..n {
                if args.verbose {
                    println!("[{}] {} => {}", i, args.input, res);
                } else { 
                    println!("{}", res);
                }
            }
        },
        None => if args.verbose {
            println!("{} => {}", args.input, res)
        } else {
            println!("{}", res)
        }
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn single_char_with_accent() {
        assert_eq!(super::slugify(&"ò"), "o");
    }

    #[test]
    fn single_char_without_accent() {
        assert_eq!(super::slugify(&"x"), "x");
    }

    #[test]
    fn single_invalid_char() {
        assert_eq!(super::slugify(&"/"), "-");
    }

    #[test]
    fn single_unknown_accent() {
        assert_eq!(super::slugify(&"ῶ"), "-");
    }

    #[test]
    fn multiple_words() {
        assert_eq!(super::slugify(&"Hello World"), "hello-world");
    }
    #[test]
    fn multiple_words_with_accents() {
        assert_eq!(super::slugify(&"Così è uno slug!"), "cosi-e-uno-slug");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(super::slugify(&""), "");
    }

    #[test]
    fn multiple_spaces() {
        assert_eq!(super::slugify(&"Hello   World!!"), "hello-world");
    }

    #[test]
    fn only_special_chars() {
        assert_eq!(super::slugify(&"!@#$%^&*()"),"-");
    }

    #[test]
    fn multiple_spaces_at_the_end() {
        assert_eq!(super::slugify(&"Hello  World!!  "), "hello-world");
    }

    #[test]
    fn multiple_invalid_chars_at_the_end() {
        assert_eq!(super::slugify(&"Hello  World/\\"), "hello-world");
    }
}
