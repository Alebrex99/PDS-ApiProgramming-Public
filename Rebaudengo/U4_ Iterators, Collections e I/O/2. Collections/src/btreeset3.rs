use std::collections::BTreeSet;

fn main() {
    // Creare alcuni BTreeSet di esempio
    let set_a: BTreeSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set_b: BTreeSet<i32> = vec![2, 3, 4].into_iter().collect();

    // Calcolare l'unione di set_a e set_b
    let union_result: BTreeSet<_> = set_a.union(&set_b).cloned().collect();
    println!("Unione: {:?}", union_result);

    // Calcolare l'intersezione di set_a e set_b
    let intersection_result: BTreeSet<_> = set_a.intersection(&set_b).cloned().collect();
    println!("Intersezione: {:?}", intersection_result);

    // Calcolare la differenza tra set_a e set_b
    let difference_result: BTreeSet<_> = set_a.difference(&set_b).cloned().collect();
    println!("Differenza: {:?}", difference_result);
}