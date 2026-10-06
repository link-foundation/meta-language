//! Geometry the portable core translates, beside items it carries.
use std::fmt;

// The squared distance of a point from the origin.
pub fn norm2(x: i64, y: i64) -> i64 {
    x * x + y * y
}

fn is_even(n: u32) -> bool {
    n % 2 == 0
}

pub struct Point {
    x: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.x)
    }
}
