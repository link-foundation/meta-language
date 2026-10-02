fn flip(x: i64) -> i64 {
    -x
}

fn main() {
    let low: i64 = -9223372036854775807 - 1;
    println!("flip 5 = {}", flip(5));
    println!("flip low = {}", flip(low));
}
