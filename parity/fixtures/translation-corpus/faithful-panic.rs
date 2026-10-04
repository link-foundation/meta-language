fn checked(n: u64) -> u64 {
    if n > 10 { panic!("n is too large") } else { n + 1 }
}

fn main() {
    println!("checked 3 = {}", checked(3));
    println!("checked 11 = {}", checked(11));
}
