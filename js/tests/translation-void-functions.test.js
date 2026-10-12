import assert from 'node:assert/strict';
import { test } from 'node:test';

import { translateProgram } from '../src/program-translation.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { preservesTranslationBehavior } from './support/translation-behavior.js';

const VOID = `function hello(name) {
  console.log(\`hello \${name}\`);
}
function report(x) {
  if (x < 0n) return;
  console.log(\`\${x}\`);
}
hello('a');
report(-1n);
`;

test('a function that returns nothing is a semantic translation in every target', () => {
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const translated = translateProgram(VOID, 'JavaScript', target);
    assert.equal(translated.diagnostic, null, target);
    assert.equal(translated.contract.support, 'semantic-translation', target);
  }
});

test('a function that finishes without a return value, or leaves with return;, returns the unit value', () => {
  const rust = translateProgram(VOID, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('pub fn hello(name: String) -> () {'), rust);
  assert.ok(rust.includes('pub fn report(x: crate::ml::Big) -> () {'), rust);
  const rocq = translateProgram(VOID, 'JavaScript', 'Rocq').code;
  assert.ok(rocq.includes('(let ml_o1 := ((String.append "hello "%string name) :: ml_out) in (ml_io1_mk ml_o1 tt)).'), rocq);
});

test('a call whose value a statement discards still runs, for the lines it prints', () => {
  const rust = translateProgram(VOID, 'JavaScript', 'Rust').code;
  assert.ok(rust.includes('let ml_main_ignored1 = crate::hello(String::from("a"));'), rust);
  const lean = translateProgram(VOID, 'JavaScript', 'Lean').code;
  assert.ok(lean.includes('let ml_run3 := (hello "a" ([] : List String))'), lean);
  const body = translateProgram('function f(x) {\n  g(x);\n  return x;\n}\nfunction g(x) {\n  return x + 1n;\n}\nconsole.log(f(1n));\n', 'JavaScript', 'Rust').code;
  assert.ok(body.includes('let ml_f_ignored1 = crate::g(x.clone());'), body);
});

test('a function that returns a value on one path preserves undefined on the other', () => {
  const original = 'function f(n) { if (n > 0n) return n; }';
  assert.deepEqual(parseJavaScript(original).items[0].ret, { kind: 'option', element: { kind: 'int' }, absence: 'undefined' });
  preservesTranslationBehavior(`${original}\nconsole.log(f(1n));\nconsole.log(f(0n));\nconsole.log(f(-1n));\n`, '1n\nundefined\nundefined\n');
});

test('an effectful void return retains its output before producing undefined', () => {
  const source = "function trace() { console.log('trace'); }\nfunction f(n) { if (n > 0n) return n; return trace(); }\nconsole.log(f(1n));\nconsole.log(f(0n));\n";
  preservesTranslationBehavior(source, '1n\ntrace\nundefined\n');
});
