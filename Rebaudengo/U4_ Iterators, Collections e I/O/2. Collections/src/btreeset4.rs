use std::collections::BTreeSet;

fn main() {
    // Creiamo due HashSet di numeri interi
    let set1: BTreeSet<i32> = [3, 4, 5].iter().cloned().collect();
    let set2: BTreeSet<i32> = [3,4,5,9].iter().cloned().collect();

    // Verifichiamo se i due set sono disgiunti
    if set1.is_disjoint(&set2) {
        println!("I due set sono disgiunti");
    } else {
        println!("I due set non sono disgiunti");
    }

    // Verifichiamo se set1 è un sottoinsieme di set2
    if set1.is_subset(&set2) {
        println!("Il set1 è un sottoinsieme di set2");
    } else {
        println!("Il set1 non è un sottoinsieme di set2");
    }

    // Verifichiamo se set2 è un sovrainsieme di set1
    if set2.is_superset(&set1) {
        println!("Il set2 è un sovrainsieme di set1");
    } else {
        println!("Il set2 non è un sovrainsieme di set1");
    }
}