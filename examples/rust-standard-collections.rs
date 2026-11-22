use core::clone::Clone as Duplicate;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

fn main() {
    let mut scores: BTreeMap<u8, u64> = BTreeMap::new();
    scores.insert(3u8, 9007199254740993u64);
    scores.insert(1u8, 4u64);
    let original = Duplicate::clone(&scores);
    assert_eq!(scores, original);
    let previous = scores.insert(1u8, 8u64);
    assert_eq!(previous, Some(4u64));
    assert_ne!(scores, original);
    let removed = scores.remove(&1u8);
    assert_eq!(removed, Some(8u64));
    assert_eq!(scores.get(&3u8).copied(), Some(9007199254740993u64));

    let mut left: BTreeSet<u8> = BTreeSet::new();
    let mut right: BTreeSet<u8> = BTreeSet::new();
    left.insert(5u8);
    left.insert(2u8);
    right.insert(3u8);
    assert!(left < right);
    assert_eq!(left.iter().copied().collect::<Vec<u8>>(), vec![2u8, 5u8]);
    let mut labels: BTreeSet<String> = BTreeSet::new();
    let label = String::from("label");
    labels.insert(label);
    assert!(labels.contains("label"));
    println!("{}", scores.len());
}
