import assert from 'node:assert/strict';
import { test } from 'node:test';

import { checkProgram } from '../src/translation/check.js';
import { parseLean } from '../src/translation/lean.js';
import { parseRust } from '../src/translation/rust.js';

const leanMain = '\ndef main : IO Unit := do\n  IO.println "x"\n';
const rustMain = '\nfn main() { println!("x"); }\n';

const checkRust = (source) => checkProgram(parseRust(`${source}${rustMain}`));
const checkLean = (source) => checkProgram(parseLean(`${source}${leanMain}`));

test('boolean literal matches covering both values check', () => {
  assert.doesNotThrow(() => checkRust('fn f(b: bool) -> u8 { match b { true => 1, false => 0 } }'));
  assert.doesNotThrow(() => checkRust('fn f(b: bool) -> u8 { match b { false => 0, true => 1 } }'));
  assert.doesNotThrow(() => checkLean('def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n  | false => 0\n'));
});

test('a repeated boolean literal is shadowed rather than rejected', () => {
  assert.doesNotThrow(() => checkRust(
    'fn f(b: bool, c: bool) -> u8 { match (b) { true => 1, true => 2, false => if c { 3 } else { 4 } } }',
  ));
});

test('boolean literal matches missing a value are rejected', () => {
  assert.throws(
    () => checkRust('fn f(b: bool) -> u8 { match b { true => 1 } }'),
    (error) => error.kind === 'type' && /non-exhaustive match on bool/u.test(error.message),
  );
  assert.throws(
    () => checkLean('def f (b : Bool) : Nat :=\n  match b with\n  | true => 1\n'),
    (error) => /non-exhaustive match on bool/u.test(error.message),
  );
});
