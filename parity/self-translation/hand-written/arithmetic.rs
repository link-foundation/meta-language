// The hand-written Rust the decorated translation of sources/arithmetic.mjs
// matches: parity/self-translation/decorators.lino rewrites the lines in which
// the generic translation differs from it.

/// The sum of `a` and `b`.
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Whether `name` names Rust.
pub fn is_rust(name: &str) -> bool {
    name == "Rust"
}
