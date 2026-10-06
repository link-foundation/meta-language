#[macro_use]
mod macros;
mod util;

use util::shapes::Square;
use util::{double, fetch, sum_to};

fn quadruple(n: u64) -> u64 {
    double(double(n))
}

fn area(square: &Square) -> u64 {
    square.side * square.side
}

async fn load() -> u64 {
    fetch().await + sum_to(3)
}

const _: () = assert!(double(2) == 4);

fn main() {
    let six = twice!(3);
    let square = Square { side: 2 };
    drop(load());
    assert_eq!(quadruple(1), double(2));
    println!("{} {} {}", quadruple(six), area(&square), sum_to(4));
}
