import assert from 'node:assert/strict';
import test from 'node:test';
import { runInNewContext } from 'node:vm';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { encodeTypedProgramLinks, decodeTypedProgramLinks, TYPED_PROGRAM_LINKS_SCHEMA as schema } from '../src/typed-program-links.js';
import { LinkNetwork } from '../src/network.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { checkJavaScriptArrayBindingWrapper } from '../src/translation/array-binding-validation.js';
import { readJavaScriptArrayDefaultConstraints } from '../src/translation/array-binding-inference.js';
import { readArrayBindingSourceModule } from '../scripts/generate-array-binding-rules.mjs';
import { accepted, refused, correctedBindingRefusals, bindingAssertionCorrectionBasis } from './fixtures/array-binding-cases.mjs';

const [nodeMajor,nodeMinor]=process.versions.node.split('.').map(Number);
assert.ok(nodeMajor===22&&nodeMinor>=15||nodeMajor===24,'uncovered Node '+process.versions.node);

function execute(source) {
  const lines = [];
  runInNewContext(source, { console: { log: (...values) => lines.push(values.map(String).join(' ')) } }, { timeout: 1000 });
  return lines.length ? lines.join('\n') + '\n' : '';
}

for (const fixture of accepted) test(fixture.id, () => {
  assert.equal(execute(fixture.source), fixture.expected, 'independent original source oracle');
  const checked = checkProgram(parseJavaScript(fixture.source));
  const packet = encodeTypedProgramLinks(checked);
  const text = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  const binary = decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root);
  const rust = emitRust(checked).text;
  for (const program of [checked, text, binary]) {
    assert.equal(execute(emitJavaScript(program).text), fixture.expected, 'checked and source-free programs preserve output order');
    assert.equal(emitRust(program).text, rust, 'checked and source-free programs emit the same complete Rust text');
  }
});

for (const [id, source] of refused) test(id + ' retains original input and its active assertion', () => {
  const correction=correctedBindingRefusals.find(row=>row.id===id);
  if(correction){
    assert.ok(bindingAssertionCorrectionBasis.source);
    assert.equal(correction.previousAssertion,'checkProgram throws for the original input');
    assert.equal(execute(source),correction.expected);
    const checked=checkProgram(parseJavaScript(source)),packet=encodeTypedProgramLinks(checked);
    const rust=emitRust(checked).text;
    for(const program of [checked,decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()),packet.root),decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)),packet.root)]){
      assert.equal(execute(emitJavaScript(program).text),correction.expected);
      assert.equal(emitRust(program).text,rust);
    }
    return;
  }
  if (id === 'reserved-prefix') {
    // ECMAScript permits ml_; retain the original input and independent oracle.
    const runnable = source + '\nconsole.log(f([2,3]));';
    const checked = checkProgram(parseJavaScript(runnable));
    assert.equal(execute(runnable),'1\n');
    assert.equal(execute(emitJavaScript(checked).text),execute(runnable));
    return;
  }
  assert.throws(() => checkProgram(parseJavaScript(source)));
});

test('defaults retain null instead of treating it as undefined', () => {
  const source = 'function selected(){const [value=7]=[null];return value;}console.log(selected());';
  assert.equal(execute(source), 'null\n');
  assert.equal(execute(emitJavaScript(checkProgram(parseJavaScript(source))).text),'null\n');
});

test('binding scope retains the RHS and initializer temporal dead zones', () => {
  for (const source of [
    'function selected(){const values=[1];{const [...values]=values;return values.length;}}console.log(selected());',
    'function selected(values){const [head=head]=values;return head;}console.log(selected([]));',
    'function selected(values){const [first=second,second=2]=values;return first;}console.log(selected([]));',
  ]) {
    assert.throws(() => execute(source), error => error.name === 'ReferenceError');
    assert.throws(() => checkProgram(parseJavaScript(source)), /temporal dead zone/u);
  }
});

