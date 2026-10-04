fn flip(x: i64) -> i64 {
    -x
}

fn split(x: i64, y: i64) -> i64 {
    x % y
}

fn main() {
    let low: i64 = -9223372036854775807 - 1;
    println!("flip 5 = {}", flip(5));
    println!("split -7 2 = {}", split(-7, 2));
    println!("split low -1 = {}", split(low, -1));
}
