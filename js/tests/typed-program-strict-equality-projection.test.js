import assert from 'node:assert/strict';
import test from 'node:test';
import vm from 'node:vm';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { TYPED_PROGRAM_LINKS_SCHEMA, encodeTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { projectLinkedProgramExpression } from '../src/typed-program-expression-projection.js';
import { recheckLinkedTypedProgram } from '../src/typed-program-checked-projection.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

function observeStrictEqualityProgram(source) {
  const lines = [];
  let error = null;
  try {
    vm.runInNewContext(source, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 });
  } catch (caught) {
    error = caught.message;
  }
  return { lines, error };
}

function readStrictEqualityExpressions(program) {
  const seen = new WeakSet(), result = [];
  function visit(value) {
    if (!value || typeof value !== 'object' || seen.has(value)) return;
    seen.add(value);
    if (value.k === 'strictEqual') result.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) visit(child);
  }
  visit(program);
  return result;
}

const strictEqualityArchiveForms = packet => [
  LinkNetwork.fromLino(packet.network.toLino()),
  decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),
];

test('strict equality projection selects only exact Boolean flags and retains operand order and spans', () => {
  const plans = TYPED_PROGRAM_LINKS_SCHEMA.expressionProjections;
  assert.deepEqual(plans.strictEqual, {});
  assert.deepEqual(TYPED_PROGRAM_LINKS_SCHEMA.expressionProjectionAlternatives.strictEqual, [
    { path: ['negate'], value: false, projection: 'strictEqual:false' },
    { path: ['negate'], value: true, projection: 'strictEqual:true' },
  ]);
  const left = { k: 'var', name: 'left', type: { kind: 'option', element: { kind: 'float' }, absence: 'undefined' }, span: { start: 5, end: 9 } };
  const right = { k: 'var', name: 'right', type: { kind: 'option', element: { kind: 'float' }, absence: 'undefined' }, span: { start: 14, end: 19 } };
  const span = { start: 5, end: 19 };
  for (const [negate, op] of [[false, 'eq'], [true, 'ne']]) {
    assert.deepEqual(plans[`strictEqual:${negate}`], { construct: 'binary', fields: [['op', 'constant', op], ['left', 'expression', 'left'], ['right', 'expression', 'right']] });
    const original = { k: 'strictEqual', left, right, negate, type: { kind: 'bool' }, span };
    const before = structuredClone(original);
    assert.deepEqual(projectLinkedProgramExpression(original), {
      k: 'binary', span, op,
      left: { k: 'name', span: left.span, path: ['left'] },
      right: { k: 'name', span: right.span, path: ['right'] },
    });
    assert.deepEqual(original, before);
  }
  for (const negate of [null, undefined, 'true', 'false', 0, 1, [], {}]) {
    assert.throws(() => projectLinkedProgramExpression({ k: 'strictEqual', left, right, negate, type: { kind: 'bool' }, span }), /strictEqual has no surface projection/u);
  }
  assert.throws(() => projectLinkedProgramExpression({ k: 'strictEqual', left, right, type: { kind: 'bool' }, span }), /strictEqual has no surface projection/u);
});

