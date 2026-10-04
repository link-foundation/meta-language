// Small programs that exercise every branch of the Rust emitter in all four
// source languages: the ml::Big prelude (small and huge literals, natural
// subtraction, every division rounding, clamped and checked conversions to
// naturals), machine integers (checked arithmetic, negation, truncating and
// Euclidean division, widening casts, unsigned zero/successor matches),
// strings, escapes and printing, data types across modules (boxed fields,
// unboxed bindings), names that clash with Rust keywords, prelude names and
// each other, theorems with bounded checks over every domain, assertions and
// main effects, keyed name → [source extension, source text], so the Rust
// port of the emitter can be compared with the JavaScript one case by case.
// The emitter's two unsupported errors (floor division on a signed machine
// integer, printing a structured value) are rejected earlier by the frontends
// or the checker, so crafted IR in rust/tests/unit/translation_emit_rust.rs
// covers them instead.

const leanTree = `inductive Tree where
  | leaf : Tree
  | node : Tree → Nat → Tree → Tree
`;
const leanMain = '\ndef main : IO Unit := do\n  IO.println "x"\n';
const rustTree = `#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tree { Leaf, Node(Box<Tree>, u64, Box<Tree>) }
`;
const rustMain = '\nfn main() { println!("x"); }\n';
const rocqHead = 'From Stdlib Require Import NArith ZArith String DecimalString Lia.\nOpen Scope string_scope.\n';
const rocqShow = 'Definition show_N (n : N) : string := NilEmpty.string_of_uint (N.to_uint n).\nDefinition show_nat (n : nat) : string := NilEmpty.string_of_uint (Nat.to_uint n).\nDefinition show_Z (z : Z) : string := NilZero.string_of_int (Z.to_int z).\n';
const rocqTree = 'Inductive Tree : Type :=\n| leaf : Tree\n| node : Tree -> nat -> Tree -> Tree.\n';
const jsTree = "/**\n * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree\n */\n";
const jsFn = (name, params, ret, body) => `/**\n${params.map(([p, t]) => ` * @param {${t}} ${p}\n`).join('')} * @returns {${ret}}\n */\nfunction ${name}(${params.map(([p]) => p).join(', ')}) {\n${body}\n}\n`;
const jsLeaf = jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };");
const jsNode = jsFn('node', [['l', 'Tree'], ['v', 'bigint'], ['r', 'Tree']], 'Tree', "  return { $: 'node', left: l, value: v, right: r };");

