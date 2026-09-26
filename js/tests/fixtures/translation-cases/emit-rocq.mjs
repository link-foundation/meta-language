// Small programs that exercise every path of the Rocq emitter (helpers,
// encodings, numeric semantics, names, recursion forms, proof plans, main
// effects and its unsupported constructs), as name → [source extension,
// source text].

const leanTree = `inductive Tree where
  | leaf : Tree
  | node : Tree → Nat → Tree → Tree
`;
const leanMain = '\ndef main : IO Unit := do\n  IO.println "x"\n';
const rustTree = `#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree { Leaf, Node(Box<Tree>, u64, Box<Tree>) }
`;
const rustMain = '\nfn main() { println!("x"); }\n';
const rocqHead = 'From Stdlib Require Import NArith ZArith String Lia.\nOpen Scope string_scope.\n';
const rocqTree = 'Inductive Tree : Type :=\n| leaf : Tree\n| node : Tree -> nat -> Tree -> Tree.\n';
const jsHead = "import assert from 'node:assert/strict';\n";
const jsTree = "/**\n * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree\n */\n";
const jsFn = (name, params, ret, body) => `/**\n${params.map(([p, t]) => ` * @param {${t}} ${p}\n`).join('')} * @returns {${ret}}\n */\nfunction ${name}(${params.map(([p]) => p).join(', ')}) {\n${body}\n}\n`;

