fn take(a: u64, b: u64) -> u64 {
    a - b
}

fn main() {
    println!("take 5 3 = {}", take(5, 3));
    println!("take 3 5 = {}", take(3, 5));
}
