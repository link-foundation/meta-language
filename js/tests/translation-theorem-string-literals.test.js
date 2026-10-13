import assert from 'node:assert/strict';
import { test } from 'node:test';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../src/program-translation.js';
import fixtures from './fixtures/translation-cases/emit-javascript.mjs';

const runTheorems = code => {
  const lines = [];
  runInNewContext(code, {
    process: { argv: ['--ml-check-theorems'] },
    console: { log: value => lines.push(String(value)) },
  }, { timeout: 3000 });
  return lines;
};

test('original theorem fixtures execute with exact source names, including the primed forall name', () => {
  const expectations = new Map([
    ['emit-javascript-lean-theorem-tree', ['mirror_mirror', 'size_mirror']],
    ['emit-javascript-lean-theorem-forall', ["add_comm'", 'add_zero']],
    ['emit-javascript-lean-theorem-bool-prop', ['zero_is_zero', 'bool_prop']],
  ]);
  for (const [name, names] of expectations) {
    const [, source] = fixtures[name];
    const translated = translateProgram(source, 'Lean', 'JavaScript');
    assert.equal(translated.diagnostic, null, name);
    assert.deepEqual(runTheorems(translated.code), names.map(name => `theorem ${name}: holds on the bounded domain`));
  }
});

test('a false property retains its primed source name in an executable diagnostic', () => {
  const source = "theorem t' (x : Nat) : x = x := by rfl\ndef main : IO Unit := do\n  IO.println \"x\"\n";
  const translated = translateProgram(source, 'Lean', 'JavaScript');
  assert.equal(translated.diagnostic, null);
  assert.deepEqual(runTheorems(translated.code), ["theorem t': holds on the bounded domain"]);
  assert.match(translated.code, /return \(x === x\);/u);
  const mutation = translated.code.replace('return (x === x);', 'return false;');
  assert.throws(() => runTheorems(mutation), error => error.name === 'Error' && error.message === "theorem t' fails on a bounded input");
});
