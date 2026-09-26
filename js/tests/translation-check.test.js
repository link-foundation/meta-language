import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { dirname, join } from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { checkProgram } from '../src/translation/check.js';
import { parseLean } from '../src/translation/lean.js';
import { parseRust } from '../src/translation/rust.js';

const jsRoot = join(dirname(fileURLToPath(import.meta.url)), '..');

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

test('numeric patterns keep their literal as exact decimal text', () => {
  const [row] = parseRust(`fn f(n: u128) -> u8 { match n { 340282366920938463463374607431768211455 => 1, _ => 0 } }${rustMain}`)
    .items[0].body.rows;
  assert.deepEqual([row.patterns[0].value, 'negative' in row.patterns[0]], ['340282366920938463463374607431768211455', false]);
  const [lean] = parseLean(`def f (n : Nat) : Nat :=\n  match n with\n  | 0 => 1\n  | _ => 0\n${leanMain}`).items[0].body.rows;
  assert.equal(lean.patterns[0].value, '0');
});

test('translation stage fixtures record the current JavaScript pipeline', () => {
  const output = execFileSync(
    process.execPath,
    ['scripts/build-translation-stage-fixtures.mjs', '--check'],
    { cwd: jsRoot, encoding: 'utf8' },
  );
  assert.match(output, /translation stage fixtures match the JavaScript pipeline/u);
});
