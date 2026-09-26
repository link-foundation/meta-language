// Small programs that exercise the checker's accepting and rejecting paths in
// all four source languages, as name → [source extension, source text].

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

export default {
  'lean-large-numeral-before-succ': ['lean', `def f : Nat → Nat
  | 1000 => 7
  | 0 => 1
  | n + 2 => n
  | _ => 3${leanMain}`],
  'lean-large-numeral-after-succ': ['lean', `def f : Nat → Nat → Nat
  | 0, _ => 1
  | n + 3, 0 => n
  | 1000000000000000000000000, m => m
  | _, m => m + 1${leanMain}`],
  'lean-large-numeral-unreachable-zero': ['lean', `def f : Nat → Nat
  | 17 => 2
  | 0 => 1
  | n + 1 => n${leanMain}`],
  'lean-large-numeral-redundant': ['lean', `def f : Nat → Nat
  | 0 => 1
  | n + 1 => n
  | 99 => 3${leanMain}`],
  'lean-large-offset-rejected': ['lean', `def f : Nat → Nat
  | n + 17 => n
  | _ => 0${leanMain}`],
  'lean-huge-offset-rejected': ['lean', `def f : Nat → Nat
  | n + 9007199254740993 => n
  | _ => 0${leanMain}`],
  'lean-offset-at-limit': ['lean', `def f : Nat → Nat
  | n + 16 => n
  | _ => 0${leanMain}`],
  'rocq-large-numeral-mixed': ['v', `${rocqHead}Definition f (n : nat) : nat :=
  match n with
  | S (S k) => k
  | 1000 => 5
  | 0 => 1
  | _ => 2
  end.
`],
  // Lean: accepted programs.
  'lean-multi-scrutinee': ['lean', `def f : Nat → Nat → Nat
  | 0, _ => 0
  | _, 0 => 1
  | n + 1, m + 1 => f n m
${leanMain}`],
  'lean-string-match': ['lean', `def f (s : String) : Nat :=
  match s with
  | "a" => 1
  | "b" => 2
  | _ => 3
${leanMain}`],
  'lean-bool-match': ['lean', `def f (b : Bool) : Nat :=
  match b with
  | true => 1
  | false => 0
${leanMain}`],
  'lean-int-literal-match': ['lean', `def f (i : Int) : Nat :=
  match i with
  | 0 => 1
  | -1 => 2
  | _ => 3
${leanMain}`],
  'lean-nested-tree': [
    'lean',
    `${leanTree}
def f : Tree → Nat
  | .node (.node _ a _) b _ => a + b
  | .node .leaf b _ => b
  | .leaf => 0
${leanMain}`,
  ],
  'lean-literal-default': ['lean', `def f (n : Nat) : Nat := if n == 0 then 1 else 2
def g : Nat := 3 - 5
def h : Int := (0 - 7) / 2
def k : Int := -7 % 3
${leanMain}`],
  'lean-let-alias': ['lean', `def f (n : Nat) : Nat :=
  if n == 0 then 0 else
  let m := n
  m - 1 + f (m - 1)
${leanMain}`],
  'lean-cast': ['lean', `def f (n : Nat) : Int := Int.ofNat n - 10
def g (i : Int) : Nat := Int.toNat i
${leanMain}`],
  'lean-theorem-cases': [
    'lean',
    `${leanTree}
def size : Tree → Nat
  | .leaf => 0
  | .node l _ r => size l + 1 + size r

theorem size_nonneg (t : Tree) : 0 ≤ size t := by
  cases t with
  | leaf => simp [size]
  | node l v r => simp [size]
${leanMain}`,
  ],
  'lean-print-values': ['lean', `def main : IO Unit := do
  IO.println s!"{1 + 2} {true} {(-3 : Int)}"
  IO.println (toString 5)
`],
  // Lean: rejected programs.
  'lean-duplicate': ['lean', `def f : Nat := 1\ndef f : Nat := 2${leanMain}`],
  'lean-reserved': ['lean', `def ml_x : Nat := 1${leanMain}`],
  'lean-unknown-type': ['lean', `def f (x : Foo) : Nat := 1${leanMain}`],
  'lean-unknown-name': ['lean', `def f : Nat := g 1${leanMain}`],
  'lean-too-many-args': ['lean', `def g (x : Nat) : Nat := x\ndef f : Nat := g 1 2${leanMain}`],
  'lean-partial': ['lean', `def g (x y : Nat) : Nat := x\ndef f : Nat := g 1${leanMain}`],
  'lean-negative-nat': ['lean', `def f : Nat := -1${leanMain}`],
  'lean-negate-nat': ['lean', `def f (n : Nat) : Nat := -n${leanMain}`],
  'lean-operand-types': ['lean', `def f (n : Nat) (i : Int) : Int := n + i${leanMain}`],
  'lean-string-order': ['lean', `def f (a b : String) : Bool := a < b${leanMain}`],
  'lean-missing-ctor': ['lean', `${leanTree}\ndef f (t : Tree) : Nat :=\n  match t with\n  | .leaf => 0\n${leanMain}`],
  'lean-bool-nonexhaustive': ['lean', `def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n${leanMain}`],
  'lean-int-add-pattern': ['lean', `def f (i : Int) : Int :=\n  match i with\n  | n + 1 => n\n  | _ => 0\n${leanMain}`],
  'lean-print-tree': ['lean', `${leanTree}\ndef main : IO Unit := do\n  IO.println s!"{Tree.leaf}"\n`],
  'lean-arith-string': ['lean', `def f (a b : String) : String := a - b${leanMain}`],
  'lean-data-equality': ['lean', `${leanTree}\ndef f (t : Tree) : Bool := t == Tree.leaf${leanMain}`],
  'lean-local-call': ['lean', `def f (g : Nat) : Nat := g 1${leanMain}`],
  'lean-wrong-arity-pattern': ['lean', `${leanTree}\ndef f : Tree → Nat\n  | .node l _ => 0\n  | .leaf => 1\n${leanMain}`],
  'lean-bad-ctor-pattern': ['lean', `${leanTree}\ndef f : Tree → Nat\n  | .branch => 0\n  | _ => 1\n${leanMain}`],
  'lean-string-literal-on-nat': ['lean', `def f (n : Nat) : Nat :=\n  match n with\n  | "a" => 0\n  | _ => 1\n${leanMain}`],
  'lean-if-mismatch': ['lean', `def f (b : Bool) : Nat := if b then 1 else "x"${leanMain}`],
  'lean-induction-bad': ['lean', `theorem t (s : String) : s = s := by\n  induction s with\n  | nil => rfl\n${leanMain}`],
  'lean-self-proof': ['lean', `theorem t (n : Nat) : n = n := by simp [t]${leanMain}`],

  // Rust: accepted programs.
  'rust-widening': ['rs', `fn f(n: u32) -> i64 { i64::from(n) + (n as i64) }${rustMain}`],
  'rust-euclid': ['rs', `fn f(x: i32, y: i32) -> i32 { x.div_euclid(y) + x.rem_euclid(y) + x / y + x % y }${rustMain}`],
  'rust-negate': ['rs', `fn f(x: i8) -> i8 { -x }\nfn g() -> i8 { -128 }${rustMain}`],
  'rust-match-bool': ['rs', `fn f(b: bool) -> u8 { match b { true => 1, false => 0 } }${rustMain}`],
  'rust-match-tuple': ['rs', `fn f(a: u64, b: u64) -> u64 { match (a, b) { (0, _) => 0, (_, 0) => 1, (x, y) => f(x - 1, y - 1) } }${rustMain}`],
  'rust-nested': ['rs', `${rustTree}\nfn f(t: &Tree) -> u64 { match t { Tree::Node(l, v, _) => match l.as_ref() { Tree::Node(_, w, _) => *v + *w, Tree::Leaf => *v }, Tree::Leaf => 0 } }${rustMain}`],
  'rust-print': ['rs', `fn main() { let x: u8 = 7; println!("{} {}", x, true); println!("{}", -3i64); }\n`],
  'rust-unsigned-recursion': ['rs', `fn f(n: u32) -> u32 { if n == 0 { 0 } else if n == 1 { 1 } else { f(n - 1) + f(n - 2) } }${rustMain}`],
  'rust-match-bool-repeat': ['rs', `fn f(b: bool, c: bool) -> u8 { match (b) { true => 1, true => 2, false => if c { 3 } else { 4 } } }${rustMain}`],
  // Rust: rejected programs.
  'rust-match-bool-partial': ['rs', `fn f(b: bool) -> u8 { match b { true => 1 } }${rustMain}`],
  'rust-narrowing': ['rs', `fn f(n: u64) -> u32 { n as u32 }${rustMain}`],
  'rust-out-of-range': ['rs', `fn f() -> u8 { 300 }${rustMain}`],
  'rust-negate-unsigned': ['rs', `fn f(n: u64) -> u64 { -n }${rustMain}`],
  'rust-negative-unsigned': ['rs', `fn f() -> u64 { -1 }${rustMain}`],
  'rust-mixed': ['rs', `fn f(a: u32, b: u64) -> u64 { a + b }${rustMain}`],
  'rust-data-eq': ['rs', `${rustTree}\nfn f(t: &Tree) -> bool { *t == Tree::Leaf }${rustMain}`],
  'rust-reserved': ['rs', `fn ml_f() -> u64 { 1 }${rustMain}`],
  'rust-super-beyond': ['rs', `fn f() -> u64 { super::g() }${rustMain}`],
  'rust-not-function': ['rs', `${rustTree}\nfn f() -> u64 { Tree(1) }${rustMain}`],
  'rust-missing-arm': ['rs', `${rustTree}\nfn f(t: &Tree) -> u64 { match t { Tree::Leaf => 0 } }${rustMain}`],
  'rust-string-order': ['rs', `fn f(a: String, b: String) -> bool { a < b }${rustMain}`],
  'rust-ctor-fields': ['rs', `${rustTree}\nfn f() -> Tree { Tree::Node(Box::new(Tree::Leaf), 1) }${rustMain}`],
  'rust-duplicate-ctor': ['rs', `pub enum E { A, A }${rustMain}`],
  'rust-modules': ['rs', `mod a { pub mod b { pub fn g() -> u64 { super::h() } } pub fn h() -> u64 { crate::k() } }\nfn k() -> u64 { self::a::b::g() }${rustMain}`],

  // Rocq: accepted programs.
  'rocq-nat-patterns': ['v', `${rocqHead}Fixpoint f (n : nat) : nat :=\n  match n with\n  | O => 0\n  | S O => 1\n  | S (S m) => f m\n  end.\n`],
  'rocq-bare-ctors': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=\n  match t with\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n  end.\nDefinition sample : Tree := node leaf 1 leaf.\n`],
  'rocq-z': ['v', `${rocqHead}Definition f (x y : Z) : Z := (x / y + x mod y - 3)%Z.\n`],
  'rocq-induction': ['v', `${rocqHead}Fixpoint add (n m : nat) : nat := match n with | O => m | S k => S (add k m) end.\nTheorem add_zero (n : nat) : add n 0 = n.\nProof.\n  induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.\nQed.\n`],
  // Rocq: rejected programs.
  'rocq-duplicate-ctor-name': ['v', `${rocqHead}Inductive A : Type := | x : A.\nInductive B : Type := | x : B.\n`],
  'rocq-unknown': ['v', `${rocqHead}Definition f (n : nat) : nat := g n.\n`],
  'rocq-too-many-cases': ['v', `${rocqHead}Theorem t (n : nat) : n = n.\nProof.\n  induction n as [| k ih | j].\n  - reflexivity.\n  - reflexivity.\nQed.\n`],

  // JavaScript: accepted programs.
  'js-guard-negate': ['mjs', jsFn('f', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('f expects a natural number');\n  return -n + (n - 1n);")],
  'js-concat': ['mjs', `${jsFn('f', [['n', 'bigint']], 'string', "  return 'n=' + n + true;")}console.log(f(3n));\n`],
  'js-ctor': ['mjs', `${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}${jsFn('node', [['v', 'bigint']], 'Tree', "  return { $: 'node', value: v, left: leaf(), right: leaf() };")}`],
  'js-print': ['mjs', 'console.log(1n + 2n);\nconsole.log(`${3n}`);\nconsole.log(String(4n));\n'],
  'js-switch-literal': ['mjs', jsFn('f', [['s', 'string']], 'bigint', "  switch (s) {\n    case 'a':\n      return 1n;\n    default:\n      return 2n;\n  }")],
  'js-deep-equal': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}assert.deepStrictEqual(leaf(), leaf());\n`],
  // JavaScript: rejected programs.
  'js-identity': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}assert.equal(leaf(), leaf());\n`],
  'js-string-data': ['mjs', `${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}console.log('t=' + leaf());\n`],
  'js-print-data': ['mjs', `${jsTree}${jsFn('leaf', [], 'Tree', "  return { $: 'leaf' };")}console.log(leaf());\n`],
  'js-unknown-tag': ['mjs', `${jsTree}${jsFn('f', [], 'Tree', "  return { $: 'branch' };")}`],
  'js-tag-fields': ['mjs', `${jsTree}${jsFn('f', [], 'Tree', "  return { $: 'node', value: 1n };")}`],
  'js-ambiguous-tag': ['mjs', `/**\n * @typedef {{ $: 'a' }} A\n */\n/**\n * @typedef {{ $: 'a' }} B\n */\nconsole.log(String(1n));\n${jsFn('f', [], 'bigint', "  const x = { $: 'a' };\n  return 1n;")}`],
  'js-field': ['mjs', `${jsTree}${jsFn('f', [['t', 'Tree']], 'bigint', '  return t.value;')}`],
  'js-higher-order': ['mjs', jsFn('f', [['g', 'bigint']], 'bigint', '  return g(1n);')],
};
