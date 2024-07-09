use std::borrow::Cow;
fn abs_all<'a>(input: &'a mut Cow<'a, [i32]>)  -> Cow<'a, [i32]>{
    // Check if the input slice contains any negative values
    if input.iter().any(|&x| x < 0) {
        // If yes, create a new vector with the absolute values
        let mut output = Vec::new();
        for &x in input.iter(){
            output.push(x.abs());
        }
        // Return the vector as an owned slice
        print!("Owned => ");
        Cow::Owned(output)
    } else {
        // If no, return the input slice as a borrowed slice
        print!("Borrowed => ");
        Cow::Borrowed(input)
    }
}
fn main() {
    let slice = [0, 1, 2];
    let mut input = Cow::from(&slice[..]);
    let mut output = abs_all(&mut input); // No clone occurs because input doesn't need to be mutated.
    println!("{:?}", output);

    let slice = [-1, 0, 1];
    let mut input = Cow::from(&slice[..]); // Clone occurs because input needs to be mutated.
    output = abs_all(&mut input);       
    println!("{:?}", output);        

    let mut input = Cow::from(vec![-1, 0, 1]); // No clone occurs because input is already owned.
    output = abs_all(&mut input);
    println!("{:?}", output);
}

