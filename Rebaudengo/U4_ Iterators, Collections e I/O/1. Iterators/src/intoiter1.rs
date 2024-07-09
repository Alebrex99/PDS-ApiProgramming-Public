fn main() {

    let v = vec![10.0, 30.0, 50.0, 90.0];

    let mut sum=f64::default();

    // into_iter consuma il contenitore
    for num in v.into_iter() {
        sum += num;
    }
    println!("{}", sum);

    let v2 = vec![1, 3, 5, 9];

    let mut sum2=i32::default();

    for num2 in v2 {
        sum2 += num2;
    }

    println!("{}", sum2);
}
