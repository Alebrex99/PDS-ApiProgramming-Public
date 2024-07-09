fn main() {
    let mut v = vec!["a".to_string(), "b".to_string(), "c".to_string()];

    let it = v.iter_mut();
    for s in it { s.push('1'); }
    let it = v.iter();
    for s in it {
        println!("{:?} ", s)
    }

    let v2 = vec![1, 2, 3, 4, 5];
    let it2 = v2.into_iter();
    let mut sum = 0;
    for n in it2 {
        sum += n;
    }
    println!("{:?}", sum);

}


