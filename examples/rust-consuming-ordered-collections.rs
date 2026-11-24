use std::collections::{BTreeMap, BTreeSet};

fn print_entry(key: String, value: String) {
    println!("{} {}", key, value);
}

fn main() {
    let empty: BTreeSet<u8> = BTreeSet::new();
    for item in empty {
        println!("{}", item);
    }

    let mut map: BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("z"), String::from("last"));
    map.insert(String::from("a"), String::from("first"));
    for (key, value) in map.into_iter() {
        print_entry(key, value);
    }

    let mut set: BTreeSet<u8> = BTreeSet::new();
    set.insert(7u8);
    set.insert(1u8);
    for item in set {
        println!("{}", item);
        break;
    }
    println!("done");
}
