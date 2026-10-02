fn fact(n: u64) -> u64 {
    if n == 0 { 1 } else { n * fact(n - 1) }
}

fn main() {
    println!("fact 20 = {}", fact(20));
    println!("fact 21 = {}", fact(21));
    println!("never printed");
}
