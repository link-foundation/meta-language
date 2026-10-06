fn pick(n: u64) -> u64 {
    match n {
        0 => 10,
        1 => 20,
        _ => unreachable!("only 0 and 1 are picked"),
    }
}

fn main() {
    println!("pick 1 = {}", pick(1));
    println!("pick 2 = {}", pick(2));
}
