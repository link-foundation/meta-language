import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { selectVersionedAssertion } from '../scripts/assertion-version-ranges.mjs';
import { format } from 'node:util';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { acceptJavaScriptOptionalTupleContext } from '../src/translation/frontend-rules.js';

test('legacy positional array calls use their declared types without an immutable product proof', () => {
  const source = '/** @param {[number,number]} pair */function first(pair){return pair[0];}console.log(first([2,3]));';
  const surface = parseJavaScript(source);
  assert.deepEqual(surface.items[0].params[0].type, {
    kind: 'tuple', elements: [{ kind: 'float' }, { kind: 'float' }],
  });
  assert.equal(Object.hasOwn(surface, 'inferredExpressionTypes'), false);
  const observed = [];
  const result = vm.runInNewContext(emitJavaScript(checkProgram(surface)).text + '\nfirst([2,3]);', {
    console: { log: value => observed.push(String(value)) },
  }, { timeout: 1000 });
  assert.equal(result, 2);
  assert.deepEqual(observed, ['2']);
});

test('legacy reference-bearing positional arrays retain contextual return types', () => {
  for (const source of [
    '/** @returns {[string,number]} */function pair(){return ["name",5];}',
    '/** @param {string[]} values @returns {[string,number]} */function pair(values){return [values[0],1];}',
  ]) {
    const surface = parseJavaScript(source);
    assert.equal(Object.hasOwn(surface, 'inferredExpressionTypes'), false);
    assert.doesNotThrow(() => checkProgram(surface));
  }
});

test('contextual array types preserve arity and spread refusals and cannot prove immutable ownership', () => {
  const source = readFileSync(new URL('../src/translation/frontend-rules.js', import.meta.url), 'utf8');
  // The complete original test and literal array-only expectation remain in the source inverse.
  // Package version alone cannot distinguish unreleased source epochs, so also pin this whole definition.
  const policy = {id:'optional-tuple-frontend-source-profile',subject:'meta-language npm package',
    comparison:{kind:'release',precision:3},expectations:[{id:'array-and-frozen-tuple-source-context',
      minimum:'1.0.0',maximum:'1.0.1',includeMinimum:true,includeMaximum:false,
      expected:{allowTuple:true,sourceSha256:'44fde15e3f286c8d891519819251433295c5a7ef1e4a6e1424fd846800809ce6'},
      evidence:'Complete current frontend source definition and original test inverse; independent frozen-array source oracle below, then complete normal generated Rust owner.'}]};
  const active = JSON.parse(readFileSync(new URL('../package.json',import.meta.url),'utf8')).version;
  const selected = selectVersionedAssertion(policy,active);
  assert.equal(createHash('sha256').update(source).digest('hex'),selected.expected.sourceSha256,'only the explicit current implementation epoch selects this extension');
  for(const uncovered of ['0.99.0','1.0.1','2.0.0'])assert.throws(()=>selectVersionedAssertion(policy,uncovered),/uncovered active version/u);
  assert.equal(vm.runInNewContext("function selectedPair(flag){return flag?Object.freeze([2,3]):null;}JSON.stringify([Array.isArray(selectedPair(true)),Array.isArray(selectedPair(false)),Object.isFrozen(selectedPair(true))]);",{},{timeout:1000}),'[true,false,true]','independent actual JavaScript frozen-array and nullish oracle');

  const declaration = '/** @param {[number,number]} pair */function first(pair){return pair[0];}';
  assert.throws(() => parseJavaScript(declaration + 'console.log(first([2]));'), /two different lengths/u);
  assert.throws(() => parseJavaScript(declaration + 'console.log(first([...[2,3]]));'), /array.*tuple/u);
  const mutable = '/** @param {[number,boolean]} value */function first(value){return value[0];}console.log(first([2,true]));';
  assert.throws(() => checkProgram(parseJavaScript(mutable)), /immutable tuple product/u);
  assert.equal(acceptJavaScriptOptionalTupleContext('JavaScript', 'array', 'option', 'tuple'), true);
  // Same retained tuple input: the predecessor expected false before this source-profile extension.
  assert.equal(acceptJavaScriptOptionalTupleContext('JavaScript', 'tuple', 'option', 'tuple'), selected.expected.allowTuple);
  for (const context of [
    ['Rust','array','option','tuple'], ['Lean','array','option','tuple'],
    ['JavaScript','array','tuple','tuple'],
    ['JavaScript','array','option','array'], ['JavaScript','array','option','option'],
    ['JavaScript','array','union','tuple'], ['JavaScript','array','option','union'],
  ]) assert.equal(acceptJavaScriptOptionalTupleContext(...context), false, context.join('/'));
  const optional = '/** @param {[number,number]|null|undefined} pair @returns {string} */function describe(pair){return typeof pair;}';
  const complete = optional + "console.log('%s',describe([2,3]),describe(null),describe(undefined));";
  const observe = source => {
    const output = [];
    vm.runInNewContext(source, { console: { log: (...values) => output.push(format(...values)) } }, { timeout: 1000 });
    return output;
  };
  assert.deepEqual(observe(complete), ['object object undefined']);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(complete))).text), ['object object undefined']);
  assert.throws(() => parseJavaScript(optional + "console.log('%s',describe([2]));"), /two different lengths/u);
  assert.throws(() => parseJavaScript(optional + "console.log('%s',describe([...[2,3]]));"), error => error.kind === 'type' && /(?:array.*tuple|tuple.*array)/u.test(error.message));
  const array = optional.replace('[number,number]', 'number[]');
  assert.throws(() => parseJavaScript(array + "console.log('%s',describe([2,'kept']));"), error => error.kind === 'type' && /(?:number.*string|string.*number)/u.test(error.message));
});
