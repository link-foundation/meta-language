pub mod shapes;

pub const fn double(n: u64) -> u64 {
    n + n
}

pub fn sum_to(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        n + sum_to(n - 1)
    }
}

pub async fn fetch() -> u64 {
    1
}
