import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import vm from 'node:vm';
import { DecoratorSet } from '../src/decorators.js';
import { applyRecordedFamilyLibraryStyles } from '../scripts/recorded-family-library-style.mjs';
import { formatProductRuleSource } from '../scripts/product-rule-style-corrections.mjs';

import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import {encodeTypedProgramLinks,decodeTypedProgramLinks} from '../src/typed-program-links.js';
import {recheckLinkedTypedProgram} from '../src/typed-program-checked-projection.js';
import {LinkNetwork} from '../src/network.js';
import {encodeLinkNetworkBinary,decodeLinkNetworkBinary} from '../src/link-network-binary.js';

function observe(source) {
  const output = [];
  vm.runInNewContext(source, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return output;
}

test('primitive array joins preserve number text nullable cells UTF16 separators and both complete source-free archives',()=>{
  for(const [source,expected] of [
    ["console.log([0,-0,NaN,Infinity,-Infinity,1e-7,1e21].join());",['0,0,NaN,Infinity,-Infinity,1e-7,1e+21']],
    ["console.log([true,false,true].join('|'));",['true|false|true']],
    ["console.log([1,null,undefined,2].join('|'));",['1|||2']],
    ["console.log([null,undefined].join());",[',']],
    ["console.log(['a','b'].join('😀'));",['a😀b']],
    ["/** @param {number[]} values @returns {string} */ function join(values){return values.join('|');} console.log(join([1,2]));",['1|2']],
    ["function separator(){console.log('separator');return '|';} console.log([1,2].join(separator()));",['separator','1|2']],
  ]) {
    assert.deepEqual(observe(source),expected);
    const program=checkProgram(parseJavaScript(source)),rust=emitRust(program),packet=encodeTypedProgramLinks(program);
    for(const network of [LinkNetwork.fromLino(packet.network.toLino()),decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      const restored=recheckLinkedTypedProgram(decodeTypedProgramLinks(network,packet.root),{wholeProgram:true});
      assert.deepEqual(observe(emitJavaScript(restored).text),expected);
      assert.deepEqual(emitRust(restored),rust);
    }
  }
});

test('primitive array joins retain structured conversion arity and isolated source units',()=>{
  assert.throws(()=>checkProgram(parseJavaScript("console.log([{value:1}].join());")),/array join: joining structured values/u);
  assert.throws(()=>checkProgram(parseJavaScript("console.log([1].join(',',','));")),/expects 0 to 1 arguments/u);
  // Corrected by ECMAScript 2025 StringLiteral SV; the preceding expectation
  // was /not a scalar value/u. Preserve the original source and observe units.
  const [major,minor]=process.versions.node.split('.').map(Number);
  assert.ok(major===22&&minor>=15||major===24,'uncovered Node '+process.versions.node);
  const source="console.log(['\\uD800'].join());";
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(source))).text),observe(source));
  assert.equal(observe(source)[0].charCodeAt(0),0xd800);
});

test('primitive methods retain default slots, UTF-16 results and actual evaluation order', () => {
  const source = '/** @param {string} text @returns {string} */function pick(text){console.log(`${text}`);return text;}\n'
    + "console.log(`${pick('A😀B').slice(1,3)}`);console.log(`${'abc'.slice()}`);console.log('a,b'.split(',').length);"
    + "console.log('abc'.split().length);console.log('abc'.indexOf('b'));console.log('aba'.lastIndexOf('a'));"
    + "console.log(`${'x'.repeat(3)}`);console.log(`${'x'.padStart(3)}`);console.log(`${'x'.padEnd(3,'.')}`);"
    + "console.log(`${'aba'.replace('a','x')}`);console.log(`${'aba'.replaceAll('a','x')}`);console.log([1,2,3].slice(1).length);"
    + "console.log(`${' A😀B '.trim().toLowerCase().slice(0,4)}`);console.log('A😀B'.length);console.log(`${'x'.repeat(1)+'Y'}`);"
    + "console.log('A😀B'.includes('😀'),'A😀B'.startsWith('A'),'A😀B'.endsWith('B'));";
  const checked = checkProgram(parseJavaScript(source));
  const expected = ['A😀B', '😀', 'abc', '2', '1', '1', '2', 'xxx', '  x', 'x..', 'xba', 'xbx', '2', 'a😀b', '4', 'xY', 'true true true'];
  assert.deepEqual(observe(source), expected);
  assert.deepEqual(observe(emitJavaScript(checked).text), expected);
  const rust = emitRust(checked).text;
  assert.ok(rust.includes('.split(None, 4294967295f64)'));
  assert.ok(rust.includes('crate::ml_array::slice('));
  for (const emit of [emitLean, emitRocq]) assert.throws(() => emit(checked), error => error.kind === 'unsupported'
    && error.reason === '.slice(): this target needs a UTF-16 operational string encoding for this operation');
});

test('projected absent split separator remains an ordered null host slot', () => {
  const surface = parseJavaScript("console.log('abc'.split().length);");
  const checked = checkProgram(surface);
  const method = checked.main.effects[0].expr.arg.array;
  assert.equal(method.k, 'method');
  assert.equal(method.args.length, 2);
  assert.equal(method.args[0], null);
  assert.equal(method.args[1].value, '4294967295');
});

test('the complete primitive method checking and rendering module self translates without runtime preludes', () => {
  const source = readFileSync(new URL('../src/translation/primitive-method-rules.js', import.meta.url), 'utf8');
  const surface = parseJavaScript(source, { referenceSemantics: false });
  const emitted = emitRust(checkProgram(surface), { arrayConsumption: true });
  assert.deepEqual(emitted.preludes, []);
  assert.equal(emitted.definitions.length, surface.items.length);
  assert.ok(emitted.definitions.some(definition => definition.includes('render_primitive_method_expression')));
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header);
  const raw = header + '\n\n' + emitted.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
  const names = [...raw.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
  const document = readFileSync(new URL('../../parity/self-translation/primitive-method-rules-decorators.lino', import.meta.url), 'utf8');
  const styles = DecoratorSet.fromLino(document);
  const base = styles.decorate('emitter', { format: 'Rust', source: raw }).source;
  assert.throws(() => applyRecordedFamilyLibraryStyles(document, formatProductRuleSource(base), {
    family: 'primitive-method', format: formatProductRuleSource, parameterLoans: true,
    baseRecordHashes: ['1d9aa8f1081ca8778572c8c33a29eb58d8c121125d6bf7dcab1a11f04dac168a'],
  }), /recorded family library styles are stale/u, 'the exact historical option refuses the current normalized-borrow record');
  const relocated = applyRecordedFamilyLibraryStyles(document, formatProductRuleSource(base), {
    family: 'primitive-method', format: formatProductRuleSource, parameterLoans: { normalizeBorrowedArguments: true },
    baseRecordHashes: ['1d9aa8f1081ca8778572c8c33a29eb58d8c121125d6bf7dcab1a11f04dac168a'],
  }).source;
  assert.deepEqual([...relocated.matchAll(/^pub fn ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]), names);
  assert.ok(names.every(name => !relocated.includes('crate::' + name)), 'every actual same-module call resolves without replacing a semantic body');
  assert.match(relocated, /pub fn read_primitive_copy_slice_form\(kind: &str\)/u);
  assert.doesNotMatch(relocated, /\b(?:form|receiver|operation): String\b/u, 'immutable decision inputs use the shared checked library loan style');
});
