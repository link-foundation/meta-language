import assert from 'node:assert/strict';
import { test } from 'node:test';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';

function observations(text) {
  const lines = [];
  runInNewContext(text, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines;
}

function preservesSource(source) {
  const translated = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  assert.deepEqual(observations(translated), observations(source));
}

test('ordinary indexed reads retain absence while numeric consumers derive Number results', () => {
  const parsed = parseJavaScript('function read(values) { return values[0]; } function plus(values) { return values[0] + 1; }');
  assert.equal(parsed.items[0].ret.kind, 'option');
  assert.equal(parsed.items[0].ret.absence, 'undefined');
  assert.equal(parsed.items[1].ret.kind, 'float');
  preservesSource(`function read(values) { return values[0]; }
function plus(values) { return values[0] + 1; }
console.log(read([])); console.log(read([2]));
console.log(plus([])); console.log(plus([2]));`);
});

test('runtime absence propagates through declared returns and tuples without changing the annotation', () => {
  const parsed = parseJavaScript(`/** @param {number[]} values @returns {number} */
function read(values) { return values[0]; }
/** @param {string[]} values @returns {[string, number]} */
function pair(values) { return [values[0], 1]; }`);
  assert.equal(parsed.items[0].ret.kind, 'float');
  assert.equal(parsed.items[0].semanticRet.kind, 'option');
  assert.equal(parsed.items[1].ret.elements[0].kind, 'string');
  assert.equal(parsed.items[1].semanticRet.elements[0].kind, 'option');
});

test('chosen shared reference representation propagates through declared signatures', () => {
  const source = '/** @param {number[]} values @returns {number[]} */ function identity(values) { return values; }';
  const parsed = parseJavaScript(source, { referenceSemantics: true });
  assert.equal(parsed.items[0].params[0].type.reference, true);
  assert.equal(parsed.items[0].ret.reference, true);
  const entry = checkProgram(parsed).declarations.get('identity');
  assert.equal(entry.params[0].type.reference, true);
  assert.equal(entry.ret.reference, true);
});

test('mutable captured locals keep one cell shared by sibling closures and outer assignments', () => {
  preservesSource(`function shared(start) {
  let value = start;
  const get = () => value;
  const set = next => { value = next; return value; };
  set(2);
  value = 3;
  return get();
}
console.log(shared(1));`);
});

test('each invocation allocates its own captured parameter cell', () => {
  preservesSource(`function counter(start) {
  const next = () => { start = start + 1; return start; };
  return next;
}
function run() {
  const first = counter(0);
  const second = counter(10);
  console.log(first()); console.log(first()); console.log(second());
  return first();
}
console.log(run());`);
});

test('a captured cell survives lifted loops without becoming a copied loop result', () => {
  preservesSource(`function count() {
  let value = 0;
  const next = () => { value += 1; return value; };
  while (value < 3) next();
  return value;
}
console.log(count());`);
});
