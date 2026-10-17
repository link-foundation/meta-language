import { readProjectData } from '../scripts/project-data.mjs';
import assert from 'node:assert/strict';

import { test } from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { translateProgram } from '../src/program-translation.js';
import { requireDataObjectFreeze } from '../src/translation/primitive-value-rules.js';

const fixture = readProjectData(new URL('../../parity/fixtures/translation-constructor-tags.json', import.meta.url));
const { tags } = fixture;
const source = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one' }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return 1; return 2; }";

test('constructor tag data is safely escaped and keeps its value through construction and dispatch', () => {
  for (const tag of tags) {
    const program = checkProgram(parseJavaScript(source));
    const entry = program.declarations.get('Tag');
    entry.ctors[0].name = tag;
    const renameReference = value => {
      if (!value || typeof value !== 'object') return;
      if (value.ctor === 'one') value.ctor = tag;
      for (const child of Object.values(value)) {
        if (Array.isArray(child)) child.forEach(renameReference);
        else renameReference(child);
      }
    };
    for (const declaration of program.declarations.values()) renameReference(declaration);
    const emitted = emitJavaScript(program).text;
    assert.ok(emitted.includes(`$: ${JSON.stringify(tag)}`), tag);
    assert.ok(emitted.includes(`case ${JSON.stringify(tag)}:`), tag);
    const context = { injected: false };
    const actual = runInNewContext(`${emitted}\n[make().$, read(make()), read({ $: 'two' })]`, context);
    assert.equal(actual[0], tag);
    assert.equal(actual[1], 1);
    assert.equal(actual[2], 2);
    assert.equal(context.injected, false, tag);
  }
});

test('constructor fields keep arbitrary names as own data properties during construction and dispatch', () => {
  const source = "/** @typedef {{ $: 'one', payload: number } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return { $: 'one', payload: 7 }; }\n/** @param {Tag} value @returns {number} */ function read(value) { if (value.$ === 'one') return value.payload; return 2; }";
  for (const field of fixture.fields) {
    const program = checkProgram(parseJavaScript(source));
    program.declarations.get('Tag').ctors[0].fields[0].name = field;
    const emitted = emitJavaScript(program).text;
    assert.ok(emitted.includes(`[${JSON.stringify(field)}]: 7`), field);
    const context = { field };
    const actual = runInNewContext(`${emitted}\n(() => { const item = make(); const supplied = { $: 'one', [field]: 9 }; return [Object.hasOwn(item, field), item[field], read(item), read(supplied), Object.getPrototypeOf(item) === Object.prototype]; })()`, context);
    assert.deepEqual(Array.from(actual), [true, 7, 7, 9, true], field);
  }
});

test('constructor descriptors retain their declared source model and explicit immutability', () => {
  for (const { language, frozen: explicitFrozen, descriptors } of fixture.models) {
    const program = checkProgram(parseJavaScript(source));
    program.sourceLanguage = language;
    if (explicitFrozen) program.declarations.get('make').body.frozen = true;
    const frozen = requireDataObjectFreeze(language, explicitFrozen);
    assert.equal(frozen, explicitFrozen || language !== 'JavaScript');
    const emitted = emitJavaScript(program).text;
    const actual = runInNewContext(`${emitted}\n(() => { const item = make(); const descriptor = Object.getOwnPropertyDescriptor(item, '$'); return [Object.isFrozen(item), Object.isExtensible(item), descriptor.writable, descriptor.configurable, Reflect.set(item, '$', 'two')]; })()`);
    assert.deepEqual(Array.from(actual), descriptors, `${language}:${explicitFrozen}`);
  }
  const frozenSource = "/** @typedef {{ $: 'one' } | { $: 'two' }} Tag */\n/** @returns {Tag} */ function make() { return Object.freeze({ $: 'one' }); }";
  assert.equal(checkProgram(parseJavaScript(frozenSource)).declarations.get('make').body.frozen, true);
  for (const target of ['Rust', 'Lean', 'Rocq']) assert.match(translateProgram(frozenSource, 'JavaScript', target).diagnostic?.message ?? '', /^explicit data object freeze:/u, target);
});


