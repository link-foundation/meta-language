use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let mut set: BTreeSet<u8> = BTreeSet::new();
    set.insert(7u8);
    set.insert(1u8);
    for &item in &set {
        println!("{}", item);
        break;
    }
    set.insert(3u8);
    for item in set.iter() {
        println!("{}", *item);
    }

    let mut map: BTreeMap<u8, u8> = BTreeMap::new();
    map.insert(2u8, 7u8);
    map.insert(1u8, 8u8);
    for (&key, value) in map.iter() {
        println!("{} {}", key, *value);
    }
    for &key in map.keys() {
        println!("{}", key);
    }
    for &value in map.values() {
        println!("{}", value);
    }
    println!("{}", map.len());
}