test('an aborting default stops before the next initializer', () => {
  const source = '/** @returns {number} */function first(){console.log("first");throw new Error("stop");}function second(){console.log("second");return 9;}function selected(values){const [left=first(),right=second()]=values;return left+right;}console.log(selected([]));';
  const checked = checkProgram(parseJavaScript(source));
  for (const code of [source, emitJavaScript(checked).text]) {
    const lines = [];
    assert.throws(() => runInNewContext(code, { console: { log: value => lines.push(String(value)) } }, { timeout: 1000 }), /stop/u);
    assert.deepEqual(lines, ['first']);
  }
});

test('a source-free stored suffix operand remains an editable semantic link', () => {
  const source = 'function selected(values){const [head,...tail]=values;return tail.length;}console.log(selected([2,3,5]));';
  const saved = encodeTypedProgramLinks(checkProgram(parseJavaScript(source)));
  for (const binary of [false, true]) {
    const network = binary ? decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network)) : LinkNetwork.fromLino(saved.network.toLino());
    const relations = network.fieldRelations();
    const field = (parent, name) => relations.find(row => row.parent.equals(parent) && row.label === (schema.fields[name] ?? name))?.child;
    const method = relations.find(row => row.label === 'construct kind' && network.link(row.child).metadata().definition === 'method'
      && network.link(field(row.parent, 'op')).metadata().definition === 'slice')?.parent;
    assert.ok(method, 'the actual checked suffix method survives serialization');
    const start = network.link(field(method, 'args')).references()[0];
    const value = network.link(field(start, 'value'));
    assert.equal(value.metadata().definition, '1');
    assert.equal(execute(emitJavaScript(decodeTypedProgramLinks(network, saved.root)).text), '2\n');
    value.setMetadata(value.metadata().withDefinition('2'));
    assert.equal(execute(emitJavaScript(decodeTypedProgramLinks(network, saved.root)).text), '1\n');
  }
});

test('raw rest wrappers reject missing forged inherited and nonliteral operands', () => {
  const object = { k: 'name', path: ['values'] };
  const rest = start => ({ k: 'arrayBindingRest', object, start });
  const valid = rest({ k: 'num', value: '0' });
  assert.doesNotThrow(() => checkJavaScriptArrayBindingWrapper(valid));
  for (const invalid of [
    { ...valid, type: { kind: 'array', element: { kind: 'float' } } },
    { ...valid, object: null }, { k: 'arrayBindingRest', start: valid.start },
    Object.assign(Object.create({ object }), { k: 'arrayBindingRest', start: valid.start }),
    { ...valid, parenthesized: null },
    ...['NaN', 'Infinity', '0.5', '-1', '9007199254740992'].map(value => rest({ k: 'num', value })),
    rest({ k: 'num', value: false }), rest({ k: 'name', path: ['offset'] }),
    rest({ k: 'num', value: '0', negative: 'false' }),
    rest({ k: 'num', value: '0', extra: true }),
  ]) assert.throws(() => checkJavaScriptArrayBindingWrapper(invalid));
  const getter = { ...valid };
  Object.defineProperty(getter, 'object', { enumerable: true, get() { throw new Error('must not evaluate a forged getter'); } });
  assert.throws(() => checkJavaScriptArrayBindingWrapper(getter), /unrepresented field/u);
  const defaultValue = { k: 'arrayBindingDefault', value: { k: 'index', object, index: { k: 'num', value: '0' } }, fallback: { k: 'num', value: '7' } };
  assert.doesNotThrow(() => checkJavaScriptArrayBindingWrapper(defaultValue));
  for (const invalid of [{ ...defaultValue, unknown: true }, { ...defaultValue, fallback: null }, { ...defaultValue, value: { k: 'none', absence: 'undefined' } }]) {
    assert.throws(() => checkJavaScriptArrayBindingWrapper(invalid));
  }
});

