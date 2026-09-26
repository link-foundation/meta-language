// Small programs that exercise every branch of the JavaScript emitter in all
// four source languages: each runtime helper, every operator (natural
// subtraction, truncating, Euclidean and floor division, machine-integer range
// checks, casts), strings and printing, nested modules, names that clash with
// JavaScript keywords, data types, theorems with bounded checks, assertions
// and main effects with lets, as name → [source extension, source text].

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

export default {
  // Lean: file shape, entry and modules.
  'emit-javascript-lean-no-main': ['lean', 'def one : Nat := 1\ndef two : Nat := one + one\n'],
  'emit-javascript-lean-main-only': ['lean', 'def main : IO Unit := do\n  IO.println "hello"\n  IO.println ""\n'],
  'emit-javascript-lean-main-lets': ['lean', 'def sq (n : Nat) : Nat := n * n\n\ndef main : IO Unit := do\n  let a := sq 3\n  let b := sq a\n  let console := a + b\n  IO.println s!"{a} {b} {console}"\n'],
  'emit-javascript-lean-namespace': ['lean', `namespace Arith\n\ndef double (n : Nat) : Nat := n + n\n\ndef quad (n : Nat) : Nat := double (double n)\n\nend Arith\n\ndef eight : Nat := Arith.quad 2\n${leanMain}`],
  'emit-javascript-lean-nested-namespaces': ['lean', `namespace A\n\ndef a (n : Nat) : Nat := n + 1\n\nnamespace B\n\ndef b (n : Nat) : Nat := A.a n * 2\n\nnamespace C\n\ndef c (n : Nat) : Nat := A.B.b n + A.a n\n\nend C\n\nend B\n\ndef after (n : Nat) : Nat := B.C.c n\n\nend A\n\nnamespace D\n\ndef d : Nat := A.after 1\n\nend D\n\ndef main : IO Unit := do\n  IO.println s!"{D.d} {A.B.C.c 2}"\n`],
  'emit-javascript-lean-keyword-namespaces': ['lean', `namespace Math\n\ndef max (a b : Nat) : Nat := if a < b then b else a\n\nend Math\n\nnamespace Object\n\ndef keys (n : Nat) : Nat := n\n\nend Object\n\nnamespace JSON\n\nnamespace default\n\ndef delete (n : Nat) : Nat := n\n\nend default\n\nend JSON\n\ndef main : IO Unit := do\n  IO.println s!"{Math.max 1 2} {Object.keys 3} {JSON.default.delete 4}"\n`],
  'emit-javascript-lean-keyword-names': ['lean', `def var (n : Nat) : Nat := n\ndef new (n : Nat) : Nat := var n\ndef typeof (n : Nat) : Nat := new n\ndef console (n : Nat) : Nat := typeof n\ndef process (arguments eval undefined : Nat) : Nat := arguments + eval + undefined\ndef constructor (prototype : Nat) : Nat := prototype\ndef get (set : Nat) : Nat := set\ndef async (await : Nat) : Nat := await\ndef yield (static : Nat) : Nat := static\n\ndef main : IO Unit := do\n  IO.println s!"{console 1} {process 1 2 3} {constructor 4} {get 5} {async 6} {yield 7}"\n`],
  'emit-javascript-lean-primed-names': ['lean', `def f' (n : Nat) : Nat := n + 1\ndef f'' (n' : Nat) : Nat := f' n' + 1\ndef α (β : Nat) : Nat := β\n\ndef main : IO Unit := do\n  IO.println s!"{f'' 1} {α 2}"\n`],
  'emit-javascript-lean-main-name-clash': ['lean', `namespace Util\n\ndef main' (n : Nat) : Nat := n\n\nend Util\n\ndef ml (n : Nat) : Nat := Util.main' n\n\ndef main : IO Unit := do\n  IO.println s!"{ml 3}"\n`],

  // Lean: arithmetic, strings and casts.
  'emit-javascript-lean-nat-sub': ['lean', `def monus (a b : Nat) : Nat := a - b\ndef chain (a : Nat) : Nat := a - 1 - 2\n\ndef main : IO Unit := do\n  IO.println s!"{monus 3 5} {chain 10}"\n`],
  'emit-javascript-lean-int-sub': ['lean', `def diff (a b : Int) : Int := a - b\ndef neg (a : Int) : Int := -a\ndef negLit : Int := -7\n\ndef main : IO Unit := do\n  IO.println s!"{diff 3 5} {neg 4} {negLit}"\n`],
  'emit-javascript-lean-nat-div-mod': ['lean', `def q (a b : Nat) : Nat := a / b\ndef r (a b : Nat) : Nat := a % b\n\ndef main : IO Unit := do\n  IO.println s!"{q 7 2} {r 7 2} {q 7 0} {r 7 0}"\n`],
  'emit-javascript-lean-int-div-mod': ['lean', `def q (a b : Int) : Int := a / b\ndef r (a b : Int) : Int := a % b\n\ndef main : IO Unit := do\n  IO.println s!"{q (-7) 2} {r (-7) 2} {q 7 (-2)} {r 7 0}"\n`],
  'emit-javascript-lean-mul-add': ['lean', `def poly (x : Nat) : Nat := 3 * x * x + 2 * x + 1\ndef ipoly (x : Int) : Int := x * x - 4 * x + 4\n\ndef main : IO Unit := do\n  IO.println s!"{poly 5} {ipoly (-3)}"\n`],
  'emit-javascript-lean-casts': ['lean', `def up (n : Nat) : Int := Int.ofNat n - 10\ndef down (i : Int) : Nat := Int.toNat i\ndef both (i : Int) : Nat := Int.toNat (i * 2) + 1\n\ndef main : IO Unit := do\n  IO.println s!"{up 3} {down (-4)} {down 4} {both 2}"\n`],
  'emit-javascript-lean-bool-ops': ['lean', `def both (a b : Bool) : Bool := a && b\ndef either (a b : Bool) : Bool := a || b\ndef flip (a : Bool) : Bool := !a\ndef same (a b : Nat) : Bool := a == b\ndef differ (a b : Nat) : Bool := a != b\ndef order (a b : Int) : Bool := a < b || a <= b && a > b || a >= b\n\ndef main : IO Unit := do\n  IO.println s!"{both true false} {either true false} {flip true} {same 1 1} {differ 1 1} {order 1 2}"\n`],
  'emit-javascript-lean-strings': ['lean', `def greet (name : String) : String := "hello, " ++ name ++ "!"\ndef same (a b : String) : Bool := a == b\n\ndef main : IO Unit := do\n  let r := same "a" "b"\n  IO.println (greet "world")\n  IO.println s!"{r}"\n`],
  'emit-javascript-lean-string-escapes': ['lean', 'def main : IO Unit := do\n  IO.println "quote \\" backslash \\\\ tab\\tend"\n  IO.println "line\\nbreak"\n  IO.println "unicode λ → ∀ 😀"\n  IO.println "dollar ${x} and `tick`"\n'],
  'emit-javascript-lean-interpolation': ['lean', `def n : Nat := 3\ndef i : Int := -3\ndef b : Bool := true\ndef s : String := "s"\n\ndef main : IO Unit := do\n  IO.println s!"{n} {i} {b} {s} {n + 1} {i * 2}"\n  IO.println s!"plain"\n`],
  'emit-javascript-lean-if-expr': ['lean', `def pick (b : Bool) (n : Nat) : Nat := (if b then n else 0) + 1\ndef sign (i : Int) : Int := if i < 0 then -1 else if i == 0 then 0 else 1\n\ndef main : IO Unit := do\n  IO.println s!"{pick true 2} {sign (-5)}"\n`],
  'emit-javascript-lean-let-expr': ['lean', `def f (n : Nat) : Nat :=\n  let a := n + 1\n  let b := a * 2\n  a + b\ndef g (n : Nat) : Nat := 1 + (let m := n * n; m + m)\n\ndef main : IO Unit := do\n  IO.println s!"{f 2} {g 3}"\n`],
  'emit-javascript-lean-match-expr': ['lean', `def f (n : Nat) : Nat := 1 + (match n with | 0 => 10 | k + 1 => k)\n\ndef main : IO Unit := do\n  IO.println s!"{f 0} {f 5}"\n`],

  // Lean: natural-number matches.
  'emit-javascript-lean-nat-zero-succ': ['lean', `def fact : Nat → Nat\n  | 0 => 1\n  | n + 1 => (n + 1) * fact n\n\ndef main : IO Unit := do\n  IO.println s!"{fact 20}"\n`],
  'emit-javascript-lean-nat-zero-wild': ['lean', `def isZero (n : Nat) : Bool :=\n  match n with\n  | 0 => true\n  | _ => false\n${leanMain}`],
  'emit-javascript-lean-nat-zero-bind': ['lean', `def pred (n : Nat) : Nat :=\n  match n with\n  | 0 => 0\n  | m => m - 1\n${leanMain}`],
  'emit-javascript-lean-nat-wild-only': ['lean', `def seven (n : Nat) : Nat :=\n  match n with\n  | _ => 7\n${leanMain}`],
  'emit-javascript-lean-nat-bind-only': ['lean', `def same (n : Nat) : Nat :=\n  match n with\n  | k => k + 1\n${leanMain}`],
  'emit-javascript-lean-nat-succ-wild': ['lean', `def f (n : Nat) : Nat :=\n  match n with\n  | k + 1 => k\n  | _ => 42\n${leanMain}`],
  'emit-javascript-lean-nat-deep': ['lean', `def fib : Nat → Nat\n  | 0 => 0\n  | 1 => 1\n  | n + 2 => fib (n + 1) + fib n\n\ndef main : IO Unit := do\n  IO.println s!"{fib 20}"\n`],
  'emit-javascript-lean-match-computed': ['lean', `def f (n : Nat) : Nat :=\n  match n + 1 with\n  | 0 => 0\n  | k + 1 => match k * 2 with\n    | 0 => 1\n    | j + 1 => j\n\ndef main : IO Unit := do\n  IO.println s!"{f 3}"\n`],
  'emit-javascript-lean-multi-scrutinee': ['lean', `def f : Nat → Nat → Nat\n  | 0, _ => 0\n  | _, 0 => 1\n  | n + 1, m + 1 => f n m\n${leanMain}`],
  'emit-javascript-lean-bool-match': ['lean', `def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n  | false => 0\n${leanMain}`],
  'emit-javascript-lean-int-literal-match': ['lean', `def f (i : Int) : Nat :=\n  match i with\n  | 0 => 1\n  | 1 => 2\n  | _ => 3\n${leanMain}`],

  // Lean: data types.
  'emit-javascript-lean-tree': ['lean', `${leanTree}\nnamespace Tree\n\ndef size : Tree → Nat\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n\ndef mirror : Tree → Tree\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n\nend Tree\n\ndef main : IO Unit := do\n  IO.println s!"{Tree.size (Tree.mirror (Tree.node Tree.leaf 1 Tree.leaf))}"\n`],
  'emit-javascript-lean-data-wild': ['lean', `${leanTree}\ndef isLeaf (t : Tree) : Bool :=\n  match t with\n  | .leaf => true\n  | _ => false\n${leanMain}`],
  'emit-javascript-lean-data-bind': ['lean', `${leanTree}\ndef grow (t : Tree) : Tree :=\n  match t with\n  | .leaf => .node .leaf 0 .leaf\n  | other => other\n${leanMain}`],
  'emit-javascript-lean-data-nested-match': ['lean', `${leanTree}\ndef f : Tree → Nat\n  | .node (.node _ a _) b _ => a + b\n  | .node .leaf b _ => b\n  | .leaf => 0\n${leanMain}`],
  'emit-javascript-lean-data-unused-fields': ['lean', `${leanTree}\ndef value : Tree → Nat\n  | .leaf => 0\n  | .node _ v _ => v\n${leanMain}`],
  'emit-javascript-lean-data-named-fields': ['lean', `inductive Shape where\n  | circle (radius : Nat) : Shape\n  | rect (width height : Nat) : Shape\n  | dot : Shape\n\ndef area : Shape → Nat\n  | .circle r => 3 * r * r\n  | .rect w h => w * h\n  | .dot => 0\n\ndef main : IO Unit := do\n  IO.println s!"{area (.rect 2 3)} {area (.circle 1)}"\n`],
  'emit-javascript-lean-data-keyword-fields': ['lean', `inductive Box where\n  | box (default new this : Nat) : Box\n\ndef sum : Box → Nat\n  | .box a b c => a + b + c\n\ndef main : IO Unit := do\n  IO.println s!"{sum (.box 1 2 3)}"\n`],
  'emit-javascript-lean-data-primed-ctor': ['lean', `inductive T where\n  | leaf' : T\n  | node' : T → T\n\ndef depth : T → Nat\n  | .leaf' => 0\n  | .node' t => depth t + 1\n\ntheorem depth_leaf : depth T.leaf' = 0 := by decide\n\ntheorem depth_any (t : T) : depth t = depth t := by simp\n\ndef main : IO Unit := do\n  IO.println s!"{depth (.node' .leaf')}"\n`],
  'emit-javascript-lean-data-in-namespace': ['lean', `namespace Shapes\n\ninductive Color where\n  | red : Color\n  | green : Color\n\ndef code : Color → Nat\n  | .red => 1\n  | .green => 2\n\nend Shapes\n\ndef main : IO Unit := do\n  IO.println s!"{Shapes.code Shapes.Color.red}"\n`],
  'emit-javascript-lean-data-keyword-type': ['lean', `inductive Object where\n  | mk : Nat → Object\n\ndef unwrap : Object → Nat\n  | .mk n => n\n\ndef main : IO Unit := do\n  IO.println s!"{unwrap (.mk 5)}"\n`],

  // Lean: theorems and bounded checks.
  'emit-javascript-lean-theorem-closed': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_two : double 2 = 4 := by decide\n${leanMain}`],
  'emit-javascript-lean-theorem-closed-no-main': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_two : double 2 = 4 := by decide\n`],
  'emit-javascript-lean-theorem-nat': ['lean', `def double (n : Nat) : Nat := n + n\n\ntheorem double_even (n : Nat) : double n = 2 * n := by simp [double]; omega\n${leanMain}`],
  'emit-javascript-lean-theorem-int': ['lean', `def neg (i : Int) : Int := -i\n\ntheorem neg_neg (i : Int) : neg (neg i) = i := by simp [neg]\n${leanMain}`],
  'emit-javascript-lean-theorem-bool-string': ['lean', `def pick (b : Bool) (s : String) : String := if b then s else ""\n\ntheorem pick_true (s : String) : pick true s = s := by simp [pick]\n\ntheorem pick_bool (b : Bool) (s : String) : pick b s = pick b s := by simp\n${leanMain}`],
  'emit-javascript-lean-theorem-many-binders': ['lean', `def f (a : Nat) (b : Int) (c : Bool) : Int := if c then b else Int.ofNat a\n\ntheorem f_true (a : Nat) (b : Int) : f a b true = b := by simp [f]\n${leanMain}`],
  'emit-javascript-lean-theorem-tree': ['lean', `${leanTree}\nnamespace Tree\n\ndef mirror : Tree → Tree\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n\ndef size : Tree → Nat\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n\nend Tree\n\ntheorem mirror_mirror (t : Tree) : Tree.mirror (Tree.mirror t) = t := by\n  induction t with\n  | leaf => rfl\n  | node l v r ihl ihr => simp [Tree.mirror, ihl, ihr]\n\ntheorem size_mirror (t : Tree) : Tree.size (Tree.mirror t) = Tree.size t := by\n  induction t with\n  | leaf => rfl\n  | node l v r ihl ihr => simp [Tree.size, Tree.mirror, ihl, ihr]; omega\n${leanMain}`],
  'emit-javascript-lean-theorem-connectives': ['lean', `def f (n : Nat) : Nat := n + 1\n\ntheorem pos (n : Nat) : f n > 0 ∧ f n ≥ 1 := by simp [f]\n\ntheorem either (n : Nat) : f n ≠ 0 ∨ n < 0 := by simp [f]\n\ntheorem imp (n : Nat) : n ≤ 3 → f n ≤ 4 := by simp [f]; omega\n\ntheorem negation (n : Nat) : ¬ (f n = 0) := by simp [f]\n${leanMain}`],
  'emit-javascript-lean-theorem-forall': ['lean', `def add (a b : Nat) : Nat := a + b\n\ntheorem add_comm' : ∀ a b : Nat, add a b = add b a := by intro a b; simp [add]; omega\n\ntheorem add_zero (a : Nat) : ∀ b : Int, add a 0 = a ∧ b = b := by simp [add]\n${leanMain}`],
  'emit-javascript-lean-theorem-bool-prop': ['lean', `def isZero (n : Nat) : Bool := n == 0\n\ntheorem zero_is_zero : isZero 0 = true := by decide\n\ntheorem bool_prop (n : Nat) : isZero (n + 1) = false := by simp [isZero]\n${leanMain}`],
  'emit-javascript-lean-theorem-namespaced': ['lean', `namespace Arith\n\ndef double (n : Nat) : Nat := n + n\n\ntheorem double_zero : double 0 = 0 := by decide\n\nnamespace Deep\n\ntheorem double_le (n : Nat) : n ≤ double n := by simp [double]\n\nend Deep\n\nend Arith\n${leanMain}`],
  'emit-javascript-lean-theorem-primed': ['lean', `def f (n : Nat) : Nat := n\n\ntheorem f_id' (n : Nat) : f n = n := by simp [f]\n${leanMain}`],
  'emit-javascript-lean-theorem-data-ne': ['lean', `${leanTree}\ntheorem leaf_ne (t : Tree) (v : Nat) : Tree.node t v t ≠ Tree.leaf := by simp\n${leanMain}`],
  'emit-javascript-lean-theorem-nonrecursive-data': ['lean', `inductive Pair where\n  | pair : Nat → Bool → Pair\n\ninductive Wrap where\n  | none : Wrap\n  | some : Pair → Wrap\n\ndef first : Pair → Nat\n  | .pair n _ => n\n\ntheorem first_pair (n : Nat) (b : Bool) : first (.pair n b) = n := by simp [first]\n\ntheorem wrap_self (w : Wrap) : w = w := by simp\n${leanMain}`],
  'emit-javascript-lean-theorem-deep-data': ['lean', `inductive Chain where\n  | link : Nat → Chain → Chain\n  | stop : Chain\n\ninductive Pair where\n  | pair : Chain → Chain → Pair\n\ntheorem pair_self (p : Pair) : p = p := by simp\n${leanMain}`],
  'emit-javascript-lean-theorem-empty-data': ['lean', `inductive Loop where\n  | more : Loop → Loop\n\ntheorem loop_self (l : Loop) : l = l := by simp\n${leanMain}`],
  'emit-javascript-lean-theorem-nested-data': ['lean', `inductive Color where\n  | red : Color\n  | blue : Color\n\ninductive Paint where\n  | coat : Color → String → Paint\n\ninductive Wall where\n  | bare : Wall\n  | painted : Paint → Wall → Wall\n\ntheorem wall_self (w : Wall) : w = w := by simp\n\ntheorem paint_self (p : Paint) : p = p := by simp\n${leanMain}`],

  // Rust: machine integers.
  'emit-javascript-rust-u8-add': ['rs', `fn add(a: u8, b: u8) -> u8 { a + b }\nfn main() { println!("{}", add(100, 27)); }\n`],
  'emit-javascript-rust-fixed-widths': ['rs', `fn a(x: u8, y: i8) -> i8 { y }\nfn b(x: u16, y: i16) -> i16 { y }\nfn c(x: u32, y: i32) -> i32 { y }\nfn d(x: u64, y: i64) -> i64 { y }\nfn e(x: u128, y: i128) -> i128 { y }\nfn f(x: usize, y: isize) -> isize { y }${rustMain}`],
  'emit-javascript-rust-checked-ops': ['rs', `fn ops(a: i32, b: i32) -> i32 { a + b - a * b }\nfn neg(a: i64) -> i64 { -a }\nfn usub(a: u64, b: u64) -> u64 { a - b }${rustMain}`],
  'emit-javascript-rust-division': ['rs', `fn q(a: i32, b: i32) -> i32 { a / b }\nfn r(a: i32, b: i32) -> i32 { a % b }\nfn uq(a: u64, b: u64) -> u64 { a / b + a % b }\nfn main() { println!("{} {}", q(-7, 2), r(-7, 2)); }\n`],
  'emit-javascript-rust-euclid': ['rs', `fn f(x: i32, y: i32) -> i32 { x.div_euclid(y) + x.rem_euclid(y) + x / y + x % y }\nfn g(x: i64) -> i64 { x.div_euclid(2) }${rustMain}`],
  'emit-javascript-rust-negate': ['rs', `fn f(x: i8) -> i8 { -x }\nfn g() -> i8 { -128 }\nfn h() -> i128 { -170141183460469231731687303715884105728 }\nfn k() -> u128 { 340282366920938463463374607431768211455 }${rustMain}`],
  'emit-javascript-rust-widening': ['rs', `fn f(n: u32) -> i64 { i64::from(n) + (n as i64) }\nfn g(n: u8) -> u64 { u64::from(n) * 2 }${rustMain}`],
  'emit-javascript-rust-unsigned-match': ['rs', `fn sum_to(n: u64) -> u64 {\n    match n {\n        0 => 0,\n        k => sum_to(k - 1) + k,\n    }\n}\nfn fib(n: u64) -> u64 {\n    match n {\n        0 => 0,\n        1 => 1,\n        _ => fib(n - 1) + fib(n - 2),\n    }\n}\nfn main() { println!("{} {}", sum_to(10), fib(10)); }\n`],
  'emit-javascript-rust-unsigned-recursion': ['rs', `fn f(n: u32) -> u32 { if n == 0 { 0 } else if n == 1 { 1 } else { f(n - 1) + f(n - 2) } }${rustMain}`],
  'emit-javascript-rust-match-bool': ['rs', `fn f(b: bool) -> u8 { match b { true => 1, false => 0 } }${rustMain}`],

  // Rust: data, modules, strings and main effects.
  'emit-javascript-rust-tree': ['rs', `${rustTree}\nmod tree {\n    use super::Tree;\n\n    pub fn size(t: &Tree) -> u64 {\n        match t {\n            Tree::Leaf => 0,\n            Tree::Node(l, _, r) => size(l) + 1 + size(r),\n        }\n    }\n\n    pub fn mirror(t: &Tree) -> Tree {\n        match t {\n            Tree::Leaf => Tree::Leaf,\n            Tree::Node(l, v, r) => Tree::Node(Box::new(mirror(r)), *v, Box::new(mirror(l))),\n        }\n    }\n}\n\nfn main() {\n    let t = Tree::Node(Box::new(Tree::Leaf), 3, Box::new(Tree::Leaf));\n    assert_eq!(tree::mirror(&tree::mirror(&t)), t);\n    assert_ne!(t, Tree::Leaf);\n    println!("{}", tree::size(&t));\n}\n`],
  'emit-javascript-rust-match-wild-data': ['rs', `${rustTree}\nfn is_leaf(t: &Tree) -> bool { match t { Tree::Leaf => true, _ => false } }\nfn keep(t: &Tree) -> Tree { match t { Tree::Leaf => Tree::Leaf, other => other.clone() } }${rustMain}`],
  'emit-javascript-rust-tuple-variants': ['rs', `#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Shape { Circle(u64), Rect(u64, u64), Dot }\n\nfn area(s: &Shape) -> u64 {\n    match s {\n        Shape::Circle(r) => 3 * *r * *r,\n        Shape::Rect(w, _) => *w * 2,\n        Shape::Dot => 0,\n    }\n}\n\nfn main() { println!("{}", area(&Shape::Rect(2, 3))); }\n`],
  'emit-javascript-rust-keyword-variants': ['rs', `#[derive(Clone, Debug, PartialEq, Eq)]\npub enum Object { Default, Delete(u64), Constructor(Box<Object>) }\n\nfn total(o: &Object) -> u64 {\n    match o {\n        Object::Default => 0,\n        Object::Delete(n) => *n,\n        Object::Constructor(inner) => total(inner) + 1,\n    }\n}\n\nfn main() { println!("{}", total(&Object::Constructor(Box::new(Object::Delete(2))))); }\n`],
  'emit-javascript-rust-modules': ['rs', `mod a { pub mod b { pub fn g() -> u64 { super::h() } } pub fn h() -> u64 { crate::k() } }\nfn k() -> u64 { self::a::b::g() }${rustMain}`],
  'emit-javascript-rust-keyword-names': ['rs', `mod console { pub fn log(n: u64) -> u64 { n } }\nmod math { pub fn new(n: u64) -> u64 { n } }\nfn delete(this: u64) -> u64 { this }\nfn function(var: u64, undefined: u64) -> u64 { var + undefined }\nfn instanceof(arguments: u64) -> u64 { arguments }\nfn constructor(eval: u64) -> u64 { eval }\nfn main() { println!("{} {} {} {} {} {}", console::log(1), math::new(2), delete(3), function(4, 5), instanceof(6), constructor(7)); }\n`],
  'emit-javascript-rust-strings': ['rs', `fn classify(n: i64) -> String {\n    if n < 0 {\n        String::from("negative")\n    } else if n == 0 {\n        String::from("zero")\n    } else {\n        String::from("positive")\n    }\n}\n\nfn describe(n: u32) -> String {\n    let doubled = n + n;\n    let label = classify(i64::from(doubled) - 10);\n    format!("{n} doubled is {doubled} ({label})")\n}\n\nfn main() {\n    println!("{}", describe(3));\n    println!("{}", classify(-4));\n    println!("{} {}", true, "quote \\" and \\\\ backslash");\n}\n`],
  'emit-javascript-rust-main-lets': ['rs', `fn sq(n: u64) -> u64 { n * n }\n\nfn main() {\n    let a = sq(3);\n    let b = sq(a);\n    let main = a + b;\n    assert!(a < b && b > 0);\n    assert!(!(a == b) || a != b);\n    assert_eq!(sq(2), 4);\n    assert_ne!(a, b);\n    println!("{} {} {}", a, b, main);\n}\n`],
  'emit-javascript-rust-panic': ['rs', `fn safe_div(a: u64, b: u64) -> u64 {\n    if b == 0 {\n        panic!("division by zero")\n    } else {\n        a / b\n    }\n}\nfn pick(n: u64) -> u64 { match n { 0 => panic!("zero \\"quoted\\""), k => k } }\nfn main() { println!("{} {}", safe_div(6, 3), pick(1)); }\n`],
  'emit-javascript-rust-print': ['rs', `fn main() { let x: u8 = 7; println!("{} {}", x, true); println!("{}", -3i64); println!(); }\n`],

  // Rocq.
  'emit-javascript-rocq-basic': ['v', `${rocqHead}${rocqShow}Definition monus (a b : N) : N := (a - b)%N.\nDefinition nmonus (a b : nat) : nat := a - b.\nDefinition halve (x : Z) : Z := (x / 2)%Z.\nDefinition remainder (x y : Z) : Z := Z.modulo x y.\nDefinition main : list string :=\n  ("monus = " ++ show_N (monus 3 5)) ::\n  ("halve = " ++ show_Z (halve (-7))) ::\n  ("remainder = " ++ show_Z (remainder (-7) 3)) ::\n  nil.\n\nEval vm_compute in main.\n`],
  'emit-javascript-rocq-z': ['v', `${rocqHead}Definition f (x y : Z) : Z := (x / y + x mod y - 3)%Z.\nDefinition g (x y : N) : N := (x / y + x mod y)%N.\nDefinition h (x : Z) : Z := (- x * 2)%Z.\n`],
  'emit-javascript-rocq-nat-patterns': ['v', `${rocqHead}Fixpoint f (n : nat) : nat :=\n  match n with\n  | O => 0\n  | S O => 1\n  | S (S m) => f m\n  end.\n`],
  'emit-javascript-rocq-tree': ['v', `${rocqHead}${rocqTree}Fixpoint size (t : Tree) : nat :=\n  match t with\n  | leaf => 0\n  | node l _ r => size l + 1 + size r\n  end.\nFixpoint mirror (t : Tree) : Tree :=\n  match t with\n  | leaf => leaf\n  | node l v r => node (mirror r) v (mirror l)\n  end.\nTheorem mirror_mirror (t : Tree) : mirror (mirror t) = t.\nProof.\n  induction t as [| l ihl v r ihr].\n  - reflexivity.\n  - simpl. rewrite ihl, ihr. reflexivity.\nQed.\nDefinition sample : Tree := node leaf 1 leaf.\n`],
  'emit-javascript-rocq-induction': ['v', `${rocqHead}Fixpoint add (n m : nat) : nat := match n with | O => m | S k => S (add k m) end.\nTheorem add_zero (n : nat) : add n 0 = n.\nProof.\n  induction n as [| k ih].\n  - reflexivity.\n  - simpl. rewrite ih. reflexivity.\nQed.\nTheorem add_two : add 1 1 = 2.\nProof. reflexivity. Qed.\n`],
  'emit-javascript-rocq-modules': ['v', `${rocqHead}${rocqShow}Module Arith.\n\nDefinition double (n : N) : N := (n + n)%N.\n\nModule Inner.\n\nDefinition quad (n : N) : N := double (double n).\n\nEnd Inner.\n\nEnd Arith.\n\nDefinition main : list string :=\n  show_N (Arith.Inner.quad 2) ::\n  let x := Arith.double 5 in\n  show_N x ::\n  nil.\n\nEval vm_compute in main.\n`],
  'emit-javascript-rocq-strings': ['v', `${rocqHead}${rocqShow}Definition classify (n : Z) : string :=\n  if (n <? 0)%Z then "negative" else if (n =? 0)%Z then "zero" else "positive".\nDefinition describe (n : N) : string :=\n  let doubled := (n + n)%N in\n  show_N n ++ " doubled is " ++ show_N doubled ++ " (" ++ classify (Z.of_N doubled - 10) ++ ")".\nDefinition main : list string :=\n  describe 3 ::\n  classify (-4) ::\n  nil.\n\nEval vm_compute in main.\n`],

  // JavaScript.
  'emit-javascript-js-edge-division': ['mjs', `${jsFn('quot', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a / b;')}${jsFn('rem', [['a', 'bigint'], ['b', 'bigint']], 'bigint', '  return a % b;')}console.log(\`\${quot(-7n, 2n)} \${rem(-7n, 2n)}\`);\n`],
  'emit-javascript-js-guard-negate': ['mjs', jsFn('f', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('f expects a natural number');\n  return -n + (n - 1n);")],
  'emit-javascript-js-concat': ['mjs', `${jsFn('f', [['n', 'bigint']], 'string', "  return 'n=' + n + true;")}console.log(f(3n));\n`],
  'emit-javascript-js-print': ['mjs', 'console.log(1n + 2n);\nconsole.log(`${3n}`);\nconsole.log(String(4n));\nconsole.log();\nconsole.log(true);\nconsole.log(0x1fn);\nconsole.log(\'quote " backslash \\\\ tab\\tend\');\n'],
  'emit-javascript-js-switch-literal': ['mjs', jsFn('f', [['s', 'bigint']], 'bigint', "  switch (s) {\n    case 1n:\n      return 1n;\n    default:\n      return 2n;\n  }")],
  'emit-javascript-js-data': ['mjs', `import assert from 'node:assert/strict';\n${jsTree}${jsLeaf}${jsNode}${jsFn('size', [['t', 'Tree']], 'bigint', "  switch (t.$) {\n    case 'leaf':\n      return 0n;\n    default:\n      return size(t.left) + 1n + size(t.right);\n  }")}const t = node(leaf(), 2n, leaf());\nassert.deepStrictEqual(t, node(leaf(), 2n, leaf()));\nassert.notDeepStrictEqual(t, leaf());\nassert(size(t) === 1n && size(leaf()) === 0n);\nassert.equal(size(t), 1n);\nassert.notEqual(size(t), 2n);\nconsole.log(size(t));\n`],
  'emit-javascript-js-keyword-names': ['mjs', `${jsFn('of', [['get', 'bigint']], 'bigint', '  return get;')}${jsFn('constructor', [['set', 'bigint']], 'bigint', '  return of(set);')}${jsFn('$dollar', [['async', 'bigint']], 'bigint', '  return async;')}${jsFn('_x', [['_$', 'bigint']], 'bigint', '  return _$;')}console.log(constructor(1n) + $dollar(2n) + _x(3n));\n`],
  'emit-javascript-js-throw': ['mjs', `${jsFn('f', [['n', 'bigint']], 'bigint', "  if (n === 0n) throw new Error('zero is not allowed');\n  return n;")}console.log(f(1n));\n`],
  'emit-javascript-js-checked-cast': ['mjs', `${jsFn('shift', [['n', 'bigint']], 'bigint', '  return n - 5n;')}${jsFn('half', [['n', 'bigint']], 'bigint', "  if (n < 0n) throw new RangeError('half expects a natural number');\n  return n / 2n;")}console.log(half(shift(9n)));\n`],
  'emit-javascript-js-const-main': ['mjs', `${jsFn('sq', [['n', 'bigint']], 'bigint', '  return n * n;')}const a = sq(3n);\nconst b = sq(a);\nconsole.log(\`\${a} \${b}\`);\n`],
};