test('terminal tag guard sequences preserve constructor fields scope and ordered effects through both semantic archives', async () => {
  const { encodeTypedProgramLinks, decodeTypedProgramLinks } = await import('../src/typed-program-links.js');
  const { recheckLinkedTypedProgram } = await import('../src/typed-program-checked-projection.js');
  const { LinkNetwork } = await import('../src/network.js');
  const { encodeLinkNetworkBinary, decodeLinkNetworkBinary } = await import('../src/link-network-binary.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const declarations = "/** @typedef {{ $: 'missing' } | { $: 'bad' } | { $: 'value', code: number, end: number }} GuardState */\n"
    + "/** @param {number} n @returns {GuardState} */function make(n){if(n===0)return {$:'missing'};if(n===1)return {$:'bad'};return {$:'value',code:3,end:4};}\n";
  const observe = text => {
    const output = [];
    runInNewContext(text, { console: { log: (...values) => output.push(values.map(String).join(' ')) } }, { timeout: 1000 });
    return output;
  };
  const variants = [
    ["if(value.$==='missing')return -1;if(value.$==='bad')return -2;return value.code+value.end;", ['-1', '-2', '7']],
    ["if(value.$==='bad')return -2;if(value.$==='missing')return -1;return value.end+value.code;", ['-1', '-2', '7']],
    ["if(value.$==='missing'){const value=99;return value;}if(value.$==='bad')return -2;return value.code+value.end;", ['99', '-2', '7']],
    ["let count=0;if(value.$==='missing'){console.log('missing');return -1;}if(value.$==='bad'){console.log('bad');return -2;}count+=value.code;return count+value.end;", ['missing', '-1', 'bad', '-2', '7']],
    ["if(value.$!=='value')return -1;return value.code+value.end;", ['-1', '-1', '7']],
  ];
  for (const [body, expected] of variants) for (const options of [{}, { referenceSemantics: false }]) {
    const original = declarations + '/** @param {GuardState} value @returns {number} */function read(value){' + body + '}\n'
      + 'console.log(read(make(0)));console.log(read(make(1)));console.log(read(make(2)));';
    assert.deepEqual(observe(original), expected);
    const checked = checkProgram(parseJavaScript(original, options));
    assert.deepEqual(observe(emitJavaScript(checked).text), expected);
    const rust = emitRust(checked, { arrayConsumption: true }).text;
    const saved = encodeTypedProgramLinks(checked);
    for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
      const rechecked = recheckLinkedTypedProgram(decodeTypedProgramLinks(network, saved.root), { wholeProgram: true });
      assert.deepEqual(observe(emitJavaScript(rechecked).text), expected);
      assert.equal(emitRust(rechecked, { arrayConsumption: true }).text, rust);
    }
  }
  const ordered = declarations.replace('function make(n){', "function make(n){console.log('make',n);")
    + "/** @param {GuardState} value @returns {number} */function read(value){if(value.$==='missing'){console.log('missing');return -1;}if(value.$==='bad'){console.log('bad');return -2;}return value.code+value.end;}\n"
    + 'console.log(read(make(0)));console.log(read(make(1)));console.log(read(make(2)));';
  const expected = ['make 0', 'missing', '-1', 'make 1', 'bad', '-2', 'make 2', '7'];
  assert.deepEqual(observe(ordered), expected);
  assert.deepEqual(observe(emitJavaScript(checkProgram(parseJavaScript(ordered))).text), expected);
  for (const body of [
    "if(value.$==='missing')return -1;return value.code;",
    "if(value.$==='missing')return -1;if(value.$==='missing')return -2;return value.code;",
    "if(value.$==='missing'){const ignored=0;}if(value.$==='bad')return -2;return value.code;",
    "if(value.$==='missing')return -1;value=make(0);if(value.$==='bad')return -2;return value.code;",
  ]) {
    const original = declarations + '/** @param {GuardState} value @returns {number} */function read(value){' + body + '}';
    assert.throws(() => checkProgram(parseJavaScript(original)), error => ['type', 'unsupported'].includes(error.kind),
      'uncovered constructors, duplicate guards, nonterminal branches and intervening writes gain no false field proof');
  }
  const different = declarations + "/** @param {GuardState} left @param {GuardState} right @returns {number} */function read(left,right){if(left.$==='missing')return -1;if(right.$==='bad')return -2;return left.code;}";
  assert.throws(() => checkProgram(parseJavaScript(different)), error => ['type', 'unsupported'].includes(error.kind),
    'separate bindings of the same data type cannot share exclusions');
});
