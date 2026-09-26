// Small programs that exercise every path of the Lean emitter (helpers,
// encodings, arithmetic in each domain and rounding, casts, strings, data
// types across modules, name clashes, recursion forms, theorems and their
// proof plans, assertions, main effects and unsupported constructs), as
// name → [source extension, source text].

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
const jsTree = "/**\n * @typedef {{ $: 'leaf' } | { $: 'node', left: Tree, value: bigint, right: Tree }} Tree\n */\n";
const jsFn = (name, params, ret, body) => `/**\n${params.map(([p, t]) => ` * @param {${t}} ${p}\n`).join('')} * @returns {${ret}}\n */\nfunction ${name}(${params.map(([p]) => p).join(', ')}) {\n${body}\n}\n`;
const jsNat = (name) => `  if (${name} < 0n) throw new RangeError('expects a natural number');\n`;
const leanSize = `def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r
`;
const leanMirror = `def mirror : Tree → Tree
  | .leaf => .leaf
  | .node l v r => .node (mirror r) v (mirror l)
`;
const leanAdd = `def add : Nat → Nat → Nat
  | 0, m => m
  | n + 1, m => add n m + 1
`;

export default {
  // Lean sources: arithmetic in each domain.
  'emit-lean-nat-arith': ['lean', `def f (a b : Nat) : Nat := a + b * 2 - a / b + a % b${leanMain}`],
  'emit-lean-int-arith': ['lean', `def f (a b : Int) : Int := a + b * 2 - a / b + a % b - (-a)${leanMain}`],
  'emit-lean-int-literals': ['lean', `def f : Int := -7 / 2 + (-7) % 3\ndef g : Nat := 0\ndef h : Int := (0 - 7)${leanMain}`],
  'emit-lean-nat-sub': ['lean', `def monus (a b : Nat) : Nat := a - b\ndef main : IO Unit := do\n  IO.println s!"{monus 3 5}"\n`],
  'emit-lean-comparisons': ['lean', `def f (a b : Nat) : Bool := a < b && a ≤ b || a > b && a ≥ b || a == b || a != b\ndef g (a b : Int) : Bool := !(a < b)${leanMain}`],
  'emit-lean-bool-ops': ['lean', `def f (a b : Bool) : Bool := (a && b) || !a\ndef g (a : Bool) : Bool := a == true${leanMain}`],
  'emit-lean-casts': ['lean', `def f (n : Nat) : Int := Int.ofNat n - 10\ndef g (i : Int) : Nat := Int.toNat i\ndef h (n : Nat) : Nat := Int.toNat (Int.ofNat n)${leanMain}`],
  'emit-lean-strings': ['lean', `def greet (s : String) : String := "hello, " ++ s ++ "!"\ndef quote : String := "quote \\" backslash \\\\ tab\\tend"\ndef show (n : Nat) (b : Bool) (i : Int) : String := s!"{n} {b} {i}"\ndef main : IO Unit := do\n  IO.println (greet "world")\n  IO.println quote\n  IO.println (show 1 true (-2))\n  IO.println (toString 5)\n`],
  'emit-lean-string-eq': ['lean', `def same (a b : String) : Bool := a == b\ndef f (s : String) : Nat := if s == "a" then 1 else if s != "b" then 2 else 3${leanMain}`],
  'emit-lean-unicode-names': ['lean', `def αβ (x : Nat) : Nat := x + 1\ndef f' (x : Nat) : Nat := αβ x\ndef main : IO Unit := do\n  IO.println s!"{f' 1}"\n`],
  'emit-lean-let': ['lean', `def f (n : Nat) : Nat :=\n  let a := n + 1\n  let b := a * a\n  b - a${leanMain}`],
  'emit-lean-nested-let-match': ['lean', `def f (n : Nat) : Nat :=\n  let a := n + 1\n  match a with\n  | 0 => 0\n  | k + 1 =>\n    let b := k * 2\n    b + a${leanMain}`],
  'emit-lean-if-chain': ['lean', `def classify (n : Int) : String :=\n  if n < 0 then "negative" else if n == 0 then "zero" else "positive"\ndef main : IO Unit := do\n  IO.println (classify (-4))\n  IO.println (classify 0)\n`],
  'emit-lean-bool-match': ['lean', `def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n  | false => 0${leanMain}`],
  'emit-lean-int-match': ['lean', `def f (i : Int) : Nat :=\n  match i with\n  | 0 => 1\n  | 1 => 2\n  | _ => 3${leanMain}`],
  'emit-lean-multi-scrutinee': ['lean', `def f : Nat → Nat → Nat\n  | 0, _ => 0\n  | _, 0 => 1\n  | n + 1, m + 1 => f n m${leanMain}`],

  // Lean sources: recursion forms.
  'emit-lean-structural-nat': ['lean', `def fact : Nat → Nat\n  | 0 => 1\n  | n + 1 => (n + 1) * fact n\ndef main : IO Unit := do\n  IO.println s!"{fact 5}"\n`],
  'emit-lean-successor-substitution': ['lean', `def sumTo (n : Nat) : Nat :=\n  match n with\n  | 0 => 0\n  | k + 1 => sumTo k + n${leanMain}`],
  'emit-lean-fib': ['lean', `def fib : Nat → Nat\n  | 0 => 0\n  | 1 => 1\n  | n + 2 => fib (n + 1) + fib n${leanMain}`],
  'emit-lean-if-zero-split': ['lean', `def fact (n : Nat) : Nat := if n == 0 then 1 else n * fact (n - 1)${leanMain}`],
  'emit-lean-structural-second': ['lean', `def f (a : Nat) : Nat → Nat\n  | 0 => a\n  | n + 1 => f a n + 1${leanMain}`],
  'emit-lean-structural-tree': ['lean', `${leanTree}\n${leanSize}\n${leanMirror}${leanMain}`],
  'emit-lean-general-recursion': ['lean', `def collatz (n : Nat) (fuel : Nat) : Nat :=\n  if n ≤ 1 then 0 else if n % 2 == 0 then collatz (n / 2) fuel + 1 else collatz (3 * n + 1) fuel + 1${leanMain}`],

  // Lean sources: data types, modules and names.
  'emit-lean-data-fields': ['lean', `inductive Shape where\n  | circle (radius : Nat) : Shape\n  | rect (w : Nat) (h : Nat) : Shape\n  | dot : Shape\ndef area : Shape → Nat\n  | .circle r => 3 * r * r\n  | .rect w h => w * h\n  | .dot => 0${leanMain}`],
  'emit-lean-data-unnamed': ['lean', `${leanTree}\ndef sample : Tree := Tree.node Tree.leaf 5 (.node .leaf 3 .leaf)\ndef isLeaf (t : Tree) : Bool :=\n  match t with\n  | .leaf => true\n  | _ => false${leanMain}`],
  'emit-lean-data-wild-binds': ['lean', `${leanTree}\ndef root : Tree → Nat\n  | .node _ v _ => v\n  | .leaf => 0${leanMain}`],
  'emit-lean-data-types-mixed': ['lean', `inductive Item where\n  | named : String → Int → Bool → Item\n  | none : Item\ndef label : Item → String\n  | .named s _ _ => s\n  | .none => "none"${leanMain}`],
  'emit-lean-namespaces': ['lean', `namespace A\ndef f (n : Nat) : Nat := n + 1\nnamespace B\ndef g (n : Nat) : Nat := f n * 2\nend B\ndef h : Nat := B.g 1\nend A\nnamespace C\ndef k : Nat := A.h + A.B.g 2\nend C\ndef main : IO Unit := do\n  IO.println s!"{C.k}"\n`],
  'emit-lean-data-namespace': ['lean', `${leanTree}\nnamespace Tree\n${leanSize}\n${leanMirror}end Tree\ndef main : IO Unit := do\n  IO.println s!"{Tree.size (Tree.mirror (.node .leaf 1 .leaf))}"\n`],
  'emit-lean-data-in-module': ['lean', `namespace M\ninductive Color where\n  | red : Color\n  | green : Color\ndef pick (b : Bool) : Color := if b then .red else .green\nend M\ndef code (c : M.Color) : Nat :=\n  match c with\n  | .red => 1\n  | .green => 2\ndef main : IO Unit := do\n  IO.println s!"{code (M.pick true)}"\n`],
  'emit-lean-reorder': ['lean', `namespace A\ndef f : Nat := 1\nend A\nnamespace B\ndef g : Nat := A.f + 1\nend B\nnamespace A\ndef h : Nat := B.g + 1\nend A${leanMain}`],
  'emit-lean-root-names': ['lean', `def List (n : Nat) : Nat := n\ndef Option : Nat := 3\ndef f : Nat := List Option${leanMain}`],
  'emit-lean-eq1-clash': ['lean', `namespace f\ndef eq_1 : Nat := 1\nend f\ndef f (n : Nat) : Nat := n + f.eq_1${leanMain}`],
  'emit-lean-type-name-clash': ['lean', `inductive Nat' where\n  | z : Nat'\ndef toString' (n : Nat) : String := toString n${leanMain}`],

  // Lean sources: theorems and every proof-plan path.
  'emit-lean-theorem-decide': ['lean', `def fact : Nat → Nat\n  | 0 => 1\n  | n + 1 => (n + 1) * fact n\ntheorem fact_five : fact 5 = 120 := by decide${leanMain}`],
  'emit-lean-theorem-rfl': ['lean', `def two : Nat := 2\ntheorem two_eq : two = 2 := by rfl${leanMain}`],
  'emit-lean-theorem-omega': ['lean', `theorem le_succ (n : Nat) : n ≤ n + 1 := by omega${leanMain}`],
  'emit-lean-theorem-simp': ['lean', `def double (n : Nat) : Nat := n + n\ntheorem double_eq (n : Nat) : double n = 2 * n := by simp [double]; omega${leanMain}`],
  'emit-lean-theorem-unfold': ['lean', `def double (n : Nat) : Nat := n + n\ntheorem double_eq (n : Nat) : double n = n + n := by unfold double; rfl${leanMain}`],
  'emit-lean-theorem-induction-nat': ['lean', `${leanAdd}\ntheorem add_zero (n : Nat) : add n 0 = n := by\n  induction n with\n  | zero => rfl\n  | succ k ih => simp [add, ih]${leanMain}`],
  'emit-lean-theorem-induction-tree': ['lean', `${leanTree}\n${leanSize}\n${leanMirror}\ntheorem size_mirror (t : Tree) : size (mirror t) = size t := by\n  induction t with\n  | leaf => rfl\n  | node l v r ihl ihr => simp [size, mirror, ihl, ihr]; omega${leanMain}`],
  'emit-lean-theorem-cases': ['lean', `${leanTree}\n${leanSize}\ntheorem size_nonneg (t : Tree) : 0 ≤ size t := by\n  cases t with\n  | leaf => simp [size]\n  | node l v r => simp [size]${leanMain}`],
  'emit-lean-theorem-cases-nat': ['lean', `def pred (n : Nat) : Nat := n - 1\ntheorem pred_le (n : Nat) : pred n ≤ n := by\n  cases n with\n  | zero => rfl\n  | succ k => simp [pred]${leanMain}`],
  'emit-lean-theorem-lemma': ['lean', `${leanAdd}\ntheorem add_zero (n : Nat) : add n 0 = n := by\n  induction n with\n  | zero => rfl\n  | succ k ih => simp [add, ih]\ntheorem add_zero_twice (n : Nat) : add (add n 0) 0 = n := by simp [add_zero]${leanMain}`],
  'emit-lean-theorem-library': ['lean', `def sq (n : Nat) : Nat := n * n\ntheorem sq_add (a : Nat) : sq (a + 1) = sq a + 2 * a + 1 := by simp [sq, Nat.mul_add, Nat.add_mul]; omega${leanMain}`],
  'emit-lean-theorem-props': ['lean', `def f (n : Nat) : Nat := n + 1\ntheorem props (a b : Nat) : (f a > a ∧ f b ≥ b) ∨ (¬ (a = b) → a ≠ b) := by omega${leanMain}`],
  'emit-lean-theorem-forall': ['lean', `def f (n : Nat) : Nat := n + 1\ntheorem all (a : Nat) : a < f a → ∀ (b : Nat), b < f b := by simp [f]${leanMain}`],
  'emit-lean-theorem-bool': ['lean', `def even (n : Nat) : Bool := n % 2 == 0\ntheorem even_four : even 4 = true := by decide\ntheorem even_two : even 2 := by decide${leanMain}`],
  'emit-lean-theorem-int': ['lean', `def neg (i : Int) : Int := -i\ntheorem neg_neg (i : Int) : neg (neg i) = i := by simp [neg]${leanMain}`],
  'emit-lean-theorem-namespace': ['lean', `namespace M\ndef two : Nat := 2\ntheorem two_pos : 0 < two := by decide\nend M\ntheorem uses : 0 < M.two := by simp [M.two_pos]${leanMain}`],
  'emit-lean-theorem-trailing': ['lean', `${leanAdd}\ntheorem add_succ (n m : Nat) : add n (m + 1) = add n m + 1 := by\n  induction n with\n  | zero => rfl\n  | succ k ih => simp [add, ih]${leanMain}`],
  'emit-lean-theorem-nested-split': ['lean', `${leanTree}\n${leanSize}\ntheorem nested (t : Tree) (n : Nat) : size t + n ≥ n := by\n  cases t with\n  | leaf =>\n    cases n with\n    | zero => rfl\n    | succ k => omega\n  | node l v r => simp [size]; omega${leanMain}`],

  // Lean sources: main effects and assertions.
  'emit-lean-main-lets': ['lean', `def f (n : Nat) : Nat := n * 2\ndef main : IO Unit := do\n  let a := f 3\n  let b := a + 1\n  IO.println s!"{a} {b}"\n  IO.println (toString (a * b))\n`],
  'emit-lean-no-main': ['lean', 'def f (n : Nat) : Nat := n\n'],

  // Rust sources: machine integers.
  'emit-lean-rust-unsigned': ['rs', `fn f(a: u8, b: u8) -> u8 { a + b * 2 - a / b + a % b }${rustMain}`],
  'emit-lean-rust-signed': ['rs', `fn f(a: i8, b: i8) -> i8 { a + b * 2 - a / b + a % b }${rustMain}`],
  'emit-lean-rust-negate': ['rs', `fn f(x: i32) -> i32 { -x }\nfn g() -> i64 { -9223372036854775808 }${rustMain}`],
  'emit-lean-rust-euclid': ['rs', `fn f(x: i64, y: i64) -> i64 { x.div_euclid(y) + x.rem_euclid(y) }\nfn g(x: u32, y: u32) -> u32 { x.div_euclid(y) + x.rem_euclid(y) }${rustMain}`],
  'emit-lean-rust-wide': ['rs', `fn f(a: u128, b: u128) -> u128 { a * b }\nfn g(a: i128, b: i128) -> i128 { a - b }\nfn h(a: u64, b: u64) -> u64 { a / b }\nfn k(a: usize, b: isize) -> isize { b % 3 }${rustMain}`],
  'emit-lean-rust-widening': ['rs', `fn f(n: u32) -> i64 { i64::from(n) + (n as i64) }\nfn g(n: u8) -> u64 { u64::from(n) }\nfn h(n: i16) -> i32 { n as i32 }${rustMain}`],
  'emit-lean-rust-panic': ['rs', `fn f(n: u64) -> u64 { if n == 0 { panic!("zero") } else { n } }\nfn g(n: u64) -> u64 { if n > 5 { unreachable!() } else { n } }${rustMain}`],
  'emit-lean-rust-recursion': ['rs', `fn fact(n: u64) -> u64 { if n == 0 { 1 } else { n * fact(n - 1) } }\nfn fib(n: u32) -> u32 { if n == 0 { 0 } else if n == 1 { 1 } else { fib(n - 1) + fib(n - 2) } }\nfn main() { println!("{} {}", fact(5), fib(10)); }\n`],
  'emit-lean-rust-tree': ['rs', `${rustTree}\nfn size(t: &Tree) -> u64 { match t { Tree::Leaf => 0, Tree::Node(l, _, r) => size(l) + 1 + size(r) } }\nfn mirror(t: &Tree) -> Tree { match t { Tree::Leaf => Tree::Leaf, Tree::Node(l, v, r) => Tree::Node(Box::new(mirror(r)), *v, Box::new(mirror(l))) } }\nfn main() { println!("{}", size(&mirror(&Tree::Node(Box::new(Tree::Leaf), 1, Box::new(Tree::Leaf))))); }\n`],
  'emit-lean-rust-modules': ['rs', `mod a { pub mod b { pub fn g() -> u64 { super::h() } } pub fn h() -> u64 { crate::k() } }\nfn k() -> u64 { 7 }\nfn main() { println!("{}", a::b::g()); }\n`],
  'emit-lean-rust-data-module': ['rs', `mod shapes {\n    #[derive(Clone, Debug, PartialEq, Eq)]\n    pub enum Shape { Circle(u64), Rect(u64, u64) }\n    pub fn area(s: &Shape) -> u64 { match s { Shape::Circle(r) => 3 * r * r, Shape::Rect(w, h) => w * h } }\n}\nfn main() { println!("{}", shapes::area(&shapes::Shape::Rect(2, 3))); }\n`],
  'emit-lean-rust-strings': ['rs', `fn greet(name: String) -> String { format!("hello, {name}!") }\nfn main() { let x: u8 = 7; println!("{} {} {}", x, true, greet(String::from("w"))); println!("{}", -3i64); }\n`],
  'emit-lean-rust-asserts': ['rs', `fn f(n: u64) -> u64 { n + 1 }\nfn main() {\n    let a = f(1);\n    assert_eq!(a, 2);\n    assert!(f(2) > a && a != 0);\n    assert_ne!(f(3), 0);\n    println!("{}", a);\n}\n`],
  'emit-lean-rust-keywords': ['rs', `fn end(x: u64) -> u64 { x }\nfn decide(x: u64) -> u64 { end(x) }\nfn theorem(fun: u64, then: u64) -> u64 { decide(fun) + then }${rustMain}`],
  'emit-lean-rust-root-names': ['rs', `fn List(n: u64) -> u64 { n }\nfn Nat(n: u64) -> u64 { List(n) }\nfn main() { println!("{}", Nat(1)); }\n`],
  'emit-lean-rust-mutual': ['rs', `fn even(n: u64) -> bool { if n == 0 { true } else { odd(n - 1) } }\nfn odd(n: u64) -> bool { if n == 0 { false } else { even(n - 1) } }${rustMain}`],
  'emit-lean-rust-general-recursion': ['rs', `fn up(n: u64, limit: u64) -> u64 { if n >= limit { n } else { up(n + 1, limit) } }${rustMain}`],
  'emit-lean-rust-empty-main': ['rs', 'fn f() -> u64 { 1 }\nfn main() {}\n'],

  // Rocq sources: Z rounding, conversions and proofs.
  'emit-lean-rocq-z-floor': ['v', `${rocqHead}Definition f (x y : Z) : Z := (x / y + x mod y - 3)%Z.\nDefinition g (x y : Z) : Z := Z.div x y + Z.modulo x y.\n`],
  'emit-lean-rocq-z-trunc': ['v', `${rocqHead}Definition f (x y : Z) : Z := Z.quot x y + Z.rem x y.\n`],
  'emit-lean-rocq-nat-div': ['v', `${rocqHead}Definition f (a b : nat) : nat := a / b + a mod b + Nat.div a b - a.\n`],
  'emit-lean-rocq-casts': ['v', `${rocqHead}Definition f (n : nat) : Z := Z.of_nat n.\nDefinition g (z : Z) : nat := Z.to_nat z.\n`],
  'emit-lean-rocq-induction': ['v', `${rocqHead}Fixpoint add (n m : nat) : nat := match n with | O => m | S k => S (add k m) end.\nTheorem add_zero (n : nat) : add n 0 = n.\nProof.\n  induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.\nQed.\n`],
  'emit-lean-rocq-tree': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=\n  match t with\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n  end.\nFixpoint mirror (t : Tree) : Tree :=\n  match t with\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n  end.\nTheorem size_mirror (t : Tree) : size (mirror t) = size t.\nProof.\n  induction t as [| l ihl v r ihr].\n  - reflexivity.\n  - simpl. rewrite ihl. rewrite ihr. lia.\nQed.\n`],
  'emit-lean-rocq-destruct': ['v', `${rocqHead}Definition pred (n : nat) : nat := match n with | O => 0 | S k => k end.\nTheorem pred_le (n : nat) : pred n <= n.\nProof.\n  destruct n as [| k].\n  - reflexivity.\n  - simpl. lia.\nQed.\n`],
  'emit-lean-rocq-strings': ['v', `${rocqHead}Definition greet (s : string) : string := "hi " ++ s.\nDefinition flag (b : bool) : bool := andb b (negb b).\n`],

  // JavaScript sources: BigInt division, checked conversions and effects.
  'emit-lean-js-int-division': ['mjs', `${jsFn('quot', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a / b;')}${jsFn('rem', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a % b;')}console.log(\`\${quot(-7n, 2n)} \${rem(-7n, 2n)}\`);\n`],
  'emit-lean-js-nat-division': ['mjs', jsFn('f', [['a', 'bigint'], ['b', 'bigint']], 'bigint', `${jsNat('a')}${jsNat('b')}  const q = a / b;\n  const r = a % b;\n  return q + r;`)],
  'emit-lean-js-checked-nat': ['mjs', `${jsFn('pred', [['n', 'bigint']], 'bigint', `${jsNat('n')}  return n - 1n;`)}${jsFn('twice', [['n', 'bigint']], 'bigint', `${jsNat('n')}  return pred(pred(n));`)}console.log(twice(5n));\n`],
  'emit-lean-js-recursion': ['mjs', `${jsTree}${jsFn('countdown', [['n', 'bigint']], 'Tree', `${jsNat('n')}  if (n === 0n) return { $: 'leaf' };\n  return { $: 'node', left: { $: 'leaf' }, value: n, right: countdown(n - 1n) };`)}${jsFn('sum', [['t', 'Tree']], 'bigint', "  switch (t.$) {\n    case 'leaf':\n      return 0n;\n    default:\n      return t.value + sum(t.right);\n  }")}console.log(sum(countdown(4n)));\n`],
  'emit-lean-js-throw': ['mjs', jsFn('f', [['n', 'bigint']], 'bigint', "  if (n > 10n) throw new Error('too big');\n  return n;")],
  'emit-lean-js-concat': ['mjs', `${jsFn('label', [['n', 'bigint']], 'string', "  return 'n=' + n + ', positive=' + (n > 0n) + '!';")}console.log(label(5n));\nconsole.log('quote \\" backslash \\\\\\\\ tab\\tend');\nconsole.log(\`nested \${\`inner \${1n + 2n}\`} done\`);\nconsole.log();\n`],
  'emit-lean-js-asserts': ['mjs', `import assert from 'node:assert/strict';\n${jsFn('inc', [['n', 'bigint']], 'bigint', '  return n + 1n;')}const a = inc(1n);\nassert.equal(a, 2n);\nassert(inc(a) === 3n && a !== 0n);\nconst b = inc(a);\nassert.equal(b, 3n);\nconsole.log(a + b);\n`],
  'emit-lean-js-deep-equal': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}assert.deepStrictEqual(leaf(), { $: 'leaf' });\n`],
  'emit-lean-js-negate': ['mjs', `${jsFn('neg', [['n', 'bigint']], 'bigint', '  return -n;')}console.log(neg(3n));\nconsole.log(0x1fn);\nconsole.log(true);\n`],
  'emit-lean-js-keywords': ['mjs', `${jsFn('match', [['then', 'bigint']], 'bigint', '  return then + 1n;')}${jsFn('Nat', [['n', 'bigint']], 'bigint', '  return match(n);')}console.log(Nat(1n));\n`],
  'emit-lean-js-mutual': ['mjs', `${jsFn('even', [['n', 'bigint']], 'boolean', `${jsNat('n')}  if (n === 0n) return true;\n  return odd(n - 1n);`)}${jsFn('odd', [['n', 'bigint']], 'boolean', `${jsNat('n')}  if (n === 0n) return false;\n  return even(n - 1n);`)}`],
  // Name clashes: Lean keywords, names Lean derives and root namespaces.
  'emit-lean-derived-names': ['lean', `def mk (n : Nat) : Nat := n\ndef rec (n : Nat) : Nat := mk n\ndef induct (this : Nat) : Nat := rec this\ndef main : IO Unit := do\n  IO.println s!"{induct 1}"\n`],
  'emit-lean-local-clash': ['lean', `def g (n : Nat) : Nat := n\ndef h (n : Nat) : Nat :=\n  let g := n + 1\n  g * 2\ndef k (h : Nat) : Nat := h + g h${leanMain}`],
  'emit-lean-js-tag-keywords': ['mjs', `/**\n * @typedef {{ $: 'end' } | { $: 'if', then: bigint, match: Tok }} Tok\n */\n${jsFn('depth', [['t', 'Tok']], 'bigint', "  switch (t.$) {\n    case 'end':\n      return 0n;\n    default:\n      return t.then + depth(t.match);\n  }")}console.log(depth({ $: 'if', then: 2n, match: { $: 'end' } }));\n`],
  'emit-lean-js-root-names': ['mjs', `${jsFn('List', [['Option', 'bigint']], 'bigint', '  return Option * 2n;')}${jsFn('IO', [['String', 'bigint']], 'bigint', '  return List(String);')}console.log(IO(3n));\n`],

  // Rust sources: more machine-integer widths and comparisons.
  'emit-lean-rust-u16-i16': ['rs', `fn f(a: u16, b: u16) -> u16 { a * b - 1 }\nfn g(a: i16, b: i16) -> i16 { a * b / 3 }${rustMain}`],
  'emit-lean-rust-compare': ['rs', `fn f(a: u8, b: u8) -> bool { a < b || a >= b && a != b }\nfn g(a: i32, b: i32) -> bool { a <= b && !(a > b) || a == b }${rustMain}`],
  'emit-lean-rust-unsigned-sub': ['rs', `fn monus(a: u64, b: u64) -> u64 { if a > b { a - b } else { 0 } }\nfn main() { println!("{}", monus(3, 5)); }\n`],
  'emit-lean-rust-shadow': ['rs', `fn f(n: u64) -> u64 { let n = n + 1; let n = n * 2; n }${rustMain}`],
  'emit-lean-rust-literals': ['rs', `fn f() -> u8 { 255 }\nfn g() -> i8 { -128 }\nfn h() -> u128 { 340282366920938463463374607431768211455 }\nfn k() -> i128 { -170141183460469231731687303715884105728 }${rustMain}`],
  'emit-lean-rust-let-main': ['rs', `fn f(n: u64) -> u64 { n * 3 }\nfn main() {\n    let a = f(2);\n    let s = format!("a={a}");\n    println!("{}", s);\n    assert_eq!(a, 6);\n}\n`],

  // Rocq sources: more arithmetic.
  'emit-lean-rocq-z-ops': ['v', `${rocqHead}Definition f (x : Z) : Z := (- x + 3 * x - 7)%Z.\nDefinition g (x y : Z) : bool := Z.ltb x y.\n`],
  'emit-lean-rocq-fixpoint': ['v', `${rocqHead}Fixpoint fact (n : nat) : nat :=\n  match n with\n  | O => 1\n  | S k => n * fact k\n  end.\nTheorem fact_three : fact 3 = 6.\nProof. reflexivity. Qed.\n`],

  // JavaScript sources: more effects.
  'emit-lean-js-bool': ['mjs', `${jsFn('both', [['a', 'boolean'], ['b', 'boolean']], 'boolean', '  return a && !b || a === b;')}console.log(both(true, false));\n`],
  'emit-lean-js-let-print': ['mjs', `${jsFn('sq', [['n', 'bigint']], 'bigint', '  const m = n * n;\n  return m + 1n;')}const x = sq(3n);\nconsole.log(\`x=\${x}\`);\nconsole.log(String(x - 20n));\n`],
};
