// Draft per-language binding rename corpus: prints each rename result or the
// rejection so the shared fixture can record the expected text.
import { analyzeProgram } from '../js/src/index.js';

export const cases = [
  // JavaScript
  ['JavaScript', ['symbolIdentity', 'commentsStringsAndLiteralsUnaffected'], 'const x = 1; const s = "x"; const r = /x/u; // x\nconst t = `x ${x}`; globalThis.result = [s, r.test("x"), t, x];\n', 'x', 0, 'value'],
  ['JavaScript', ['shadowing', 'nestedScopes'], 'const x = 1; function f() { const x = 2; { const y = x * 3; return y; } } globalThis.result = [f(), x];\n', 'x', 1, 'inner'],
  ['JavaScript', ['qualifiedNames'], 'const x = 1; const object = { x, y: x }; globalThis.result = [object.x, object.y, x];\n', 'x', 0, 'value'],
  ['JavaScript', ['unicodeIdentifiers'], 'const π = 3; const café = π * 2; globalThis.result = [café, π, "π"];\n', 'π', 0, 'τ'],
  ['JavaScript', ['macroOrProofBinders', 'shadowing'], 'const x = 10; const add = (x, y = x) => x + y; const one = y => y; globalThis.result = [add(1), one(x), x];\n', 'x', 1, 'left'],
  ['JavaScript', ['captureAvoidance'], 'const x = 1; function f(y) { return x + y; } globalThis.result = f(2);\n', 'x', 0, 'y'],
  ['JavaScript', ['captureAvoidance'], 'const y = 1; const f = (x) => x + y; globalThis.result = f(2);\n', 'x', 0, 'y'],
  // Rust
  ['Rust', ['symbolIdentity', 'commentsStringsAndLiteralsUnaffected'], 'pub fn g(x: u32) -> String {\n    let s = "x"; // x\n    let c = \'x\';\n    format!("{x} {} {c} {{x}}", s) + &x.to_string()\n}\n', 'x', 0, 'value'],
  ['Rust', ['shadowing', 'nestedScopes'], 'pub fn g(x: u32) -> u32 {\n    let x = x + 1;\n    let y = {\n        let x = x * 2;\n        x\n    };\n    x + y\n}\n', 'x', 1, 'step'],
  ['Rust', ['shadowing'], 'pub fn g(x: u32) -> u32 {\n    let x = x + 1;\n    let y = {\n        let x = x * 2;\n        x\n    };\n    x + y\n}\n', 'x', 2, 'y'],
  ['Rust', ['nestedScopes', 'symbolIdentity'], 'pub fn g(values: &[u32]) -> u32 {\n    let mut total = 0;\n    for v in values {\n        total += v;\n    }\n    if let Some(v) = values.first() {\n        total += v;\n    }\n    let add = |v: u32| v + total;\n    add(1)\n}\n', 'total', 0, 'sum'],
  ['Rust', ['qualifiedNames'], 'mod shapes {\n    pub fn area(w: u32) -> u32 {\n        w * w\n    }\n    pub mod nested {\n        pub fn twice() -> u32 {\n            super::area(2) * 2\n        }\n    }\n}\nuse shapes::area;\npub fn total() -> u32 {\n    area(3) + shapes::area(1) + crate::shapes::nested::twice() + self::shapes::area(4)\n}\n', 'area', 0, 'surface'],
  ['Rust', ['qualifiedNames'], 'pub struct P {\n    pub x: u32,\n}\npub fn make(x: u32) -> P {\n    let p = P { x };\n    P { x: p.x + x }\n}\n', 'x', 0, 'value'],
  ['Rust', ['unicodeIdentifiers'], 'pub fn área(λ: u32) -> u32 {\n    λ * 2 // λ\n}\n', 'λ', 0, 'μ'],
  ['Rust', ['macroOrProofBinders'], 'macro_rules! twice {\n    ($e:expr) => {\n        $e + $e\n    };\n}\npub fn g(e: u32) -> u32 {\n    twice!(e)\n}\n', 'e', 0, 'value'],
  ['Rust', ['macroOrProofBinders'], 'macro_rules! twice {\n    ($e:expr) => {\n        $e + $e\n    };\n}\npub fn g(e: u32) -> u32 {\n    twice!(e)\n}\n', 'e', 1, 'input'],
  ['Rust', ['captureAvoidance'], 'pub fn g(x: u32) -> u32 {\n    let y = 2;\n    x + y\n}\n', 'x', 0, 'y'],
  ['Rust', ['captureAvoidance'], 'pub fn g(x: u32) -> u32 {\n    let y = x;\n    y + x\n}\n', 'y', 0, 'x'],
  ['Rust', ['captureAvoidance'], 'mod m {\n    pub fn f() {}\n    pub fn h() {}\n}\npub fn g() {\n    m::f();\n    m::h();\n}\n', 'f', 0, 'h'],
  // Lean
  ['Lean', ['symbolIdentity', 'commentsStringsAndLiteralsUnaffected'], 'def f (x : Nat) : String := s!"x = {x}" ++ "x" -- x\n', 'x', 0, 'value'],
  ['Lean', ['shadowing', 'nestedScopes'], 'def g (x : Nat) : Nat :=\n  let x := x + 1\n  x * 2\n', 'x', 0, 'n'],
  ['Lean', ['nestedScopes'], 'def h (n : Nat) : Nat → Nat := fun m => n + m\n', 'n', 0, 'k'],
  ['Lean', ['qualifiedNames'], 'namespace Geo\ndef area (w : Nat) : Nat := w * w\nend Geo\ndef total : Nat := Geo.area 2\n', 'area', 0, 'surface'],
  ['Lean', ['unicodeIdentifiers'], 'def double (α : Nat) : Nat := α + α\n', 'α', 0, 'β'],
  ['Lean', ['macroOrProofBinders'], 'theorem t : ∀ n : Nat, n = n := fun n => Eq.refl n\n', 'n', 0, 'k'],
  ['Lean', ['macroOrProofBinders'], 'theorem t : ∀ n : Nat, n = n := fun n => Eq.refl n\n', 'n', 1, 'm'],
  ['Lean', ['captureAvoidance'], 'def g (x : Nat) (y : Nat) : Nat := x + y\n', 'x', 0, 'y'],
  ['Lean', ['captureAvoidance'], 'theorem t (y : Nat) : ∀ x : Nat, x + y = x + y := fun _ => rfl\n', 'x', 0, 'y'],
  // Rocq
  ['Rocq', ['symbolIdentity', 'commentsStringsAndLiteralsUnaffected'], 'Definition f (x : nat) : nat := (* x *) x + 1.\nDefinition s : string := "x".\n', 'x', 0, 'value'],
  ['Rocq', ['shadowing', 'nestedScopes'], 'Definition g (x : nat) : nat := let x := x + 1 in x * 2.\n', 'x', 0, 'n'],
  ['Rocq', ['nestedScopes'], 'Definition h (n : nat) : nat -> nat := fun m => n + m.\n', 'n', 0, 'k'],
  ['Rocq', ['qualifiedNames'], 'Module Geo.\nDefinition area (w : nat) : nat := w * w.\nEnd Geo.\nDefinition total : nat := Geo.area 2.\n', 'area', 0, 'surface'],
  ['Rocq', ['unicodeIdentifiers'], 'Definition double (α : nat) : nat := α + α.\n', 'α', 0, 'β'],
  ['Rocq', ['macroOrProofBinders'], 'Lemma l : forall n : nat, n = n.\nProof. intros n. reflexivity. Qed.\n', 'n', 0, 'k'],
  ['Rocq', ['macroOrProofBinders'], 'Definition twice : nat -> nat := fun n => n + n.\n', 'n', 0, 'm'],
  ['Rocq', ['captureAvoidance'], 'Definition g (x y : nat) : nat := x + y.\n', 'x', 0, 'y'],
  ['Rocq', ['captureAvoidance'], 'Lemma l (y : nat) : forall x : nat, x + y = x + y.\n', 'x', 0, 'y'],
];

if (import.meta.url === `file://${process.argv[1]}`) {
  for (const [language, assertions, source, name, occurrence, replacement] of cases) {
    const program = analyzeProgram(source, language);
    const binding = program.bindings.filter((b) => b.name === name)[occurrence];
    console.log(`\n# ${language} ${assertions.join(',')} ${name}#${occurrence} -> ${replacement} clean=${program.network.verifyFullMatch().isClean()}`);
    if (!binding) { console.log(' NO BINDING', program.bindings.map((b) => `${b.kind}:${b.name}`).join(' ')); continue; }
    console.log(` ${binding.kind} refs=${binding.references.length}`);
    try {
      console.log(' ->', JSON.stringify(program.renameBinding(binding.id, replacement).emit()));
    } catch (error) {
      console.log(' REJECTED', error.message);
    }
  }
}
