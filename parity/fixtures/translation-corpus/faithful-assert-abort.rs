fn f(n: u64) -> u64 { n + 1 }
fn main() {
    let a = f(1);
    assert_eq!(a, 2);
    assert!(f(2) > a && a != 0);
    assert_ne!(f(3), 0);
    println!("{}", a);
}
