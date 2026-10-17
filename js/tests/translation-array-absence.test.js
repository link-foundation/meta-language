import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { translateProgram } from '../src/program-translation.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';

const fixtures = readProjectData(new URL('../../parity/fixtures/translation-array-absence.json', import.meta.url)).cases;
const execute = (source) => {
  const lines = [];
  runInNewContext(source, { console: { log: (...values) => lines.push(values.map((value) => Object.is(value, -0) ? '-0' : String(value)).join(' ')) } });
  return lines.join('\n') + '\n';
};

test('ordinary reads retain undefined through output, coercions, returns and strict comparisons', () => {
  for (const fixture of fixtures) {
    const options = { referenceSemantics: fixture.referenceSemantics ?? false };
    const emitted = emitJavaScript(checkProgram(parseJavaScript(fixture.source, options)));
    assert.equal(execute(fixture.source), fixture.expectedStdout, fixture.name);
    assert.equal(execute(emitted.text), fixture.expectedStdout, fixture.name);
    const rust = translateProgram(fixture.source, 'JavaScript', 'Rust', options);
    assert.equal(rust.diagnostic, null, fixture.name);
    assert.equal(rust.semantics.assumptions.some(({ id }) => id === 'in-bounds-array-reads'), false, fixture.name);
    assert.doesNotMatch(rust.code, /portable-source-envelope/u, fixture.name);
    if (!fixture.referenceSemantics) for (const target of ['Lean', 'Rocq']) {
      const result = translateProgram(fixture.source, 'JavaScript', target, options);
      assert.equal(result.diagnostic, null, `${fixture.name}:${target}`);
      assert.equal(result.semantics.assumptions.some(({ id }) => id === 'in-bounds-array-reads'), false, `${fixture.name}:${target}`);
    }
  }
});

test('read offsets preserve negative zero, non-integers and non-finite values', () => {
  const source = '/** @param {number[]} values @param {number} index */ function read(values, index) { return values[index]; }\n/** @param {number[]} values @param {number} index */ function add(values, index) { return values[index] + 1; }\n';
  const original = runInNewContext(source + '[read, add]');
  const translated = runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source))).text + '[read, add]');
  for (const index of [-Infinity, -1, -0, 0, 1, 1.5, NaN, Infinity]) for (const values of [[], [-0, 2]]) {
    for (let functionIndex = 0; functionIndex < original.length; functionIndex += 1) assert.ok(Object.is(original[functionIndex](values, index), translated[functionIndex](values, index)), String(index));
  }
});

test('nullable computed indices retain missing property keys rather than coercing null to zero', () => {
  const source = '/** @param {number[]} values @param {(number|null|undefined)[]} indices @param {number} at */ function select(values, indices, at) { return values[indices[at]]; }\n';
  const original = runInNewContext(source + 'select');
  const translated = runInNewContext(emitJavaScript(checkProgram(parseJavaScript(source))).text + 'select');
  const indices = [null, undefined, -0, 0, 1, 1.5, NaN, Infinity];
  for (const at of [-1, ...indices.keys(), indices.length]) assert.ok(Object.is(original([7, 9], indices, at), translated([7, 9], indices, at)), String(at));
  for (const target of ['Rust', 'Lean', 'Rocq']) {
    const result = translateProgram(source, 'JavaScript', target);
    assert.equal(result.diagnostic, null, target);
    assert.equal(result.semantics.assumptions.some(({ id }) => id === 'in-bounds-array-reads'), false, target);
  }
});

test('nullable format directives remain explicit until their source conversions are modeled', () => {
  for (const directive of ['%d', '%i']) {
    const source = `/** @param {(number|null)[]} values */ function print(values) { console.log('${directive}', values[0]); }\n`;
    assert.match(translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message ?? '', /^numeric format of a nullable value:/u);
  }
  const source = '/** @param {string[]} values */ function print(values) { console.log(values[0], 4); }\n';
  assert.match(translateProgram(source, 'JavaScript', 'Rust').diagnostic?.message ?? '', /^console.log with a computed first string:/u);
});

test('implicit undefined branch conversion preserves the effects of a void-returning call', () => {
  const source = 'function touch() { console.log(1); }\n/** @param {number} n @returns {number|undefined} */ function choose(n) { if (n > 0) return 7; return touch(); }\nconsole.log(choose(0));\n';
  const emitted = emitJavaScript(checkProgram(parseJavaScript(source))).text;
  assert.equal(execute(source), '1\nundefined\n');
  assert.equal(execute(emitted), execute(source));
  const rust = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(rust.diagnostic, null);
  assert.match(rust.code, /let ml_discarded_unit = crate::touch\(\);/u);
});
