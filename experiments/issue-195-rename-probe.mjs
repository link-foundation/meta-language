import { analyzeProgram } from '../js/src/index.js';
const cases = [
  ['JavaScript', 'const x = 1; const s = "x"; const r = /x/u; // x\nconst t = `x ${x}`; globalThis.result = [s, r.test("x"), t, x];\n', 'x', 0, 'value'],
  ['JavaScript', 'const x = 1; function f() { const x = 2; return x; } globalThis.result = [f(), x];\n', 'x', 1, 'value'],
  ['JavaScript', 'const x = 1; const { x: y = x } = {}; globalThis.result = y;\n', 'x', 0, 'value'],
  ['Rust', 'mod m { pub fn f() -> u32 { 1 } }\nfn g() -> u32 { m::f() + self::m::f() }\n', 'f', 0, 'h'],
  ['Rust', 'fn g(x: u32) -> Vec<u32> { let s = "x"; println!("{}", x); vec![x, x] } // x\n', 'x', 0, 'value'],
  ['Rust', 'fn g(x: u32) -> u32 { let y = { let x = x + 1; x }; x + y }\n', 'x', 0, 'value'],
  ['Rust', 'macro_rules! twice { ($e:expr) => { $e + $e }; }\nfn g(x: u32) -> u32 { twice!(x) }\n', 'x', 0, 'value'],
  ['Lean', 'namespace Foo\ndef x : Nat := 1\nend Foo\ndef y : Nat := Foo.x -- x\n', 'x', 0, 'value'],
  ['Lean', 'theorem t : ∀ n : Nat, n = n := fun n => rfl\n', 'n', 0, 'k'],
  ['Lean', 'def f (x : Nat) : String := s!"x {x}" -- x\n', 'x', 0, 'value'],
  ['Lean', 'theorem t (n : Nat) : n + 0 = n := by\n  induction n with\n  | zero => rfl\n  | succ k ih => simp\n', 'n', 0, 'm'],
  ['Rocq', 'Module M.\nDefinition x : nat := 1.\nEnd M.\nDefinition y : nat := M.x.\n', 'x', 0, 'value'],
  ['Rocq', 'Lemma l : forall n : nat, n = n.\nProof. intros n. reflexivity. Qed.\n', 'n', 0, 'k'],
  ['Rocq', 'Definition f (x : nat) : nat := (* x *) let y := x in y + x.\nDefinition s : string := "x"%string.\n', 'x', 0, 'value'],
];
for (const [language, source, name, occurrence, replacement] of cases) {
  try {
    const program = analyzeProgram(source, language);
    const bindings = program.bindings.filter((b) => b.name === name);
    console.log(`\n# ${language}: ${JSON.stringify(source)}`);
    console.log(' bindings', program.bindings.map((b) => `${b.name}@${b.declaration.start}[${b.references.length}]`).join(' '), 'clean', program.network.verifyFullMatch().isClean());
    console.log(' unresolved', program.unresolvedReferences.map((r) => r.name).join(' '));
    const binding = bindings[occurrence];
    if (!binding) { console.log(' NO BINDING'); continue; }
    const renamed = program.renameBinding(binding.id, replacement);
    console.log(' ->', JSON.stringify(renamed.emit()));
  } catch (error) { console.log(' ERROR', error.message); }
}
