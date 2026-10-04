import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const FIBONACCI = `let a = 0n;
let b = 1n;
while (b < 100n) {
  const next = a + b;
  a = b;
  b = next;
}
let label = 'small';
if (a + b > 100n) label = 'large';
console.log(a + b);
console.log(label);
`;

test('top-level let, assignments, ifs and loops are a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(FIBONACCI, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
  const { text } = emitJavaScript(checkProgram(parseJavaScript(FIBONACCI)));
  assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', text], { encoding: 'utf8' }), '233n\nlarge\n');
});

test('main binds each top-level variable the statements before a print declare or assign', () => {
  const program = parseJavaScript(FIBONACCI);
  const data = program.items.find((item) => item.k === 'data' && item.name === 'ml_main_top2');
  assert.ok(data?.generated);
  assert.deepEqual(data.ctors.map((ctor) => ctor.fields.map((field) => field.name)), [['a', 'b', 'label']]);
  assert.deepEqual(program.main.effects.map((effect) => effect.k === 'let' ? effect.name : effect.k), ['ml_main_top2_value', 'a', 'b', 'label', 'print', 'print']);
});

test('top-level statements JavaScript could not run as translated are rejected', () => {
  const cases = [
    ['let x = 1n;\nif (x > 0n) { return; }\n', 'top-level return statement: return leaves a function, and a module has none to leave at 26..32'],
    ['let s = 0n;\nswitch (s) { case 0n: s = 1n; }\n', 'top-level switch statement: the top level prints with console.log, binds with const or let, assigns, branches with if, loops and asserts at 12..18'],
    ['let x = 1n;\nconst f = () => 2n;\nconsole.log(x);\n', 'function after a top-level statement: the statements before const f could call it before it is initialised; declare every function first at 12..18'],
    ['const c = 1n;\nc = 2n;\n', 'assignment of constant c: assigning a const binding throws a TypeError; declare it with let at 14..18'],
  ];
  for (const [source, message] of cases) {
    assert.throws(() => parseJavaScript(source), (error) => error.message === message, source);
  }
});