export default {
  // Helpers and printing: each textual conversion pulls in its helper, and
  // only the helpers a program uses are emitted.
  'emit-rocq-no-helpers': ['lean', 'def one : Nat := 1\n'],
  'emit-rocq-print-string': ['lean', 'def main : IO Unit := do\n  IO.println "hello"\n  IO.println ""\n'],
  'emit-rocq-print-nat': ['lean', 'def main : IO Unit := do\n  IO.println s!"{2 + 3}"\n'],
  'emit-rocq-print-int': ['lean', 'def main : IO Unit := do\n  IO.println s!"{(-3 : Int)}"\n'],
  'emit-rocq-print-bool': ['lean', 'def main : IO Unit := do\n  IO.println s!"{true && false}"\n'],
  'emit-rocq-print-all': ['lean', `def label (n : Nat) (i : Int) (b : Bool) (s : String) : String :=
  s!"{n} {i} {b} {s}" ++ toString n
def main : IO Unit := do
  IO.println (label 1 (-2) true "s")
  IO.println (toString "t")
`],
  'emit-rocq-string-quotes': ['lean', `def q : String := "say \\"hi\\" twice"
def main : IO Unit := do
  IO.println q
  IO.println "a \\"b\\""
`],
  'emit-rocq-string-comment-close': ['rs', `fn f(n: u64) -> u64 { if n == 0 { panic!("bad *) input") } else { n } }${rustMain}`],
  'emit-rocq-string-compare': ['lean', `def same (a b : String) : Bool := a == b
def differ (a b : String) : Bool := a != b
def glue (a b : String) : String := a ++ b
${leanMain}`],
  'emit-rocq-bool-ops': ['lean', `def f (a b : Bool) : Bool := (a && b) || !a
def g (a b : Bool) : Bool := a == b
def h (a b : Bool) : Bool := a != b
${leanMain}`],
  'emit-rocq-unit': ['lean', `def u : Unit := ()
def keep (x : Unit) : Unit := x
${leanMain}`],

  // Naturals: N with truncated subtraction and total division.
  'emit-rocq-nat-arith': ['lean', `def f (a b : Nat) : Nat := a + b * 2 - a / b + a % b
${leanMain}`],
  'emit-rocq-nat-compare': ['lean', `def f (a b : Nat) : Bool := a < b && a ≤ b && a > b && a ≥ b && a == b && a != b
${leanMain}`],
  'emit-rocq-nat-literal-big': ['lean', `def big : Nat := 123456789012345678901234567890
${leanMain}`],
  'emit-rocq-nat-monus-rocq': ['v', `${rocqHead}Definition monus (a b : nat) : nat := a - b.
Definition half (a : N) : N := N.div a 2.
Definition parity (a : N) : N := N.modulo a 2.
`],

  // Integers: Z with each rounding.
  'emit-rocq-int-euclid': ['lean', `def f (x y : Int) : Int := x / y + x % y
${leanMain}`],
  'emit-rocq-int-euclid-print': ['lean', `def main : IO Unit := do
  IO.println s!"{(-7 : Int) / 2} {(-7 : Int) % 2}"
`],
  'emit-rocq-int-negate': ['lean', `def f (x : Int) : Int := -x - (-4)
${leanMain}`],
  'emit-rocq-int-compare': ['lean', `def f (a b : Int) : Bool := a < b || a ≤ b || a > b || a ≥ b || a == b || a != b
${leanMain}`],
  'emit-rocq-int-floor': ['v', `${rocqHead}Definition f (x y : Z) : Z := (x / y + x mod y)%Z.
Definition g (x y : Z) : Z := Z.div x y + Z.modulo x y.
`],
  'emit-rocq-int-trunc-rocq': ['v', `${rocqHead}Definition f (x y : Z) : Z := Z.quot x y + Z.rem x y.
Definition g (x : Z) : Z := Z.opp x.
`],
  'emit-rocq-int-trunc-js': ['mjs', `${jsFn('quot', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a / b;')}${jsFn('rem', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a % b;')}console.log(quot(-7n, 2n));\n`],
  'emit-rocq-int-literals': ['v', `${rocqHead}Definition neg : Z := (-12)%Z.
Definition pos : Z := 12%Z.
Definition zero : Z := 0%Z.
`],

  // Casts between naturals and integers.
  'emit-rocq-cast-lean': ['lean', `def up (n : Nat) : Int := Int.ofNat n - 10
def down (i : Int) : Nat := Int.toNat i
def arrow (n : Nat) : Int := ↑n
def ascribe (n : Nat) : Int := (n : Int)
${leanMain}`],
  'emit-rocq-cast-rocq': ['v', `${rocqHead}Definition up (n : N) : Z := Z.of_N n.
Definition down (z : Z) : N := Z.to_N z.
Definition same (n : N) : N := N.of_nat (N.to_nat n).
`],
  'emit-rocq-cast-js-checked': ['mjs', `${jsFn('need', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('need expects a natural number');\n  return n;")}${jsFn('pass', [['m', 'bigint']], 'bigint', '  return need(m - 1n);')}console.log(pass(3n));\n`],
  'emit-rocq-cast-rust-widen': ['rs', `fn f(n: u32) -> i64 { i64::from(n) + (n as i64) }
fn g(n: u8) -> u32 { n as u32 }
fn h(n: i8) -> i32 { n as i32 }
fn k(n: u16) -> u64 { u64::from(n) }
${rustMain}`],

  // Machine integers: unbounded carriers under the non-aborting assumption.
  'emit-rocq-fixed-unsigned': ['rs', `fn f(a: u64, b: u64) -> u64 { a + b * 2 - a / b + a % b }${rustMain}`],
  'emit-rocq-fixed-signed': ['rs', `fn f(a: i32, b: i32) -> i32 { a + b * 2 - a / b + a % b }${rustMain}`],
  'emit-rocq-fixed-euclid': ['rs', `fn f(x: i64, y: i64) -> i64 { x.div_euclid(y) + x.rem_euclid(y) }
fn g(x: u32, y: u32) -> u32 { x.div_euclid(y) + x.rem_euclid(y) }
${rustMain}`],
  'emit-rocq-fixed-negate': ['rs', `fn f(x: i8) -> i8 { -x }
fn g() -> i8 { -128 }
fn h() -> i16 { 7 }
${rustMain}`],
  'emit-rocq-fixed-compare': ['rs', `fn f(a: u8, b: u8) -> bool { a < b && a <= b && a > b && a >= b && a == b && a != b }
fn g(a: i16, b: i16) -> bool { a < b || a == b }
${rustMain}`],
  'emit-rocq-fixed-literals': ['rs', `fn f() -> u8 { 255 }
fn g() -> i64 { -9 }
fn h() -> usize { 3 }
${rustMain}`],
  'emit-rocq-fixed-print': ['rs', `fn main() {
    let x: u8 = 7;
    let y: i32 = -3;
    println!("{} {}", x, y);
    println!("{}", -3i64);
    println!("{}", true);
}
`],
  'emit-rocq-fixed-print-unsigned-only': ['rs', `fn main() {
    let x: u16 = 7;
    println!("{}", x);
}
`],
  'emit-rocq-fixed-print-signed-only': ['rs', `fn main() {
    let x: i16 = -7;
    println!("{}", x);
}
`],
  'emit-rocq-fixed-recursion': ['rs', `fn f(n: u32) -> u32 { if n == 0 { 0 } else if n == 1 { 1 } else { f(n - 1) + f(n - 2) } }${rustMain}`],
  'emit-rocq-fixed-string': ['rs', `fn label(n: u32) -> String { format!("n={n}") }
fn signed(n: i32) -> String { format!("{}!", n) }
fn main() { println!("{}", label(3)); println!("{}", signed(-3)); }
`],

  // Aborts become an inhabitant of the expected type.
  'emit-rocq-abort-nat': ['mjs', `${jsFn('pred', [['n', 'bigint']], 'bigint', "  if (n < 1n) throw new RangeError('pred expects a positive number');\n  return n - 1n;")}console.log(pred(3n));\n`],
  'emit-rocq-abort-kinds': ['rs', `${rustTree}
fn a(n: u64) -> u64 { if n == 0 { panic!("zero") } else { n } }
fn b(n: i64) -> i64 { if n == 0 { unreachable!() } else { n } }
fn c(n: u64) -> bool { if n == 0 { todo!() } else { true } }
fn d(n: u64) -> String { if n == 0 { unimplemented!() } else { String::from("s") } }
fn e(n: u64) -> Tree { if n == 0 { panic!("tree") } else { Tree::Leaf } }
fn f(n: u64) -> u8 { if n == 0 { panic!("u8") } else { 1 } }
fn g(n: u64) -> i8 { if n == 0 { panic!("i8") } else { 1 } }
${rustMain}`],
  'emit-rocq-abort-data-first-leaf': ['rs', `pub enum Shape { Pair(Box<Shape>, Box<Shape>), Point(u64, bool), Empty }
fn f(n: u64) -> Shape { if n == 0 { panic!("shape") } else { Shape::Empty } }
${rustMain}`],
  'emit-rocq-abort-data-recursive-only': ['rs', `pub enum Wrap { Leaf(u64, i64, String, bool), More(Box<Wrap>) }
fn f(n: u64) -> Wrap { if n == 0 { panic!("wrap") } else { Wrap::Leaf(n, -1, String::from("w"), true) } }
${rustMain}`],
  'emit-rocq-abort-js-data': ['mjs', `${jsTree}${jsFn('pick', [['n', 'bigint']], 'Tree', "  if (n === 0n) throw new RangeError('no tree');\n  return { $: 'leaf' };")}`],
  'emit-rocq-abort-division-js': ['mjs', `${jsFn('f', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a / b + a % b;')}${jsFn('g', [['a', 'bigint']], 'bigint', "  if (a === 0n) throw new RangeError('zero');\n  return a;")}`],

  // Data across modules and constructor references.
  'emit-rocq-data-lean': ['lean', `${leanTree}
namespace Tree
def size : Tree → Nat
  | leaf => 0
  | node l _ r => size l + 1 + size r
def mirror : Tree → Tree
  | leaf => leaf
  | node l v r => node (mirror r) v (mirror l)
end Tree
def sample : Tree := Tree.node Tree.leaf 1 Tree.leaf
${leanMain}`],
  'emit-rocq-data-rust-modules': ['rs', `mod shapes {
    pub enum Shape { Dot, Line(u64), Box2(u64, u64) }
    pub fn area(s: &Shape) -> u64 {
        match s {
            Shape::Dot => 0,
            Shape::Line(n) => *n,
            Shape::Box2(w, h) => w * h,
        }
    }
}
mod geometry {
    pub mod deep {
        pub fn unit() -> crate::shapes::Shape { crate::shapes::Shape::Box2(1, 1) }
    }
    pub fn twice() -> u64 { crate::shapes::area(&deep::unit()) * 2 }
}
fn main() { println!("{}", geometry::twice()); }
`],
  'emit-rocq-data-nested-modules-lean': ['lean', `namespace A
namespace B
def f (n : Nat) : Nat := n + 1
end B
def g (n : Nat) : Nat := B.f n
end A
namespace C
def h (n : Nat) : Nat := A.g n
end C
def main : IO Unit := do
  IO.println s!"{C.h 1}"
`],
  'emit-rocq-data-js': ['mjs', `${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}${jsFn('node', [['v', 'bigint']], 'Tree', "  return { $: 'node', value: v, left: leaf(), right: leaf() };")}${jsFn('total', [['t', 'Tree']], 'bigint', "  switch (t.$) {\n    case 'leaf':\n      return 0n;\n    default:\n      return total(t.left) + t.value + total(t.right);\n  }")}console.log(total(node(3n)));\n`],
  'emit-rocq-data-rocq': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=
  match t with
  | leaf => 0
  | node l _ r => size l + 1 + size r
  end.
Definition sample : Tree := node leaf 1 leaf.
`],
  'emit-rocq-data-empty': ['rs', `pub enum Never {}
fn f(n: Never) -> u64 { match n { _ => 1 } }
${rustMain}`],
  'emit-rocq-data-fields': ['lean', `inductive Rec where
  | mk : Nat → Int → Bool → String → Rec

def get (r : Rec) : Int :=
  match r with
  | .mk _ i _ _ => i
${leanMain}`],
  'emit-rocq-data-two-types': ['lean', `inductive Color where
  | red : Color
  | green : Color
inductive Pixel where
  | px : Color → Nat → Pixel
def bright (p : Pixel) : Bool :=
  match p with
  | .px .red _ => true
  | .px .green n => n > 3
${leanMain}`],

  // Matches: naturals test for zero, data matches list every constructor.
  'emit-rocq-match-nat-wild': ['lean', `def f (n : Nat) : Nat :=
  match n with
  | 0 => 1
  | _ => 2
${leanMain}`],
  'emit-rocq-match-nat-bind': ['lean', `def f (n : Nat) : Nat :=
  match n with
  | 0 => 1
  | m => m + 1
${leanMain}`],
  'emit-rocq-match-nat-succ-only': ['lean', `def f (n : Nat) : Nat :=
  match n with
  | k + 1 => k
  | _ => 0
${leanMain}`],
  'emit-rocq-match-nat-succ-bind': ['lean', `def f (n : Nat) : Nat :=
  match n with
  | k + 1 => k
  | m => m
${leanMain}`],
  'emit-rocq-match-nat-both': ['lean', `def f (n : Nat) : Nat :=
  match n with
  | 0 => 5
  | k + 1 => k * 2
${leanMain}`],
  'emit-rocq-match-scrutinee': ['lean', `${leanTree}
def g (n : Nat) : Nat := n + 1
def f (n : Nat) : Nat :=
  match g n with
  | 0 => 1
  | k + 1 => k
def pick (b : Bool) : Tree := if b then Tree.leaf else Tree.node Tree.leaf 1 Tree.leaf
def h (b : Bool) : Nat :=
  match pick b with
  | .leaf => 0
  | .node _ v _ => v
${leanMain}`],
  'emit-rocq-match-data-wild': ['lean', `inductive Color where
  | red : Color
  | green : Color
  | blue : Color
def isRed (c : Color) : Bool :=
  match c with
  | .red => true
  | _ => false
${leanMain}`],
  'emit-rocq-match-data-bind': ['lean', `${leanTree}
def keep (t : Tree) : Tree :=
  match t with
  | .node _ _ _ => Tree.leaf
  | other => other
${leanMain}`],
  'emit-rocq-match-data-underscores': ['rs', `${rustTree}
fn value(t: &Tree) -> u64 { match t { Tree::Node(_, v, _) => *v, _ => 0 } }
${rustMain}`],
  'emit-rocq-match-bool': ['lean', `def f (b : Bool) : Nat :=
  match b with
  | true => 1
  | false => 0
${leanMain}`],
  'emit-rocq-match-bool-rust': ['rs', `fn f(b: bool, c: bool) -> u8 { match b { true => 1, false => if c { 3 } else { 4 } } }${rustMain}`],
  'emit-rocq-match-nested': ['lean', `${leanTree}
def f : Tree → Nat
  | .node (.node _ a _) b _ => a + b
  | .node .leaf b _ => b
  | .leaf => 0
${leanMain}`],
  'emit-rocq-let-if': ['lean', `def f (n : Nat) : Nat :=
  let m := n * 2
  let k := m + 1
  if k > 10 then k else m
${leanMain}`],

  // Recursion forms.
  'emit-rocq-fixpoint-data': ['lean', `${leanTree}
def insert (t : Tree) (x : Nat) : Tree :=
  match t with
  | .leaf => Tree.node Tree.leaf x Tree.leaf
  | .node l v r => if x < v then Tree.node (insert l x) v r else Tree.node l v (insert r x)
${leanMain}`],
  'emit-rocq-fixpoint-second-param': ['lean', `${leanTree}
def count (x : Nat) (t : Tree) : Nat :=
  match t with
  | .leaf => x
  | .node l _ r => count x l + count x r
${leanMain}`],
  'emit-rocq-function-nat': ['lean', `def fact : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * fact n
${leanMain}`],
  'emit-rocq-function-nat-second': ['lean', `def add (a : Nat) : Nat → Nat
  | 0 => a
  | n + 1 => add a n + 1
${leanMain}`],
  'emit-rocq-function-fib': ['lean', `def fib : Nat → Nat
  | 0 => 0
  | 1 => 1
  | n + 2 => fib (n + 1) + fib n
${leanMain}`],
  'emit-rocq-function-guard': ['rs', `fn sum_to(n: u64) -> u64 { if n == 0 { 0 } else { n + sum_to(n - 1) } }${rustMain}`],
  'emit-rocq-function-js': ['mjs', `${jsFn('down', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('down expects a natural number');\n  if (n === 0n) return 0n;\n  return down(n - 1n) + 2n;")}console.log(down(4n));\n`],
  'emit-rocq-function-rocq': ['v', `${rocqHead}Fixpoint f (n : nat) : nat :=
  match n with
  | O => 0
  | S O => 1
  | S (S m) => f m
  end.
`],
  'emit-rocq-general-recursion': ['lean', `def up (n : Nat) : Nat := if n > 10 then n else up (n + 1)
${leanMain}`],
  'emit-rocq-general-recursion-rust': ['rs', `fn collatz(n: u64) -> u64 { if n <= 1 { 0 } else if n % 2 == 0 { 1 + collatz(n / 2) } else { 1 + collatz(3 * n + 1) } }${rustMain}`],
  'emit-rocq-cyclic': ['rs', `fn even(n: u64) -> bool { if n == 0 { true } else { odd(n - 1) } }
fn odd(n: u64) -> bool { if n == 0 { false } else { even(n - 1) } }
${rustMain}`],
  'emit-rocq-interleaved-modules': ['lean', `namespace A
def f (n : Nat) : Nat := n + 1
end A
def g (n : Nat) : Nat := A.f n
namespace A
def h (n : Nat) : Nat := g n
end A
${leanMain}`],
  'emit-rocq-reopened-module': ['lean', `namespace A
def f (n : Nat) : Nat := n + 1
end A
def g (n : Nat) : Nat := n * 2
namespace A
def h (n : Nat) : Nat := f n
end A
${leanMain}`],

  // Names: Rocq keywords, standard-library names, generated names.
  'emit-rocq-keyword-functions': ['rs', `fn fix(n: u64) -> u64 { n }
fn measure(n: u64) -> u64 { fix(n) }
fn lia(wf: u64) -> u64 { measure(wf) }
fn nil() -> u64 { 0 }
fn tt() -> bool { true }
fn left(right: u64) -> u64 { right }
${rustMain}`],
  'emit-rocq-keyword-locals': ['lean', `def f (left right : Nat) : Nat :=
  let using := left + right
  let S := using * 2
  let O := S + 1
  O
def g (N Z : Nat) : Nat := N + Z
${leanMain}`],
  'emit-rocq-keyword-module': ['rs', `mod list {
    pub fn cons(n: u64) -> u64 { n + 1 }
}
mod string {
    pub fn length() -> u64 { crate::list::cons(1) }
}
fn main() { println!("{}", string::length()); }
`],
  'emit-rocq-keyword-data': ['lean', `inductive option where
  | Some : Nat → option
  | None : option
def get (o : option) : Nat :=
  match o with
  | .Some n => n
  | .None => 0
def mk : option := option.Some 3
${leanMain}`],
  'emit-rocq-keyword-ctor': ['lean', `inductive Pair where
  | pair : Nat → Nat → Pair
  | left : Pair
  | right : Pair
def first (p : Pair) : Nat :=
  match p with
  | .pair a _ => a
  | .left => 1
  | .right => 2
${leanMain}`],
  'emit-rocq-keyword-js': ['mjs', `${jsFn('Prop', [['then', 'bigint']], 'bigint', '  return then + 1n;')}${jsFn('exists', [['with_', 'bigint'], ['end', 'bigint']], 'bigint', '  return Prop(with_) + end;')}console.log(exists(1n, 2n));\n`],
  'emit-rocq-generated-clash': ['lean', `def fact : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * fact n
def fact_equation (n : Nat) : Nat := n
def R_fact (n : Nat) : Nat := n
def fact_ind (n : Nat) : Nat := n
${leanMain}`],
  'emit-rocq-generated-clash-data': ['lean', `${leanTree}
def Tree_rect (n : Nat) : Nat := n
def Tree_ind (n : Nat) : Nat := n + 1
def Tree_rec (n : Nat) : Nat := n + 2
def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r
def size_ind (n : Nat) : Nat := n
${leanMain}`],
  'emit-rocq-name-characters': ['lean', `def f' (x' : Nat) : Nat := x' + 1
def café (n : Nat) : Nat := f' n
def x₁ (n : Nat) : Nat := café n
${leanMain}`],
  'emit-rocq-name-collapse': ['lean', `def x₁ (n : Nat) : Nat := n + 1
def x₂ (n : Nat) : Nat := x₁ n + 2
def x_ (n : Nat) : Nat := x₂ n
${leanMain}`],
  'emit-rocq-name-dollar-js': ['mjs', `${jsFn('a$b', [['c$', 'bigint']], 'bigint', '  return c$ + 1n;')}console.log(a$b(1n));\n`],
  'emit-rocq-name-main-local': ['lean', `def main : IO Unit := do
  let nil := 3
  let S := nil + 1
  let x := S + 1
  IO.println s!"{x}"
`],
  'emit-rocq-name-shadow-function': ['lean', `def g (n : Nat) : Nat := n
def f (g : Nat) : Nat := g + 1
${leanMain}`],

  // Theorems and every proof-plan path.
  'emit-rocq-theorem-closed': ['lean', `def fact : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * fact n
theorem fact_five : fact 5 = 120 := by decide
${leanMain}`],
  'emit-rocq-theorem-rfl': ['lean', `def double (n : Nat) : Nat := n + n
theorem double_two : double 2 = 4 := by rfl
${leanMain}`],
  'emit-rocq-theorem-omega': ['lean', `def double (n : Nat) : Nat := n + n
theorem double_eq (n : Nat) : double n = 2 * n := by
  unfold double
  omega
${leanMain}`],
  'emit-rocq-theorem-props': ['lean', `def f (n : Nat) : Nat := n + 1
theorem props (n : Nat) (i : Int) : (n < f n ∧ n ≤ f n) ∨ (f n > n → f n ≥ n) ∨ ¬ (f n = n) ∨ f n ≠ 0 ∨ i < i + 1 := by
  simp [f]
  omega
${leanMain}`],
  'emit-rocq-theorem-forall': ['lean', `def f (n : Nat) : Nat := n + 1
theorem all : ∀ n : Nat, f n > n := by
  intro n
  simp [f]
${leanMain}`],
  'emit-rocq-theorem-forall-nested': ['lean', `def add (a b : Nat) : Nat := a + b
theorem comm (a : Nat) : ∀ (b : Nat) (c : Int), add a b = add b a ∧ c = c := by
  intro b c
  simp [add]
  omega
${leanMain}`],
  'emit-rocq-theorem-bool': ['lean', `def isZero (n : Nat) : Bool := n == 0
theorem zero_is_zero : isZero 0 = true := by rfl
theorem bool_prop : isZero 0 := by decide
${leanMain}`],
  'emit-rocq-theorem-int': ['lean', `def neg (x : Int) : Int := -x
theorem neg_neg (x : Int) : neg (neg x) = x := by
  simp [neg]
theorem neg_lt (x : Int) : x > 0 → neg x < 0 := by
  simp [neg]
  omega
${leanMain}`],
  'emit-rocq-theorem-induction-nat': ['lean', `def sumTo : Nat → Nat
  | 0 => 0
  | k + 1 => sumTo k + (k + 1)
theorem sumTo_formula (n : Nat) : 2 * sumTo n = n * (n + 1) := by
  induction n with
  | zero => rfl
  | succ k ih => simp [sumTo, Nat.mul_add, Nat.add_mul, ih]; omega
${leanMain}`],
  'emit-rocq-theorem-cases-nat': ['lean', `def pred (n : Nat) : Nat := n - 1
theorem pred_le (n : Nat) : pred n ≤ n := by
  cases n with
  | zero => rfl
  | succ k => simp [pred]
${leanMain}`],
  'emit-rocq-theorem-induction-data': ['lean', `${leanTree}
def mirror : Tree → Tree
  | .leaf => Tree.leaf
  | .node l v r => Tree.node (mirror r) v (mirror l)
def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r
theorem mirror_mirror (t : Tree) : mirror (mirror t) = t := by
  induction t with
  | leaf => rfl
  | node l v r ihl ihr => simp [mirror, ihl, ihr]
theorem size_mirror (t : Tree) : size (mirror t) = size t := by
  induction t with
  | leaf => rfl
  | node l v r ihl ihr => simp [size, mirror, ihl, ihr]; omega
${leanMain}`],
  'emit-rocq-theorem-cases-data': ['lean', `${leanTree}
def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r
theorem size_nonneg (t : Tree) : 0 ≤ size t := by
  cases t with
  | leaf => simp [size]
  | node l v r => simp [size]
${leanMain}`],
  'emit-rocq-theorem-lemma': ['lean', `def double (n : Nat) : Nat := n + n
theorem double_eq (n : Nat) : double n = 2 * n := by
  simp [double]
  omega
theorem double_double (n : Nat) : double (double n) = 4 * n := by
  rw [double_eq, double_eq]
  omega
theorem double_back (n : Nat) : 2 * n = double n := by
  rw [← double_eq]
${leanMain}`],
  'emit-rocq-theorem-nat-function-unfold': ['lean', `def add (a : Nat) : Nat → Nat
  | 0 => a
  | n + 1 => add a n + 1
theorem add_zero (a : Nat) : add a 0 = a := by
  simp [add]
theorem add_succ (a n : Nat) : add a (n + 1) = add a n + 1 := by
  simp [add]
theorem add_comm_zero (n : Nat) : add 0 n = n := by
  induction n with
  | zero => simp [add]
  | succ k ih => simp [add, ih]
${leanMain}`],
  'emit-rocq-theorem-nat-function-closed': ['lean', `def fact : Nat → Nat
  | 0 => 1
  | n + 1 => (n + 1) * fact n
theorem fact_three : fact 3 = 6 := by
  simp [fact]
${leanMain}`],
  'emit-rocq-theorem-trailing': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=
  match t with
  | leaf => 0
  | node l _ r => size l + 1 + size r
  end.
Theorem size_nonneg (t : Tree) : 0 <= size t.
Proof.
  destruct t as [| l v r]; simpl; lia.
Qed.
Theorem size_again (t : Tree) : size t = size t.
Proof.
  induction t; simpl; reflexivity.
Qed.
`],
  'emit-rocq-theorem-missing-case': ['lean', `${leanTree}
def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r
theorem size_nonneg (t : Tree) : 0 ≤ size t := by
  cases t with
  | leaf => simp [size]
${leanMain}`],
  'emit-rocq-theorem-nested-split': ['lean', `def f (a b : Nat) : Nat := a + b
theorem nested (a b : Nat) : f a b = f b a := by
  induction a with
  | zero =>
    cases b with
    | zero => rfl
    | succ k => simp [f]
  | succ k ih => simp [f]; omega
${leanMain}`],
  'emit-rocq-theorem-data-no-recursion': ['lean', `inductive Color where
  | red : Color
  | mix : Color → Nat → Color
def weight (c : Color) : Nat :=
  match c with
  | .red => 1
  | .mix _ n => n
theorem weight_pos (c : Color) : weight c ≥ 0 := by
  induction c with
  | red => simp [weight]
  | mix d n ih => simp [weight]
${leanMain}`],
  'emit-rocq-theorem-rocq': ['v', `${rocqHead}Fixpoint add (n m : nat) : nat := match n with | O => m | S k => S (add k m) end.
Theorem add_zero (n : nat) : add n 0 = n.
Proof.
  induction n as [| k ih].
  - reflexivity.
  - simpl. rewrite ih. reflexivity.
Qed.
Theorem add_zero_again (n : nat) : add n 0 = n.
Proof.
  rewrite add_zero. reflexivity.
Qed.
`],
  'emit-rocq-theorem-rocq-tree': ['v', `${rocqHead}${rocqTree}Fixpoint mirror (t : Tree) : Tree :=
  match t with
  | leaf => leaf
  | node l v r => node (mirror r) v (mirror l)
  end.
Theorem mirror_mirror (t : Tree) : mirror (mirror t) = t.
Proof.
  induction t as [| l ihl v r ihr].
  - reflexivity.
  - simpl. rewrite ihl, ihr. reflexivity.
Qed.
Theorem mirror_cases (t : Tree) : mirror t = mirror t.
Proof.
  destruct t as [| l v r].
  - reflexivity.
  - reflexivity.
Qed.
`],
  'emit-rocq-theorem-rocq-props': ['v', `${rocqHead}Definition f (n : nat) : nat := n + 1.
Theorem props (n : nat) (z : Z) : (n < f n /\\ n <= f n) \\/ ~ (f n = n) \\/ (z < z + 1)%Z -> n <> f n.
Proof.
  unfold f. lia.
Qed.
Theorem bool_prop (a b : bool) : andb a b = andb a b.
Proof.
  reflexivity.
Qed.
`],
  'emit-rocq-theorem-module': ['lean', `namespace Arith
def double (n : Nat) : Nat := n + n
theorem double_eq (n : Nat) : double n = 2 * n := by
  simp [double]
  omega
end Arith
theorem use (n : Nat) : Arith.double n = n + n := by
  rw [Arith.double_eq]
  omega
${leanMain}`],
  'emit-rocq-theorem-keyword-binders': ['lean', `def f (n : Nat) : Nat := n + 1
theorem t (left right : Nat) : f left + right = right + f left := by
  omega
${leanMain}`],

  // Main: prints, lets and assertions, which become decided theorems.
  'emit-rocq-main-lets': ['lean', `def main : IO Unit := do
  let a := 3
  let b := a + 4
  IO.println s!"{a}"
  let c := b * 2
  IO.println s!"{b} {c}"
`],
  'emit-rocq-assert-rust': ['rs', `fn double(n: u64) -> u64 { n * 2 }
fn main() {
    assert_eq!(double(2), 4);
    let x = double(3);
    assert_ne!(x, 5);
    assert!(x > 5 && x <= 6);
    assert!(x < 7 || x >= 100);
    assert!(!(x == 1));
    println!("{}", x);
}
`],
  'emit-rocq-assert-fixed': ['rs', `fn main() {
    let a: u8 = 3;
    let b: i32 = -4;
    assert!(a < 5);
    assert!(b >= -4);
    println!("{} {}", a, b);
}
`],
  'emit-rocq-assert-js': ['mjs', `${jsHead}${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}${jsFn('inc', [['n', 'bigint']], 'bigint', '  return n + 1n;')}assert.equal(inc(1n), 2n);
assert.notEqual(inc(1n), 3n);
assert(inc(1n) === 2n && inc(2n) > 2n);
assert.deepStrictEqual(leaf(), leaf());
assert.notDeepStrictEqual(inc(1n), inc(2n));
assert.ok(true);
const y = inc(5n);
assert(y === 6n || y < 0n);
console.log(y);
`],
  'emit-rocq-assert-only': ['rs', `fn main() {
    assert!(1 + 1 == 2);
}
`],
  'emit-rocq-assert-lets-before': ['rs', `fn main() {
    let a: u64 = 3;
    let b: u64 = a * 2;
    println!("{}", b);
    let c: u64 = b + a;
    assert_eq!(c, 9);
    assert!(a < c);
}
`],
  'emit-rocq-assert-string': ['rs', `fn greet(s: &str) -> String { format!("hi {}", s) }
fn main() {
    assert_eq!(greet("x"), String::from("hi x"));
    assert!(true != false);
}
`],
  'emit-rocq-main-data': ['lean', `${leanTree}
namespace Tree
def size : Tree → Nat
  | leaf => 0
  | node l _ r => size l + 1 + size r
end Tree
def main : IO Unit := do
  let t := Tree.node Tree.leaf 5 Tree.leaf
  IO.println s!"{Tree.size t}"
`],
  'emit-rocq-main-empty-print': ['mjs', 'console.log();\nconsole.log(true);\nconsole.log(-5n);\n'],
  'emit-rocq-main-concat-js': ['mjs', `${jsFn('label', [['n', 'bigint']], 'string', "  return 'n=' + n + ', positive=' + (n > 0n) + '!';")}console.log(label(5n));\nconsole.log(\`nested \${\`inner \${1n + 2n}\`} done\`);\n`],
  'emit-rocq-corpus-like': ['rs', `mod arith {
    pub fn fact(n: u64) -> u64 { if n == 0 { 1 } else { n * fact(n - 1) } }
    pub fn halve(x: i64) -> i64 { x.div_euclid(2) }
}
fn classify(n: i64) -> String {
    if n < 0 { String::from("negative") } else if n == 0 { String::from("zero") } else { String::from("positive") }
}
fn main() {
    assert_eq!(arith::fact(5), 120);
    println!("fact 20 = {}", arith::fact(20));
    println!("halve -7 = {}", arith::halve(-7));
    println!("{}", classify(-4));
}
`],
};