const cases = {
  // Lean: file shape, entry and modules.
  'lean-no-main': ['lean', 'def one : Nat := 1\ndef two : Nat := one + one\n'],
  'lean-main-only': ['lean', 'def main : IO Unit := do\n  IO.println "hello"\n  IO.println ""\n'],
  'lean-main-lets': ['lean', 'def sq (n : Nat) : Nat := n * n\n\ndef main : IO Unit := do\n  let a := sq 3\n  let b := sq a\n  let console := a + b\n  IO.println s!"{a} {b} {console}"\n'],
  'lean-namespace': ['lean', `namespace Arith\n\ndef double (n : Nat) : Nat := n + n\n\ndef quad (n : Nat) : Nat := double (double n)\n\nend Arith\n\ndef eight : Nat := Arith.quad 2\n${leanMain}`],
  'lean-nested-namespaces': ['lean', `namespace A\n\ndef a (n : Nat) : Nat := n + 1\n\nnamespace B\n\ndef b (n : Nat) : Nat := A.a n * 2\n\nnamespace C\n\ndef c (n : Nat) : Nat := A.B.b n + A.a n\n\nend C\n\nend B\n\ndef after (n : Nat) : Nat := B.C.c n\n\nend A\n\nnamespace D\n\ndef d : Nat := A.after 1\n\nend D\n\ndef main : IO Unit := do\n  IO.println s!"{D.d} {A.B.C.c 2}"\n`],
  'lean-keyword-namespaces': ['lean', `namespace Math\n\ndef max (a b : Nat) : Nat := if a < b then b else a\n\nend Math\n\nnamespace Object\n\ndef keys (n : Nat) : Nat := n\n\nend Object\n\nnamespace JSON\n\nnamespace default\n\ndef delete (n : Nat) : Nat := n\n\nend default\n\nend JSON\n\ndef main : IO Unit := do\n  IO.println s!"{Math.max 1 2} {Object.keys 3} {JSON.default.delete 4}"\n`],
  'lean-keyword-names': ['lean', `def var (n : Nat) : Nat := n\ndef new (n : Nat) : Nat := var n\ndef typeof (n : Nat) : Nat := new n\ndef console (n : Nat) : Nat := typeof n\ndef process (arguments eval undefined : Nat) : Nat := arguments + eval + undefined\ndef constructor (prototype : Nat) : Nat := prototype\ndef get (set : Nat) : Nat := set\ndef async (await : Nat) : Nat := await\ndef yield (static : Nat) : Nat := static\n\ndef main : IO Unit := do\n  IO.println s!"{console 1} {process 1 2 3} {constructor 4} {get 5} {async 6} {yield 7}"\n`],
  'lean-primed-names': ['lean', `def f' (n : Nat) : Nat := n + 1\ndef f'' (n' : Nat) : Nat := f' n' + 1\ndef α (β : Nat) : Nat := β\n\ndef main : IO Unit := do\n  IO.println s!"{f'' 1} {α 2}"\n`],
  'lean-main-name-clash': ['lean', `namespace Util\n\ndef main' (n : Nat) : Nat := n\n\nend Util\n\ndef ml (n : Nat) : Nat := Util.main' n\n\ndef main : IO Unit := do\n  IO.println s!"{ml 3}"\n`],

  // Lean: arithmetic, strings and casts.
  'lean-nat-sub': ['lean', `def monus (a b : Nat) : Nat := a - b\ndef chain (a : Nat) : Nat := a - 1 - 2\n\ndef main : IO Unit := do\n  IO.println s!"{monus 3 5} {chain 10}"\n`],
  'lean-int-sub': ['lean', `def diff (a b : Int) : Int := a - b\ndef neg (a : Int) : Int := -a\ndef negLit : Int := -7\n\ndef main : IO Unit := do\n  IO.println s!"{diff 3 5} {neg 4} {negLit}"\n`],
  'lean-nat-div-mod': ['lean', `def q (a b : Nat) : Nat := a / b\ndef r (a b : Nat) : Nat := a % b\n\ndef main : IO Unit := do\n  IO.println s!"{q 7 2} {r 7 2} {q 7 0} {r 7 0}"\n`],
  'lean-int-div-mod': ['lean', `def q (a b : Int) : Int := a / b\ndef r (a b : Int) : Int := a % b\n\ndef main : IO Unit := do\n  IO.println s!"{q (-7) 2} {r (-7) 2} {q 7 (-2)} {r 7 0}"\n`],
  'lean-mul-add': ['lean', `def poly (x : Nat) : Nat := 3 * x * x + 2 * x + 1\ndef ipoly (x : Int) : Int := x * x - 4 * x + 4\n\ndef main : IO Unit := do\n  IO.println s!"{poly 5} {ipoly (-3)}"\n`],
  'lean-casts': ['lean', `def up (n : Nat) : Int := Int.ofNat n - 10\ndef down (i : Int) : Nat := Int.toNat i\ndef both (i : Int) : Nat := Int.toNat (i * 2) + 1\n\ndef main : IO Unit := do\n  IO.println s!"{up 3} {down (-4)} {down 4} {both 2}"\n`],
  'lean-bool-ops': ['lean', `def both (a b : Bool) : Bool := a && b\ndef either (a b : Bool) : Bool := a || b\ndef flip (a : Bool) : Bool := !a\ndef same (a b : Nat) : Bool := a == b\ndef differ (a b : Nat) : Bool := a != b\ndef order (a b : Int) : Bool := a < b || a <= b && a > b || a >= b\n\ndef main : IO Unit := do\n  IO.println s!"{both true false} {either true false} {flip true} {same 1 1} {differ 1 1} {order 1 2}"\n`],
  'lean-strings': ['lean', `def greet (name : String) : String := "hello, " ++ name ++ "!"\ndef same (a b : String) : Bool := a == b\n\ndef main : IO Unit := do\n  let r := same "a" "b"\n  IO.println (greet "world")\n  IO.println s!"{r}"\n`],
  'lean-string-escapes': ['lean', 'def main : IO Unit := do\n  IO.println "quote \\" backslash \\\\ tab\\tend"\n  IO.println "line\\nbreak"\n  IO.println "unicode λ → ∀ 😀"\n  IO.println "dollar ${x} and `tick`"\n'],
  'lean-interpolation': ['lean', `def n : Nat := 3\ndef i : Int := -3\ndef b : Bool := true\ndef s : String := "s"\n\ndef main : IO Unit := do\n  IO.println s!"{n} {i} {b} {s} {n + 1} {i * 2}"\n  IO.println s!"plain"\n`],
  'lean-if-expr': ['lean', `def pick (b : Bool) (n : Nat) : Nat := (if b then n else 0) + 1\ndef sign (i : Int) : Int := if i < 0 then -1 else if i == 0 then 0 else 1\n\ndef main : IO Unit := do\n  IO.println s!"{pick true 2} {sign (-5)}"\n`],
  'lean-let-expr': ['lean', `def f (n : Nat) : Nat :=\n  let a := n + 1\n  let b := a * 2\n  a + b\ndef g (n : Nat) : Nat := 1 + (let m := n * n; m + m)\n\ndef main : IO Unit := do\n  IO.println s!"{f 2} {g 3}"\n`],
  'lean-match-expr': ['lean', `def f (n : Nat) : Nat := 1 + (match n with | 0 => 10 | k + 1 => k)\n\ndef main : IO Unit := do\n  IO.println s!"{f 0} {f 5}"\n`],

  // Lean: natural-number matches.
  'lean-nat-zero-succ': ['lean', `def fact : Nat → Nat\n  | 0 => 1\n  | n + 1 => (n + 1) * fact n\n\ndef main : IO Unit := do\n  IO.println s!"{fact 20}"\n`],
  'lean-nat-zero-wild': ['lean', `def isZero (n : Nat) : Bool :=\n  match n with\n  | 0 => true\n  | _ => false\n${leanMain}`],
  'lean-nat-zero-bind': ['lean', `def pred (n : Nat) : Nat :=\n  match n with\n  | 0 => 0\n  | m => m - 1\n${leanMain}`],
  'lean-nat-wild-only': ['lean', `def seven (n : Nat) : Nat :=\n  match n with\n  | _ => 7\n${leanMain}`],
  'lean-nat-bind-only': ['lean', `def same (n : Nat) : Nat :=\n  match n with\n  | k => k + 1\n${leanMain}`],
  'lean-nat-succ-wild': ['lean', `def f (n : Nat) : Nat :=\n  match n with\n  | k + 1 => k\n  | _ => 42\n${leanMain}`],
  'lean-nat-deep': ['lean', `def fib : Nat → Nat\n  | 0 => 0\n  | 1 => 1\n  | n + 2 => fib (n + 1) + fib n\n\ndef main : IO Unit := do\n  IO.println s!"{fib 20}"\n`],
  'lean-match-computed': ['lean', `def f (n : Nat) : Nat :=\n  match n + 1 with\n  | 0 => 0\n  | k + 1 => match k * 2 with\n    | 0 => 1\n    | j + 1 => j\n\ndef main : IO Unit := do\n  IO.println s!"{f 3}"\n`],
  'lean-multi-scrutinee': ['lean', `def f : Nat → Nat → Nat\n  | 0, _ => 0\n  | _, 0 => 1\n  | n + 1, m + 1 => f n m\n${leanMain}`],
  'lean-bool-match': ['lean', `def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n  | false => 0\n${leanMain}`],
  'lean-int-literal-match': ['lean', `def f (i : Int) : Nat :=\n  match i with\n  | 0 => 1\n  | 1 => 2\n  | _ => 3\n${leanMain}`],

  // Lean: data types.
  'lean-tree': ['lean', `${leanTree}\nnamespace Tree\n\ndef size : Tree → Nat\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n\ndef mirror : Tree → Tree\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n\nend Tree\n\ndef main : IO Unit := do\n  IO.println s!"{Tree.size (Tree.mirror (Tree.node Tree.leaf 1 Tree.leaf))}"\n`],
  'lean-data-wild': ['lean', `${leanTree}\ndef isLeaf (t : Tree) : Bool :=\n  match t with\n  | .leaf => true\n  | _ => false\n${leanMain}`],
  'lean-data-bind': ['lean', `${leanTree}\ndef grow (t : Tree) : Tree :=\n  match t with\n  | .leaf => .node .leaf 0 .leaf\n  | other => other\n${leanMain}`],
  'lean-data-nested-match': ['lean', `${leanTree}\ndef f : Tree → Nat\n  | .node (.node _ a _) b _ => a + b\n  | .node .leaf b _ => b\n  | .leaf => 0\n${leanMain}`],
  'lean-data-unused-fields': ['lean', `${leanTree}\ndef value : Tree → Nat\n  | .leaf => 0\n  | .node _ v _ => v\n${leanMain}`],
  'lean-data-named-fields': ['lean', `inductive Shape where\n  | circle (radius : Nat) : Shape\n  | rect (width height : Nat) : Shape\n  | dot : Shape\n\ndef area : Shape → Nat\n  | .circle r => 3 * r * r\n  | .rect w h => w * h\n  | .dot => 0\n\ndef main : IO Unit := do\n  IO.println s!"{area (.rect 2 3)} {area (.circle 1)}"\n`],
  'lean-data-keyword-fields': ['lean', `inductive Box where\n  | box (default new this : Nat) : Box\n\ndef sum : Box → Nat\n  | .box a b c => a + b + c\n\ndef main : IO Unit := do\n  IO.println s!"{sum (.box 1 2 3)}"\n`],
  'lean-data-primed-ctor': ['lean', `inductive T where\n  | leaf' : T\n  | node' : T → T\n\ndef depth : T → Nat\n  | .leaf' => 0\n  | .node' t => depth t + 1\n\ntheorem depth_leaf : depth T.leaf' = 0 := by decide\n\ntheorem depth_any (t : T) : depth t = depth t := by simp\n\ndef main : IO Unit := do\n  IO.println s!"{depth (.node' .leaf')}"\n`],
  'lean-data-in-namespace': ['lean', `namespace Shapes\n\ninductive Color where\n  | red : Color\n  | green : Color\n\ndef code : Color → Nat\n  | .red => 1\n  | .green => 2\n\nend Shapes\n\ndef main : IO Unit := do\n  IO.println s!"{Shapes.code Shapes.Color.red}"\n`],
  'lean-data-keyword-type': ['lean', `inductive Object where\n  | mk : Nat → Object\n\ndef unwrap : Object → Nat\n  | .mk n => n\n\ndef main : IO Unit := do\n  IO.println s!"{unwrap (.mk 5)}"\n`],

  // Lean: theorems and bounded checks.
  'lean-theorem-closed': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_two : double 2 = 4 := by decide\n${leanMain}`],
  'lean-theorem-closed-no-main': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_two : double 2 = 4 := by decide\n`],
  'lean-theorem-nat': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_even (n : Nat) : double n = 2 * n := by simp [double]; omega\n${leanMain}`],
  'lean-theorem-int': ['lean', `def neg (i : Int) : Int := -i\n\ntheorem neg_neg (i : Int) : neg (neg i) = i := by simp [neg]\n${leanMain}`],
  'lean-theorem-bool-string': ['lean', `def pick (b : Bool) (s : String) : String := if b then s else ""\n\ntheorem pick_true (s : String) : pick true s = s := by simp [pick]\n\ntheorem pick_bool (b : Bool) (s : String) : pick b s = pick b s := by simp\n${leanMain}`],
  'lean-theorem-many-binders': ['lean', `def f (a : Nat) (b : Int) (c : Bool) : Int := if c then b else Int.ofNat a\n\ntheorem f_true (a : Nat) (b : Int) : f a b true = b := by simp [f]\n${leanMain}`],
  'lean-theorem-tree': ['lean', `${leanTree}\nnamespace Tree\n\ndef mirror : Tree → Tree\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n\ndef size : Tree → Nat\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n\nend Tree\n\ntheorem mirror_mirror (t : Tree) : Tree.mirror (Tree.mirror t) = t := by\n  induction t with\n  | leaf => rfl\n  | node l v r ihl ihr => simp [Tree.mirror, ihl, ihr]\n\ntheorem size_mirror (t : Tree) : Tree.size (Tree.mirror t) = Tree.size t := by\n  induction t with\n  | leaf => rfl\n  | node l v r ihl ihr => simp [Tree.size, Tree.mirror, ihl, ihr]; omega\n${leanMain}`],
  'lean-theorem-connectives': ['lean', `def f (n : Nat) : Nat := n + 1\n\ntheorem pos (n : Nat) : f n > 0 ∧ f n ≥ 1 := by simp [f]\n\ntheorem either (n : Nat) : f n ≠ 0 ∨ n < 0 := by simp [f]\n\ntheorem imp (n : Nat) : n ≤ 3 → f n ≤ 4 := by simp [f]; omega\n\ntheorem negation (n : Nat) : ¬ (f n = 0) := by simp [f]\n${leanMain}`],
  'lean-theorem-forall': ['lean', `def add (a b : Nat) : Nat := a + b\n\ntheorem add_comm' : ∀ a b : Nat, add a b = add b a := by intro a b; simp [add]; omega\n\ntheorem add_zero (a : Nat) : ∀ b : Int, add a 0 = a ∧ b = b := by simp [add]\n${leanMain}`],
  'lean-theorem-bool-prop': ['lean', `def isZero (n : Nat) : Bool := n == 0\n\ntheorem zero_is_zero : isZero 0 = true := by decide\n\ntheorem bool_prop (n : Nat) : isZero (n + 1) = false := by simp [isZero]\n${leanMain}`],
  'lean-theorem-namespaced': ['lean', `namespace Arith\n\ndef double (n : Nat) : Nat := n + n\n\ntheorem double_zero : double 0 = 0 := by decide\n\nnamespace Deep\n\ntheorem double_le (n : Nat) : n ≤ double n := by simp [double]\n\nend Deep\n\nend Arith\n${leanMain}`],
  'lean-theorem-primed': ['lean', `def f (n : Nat) : Nat := n\n\ntheorem f_id' (n : Nat) : f n = n := by simp [f]\n${leanMain}`],
  'lean-theorem-data-ne': ['lean', `${leanTree}\ntheorem leaf_ne (t : Tree) (v : Nat) : Tree.node t v t ≠ Tree.leaf := by simp\n${leanMain}`],
  'lean-theorem-nonrecursive-data': ['lean', `inductive Pair where\n  | pair : Nat → Bool → Pair\n\ninductive Wrap where\n  | none : Wrap\n  | some : Pair → Wrap\n\ndef first : Pair → Nat\n  | .pair n _ => n\n\ntheorem first_pair (n : Nat) (b : Bool) : first (.pair n b) = n := by simp [first]\n\ntheorem wrap_self (w : Wrap) : w = w := by simp\n${leanMain}`],
  'lean-theorem-nested-data': ['lean', `inductive Color where\n  | red : Color\n  | blue : Color\n\ninductive Paint where\n  | coat : Color → String → Paint\n\ninductive Wall where\n  | bare : Wall\n  | painted : Paint → Wall → Wall\n\ntheorem wall_self (w : Wall) : w = w := by simp\n\ntheorem paint_self (p : Paint) : p = p := by simp\n${leanMain}`],

  // Rust: machine integers.
  'rust-u8-add': ['rs', `fn add(a: u8, b: u8) -> u8 { a + b }\nfn main() { println!("{}", add(100, 27)); }\n`],
  'rust-fixed-widths': ['rs', `fn a(x: u8, y: i8) -> i8 { y }\nfn b(x: u16, y: i16) -> i16 { y }\nfn c(x: u32, y: i32) -> i32 { y }\nfn d(x: u64, y: i64) -> i64 { y }\nfn e(x: u128, y: i128) -> i128 { y }\nfn f(x: usize, y: isize) -> isize { y }${rustMain}`],
  'rust-checked-ops': ['rs', `fn ops(a: i32, b: i32) -> i32 { a + b - a * b }\nfn neg(a: i64) -> i64 { -a }\nfn usub(a: u64, b: u64) -> u64 { a - b }${rustMain}`],
  'rust-division': ['rs', `fn q(a: i32, b: i32) -> i32 { a / b }\nfn r(a: i32, b: i32) -> i32 { a % b }\nfn uq(a: u64, b: u64) -> u64 { a / b + a % b }\nfn main() { println!("{} {}", q(-7, 2), r(-7, 2)); }\n`],
  'rust-euclid': ['rs', `fn f(x: i32, y: i32) -> i32 { x.div_euclid(y) + x.rem_euclid(y) + x / y + x % y }\nfn g(x: i64) -> i64 { x.div_euclid(2) }${rustMain}`],
  'rust-negate': ['rs', `fn f(x: i8) -> i8 { -x }\nfn g() -> i8 { -128 }\nfn h() -> i128 { -170141183460469231731687303715884105728 }\nfn k() -> u128 { 340282366920938463463374607431768211455 }${rustMain}`],
  'rust-widening': ['rs', `fn f(n: u32) -> i64 { i64::from(n) + (n as i64) }\nfn g(n: u8) -> u64 { u64::from(n) * 2 }${rustMain}`],
  'rust-unsigned-match': ['rs', `fn sum_to(n: u64) -> u64 {\n    match n {\n        0 => 0,\n        k => sum_to(k - 1) + k,\n    }\n}\nfn fib(n: u64) -> u64 {\n    match n {\n        0 => 0,\n        1 => 1,\n        _ => fib(n - 1) + fib(n - 2),\n    }\n}\nfn main() { println!("{} {}", sum_to(10), fib(10)); }\n`],
  'rust-unsigned-recursion': ['rs', `fn f(n: u32) -> u32 { if n == 0 { 0 } else if n == 1 { 1 } else { f(n - 1) + f(n - 2) } }${rustMain}`],
  'rust-match-bool': ['rs', `fn f(b: bool) -> u8 { match b { true => 1, false => 0 } }${rustMain}`],

  // Rust: data, modules, strings and main effects.
  'rust-tree': ['rs', `${rustTree}\nmod tree {\n    use super::Tree;\n\n    pub fn size(t: &Tree) -> u64 {\n        match t {\n            Tree::Leaf => 0,\n            Tree::Node(l, _, r) => size(l) + 1 + size(r),\n        }\n    }\n\n    pub fn mirror(t: &Tree) -> Tree {\n        match t {\n            Tree::Leaf => Tree::Leaf,\n            Tree::Node(l, v, r) => Tree::Node(Box::new(mirror(r)), *v, Box::new(mirror(l))),\n        }\n    }\n}\n\nfn main() {\n    let t = Tree::Node(Box::new(Tree::Leaf), 3, Box::new(Tree::Leaf));\n    assert_eq!(tree::mirror(&tree::mirror(&t)), t);\n    assert_ne!(t, Tree::Leaf);\n    println!("{}", tree::size(&t));\n}\n`],
  'rust-match-wild-data': ['rs', `${rustTree}\nfn is_leaf(t: &Tree) -> bool { match t { Tree::Leaf => true, _ => false } }\nfn keep(t: &Tree) -> Tree { match t { Tree::Leaf => Tree::Leaf, other => other.clone() } }${rustMain}`],
  'rust-tuple-variants': ['rs', `#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Shape { Circle(u64), Rect(u64, u64), Dot }\n\nfn area(s: &Shape) -> u64 {\n    match s {\n        Shape::Circle(r) => 3 * *r * *r,\n        Shape::Rect(w, _) => *w * 2,\n        Shape::Dot => 0,\n    }\n}\n\nfn main() { println!("{}", area(&Shape::Rect(2, 3))); }\n`],
  'rust-keyword-variants': ['rs', `#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Object { Default, Delete(u64), Constructor(Box<Object>) }\n\nfn total(o: &Object) -> u64 {\n    match o {\n        Object::Default => 0,\n        Object::Delete(n) => *n,\n        Object::Constructor(inner) => total(inner) + 1,\n    }\n}\n\nfn main() { println!("{}", total(&Object::Constructor(Box::new(Object::Delete(2))))); }\n`],
  'rust-modules': ['rs', `mod a { pub mod b { pub fn g() -> u64 { super::h() } } pub fn h() -> u64 { crate::k() } }\nfn k() -> u64 { self::a::b::g() }${rustMain}`],
  'rust-keyword-names': ['rs', `mod console { pub fn log(n: u64) -> u64 { n } }\nmod math { pub fn new(n: u64) -> u64 { n } }\nfn delete(this: u64) -> u64 { this }\nfn function(var: u64, undefined: u64) -> u64 { var + undefined }\nfn instanceof(arguments: u64) -> u64 { arguments }\nfn constructor(eval: u64) -> u64 { eval }\nfn main() { println!("{} {} {} {} {} {}", console::log(1), math::new(2), delete(3), function(4, 5), instanceof(6), constructor(7)); }\n`],
  'rust-strings': ['rs', `fn classify(n: i64) -> String {\n    if n < 0 {\n        String::from("negative")\n    } else if n == 0 {\n        String::from("zero")\n    } else {\n        String::from("positive")\n    }\n}\n\nfn describe(n: u32) -> String {\n    let doubled = n + n;\n    let label = classify(i64::from(doubled) - 10);\n    format!("{n} doubled is {doubled} ({label})")\n}\n\nfn main() {\n    println!("{}", describe(3));\n    println!("{}", classify(-4));\n    println!("{} {}", true, "quote \\" and \\\\ backslash");\n}\n`],
  'rust-main-lets': ['rs', `fn sq(n: u64) -> u64 { n * n }\n\nfn main() {\n    let a = sq(3);\n    let b = sq(a);\n    let main = a + b;\n    assert!(a < b && b > 0);\n    assert!(!(a == b) || a != b);\n    assert_eq!(sq(2), 4);\n    assert_ne!(a, b);\n    println!("{} {} {}", a, b, main);\n}\n`],
  'rust-panic': ['rs', `fn safe_div(a: u64, b: u64) -> u64 {\n    if b == 0 {\n        panic!("division by zero")\n    } else {\n        a / b\n    }\n}\nfn pick(n: u64) -> u64 { match n { 0 => panic!("zero \\"quoted\\""), k => k } }\nfn main() { println!("{} {}", safe_div(6, 3), pick(1)); }\n`],
  'rust-print': ['rs', `fn main() { let x: u8 = 7; println!("{} {}", x, true); println!("{}", -3i64); println!(); }\n`],

  // Rocq.
  'rocq-basic': ['v', `${rocqHead}${rocqShow}Definition monus (a b : N) : N := (a - b)%N.\nDefinition nmonus (a b : nat) : nat := a - b.\nDefinition halve (x : Z) : Z := (x / 2)%Z.\nDefinition remainder (x y : Z) : Z := Z.modulo x y.\nDefinition main : list string :=\n  ("monus = " ++ show_N (monus 3 5)) ::\n  ("halve = " ++ show_Z (halve (-7))) ::\n  ("remainder = " ++ show_Z (remainder (-7) 3)) ::\n  nil.\n\nEval vm_compute in main.\n`],
  'rocq-z': ['v', `${rocqHead}Definition f (x y : Z) : Z := (x / y + x mod y - 3)%Z.\nDefinition g (x y : N) : N := (x / y + x mod y)%N.\nDefinition h (x : Z) : Z := (- x * 2)%Z.\n`],
  'rocq-nat-patterns': ['v', `${rocqHead}Fixpoint f (n : nat) : nat :=\n  match n with\n  | O => 0\n  | S O => 1\n  | S (S m) => f m\n  end.\n`],
  'rocq-tree': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=\n  match t with\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n  end.\nFixpoint mirror (t : Tree) : Tree :=\n  match t with\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n  end.\nTheorem mirror_mirror (t : Tree) : mirror (mirror t) = t.\nProof.\n  induction t as [| l ihl v r ihr].\n  - reflexivity.\n  - simpl. rewrite ihl, ihr. reflexivity.\nQed.\nDefinition sample : Tree := node leaf 1 leaf.\n`],
  'rocq-induction': ['v', `${rocqHead}Fixpoint add (n m : nat) : nat := match n with | O => m | S k => S (add k m) end.\nTheorem add_zero (n : nat) : add n 0 = n.\nProof.\n  induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.\nQed.\nTheorem add_two : add 1 1 = 2.\nProof. reflexivity. Qed.\n`],
  'rocq-modules': ['v', `${rocqHead}${rocqShow}Module Arith.\n\nDefinition double (n : N) : N := (n + n)%N.\n\nModule Inner.\n\nDefinition quad (n : N) : N := double (double n).\n\nEnd Inner.\n\nEnd Arith.\n\nDefinition main : list string :=\n  show_N (Arith.Inner.quad 2) ::\n  let x := Arith.double 5 in\n  show_N x ::\n  nil.\n\nEval vm_compute in main.\n`],
  'rocq-strings': ['v', `${rocqHead}${rocqShow}Definition classify (n : Z) : string :=\n  if (n <? 0)%Z then "negative" else if (n =? 0)%Z then "zero" else "positive".\nDefinition describe (n : N) : string :=\n  let doubled := (n + n)%N in\n  show_N n ++ " doubled is " ++ show_N doubled ++ " (" ++ classify (Z.of_N doubled - 10) ++ ")".\nDefinition main : list string :=\n  describe 3 ::\n  classify (-4) ::\n  nil.\n\nEval vm_compute in main.\n`],

  // JavaScript.
  'js-edge-division': ['mjs', `${jsFn('quot', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a / b;')}${jsFn('rem', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a % b;')}console.log(\`\${quot(-7n, 2n)} \${rem(-7n, 2n)}\`);\n`],
  'js-guard-negate': ['mjs', jsFn('f', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('f expects a natural number');\n  return -n + (n - 1n);")],
  'js-concat': ['mjs', `${jsFn('f', [['n', 'bigint']], 'string', "  return 'n=' + n + true;")}console.log(f(3n));\n`],
  'js-print': ['mjs', 'console.log(1n + 2n);\nconsole.log(`${3n}`);\nconsole.log(String(4n));\nconsole.log();\nconsole.log(true);\nconsole.log(0x1fn);\nconsole.log(\'quote " backslash \\\\ tab\\tend\');\n'],
  'js-switch-literal': ['mjs', jsFn('f', [['s', 'bigint']], 'bigint', "  switch (s) {\n    case 1n:\n      return 1n;\n    default:\n      return 2n;\n  }")],
  'js-data': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsLeaf}${jsNode}${jsFn('size', [['t', 'Tree']], 'bigint', "  switch (t.$) {\n    case 'leaf':\n      return 0n;\n    default:\n      return size(t.left) + 1n + size(t.right);\n  }")}const t = node(leaf(), 2n, leaf());\nassert.deepStrictEqual(t, node(leaf(), 2n, leaf()));\nassert.notDeepStrictEqual(t, leaf());\nassert(size(t) === 1n && size(leaf()) === 0n);\nassert.equal(size(t), 1n);\nassert.notEqual(size(t), 2n);\nconsole.log(size(t));\n`],
  'js-keyword-names': ['mjs', `${jsFn('of', [['get', 'bigint']], 'bigint', '  return get;')}${jsFn('constructor', [['set', 'bigint']], 'bigint', '  return of(set);')}${jsFn('$dollar', [['async', 'bigint']], 'bigint', '  return async;')}${jsFn('_x', [['_$', 'bigint']], 'bigint', '  return _$;')}console.log(constructor(1n) + $dollar(2n) + _x(3n));\n`],
  'js-throw': ['mjs', `${jsFn('f', [['n', 'bigint']], 'bigint', "  if (n === 0n) throw new Error('zero is not allowed');\n  return n;")}console.log(f(1n));\n`],
  'js-const-main': ['mjs', `${jsFn('sq', [['n', 'bigint']], 'bigint', '  return n * n;')}const a = sq(3n);\nconst b = sq(a);\nconsole.log(\`\${a} \${b}\`);\n`],
  // Rust target: names that clash with Rust keywords, prelude names and each other.
  'lean-rust-keyword-names': ['lean', `def impl (loop : Nat) : Nat := loop\ndef abstract (ref mut : Nat) : Nat := ref + mut\ndef trait (unsafe : Nat) : Nat := unsafe\ndef dyn (const static : Nat) : Nat := const + static\ndef ml (std core : Nat) : Nat := std + core\ndef Big (Vec : Nat) : Nat := Vec\ndef Some (u8 : Nat) : Nat := u8\ndef Result (str Ok : Nat) : Nat := str + Ok\ndef union (raw yield : Nat) : Nat := raw + yield\n\ndef main : IO Unit := do\n  let box := impl 1\n  let move := abstract box 2\n  IO.println s!"{trait move} {dyn 1 2} {ml 3 4} {Big 5} {Some 6} {Result 7 8} {union 8 9}"\n`],
  'lean-rust-keyword-namespaces': ['lean', `namespace Std'\n\ndef enum (n : Nat) : Nat := n\n\nend Std'\n\nnamespace Box\n\ndef struct (n : Nat) : Nat := n + 1\n\nnamespace ml\n\ndef pub (n : Nat) : Nat := Box.struct n\n\nend ml\n\nend Box\n\ndef main : IO Unit := do\n  IO.println s!"{Std'.enum 1} {Box.ml.pub 2}"\n`],
  'lean-snake-camel': ['lean', `def myHTTPServer (fooBar : Nat) : Nat := fooBar\ndef ABCdef (x2Y : Nat) : Nat := x2Y\ndef getURLForID (aBCDe : Nat) : Nat := aBCDe\ndef already_snake (__x : Nat) : Nat := __x\ndef αβ (γ : Nat) : Nat := γ\ndef x1 (_y : Nat) : Nat := _y\n\ndef main : IO Unit := do\n  IO.println s!"{myHTTPServer 1} {ABCdef 2} {getURLForID 3} {already_snake 4} {αβ 5} {x1 6}"\n`],
  'lean-snake-collisions': ['lean', `def fooBar (n : Nat) : Nat := n\ndef foo_bar (n : Nat) : Nat := fooBar n + 1\ndef FooBar (n : Nat) : Nat := foo_bar n + 1\n\nnamespace MyMod\n\ndef f (n : Nat) : Nat := n\n\nend MyMod\n\nnamespace my_mod\n\ndef f (n : Nat) : Nat := MyMod.f n\n\nend my_mod\n\ndef main : IO Unit := do\n  IO.println s!"{FooBar 1} {my_mod.f 2}"\n`],
  'lean-type-name-clashes': ['lean', `inductive Vec' where\n  | nil : Vec'\n  | cons : Nat → Vec' → Vec'\n\ninductive my_type where\n  | some : Nat → my_type\n  | none : my_type\n  | Big : my_type\n\ndef len : Vec' → Nat\n  | .nil => 0\n  | .cons _ rest => len rest + 1\n\ndef get : my_type → Nat\n  | .some n => n\n  | .none => 0\n  | .Big => 1\n\ndef main : IO Unit := do\n  IO.println s!"{len (.cons 1 (.cons 2 .nil))} {get (.some 3)} {get .Big}"\n`],
  'lean-type-and-function-share-name': ['lean', `inductive Color where\n  | red : Color\n  | blue : Color\n\ndef color (c : Color) : Nat :=\n  match c with\n  | .red => 1\n  | .blue => 2\n\ndef Color' (n : Nat) : Color := if n == 0 then .red else .blue\n\ntheorem color_red : color .red = 1 := by decide\n\ndef main : IO Unit := do\n  IO.println s!"{color (Color' 1)}"\n`],

  // Rust target: the ml::Big prelude and literals.
  'lean-big-literals': ['lean', `def below : Nat := 85070591730234615865843651857942052863\ndef at : Nat := 85070591730234615865843651857942052864\ndef huge : Nat := 123456789012345678901234567890123456789012345678901234567890\ndef negBelow : Int := -85070591730234615865843651857942052864\ndef negAt : Int := -85070591730234615865843651857942052865\ndef negHuge : Int := -123456789012345678901234567890123456789012345678901234567890\n\ndef main : IO Unit := do\n  IO.println s!"{below} {at} {huge} {negBelow} {negAt} {negHuge} {huge * huge}"\n`],
  'lean-no-big': ['lean', `def pick (b : Bool) (s : String) : String := if b then s else "no"\n\ndef main : IO Unit := do\n  IO.println (pick true "only strings and booleans")\n`],
  'lean-sub-computed-left': ['lean', `def f (a b : Nat) : Nat := (a * 2) - b\ndef g (n : Nat) : Nat := (match n with | 0 => 5 | k + 1 => k) - 1\ndef h (a b : Nat) : Nat := (match a with | 0 => 1 | k + 1 => k) / (b + 1) + (match a with | 0 => 1 | k + 1 => k) % 3\n\ndef main : IO Unit := do\n  IO.println s!"{f 3 1} {g 4} {h 5 1}"\n`],
  'lean-int-computed-ops': ['lean', `def f (a b : Int) : Int := -(a + b) * (a - b)\ndef g (a b : Int) : Int := (a * 3) / (b - 1) + (a * 3) % (b - 1)\ndef flip (a b : Bool) : Bool := !(a && b)\n\ndef main : IO Unit := do\n  IO.println s!"{f 3 1} {g 7 3} {flip true false}"\n`],
  'lean-clamp-cast': ['lean', `def clamp (i : Int) : Nat := Int.toNat i\ndef clampSum (a b : Int) : Nat := Int.toNat (a + b) + Int.toNat (a - b)\ndef widen (n : Nat) : Int := Int.ofNat (n * 2)\n\ndef main : IO Unit := do\n  IO.println s!"{clamp (-3)} {clamp 5} {clampSum 2 7} {widen 4}"\n`],
  'lean-nat-match-shared-subject': ['lean', `def f (n : Nat) : Nat :=\n  match n * 2 with\n  | 0 => 0\n  | _ => 1\ndef g (n : Nat) : Nat :=\n  match n + 3 with\n  | k => k\ndef h (n : Nat) : Nat :=\n  match n + 1 with\n  | k + 1 => k + (match k with | 0 => 1 | j + 1 => j)\n  | m => m\n\ndef main : IO Unit := do\n  IO.println s!"{f 1} {g 2} {h 3}"\n`],
  'lean-int-match-fallback': ['lean', `def f (i : Int) : Int :=\n  match i with\n  | k => k * 2\ndef g (s : String) : String :=\n  match s with\n  | _ => "any"\n\ndef main : IO Unit := do\n  let x := g "x"\n  IO.println s!"{f 3} {x}"\n`],
  // The control characters are written into the source raw: the frontends accept only the common escapes.
  'lean-string-control-chars': ['lean', 'def main : IO Unit := do\n  IO.println "bell \u0007 escape \u001b delete \u007f braces {} end"\n  IO.println "cr\\rlf"\n'],
  'lean-theorem-keyword-name': ['lean', `def f (n : Nat) : Nat := n\n\ntheorem impl (n : Nat) : f n = n := by simp [f]\n${leanMain}`],
  'lean-theorem-deep-data-domain': ['lean', `inductive Expr where\n  | num : Nat → Expr\n  | neg : Expr → Expr\n  | add : Expr → Expr → Expr\n  | flag : Bool → Int → Expr\n\ndef eval : Expr → Int\n  | .num n => Int.ofNat n\n  | .neg e => -(eval e)\n  | .add a b => eval a + eval b\n  | .flag b i => if b then i else 0\n\ntheorem eval_neg (e : Expr) : eval (.neg e) = -(eval e) := by simp [eval]\n\ntheorem eval_add (a b : Expr) : eval (.add a b) = eval a + eval b := by simp [eval]\n\ndef main : IO Unit := do\n  IO.println s!"{eval (.add (.num 2) (.neg (.num 5)))}"\n`],
  'lean-data-across-namespaces': ['lean', `namespace Geo\n\ninductive Point where\n  | pt : Int → Int → Point\n\nnamespace Ops\n\ndef shift (p : Point) (d : Int) : Point :=\n  match p with\n  | .pt x y => .pt (x + d) (y + d)\n\nend Ops\n\nend Geo\n\nnamespace Draw\n\ninductive Path where\n  | stop : Path\n  | step : Geo.Point → Path → Path\n\ndef length : Path → Nat\n  | .stop => 0\n  | .step _ rest => length rest + 1\n\ndef last : Path → Geo.Point\n  | .stop => .pt 0 0\n  | .step p .stop => p\n  | .step _ rest => last rest\n\nend Draw\n\ntheorem shift_zero (x y : Int) : Geo.Ops.shift (.pt x y) 0 = .pt x y := by simp [Geo.Ops.shift]\n\ndef main : IO Unit := do\n  IO.println s!"{Draw.length (.step (.pt 1 2) (.step (Geo.Ops.shift (.pt 0 0) 3) .stop))}"\n`],

  // Rust sources: Rust-target-specific machine-integer and data shapes.
  'rust-euclid-unsigned': ['rs', `fn f(x: u32, y: u32) -> u32 { x.div_euclid(y) + x.rem_euclid(y) }\nfn g(x: u8, y: u8) -> u8 { x / y + x % y }\nfn h(x: i16, y: i16) -> i16 { (x + 1).div_euclid(y - 1) }\nfn main() { println!("{} {} {}", f(7, 2), g(7, 2), h(-7, 3)); }\n`],
  'rust-checked-computed': ['rs', `fn f(a: i32, b: i32) -> i32 { -(a + b) * (a - b) }\nfn g(a: u16, b: u16) -> u16 { (a * 2) / (b + 1) + (a * 2) % (b + 1) }\nfn main() { println!("{} {}", f(3, 1), g(10, 2)); }\n`],
  'rust-all-widths-ops': ['rs', `fn a(x: u8) -> u8 { x + 1 }\nfn b(x: i8) -> i8 { x - 1 }\nfn c(x: u16) -> u16 { x * 2 }\nfn d(x: i16) -> i16 { -x }\nfn e(x: u32) -> u32 { x / 3 }\nfn f(x: i32) -> i32 { x % 3 }\nfn g(x: u64) -> u64 { x + 1 }\nfn h(x: i64) -> i64 { x - 1 }\nfn i(x: u128) -> u128 { x * 2 }\nfn j(x: i128) -> i128 { -x }\nfn k(x: usize) -> usize { x / 2 }\nfn l(x: isize) -> isize { x % 2 }\nfn main() { println!("{} {} {} {} {} {} {} {} {} {} {} {}", a(1), b(1), c(1), d(1), e(9), f(-7), g(1), h(1), i(1), j(1), k(9), l(-9)); }\n`],
  'rust-widening-all': ['rs', `fn a(n: u8) -> u16 { u16::from(n) }\nfn b(n: u16) -> u32 { u32::from(n) }\nfn c(n: u32) -> u64 { u64::from(n) }\nfn d(n: u64) -> u128 { u128::from(n) }\nfn e(n: i8) -> i16 { i16::from(n) }\nfn f(n: u8) -> i32 { i32::from(n) }\nfn g(n: i32) -> i64 { n as i64 }\nfn main() { println!("{} {} {} {} {} {} {}", a(1), b(2), c(3), d(4), e(-5), f(6), g(-7)); }\n`],
  'rust-unsigned-match-computed': ['rs', `fn f(n: u32) -> u32 {\n    match n + 1 {\n        0 => 0,\n        k => k * 2,\n    }\n}\nfn g(n: u8) -> u8 {\n    match n * 2 {\n        0 => 1,\n        _ => 2,\n    }\n}\nfn main() { println!("{} {}", f(3), g(4)); }\n`],
  'rust-data-boxed-fields': ['rs', `#[derive(Clone, Debug, PartialEq, Eq)]\npub enum List { Nil, Cons(i64, Box<List>) }\n\n#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Pair { Both(Box<List>, Box<List>), One(bool) }\n\nfn sum(l: &List) -> i64 {\n    match l {\n        List::Nil => 0,\n        List::Cons(h, t) => *h + sum(t),\n    }\n}\n\nfn pick(p: &Pair) -> i64 {\n    match p {\n        Pair::Both(a, _) => sum(a),\n        Pair::One(_) => 0,\n    }\n}\n\nfn main() {\n    let l = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));\n    let p = Pair::Both(Box::new(l.clone()), Box::new(List::Nil));\n    assert_eq!(sum(&l), 3);\n    assert!(p != Pair::One(true));\n    println!("{} {}", sum(&l), pick(&p));\n}\n`],
  'rust-data-in-modules': ['rs', `mod shapes {\n    #[derive(Clone, Debug, PartialEq, Eq)]\n    pub enum Shape { Square(u32), Line(u32, u32) }\n\n    pub mod area {\n        use super::Shape;\n\n        pub fn of(s: &Shape) -> u32 {\n            match s {\n                Shape::Square(n) => *n * *n,\n                Shape::Line(_, _) => 0,\n            }\n        }\n    }\n}\n\nfn main() { println!("{}", shapes::area::of(&shapes::Shape::Square(3))); }\n`],
  'rust-keyword-prelude-names': ['rs', `fn ml(big: u64) -> u64 { big }\nfn vec(string: u64) -> u64 { string }\nmod ml { pub fn range(big: u64) -> u64 { big } }\n#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Maybe { Some(u64), None }\nfn get(m: &Maybe) -> u64 { match m { Maybe::Some(n) => *n, Maybe::None => 0 } }\nfn main() { println!("{} {} {} {}", ml(1), vec(2), ml::range(3), get(&Maybe::Some(4))); }\n`],
  'rust-string-escapes': ['rs', `fn main() {\n    println!("{}", "bell \u0007 esc \u001b del \u007f tab\\t cr\\r end");\n    println!("{}", "braces {} and {{}} quote \\" backslash \\\\");\n    println!("{}", String::from("unicode λ → 😀"));\n}\n`],
  'rust-to-string': ['rs', `fn show(n: u32, b: bool, i: i64) -> String { format!("{}-{}-{}", n.to_string(), b, (i - 1).to_string()) }\nfn main() { println!("{}", show(1, true, -2)); }\n`],

  // JavaScript sources: checked conversions and abort messages.
  'js-checked-conversion': ['mjs', `${jsFn('half', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('half expects a natural number');\n  return n / 2n;")}${jsFn('f', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return half(a - b) + half(a * b);')}console.log(f(9n, 3n));\n`],
  'js-nat-sub-guarded': ['mjs', `${jsFn('monus', [['a', 'bigint'], ['b', 'bigint']], 'bigint', "  if (a < 0n) throw new RangeError('monus expects a natural number');\n  if (b < 0n) throw new RangeError('monus expects a natural number');\n  return a - b;")}console.log(monus(3n, 5n));\n`],
  'js-control-chars': ['mjs', "console.log('bell \u0007 del \u007f start \u0001 esc \u001b {braces}');\nconsole.log(\"cr\\r end\");\n"],
  'js-abort-message': ['mjs', `${jsFn('f', [['n', 'bigint']], 'bigint', '  if (n === 0n) throw new Error(\'zero {} is "not" allowed\\\\\');\n  return n;')}console.log(f(1n));\n`],
  'js-assert-data-props': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsLeaf}${jsNode}const t = node(leaf(), 2n, leaf());\nassert.notDeepStrictEqual(t, leaf());\nassert(1n < 2n && (2n <= 2n || 3n > 4n));\nassert.equal('a' + 'b', 'ab');\nconsole.log('ok');\n`],

  // Rocq sources: clamped conversions and floor division.
  'rocq-conversions': ['v', `${rocqHead}Definition up (n : nat) : Z := Z.of_nat n.\nDefinition down (z : Z) : nat := Z.to_nat z.\nDefinition nn (n : nat) : N := N.of_nat n.\nDefinition floor_div (a b : Z) : Z := (a / b)%Z.\nDefinition floor_mod (a b : Z) : Z := (a mod b)%Z.\nDefinition ndiv (a b : nat) : nat := a / b.\n`],
  'rocq-theorems': ['v', `${rocqHead}Definition double (n : nat) : nat := n + n.\nTheorem double_zero : double 0 = 0.\nProof. reflexivity. Qed.\nTheorem double_le (n : nat) : n <= double n.\nProof. unfold double. lia. Qed.\nTheorem z_self (z : Z) : (z + 0 = z)%Z.\nProof. lia. Qed.\n`],
};

export default Object.fromEntries(
  Object.entries(cases).map(([name, value]) => [`emit-rust-${name}`, value]),
);
