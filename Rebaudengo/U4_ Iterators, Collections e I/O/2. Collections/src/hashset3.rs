use std::collections::HashSet;

fn main() {
    // Primo HashSet
    let set1: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
    // Secondo HashSet
    let set2: HashSet<i32> = [3, 4, 5, 6, 7].iter().cloned().collect();

    // Union: Unione dei due HashSet
    let union: HashSet<_> = set1.union(&set2).cloned().collect();
    println!("Unione dei due set: {:?}", union); // Stampa: Unione dei due set: {1, 2, 3, 4, 5, 6, 7}

    // Intersection: Intersezione dei due HashSet
    let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
    println!("Intersezione dei due set: {:?}", intersection);
    // Stampa: Intersezione dei due set: {3, 4, 5}

    // Difference: Elementi presenti solo in set1
    let difference1: HashSet<_> = set1.difference(&set2).cloned().collect();
    println!("Elementi presenti solo in set1: {:?}", difference1);
    // Stampa: Elementi presenti solo in set1: {1, 2}

    // Difference: Elementi presenti solo in set2
    let difference2: HashSet<_> = set2.difference(&set1).cloned().collect();
    println!("Elementi presenti solo in set2: {:?}", difference2);
    // Stampa: Elementi presenti solo in set2: {6, 7}

    // Symmetric Difference: Elementi presenti solo in uno dei due set
    let symmetric_difference: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
    println!("Elementi presenti solo in uno dei due set: {:?}", symmetric_difference);
    // Stampa: Elementi presenti solo in uno dei due set: {1, 2, 6, 7}
}

