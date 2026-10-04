// Portable-core conformance project: control flow, recursion, data types,
// modules, effects, and proofs in one Rust program. Arithmetic is checked:
// the program is compiled with overflow checks, so overflow aborts.

mod arith {
    pub fn fact(n: u64) -> u64 {
        if n == 0 {
            1
        } else {
            n * fact(n - 1)
        }
    }

    pub fn sum_to(n: u64) -> u64 {
        match n {
            0 => 0,
            k => sum_to(k - 1) + k,
        }
    }

    pub fn fib(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib(n - 1) + fib(n - 2),
        }
    }

    pub fn monus(a: u64, b: u64) -> u64 {
        if a > b {
            a - b
        } else {
            0
        }
    }

    pub fn halve(x: i64) -> i64 {
        x.div_euclid(2)
    }

    pub fn remainder(x: i64, y: i64) -> i64 {
        if y == 0 {
            x
        } else {
            x.rem_euclid(y)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree {
    Leaf,
    Node(Box<Tree>, u64, Box<Tree>),
}

mod tree {
    use super::Tree;

    pub fn size(t: &Tree) -> u64 {
        match t {
            Tree::Leaf => 0,
            Tree::Node(l, _, r) => size(l) + 1 + size(r),
        }
    }

    pub fn total(t: &Tree) -> u64 {
        match t {
            Tree::Leaf => 0,
            Tree::Node(l, v, r) => total(l) + *v + total(r),
        }
    }

    pub fn mirror(t: &Tree) -> Tree {
        match t {
            Tree::Leaf => Tree::Leaf,
            Tree::Node(l, v, r) => Tree::Node(Box::new(mirror(r)), *v, Box::new(mirror(l))),
        }
    }

    pub fn insert(t: &Tree, x: u64) -> Tree {
        match t {
            Tree::Leaf => Tree::Node(Box::new(Tree::Leaf), x, Box::new(Tree::Leaf)),
            Tree::Node(l, v, r) => {
                if x < *v {
                    Tree::Node(Box::new(insert(l, x)), *v, r.clone())
                } else {
                    Tree::Node(l.clone(), *v, Box::new(insert(r, x)))
                }
            }
        }
    }
}

fn classify(n: i64) -> String {
    if n < 0 {
        String::from("negative")
    } else if n == 0 {
        String::from("zero")
    } else {
        String::from("positive")
    }
}

fn describe(n: u32) -> String {
    let doubled = n + n;
    let label = classify(i64::from(doubled) - 10);
    format!("{n} doubled is {doubled} ({label})")
}

fn sample() -> Tree {
    tree::insert(&tree::insert(&tree::insert(&tree::insert(&Tree::Leaf, 5), 2), 8), 3)
}

fn main() {
    assert_eq!(arith::fact(5), 120);
    println!("fact 20 = {}", arith::fact(20));
    println!("sumTo 100 = {}", arith::sum_to(100));
    println!("fib 25 = {}", arith::fib(25));
    println!("monus 3 5 = {}", arith::monus(3, 5));
    println!("halve -7 = {}", arith::halve(-7));
    println!("remainder -7 3 = {}", arith::remainder(-7, 3));
    println!("remainder 7 0 = {}", arith::remainder(7, 0));
    let t = sample();
    println!("size = {}, total = {}", tree::size(&t), tree::total(&t));
    println!("mirrored total = {}", tree::total(&tree::mirror(&t)));
    println!("{}", describe(3));
    println!("{}", describe(7));
    println!("{}", classify(-4));
}