test('strict equality semantics survive whole-program text and binary rechecks for every optional primitive domain', () => {
  const cases = [
    ['numbers', '/** @param {number[]} left @param {number[]} right @returns {boolean} */ function compare(left,right){return left[0]===right[0];} console.log(compare([],[]));console.log(compare([0],[-0]));console.log(compare([NaN],[NaN]));console.log(compare([Infinity],[Infinity]));console.log(compare([1],[]));console.log(compare([],[1]));', ['true', 'true', 'false', 'true', 'false', 'false']],
    ['inequality', '/** @param {number[]} left @param {number[]} right @returns {boolean} */ function compare(left,right){return left[0]!==right[0];} console.log(compare([],[]));console.log(compare([0],[-0]));console.log(compare([NaN],[NaN]));console.log(compare([Infinity],[Infinity]));console.log(compare([1],[]));console.log(compare([],[1]));', ['false', 'false', 'true', 'false', 'true', 'true']],
    ['strings', '/** @param {string[]} left @param {string[]} right @returns {boolean} */ function compare(left,right){return left[0]===right[0];} console.log(compare([],[]));console.log(compare([\'\'],[\'\']));console.log(compare([\'kept\'],[\'kept\']));console.log(compare([\'\'],[]));console.log(compare([],[\'\']));', ['true', 'true', 'true', 'false', 'false']],
    ['booleans', '/** @param {boolean[]} left @param {boolean[]} right @returns {boolean} */ function compare(left,right){return left[0]===right[0];} console.log(compare([],[]));console.log(compare([false],[false]));console.log(compare([true],[false]));console.log(compare([false],[]));console.log(compare([],[false]));', ['true', 'true', 'false', 'false', 'false']],
    ['distinct primitive kinds', '/** @param {number[]} left @param {string[]} right @returns {boolean} */ function compare(left,right){return left[0]===right[0];} console.log(compare([],[]));console.log(compare([0],[\'0\']));console.log(compare([0],[\'\']));console.log(compare([1],[]));', ['true', 'false', 'false', 'false']],
    ['both absence tags', '/** @param {number|null|undefined} left @param {number|null|undefined} right @returns {boolean} */ function compare(left,right){return left===right;} console.log(compare(null,null));console.log(compare(undefined,undefined));console.log(compare(null,undefined));console.log(compare(undefined,null));console.log(compare(0,-0));console.log(compare(NaN,NaN));console.log(compare(null,0));', ['true', 'true', 'false', 'false', 'true', 'false', 'false']],
  ];
  for (const [label, source, lines] of cases) {
    const expected = { lines, error: null };
    assert.deepEqual(observeStrictEqualityProgram(source), expected, label + ': independent original JavaScript');
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    const equality = readStrictEqualityExpressions(checked);
    assert.equal(equality.length, 1, label);
    assert.equal(equality[0].negate, label === 'inequality');
    const javascript = emitJavaScript(checked).text;
    const rust = emitRust(checked, { arrayConsumption: true }).text;
    assert.deepEqual(observeStrictEqualityProgram(javascript), expected, label + ': direct checked JavaScript');
    const packet = encodeTypedProgramLinks(checked);
    for (const network of strictEqualityArchiveForms(packet)) {
      const program = decodeTypedProgramLinks(network, packet.root), before = structuredClone(program);
      for (let repeat = 0; repeat < 2; repeat += 1) {
        assert.strictEqual(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
        assert.deepEqual(program, before);
      }
      assert.deepEqual(readStrictEqualityExpressions(program), equality);
      assert.equal(emitTypedProgramLinks(network, packet.root, 'JavaScript').code, javascript);
      assert.equal(emitTypedProgramLinks(network, packet.root, 'Rust').code, rust);
      assert.deepEqual(observeStrictEqualityProgram(emitJavaScript(program).text), expected, label + ': source-free checked JavaScript');
    }
  }
});

test('strict equality archives retain once-only left-to-right calls and abort boundaries without short circuiting', () => {
  for (const stopped of [null, 'left', 'right']) {
    const source = '/** @returns {number[]} */ function left(){console.log(\'left\');' + (stopped === 'left' ? 'throw new Error(\'left stopped\');' : 'return [];') + '}'
      + '/** @returns {number[]} */ function right(){console.log(\'right\');' + (stopped === 'right' ? 'throw new Error(\'right stopped\');' : 'return [];') + '}'
      + 'console.log(left()[0]===right()[0]);';
    const expected = { lines: stopped === 'left' ? ['left'] : stopped === 'right' ? ['left', 'right'] : ['left', 'right', 'true'], error: stopped === null ? null : stopped + ' stopped' };
    assert.deepEqual(observeStrictEqualityProgram(source), expected);
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    assert.equal(readStrictEqualityExpressions(checked).length, 1);
    assert.deepEqual(observeStrictEqualityProgram(emitJavaScript(checked).text), expected);
    const packet = encodeTypedProgramLinks(checked);
    for (const network of strictEqualityArchiveForms(packet)) {
      const program = decodeTypedProgramLinks(network, packet.root);
      recheckLinkedTypedProgram(program, { wholeProgram: true });
      assert.deepEqual(observeStrictEqualityProgram(emitTypedProgramLinks(network, packet.root, 'JavaScript').code), expected);
      assert.equal(emitTypedProgramLinks(network, packet.root, 'Rust').code, emitRust(checked, { arrayConsumption: true }).text);
    }
  }
});

test('strict equality graphs permit genuine Boolean edits and refuse forged flags types operands and reference identity', () => {
  const source = '/** @param {number[]} left @param {number[]} right @returns {boolean} */ function compare(left,right){return left[0]===right[0];} console.log(compare([],[]));console.log(compare([1],[]));console.log(compare([0],[-0]));console.log(compare([NaN],[NaN]));';
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const packet = encodeTypedProgramLinks(checked);
  assert.deepEqual(observeStrictEqualityProgram(source), { lines: ['true', 'false', 'true', 'false'], error: null });
  for (const network of strictEqualityArchiveForms(packet)) {
    const edited = decodeTypedProgramLinks(network, packet.root);
    readStrictEqualityExpressions(edited)[0].negate = true;
    recheckLinkedTypedProgram(edited, { wholeProgram: true });
    const saved = encodeTypedProgramLinks(edited);
    for (const roundtrip of strictEqualityArchiveForms(saved)) {
      const restored = decodeTypedProgramLinks(roundtrip, saved.root);
      assert.strictEqual(recheckLinkedTypedProgram(restored, { wholeProgram: true }), restored);
      assert.deepEqual(observeStrictEqualityProgram(emitTypedProgramLinks(roundtrip, saved.root, 'JavaScript').code), { lines: ['false', 'true', 'false', 'true'], error: null });
      assert.equal(readStrictEqualityExpressions(restored)[0].negate, true);
    }
  }
  const mutations = [
    (_program, expression) => { delete expression.negate; },
    ...[undefined, null, 'false', 'true', 0, 1, [], {}].map(negate => (_program, expression) => { expression.negate = negate; }),
    (_program, expression) => { expression.type = { kind: 'float' }; },
    (_program, expression) => { expression.left = null; },
    (_program, expression) => { expression.left.type = { kind: 'option', element: { kind: 'string' }, absence: 'undefined' }; },
    (_program, expression) => { expression.loose = true; },
    (_program, expression) => { expression.domain = { kind: 'float' }; },
    (program, _expression) => { program.schemaVersion = 2; },
  ];
  for (const mutate of mutations) {
    const program = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
    mutate(program, readStrictEqualityExpressions(program)[0]);
    assert.throws(() => recheckLinkedTypedProgram(program, { wholeProgram: true }));
  }
  const referenceSource = '/** @param {number[]|null} value @returns {boolean} */ function compare(value){return value===value;}console.log(compare(null));';
  assert.deepEqual(observeStrictEqualityProgram(referenceSource), { lines: ['true'], error: null });
  assert.throws(() => checkProgram(parseJavaScript(referenceSource, { referenceSemantics: false })), /nullable object equality/u);
});

test('deferred equality inference preserves independent optional results and historical unknown constraints', async () => {
  const { readJavaScriptEqualityInferenceStep } = await import('../src/translation/javascript-surface-rules.js');
  const { readFileSync } = await import('node:fs');
  const { createHash } = await import('node:crypto');
  const { DecoratorSet } = await import('../src/decorators.js');
  const { readSourceTextRustBlock } = await import('../scripts/source-text-byte-library-style.mjs');
  const { readPortableSourceParameterStyles, readPortableSourceLintStyles } = await import('../scripts/portable-source-library-style.mjs');
  const source = readFileSync(new URL('../src/translation/javascript-surface-rules.js', import.meta.url), 'utf8');
  const rules = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const emitted = emitRust(rules, { arrayConsumption: true });
  assert.deepEqual(emitted.preludes, []);
  const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
  assert.ok(header);
  const raw = header + '\n\n' + emitted.definitions.join('\n\n').replace(/^pub fn /gmu, '#[must_use]\npub fn ');
  const document = readFileSync(new URL('../../parity/self-translation/javascript-surface-rules-decorators.lino', import.meta.url), 'utf8');
  const lines = document.match(/[^\n]*\n|[^\n]+$/gu) ?? [];
  for (const [index, bytes, digest] of [[0, 106060, 'ac9cf3ef2a0a4a4b16e59b9c2d33a2619fd8cbe44bf4042913f52419cc275e6a'],
    [1, 1287, '61cbbe05766356d057df68a3d53fcf20bfb257f42b9ea3a45430d9d0a0112650']]) {
    assert.equal(Buffer.byteLength(lines[index]), bytes);
    assert.equal(createHash('sha256').update(lines[index]).digest('hex'), digest);
  }
  const id = 'javascript-equality-inference-library-style';
  assert.equal(lines.filter(line => line.startsWith('(decorator ' + id + ' ')).length, 1);
  const base = DecoratorSet.fromLino(lines.filter(line => !line.startsWith('(decorator ' + id + ' ')).join(''))
    .decorate('emitter', { format: 'Rust', source: raw }).source;
  const from = '#[must_use]\n' + readSourceTextRustBlock(base, 'fn', 'read_java_script_equality_inference_step');
  assert.equal(base.split(from).length, 2);
  const to = readPortableSourceLintStyles(readPortableSourceParameterStyles(from).source).source;
  assert.match(to, /pub fn read_java_script_equality_inference_step\(\s*operation: &str,\s*left: &str,\s*right: &str,\s*force: bool\s*\)/u);
  assert.equal((to.match(/^#\[must_use\]$/gmu) ?? []).length, 1);
  assert.equal(readPortableSourceLintStyles(to).source, to);
  assert.equal(DecoratorSet.fromLino(document).decorate('emitter', { format: 'Rust', source: raw }).source, base.replace(from, to));
  const translated = vm.runInNewContext(emitJavaScript(rules).text + '\nreadJavaScriptEqualityInferenceStep;');
  for (const operation of ['eq', 'ne', 'lt', 'le', 'gt', 'ge']) for (const left of ['var', 'option', 'float', 'string', 'array']) for (const right of ['var', 'option', 'float', 'string', 'array']) for (const force of [false, true]) {
    const expected = operation !== 'eq' && operation !== 'ne' ? 'unify'
      : left === 'var' || right === 'var' ? force ? 'unify' : 'defer'
        : left === 'option' || right === 'option' ? 'independent' : 'unify';
    assert.equal(readJavaScriptEqualityInferenceStep(operation, left, right, force), expected);
    assert.equal(translated(operation, left, right, force), expected);
  }
  const readers = ["function readNumber(values){console.log('left');return values[0];}",
    "function readText(values){console.log('right');return values[0];}"];
  for (const declarations of [readers, [...readers].reverse()]) {
    const source = '/** @param {number[]} numbers @param {string[]} texts @returns {boolean} */function compare(numbers,texts){return readNumber(numbers)===readText(texts);}'
      + declarations.join('\n') + 'console.log(compare([],[]));console.log(compare([0],[\'0\']));';
    const expected = { lines: ['left', 'right', 'true', 'left', 'right', 'false'], error: null };
    assert.deepEqual(observeStrictEqualityProgram(source), expected);
    const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
    assert.equal(readStrictEqualityExpressions(checked).length, 1);
    const packet = encodeTypedProgramLinks(checked);
    for (const network of strictEqualityArchiveForms(packet)) {
      const program = decodeTypedProgramLinks(network, packet.root);
      assert.strictEqual(recheckLinkedTypedProgram(program, { wholeProgram: true }), program);
      assert.deepEqual(observeStrictEqualityProgram(emitJavaScript(program).text), expected);
      assert.equal(emitRust(program, { arrayConsumption: true }).text, emitRust(checked, { arrayConsumption: true }).text);
    }
  }
  const unknown = checkProgram(parseJavaScript('function compare(left,right){return left===right;}', { referenceSemantics: false }));
  assert.deepEqual(unknown.declarations.get('compare').params.map(parameter => parameter.type.kind), ['float', 'float']);
  const optionalUnknown = checkProgram(parseJavaScript('/** @param {number[]} values @returns {number|undefined} */function read(values){return values[0];}function compare(value){return value===read([1]);}', { referenceSemantics: false }));
  assert.equal(optionalUnknown.declarations.get('compare').params[0].type.kind, 'option');
  assert.equal(optionalUnknown.declarations.get('compare').params[0].type.absence, 'undefined');
  for (const source of [
    '/** @param {number} left @param {string} right @returns {boolean} */function compare(left,right){return left===right;}console.log(compare(0,\'0\'));',
    '/** @param {number[]} left @param {string[]} right @returns {boolean} */function compare(left,right){return left[0]<right[0];}console.log(compare([0],[\'0\']));',
  ]) {
    assert.deepEqual(observeStrictEqualityProgram(source), { lines: ['false'], error: null });
    assert.throws(() => checkProgram(parseJavaScript(source, { referenceSemantics: false })), error => error.kind === 'type');
  }
  const referenceSource = 'function maybe(present){if(present)return [1];}function compare(present){return maybe(present)===maybe(present);}console.log(compare(false));';
  assert.deepEqual(observeStrictEqualityProgram(referenceSource), { lines: ['true'], error: null });
  assert.throws(() => checkProgram(parseJavaScript(referenceSource, { referenceSemantics: false })), /nullable object equality/u);
});