test('default inference plans retain original terms and defer unresolved values', () => {
  const element = { kind: 'var', id: 1 }, fallback = { kind: 'var', id: 2 }, result = { kind: 'var', id: 3 };
  assert.equal(readJavaScriptArrayDefaultConstraints({ kind: 'var', id: 0 }, fallback, result), null);
  const plan = readJavaScriptArrayDefaultConstraints({ kind: 'option', element, absence: 'undefined' }, fallback, result);
  assert.deepEqual(plan.pairs, [[element, fallback], [result, element]]);
  assert.equal(plan.pairs[0][0], element, 'actual element identity, not a reconstructed type');
  assert.equal(plan.pairs[0][1], fallback);
  assert.equal(plan.pairs[1][0], result);
  for (const absence of ['null', 'both']) assert.equal(readJavaScriptArrayDefaultConstraints({ kind: 'option', element, absence }, fallback, result).refused, true);
});

test('validated offset payloads normalize only the internal Number while retaining the raw surface', () => {
  const source = 'function selected(values){const [head,...tail]=values;return tail.length;}console.log(selected([2,3,5]));';
  const find = value => {
    if (!value || typeof value !== 'object') return null;
    if (value.k === 'arrayBindingRest') return value;
    for (const child of Object.values(value)) {
      const result = find(child);
      if (result) return result;
    }
    return null;
  };
  for (const [payload, canonical, expected] of [['', '0', '3\n'], [' \t\n', '0', '3\n'], ['0x1', '1', '2\n'], ['1e0', '1', '2\n'], ['-0', '-0', '3\n']]) {
    const surface = parseJavaScript(source);
    const wrapper = find(surface.items[0].body);
    assert.ok(wrapper);
    wrapper.start.value = payload;
    const before = JSON.stringify(wrapper);
    const checked = checkProgram(surface);
    assert.equal(JSON.stringify(wrapper), before, 'the original raw offset record is unchanged');
    const saved = encodeTypedProgramLinks(checked);
    for (const network of [LinkNetwork.fromLino(saved.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(saved.network))]) {
      const program = decodeTypedProgramLinks(network, saved.root);
      assert.equal(execute(emitJavaScript(program).text), expected);
      assert.equal(emitRust(program).text, emitRust(checked).text);
      const literals = network.fieldRelations().filter(row => row.label === (schema.fields.value ?? 'value')).map(row => network.link(row.child).metadata().definition);
      assert.ok(literals.includes(canonical), 'the actual checked Number spelling is retained');
    }
  }
  for (const payload of ['1_0', '-0x1', '+0x1']) {
    const surface = parseJavaScript(source);
    find(surface.items[0].body).start.value = payload;
    assert.throws(() => checkProgram(surface), /actual nonnegative safe integer literal/u);
  }
});

test('the complete portable array binding kernel checks and survives both Links transports', () => {
  const source = readArrayBindingSourceModule();
  const checked = checkProgram(parseJavaScript(source, { referenceSemantics: false }));
  const packet = encodeTypedProgramLinks(checked);
  const restored = [
    decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root),
    decodeTypedProgramLinks(decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network)), packet.root),
  ];
  const rust = emitRust(checked, { arrayConsumption: true }).text;
  assert.ok(rust.length > 0);
  for (const program of restored) assert.equal(emitRust(program, { arrayConsumption: true }).text, rust);
  const context = {};
  runInNewContext(emitJavaScript(checked).text, context, { timeout: 1000 });
  assert.equal(context.acceptJavaScriptArrayRestReceiver('array'), true);
  assert.equal(context.acceptJavaScriptArrayRestReceiver('string'), false);
  assert.equal(context.readJavaScriptArrayDefaultOperation('JavaScript', 'option', 'undefined', 'float', true), 'coalesce');
  assert.equal(context.readJavaScriptArrayDefaultOperation('JavaScript', 'option', 'both', 'float', true), 'refuse-absence');
});
