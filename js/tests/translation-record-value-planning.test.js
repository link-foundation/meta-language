import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { format } from 'node:util';
import vm from 'node:vm';
import { readProjectData } from '../scripts/project-data.mjs';
import { recordValueRulesPolicy, readRecordValueSourceModule } from '../scripts/generate-record-value-rules.mjs';
import { acceptRecordFieldName, compareRecordFieldNames, readRecordValueOrder, readRecordProjection, readRecordProjectionError, readRecordFieldPosition, readRecordFieldOrder, readRecordFieldInsertionOrder, readRecordSpreadRole, readRecordTypeConstraintError, readRecordTypeConstraintStep, readRecordMethodError, readRecordFieldInferenceStep } from '../src/translation/record-value-rules.js';
import { typeError } from '../src/translation/diagnostics.js';
import { containsRecordTypeOwner } from '../src/translation/record-type-facts.js';
import { readCallableTypeChildren } from '../src/translation/callable-source.js';
import { createSurfaceReferenceFacts } from '../src/translation/surface-reference-facts.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, encodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { encodeLinkNetworkBinary, decodeLinkNetworkBinary } from '../src/link-network-binary.js';
import { LinkNetwork } from '../src/network.js';
import { tokenize } from '../src/translation/lexer.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitRust } from '../src/translation/emit-rust.js';

const units = name => Array.from({ length: name.length }, (_, index) => name.charCodeAt(index));
const hash = text => createHash('sha256').update(text).digest('hex');

function recordOwnerArguments(term, owner, bindings) {
  const facts = createSurfaceReferenceFacts([term, owner, ...Array.from(bindings)], { retainValues: true });
  const roots = facts.targets.slice(facts.starts[facts.root], facts.starts[facts.root] + facts.counts[facts.root]);
  return [facts.texts, facts.values.map(value => typeof value === 'number' ? value : NaN), facts.starts,
    facts.counts, facts.labels, facts.targets, roots.slice(2), roots[0], roots[1]];
}

function readSourceMethod(source, name) {
  const { tokens } = tokenize(source, 'JavaScript');
  const stack = [], matching = new Map();
  tokens.forEach((token, index) => {
    if (token.kind !== 'punct') return;
    if (['(', '[', '{'].includes(token.value)) stack.push(index);
    else if ([')', ']', '}'].includes(token.value)) {
      const open = stack.pop();
      assert.equal(tokens[open]?.value, { ')': '(', ']': '[', '}': '{' }[token.value]);
      matching.set(open, index);
    }
  });
  assert.equal(stack.length, 0);
  const methods = [];
  tokens.forEach((token, index) => {
    if (token.kind !== 'identifier' || token.value !== name || tokens[index + 1]?.value !== '(') return;
    const close = matching.get(index + 1);
    if (tokens[close + 1]?.value !== '{') return;
    const end = matching.get(close + 1);
    assert.notEqual(end, undefined);
    methods.push(source.slice(token.start, tokens[end].end));
  });
  assert.equal(methods.length, 1, `actual complete method ${name}`);
  return methods[0];
}

test('record plans retain last occurrences in actual UTF16 name order and visit every source binding', () => {
  const names = ['z', 'a', 'z', '\u{10000}', '\uE000', '', 'aa', 'a'];
  assert.deepEqual(readRecordValueOrder(names.length, false), [0, 1, 2, 3, 4, 5, 6, 7]);
  assert.deepEqual(readRecordValueOrder(names.length, true), [7, 6, 5, 4, 3, 2, 1, 0]);
  assert.deepEqual(readRecordFieldOrder(names.map(units)), [5, 7, 6, 2, 3, 4]);
  assert.deepEqual(readRecordFieldInsertionOrder(names.map(units)), [2, 7, 3, 4, 5, 6]);
  assert.equal(readRecordFieldPosition(names.map(units), units('a')), 1);
  assert.equal(readRecordFieldPosition(names.map(units), units('absent')), -1);
  assert.equal(compareRecordFieldNames(units('\u{10000}'), units('\uE000')), -1);
  assert.equal(compareRecordFieldNames(units('a'), units('aa')), -1);
  assert.equal(compareRecordFieldNames([0], [-0]), 0);
  const vocabulary = ['', 'a', 'aa', 'z', '2', '10', '__proto__', '\uD800', '\u{10000}', '\uE000'];
  for (let count = 0; count < 64; count += 1) {
    const input = Array.from({ length: count }, (_, index) => vocabulary[(index * 7 + count) % vocabulary.length]);
    const old = new Map();
    input.forEach((name, index) => old.set(name, index));
    const expected = [...old].sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([, index]) => index);
    assert.deepEqual(readRecordFieldOrder(input.map(units)), expected);
    assert.deepEqual(readRecordFieldInsertionOrder(input.map(units)), [...old.values()]);
  }
  for (const count of [-1, 0.5, NaN, Infinity, Number.MAX_SAFE_INTEGER + 1]) assert.deepEqual(readRecordValueOrder(count, false), [-1]);
  const beyondRemovedCap = readRecordValueOrder(100001, false);
  assert.equal(beyondRemovedCap.length, 100001);
  assert.equal(beyondRemovedCap[0], 0);
  assert.equal(beyondRemovedCap[100000], 100000);
  for (const unit of [-1, 65536, 0.5, NaN, Infinity, '97', null, undefined]) {
    assert.equal(acceptRecordFieldName([unit]), false);
    assert.deepEqual(readRecordFieldOrder([[unit]]), [-1]);
    assert.deepEqual(readRecordFieldInsertionOrder([[unit]]), [-1]);
  }
  assert.equal(acceptRecordFieldName(new Array(1)), false);
  assert.deepEqual(readRecordFieldOrder([new Array(1)]), [-1]);
  assert.equal(readRecordFieldPosition([units('a'), [NaN]], units('a')), -1);
});

test('both packages retain the complete shared planner and authenticated original checker method', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  assert.deepEqual(recordValueRulesPolicy(), policy);
  assert.deepEqual(readProjectData(new URL('../../rust/src/data/record-values.lino', import.meta.url)), policy);
  assert.equal(hash(policy.original.method), policy.original.methodSha256);
  assert.equal(hash(readFileSync(new URL('../src/translation/record-value-rules.js', import.meta.url), 'utf8')), policy.sourceSha256);
  assert.equal(policy.sourceClosure.source, readRecordValueSourceModule());
  assert.equal(hash(policy.sourceClosure.source), policy.sourceClosure.sourceSha256);
  assert.equal(policy.sourceClosure.inputs[1].sourceSha256, hash(readFileSync(new URL('../src/translation/fact-column-rules.js', import.meta.url), 'utf8')));
  assert.ok(policy.original.method.includes('const fields = new Map();'));
  assert.ok(policy.original.method.includes('bindings.reverse()'));
  assert.equal(hash(policy.originalInference.record), policy.originalInference.recordSha256);
  assert.equal(hash(policy.originalInference.unify), policy.originalInference.unifySha256);
  assert.equal(hash(policy.originalOwner.contains), policy.originalOwner.containsSha256);
  assert.equal(hash(policy.originalOwner.add), policy.originalOwner.addSha256);
});

test('record methods keep missing nonfunction and arity errors before argument checking', () => {
  const baseline = (kind, name, count, parameters) => {
    if (kind !== 'function') return `record has no method ${name}`;
    if (count !== parameters) return `.${name}() expects ${parameters} arguments`;
    return '';
  };
  for (const kind of ['', 'int', 'string', 'function']) {
    for (const count of [0, 1, 2]) assert.equal(readRecordMethodError(kind, 'next', 1, count), baseline(kind, 'next', count, 1));
  }
  const checkerSource = readFileSync(new URL('../src/translation/check.js', import.meta.url), 'utf8');
  const method = vm.runInNewContext(`({${readSourceMethod(checkerSource, 'method')}})`, { typeError, readRecordMethodError }).method;
  for (const [fields, count, message] of [
    [[], 1, 'record has no method slice'],
    [[{ name: 'slice', type: { kind: 'int' } }], 1, 'record has no method slice'],
    [[{ name: 'slice', type: { kind: 'function', params: [{ kind: 'int' }], ret: { kind: 'int' } } }], 2, '.slice() expects 1 arguments'],
  ]) {
    const receiver = { k: 'var', name: 'value', type: { kind: 'record', fields } };
    const seen = [];
    const host = { referenceValue: value => value, expr(value) {
      seen.push(value);
      if (value !== receiver) throw new Error('an argument was checked before the record method diagnostic');
      return value;
    } };
    assert.throws(() => method.call(host, { object: receiver, op: 'slice', args: Array.from({ length: count }, () => ({ k: 'unknown' })) }), error => error.message === message);
    assert.deepEqual(seen, [receiver]);
  }
  // An unregistered member name follows the existing general application
  // route; it does not establish the separate Method-node precedence above.
  assert.throws(() => createTypedProgramLinks('function choose(){const value={next:1n};return value.missing(unknown);}choose();', 'JavaScript'), /function value unknown/u);
});

test('record field cycle admission retains original object identity variable aliases and every structural type child', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  const source = readFileSync(new URL('../src/translation/javascript-infer.js', import.meta.url), 'utf8');
  const original = vm.runInNewContext(`({${readSourceMethod(source, 'resolve')},${policy.originalOwner.contains}})`, { readCallableTypeChildren });
  const owner = { kind: 'record', fields: [] }, other = { kind: 'record', fields: [] };
  const wrappers = [value => ({ kind: 'array', element: value }), value => ({ kind: 'option', element: value, absence: 'undefined' }),
    value => ({ kind: 'set', element: value }), value => ({ kind: 'cell', element: value }),
    value => ({ kind: 'tuple', elements: [value] }), value => ({ kind: 'union', alternatives: [{ kind: 'int' }, value] }),
    value => ({ kind: 'map', key: value, value: { kind: 'bool' } }), value => ({ kind: 'map', key: { kind: 'bool' }, value }),
    value => ({ kind: 'function', params: [value], ret: { kind: 'bool' } }), value => ({ kind: 'function', params: [], ret: value }),
    value => ({ kind: 'record', fields: [{ name: 'child', type: value }] })];
  const cycle = { kind: 'record', fields: [] };
  cycle.fields.push({ name: 'self', type: cycle });
  const cases = [{ term: owner, bindings: [], expected: true }, { term: other, bindings: [], expected: false },
    { term: cycle, bindings: [], expected: false },
    { term: { kind: 'var', id: 0 }, bindings: [owner], expected: true },
    { term: { kind: 'var', id: 0 }, bindings: [{ kind: 'var', id: 1 }, wrappers[0](owner)], expected: true },
    { term: { kind: 'var', id: 0 }, bindings: [null], expected: false }];
  for (const [index, wrap] of wrappers.entries()) cases.push({ term: wrap(owner), bindings: [], expected: true, originalExpected: index === 5 ? false : true }, { term: wrap(other), bindings: [], expected: false });
  for (const entry of cases) {
    const host = { ...original, bindings: entry.bindings };
    assert.equal(host.contains(entry.term, owner), entry.originalExpected ?? entry.expected, 'retained original ' + entry.term.kind);
    assert.equal(containsRecordTypeOwner(entry.term, owner, entry.bindings), entry.expected, 'shared occurrence ' + entry.term.kind);
  }
});

test('record field inference retains existing open mutation actual child identity and closed missing-field precedence', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  const source = readFileSync(new URL('../src/translation/javascript-infer.js', import.meta.url), 'utf8');
  assert.equal(hash(policy.originalOwner.field), policy.originalOwner.fieldSha256);
  const context = { readRecordFieldInferenceStep, recordNameUnits: fields => fields.map(field => units(field.name)) };
  const old = vm.runInNewContext(`({${policy.originalOwner.field}})`, context).fieldStep;
  const current = vm.runInNewContext(`({${readSourceMethod(source, 'fieldStep')}})`, context).fieldStep;
  for (const open of [false, true]) for (const field of ['known', 'missing']) {
    const evaluate = method => {
      const known = { kind: 'int' }, result = { kind: 'var', id: 0 };
      const object = { kind: 'record', fields: [{ name: 'known', type: known }], ...(open ? { open } : {}) };
      const trace = [];
      const host = { resolve: value => value, addInferredField(record, entry) { trace.push(['add', record === object, entry.name, entry.type === result]); record.fields.push({ ...entry }); },
        unify(left, right) { trace.push(['unify', left === result, right === known, right === result]); } };
      const returned = method.call(host, { object, field, result });
      return JSON.parse(JSON.stringify({ returned, trace, names: object.fields.map(field => field.name) }));
    };
    assert.deepEqual(evaluate(current), evaluate(old));
  }
});

test('the original complete record method and planned method retain every checked value binding and expected type', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  const source = readFileSync(new URL('../src/translation/check.js', import.meta.url), 'utf8');
  const context = { typeError, readRecordValueOrder, readRecordFieldPosition, readRecordFieldOrder };
  const previous = vm.runInNewContext(`({${policy.original.method}})`, context).recordLiteral;
  const current = vm.runInNewContext(`({${readSourceMethod(source, 'recordLiteral')}})`, context).recordLiteral;
  const integer = { kind: 'int' };
  const literal = (id, value) => ({ id, checked: { k: 'lit', value, type: integer } });
  const spread = { id: 'spread', checked: { k: 'var', name: 'base', type: { kind: 'record', fields: [{ name: 'a', type: integer }, { name: 'b', type: integer }] } } };
  for (const entries of [[], [{ name: 'z', value: literal('first', '1') }, { name: 'a', value: literal('second', '2') }, { name: 'z', value: literal('third', '3') }], [{ spread: true, value: spread }, { name: 'b', value: literal('override', '4') }]]) {
    const evaluate = method => {
      const seen = [], wants = [];
      let counter = 0;
      const host = { expr(value, _env, _path, expected) { seen.push(value.id); wants.push(expected ?? null); return structuredClone(value.checked); }, freshName(prefix) { counter += 1; return prefix + counter; } };
      const expected = { kind: 'record', fields: [{ name: 'a', type: integer }, { name: 'b', type: integer }] };
      const result = method.call(host, { entries }, new Map(), [], expected);
      return JSON.parse(JSON.stringify({ result, seen, wants, counter }));
    };
    assert.deepEqual(evaluate(current), evaluate(previous));
  }
});

test('record inference preserves complete original child checks insertion order and unresolved spread identity', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  const source = readFileSync(new URL('../src/translation/javascript-infer.js', import.meta.url), 'utf8');
  const context = { typeError, readRecordFieldInsertionOrder, readRecordFieldPosition, readRecordSpreadRole,
    recordNameUnits: fields => fields.map(field => units(field.name)) };
  const old = vm.runInNewContext(`({${policy.originalInference.record}})`, context).record;
  const current = vm.runInNewContext(`({${readSourceMethod(source, 'record')}})`, context).record;
  const integer = { kind: 'int' }, string = { kind: 'string' };
  const value = (id, type) => ({ id, type });
  const entries = [
    [{ name: 'z', value: value('first', integer) }, { name: 'a', value: value('second', string) }, { name: 'z', value: value('third', integer) }],
    [{ spread: true, value: value('spread', { kind: 'record', fields: [{ name: 'b', type: integer }, { name: 'a', type: string }] }) }, { name: 'b', value: value('override', integer) }],
    [{ spread: true, value: value('unknown', { kind: 'var', id: 4 }) }, { name: 'a', value: value('known', string) }],
  ];
  for (const originalEntries of entries) {
    const evaluate = method => {
      const node = { entries: structuredClone(originalEntries) };
      const seen = [], constraints = [];
      const host = { expr(child, _environment, expected) { seen.push([child.id, expected ?? null]); return child.type; },
        resolve: type => type, unify(left, right) { constraints.push([left, right]); }, records: new Map(), recordSpreads: [] };
      const term = method.call(host, node, new Map(), { kind: 'record', fields: [{ name: 'a', type: string }] });
      assert.equal(host.records.get(node), term);
      if (host.recordSpreads.length) assert.equal(host.recordSpreads[0].entries[0].type, node.entries[0].value.type);
      const spread = node.entries.find(entry => entry.spread && entry.value.type.kind === 'record');
      if (spread) {
        const actual = term.fields.find(field => field.name === 'a');
        const original = spread.value.type.fields.find(field => field.name === 'a');
        assert.notEqual(actual, original);
        assert.equal(actual.type, original.type);
      }
      return JSON.parse(JSON.stringify({ term, seen, constraints, spreads: host.recordSpreads.length }));
    };
    assert.deepEqual(evaluate(current), evaluate(old));
  }
});

test('record constraint steps preserve the actual original recursive solver mutation and failure order', () => {
  const policy = readProjectData(new URL('../src/translation/data/record-values.lino', import.meta.url));
  const source = readFileSync(new URL('../src/translation/javascript-infer.js', import.meta.url), 'utf8');
  const common = ['resolve', 'contains'].map(name => readSourceMethod(source, name)).join(',\n');
  const context = { typeError, isInteger: type => ['int', 'nat', 'fixed'].includes(type.kind), describe: type => type.kind,
    readRecordTypeConstraintStep, readRecordTypeConstraintError, containsRecordTypeOwner, recordNameUnits: fields => fields.map(field => units(field.name)) };
  const previous = vm.runInNewContext(`({${common},${policy.originalOwner.add},${policy.originalInference.unify}})`, context);
  const current = vm.runInNewContext(`({${common},${readSourceMethod(source, 'addInferredField')},${readSourceMethod(source, 'unify')}})`, context);
  const record = (fields, open = false) => ({ kind: 'record', fields: fields.map(([name, type]) => ({ name, type })), ...(open ? { open } : {}) });
  const integer = { kind: 'int' }, string = { kind: 'string' }, boolean = { kind: 'bool' };
  const recursive = record([], true);
  const cases = [
    [record([['x', { kind: 'var', id: 0 }]], true), record([['x', integer], ['y', string]])],
    [record([['x', integer]]), record([['x', integer], ['y', string]])],
    [record([['x', string]], true), record([['x', integer], ['y', boolean]], true)],
    [record([['x', { kind: 'var', id: 0 }], ['x', string]], true), record([], true)],
    [recursive, record([['self', recursive]])],
  ];
  for (const pair of cases) {
    const evaluate = methods => {
      const [left, right] = structuredClone(pair);
      const ids = new WeakMap(), trace = [];
      let next = 0;
      const id = value => { if (!ids.has(value)) ids.set(value, ++next); return ids.get(value); };
      const host = { ...methods, bindings: [], bigints: new Set(), unificationChanges: 0 };
      host.unify = (a, b, where) => { trace.push([id(a), id(b), a.kind, b.kind]); return methods.unify.call(host, a, b, where); };
      let error = null;
      try { host.unify(left, right); } catch (failure) { error = failure.message; }
      return JSON.parse(JSON.stringify({ left, right, bindings: host.bindings, changes: host.unificationChanges, trace, error }));
    };
    assert.deepEqual(evaluate(current), evaluate(previous));
  }
});

test('the entire actual record planning module survives checked Links replay and automatic Rust emission', () => {
  const source = readRecordValueSourceModule();
  const names = ['readRecordValueOrder', 'readRecordProjection', 'readRecordProjectionError', 'acceptRecordFieldName', 'compareRecordFieldNames', 'readRecordFieldPosition', 'readRecordFieldOrder', 'readRecordFieldInsertionOrder', 'readRecordSpreadRole', 'readRecordTypeConstraintStep', 'readRecordTypeConstraintError', 'readRecordMethodError', 'readRecordFieldInferenceStep', 'readRecordTypeOwnerPresence'];
  const privateNames = ['readRecordSortedFieldPositions', 'readRequiredRecordName', 'refuseRequiredRecordName', 'recordTypeFactField', 'recordTypeFactKind', 'recordTypeFactElements'];
  const dependencyNames = ['readRequiredNumberFact', 'readRequiredTextFact', 'readRequiredBooleanFact', 'refuseRequiredNumberFact', 'refuseRequiredTextFact', 'refuseRequiredBooleanFact'];
  const surface = parseJavaScript(source, { referenceSemantics: false });
  assert.deepEqual(surface.items.filter(item => item.k === 'fn' && !item.generated).map(item => item.name).sort(), [...names, ...privateNames, ...dependencyNames].sort());
  const checked = checkProgram(surface);
  const packet = encodeTypedProgramLinks(checked);
  const originalRust = emitRust(checked, { arrayConsumption: true }).text;
  for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
    const program = decodeTypedProgramLinks(network, packet.root);
    const rust = emitRust(program, { arrayConsumption: true });
    assert.equal(rust.text, originalRust);
    assert.ok(rust.definitions.length >= names.length + privateNames.length + dependencyNames.length);
    for (const definition of rust.definitions) assert.ok(rust.text.includes(definition));
    const code = emitJavaScript(program).text;
    const api = vm.runInNewContext(code + '\n({' + [...names, 'readRequiredRecordName'].join(',') + '});', {}, { timeout: 1000 });
    const copy = value => JSON.parse(JSON.stringify(value));
    assert.deepEqual(copy(api.readRecordValueOrder(3, false)), [0, 1, 2]);
    assert.equal(api.readRecordProjection(), 'checked ordered record values');
    for (const [projection, names, entries, expected] of [
      ['checked ordered record values', [], [], ''],
      ['checked ordered record values', [[97], [98]], [true, true], ''],
      ['unchecked', [], [], 'unknown checked record projection'],
      ['checked ordered record values', [[97]], [], 'checked record projection has inconsistent entry facts'],
      ['checked ordered record values', [[97]], [false], 'checked record projection has a malformed field entry'],
      ['checked ordered record values', [[65536]], [true], 'checked record projection has an invalid UTF16 field name'],
      ['checked ordered record values', [[98], [97]], [true, true], 'checked record projection needs unique fields in UTF16 order'],
      ['checked ordered record values', [[97], [97]], [true, true], 'checked record projection needs unique fields in UTF16 order'],
    ]) {
      assert.equal(readRecordProjectionError(projection, names, entries), expected);
      assert.equal(api.readRecordProjectionError(projection, names, entries), expected);
    }
    assert.equal(readRecordProjection(), api.readRecordProjection());
    assert.equal(api.acceptRecordFieldName([0, 65535]), true);
    assert.equal(api.compareRecordFieldNames([97], [98]), -1);
    assert.equal(api.readRecordFieldPosition([[97], [98]], [98]), 1);
    assert.deepEqual(copy(api.readRecordFieldOrder([[122], [97], [122]])), [1, 2]);
    assert.deepEqual(copy(api.readRecordFieldInsertionOrder([[122], [97], [122]])), [2, 1]);
    assert.equal(api.readRecordSpreadRole('var'), 'open');
    assert.deepEqual(copy(api.readRecordTypeConstraintStep([[97]], [[97], [98]], false, false, 0, 0)), [0, 0, 0, 0, 1]);
    assert.equal(api.readRecordTypeConstraintError(3, 'a'), 'record field a is missing');
    assert.equal(api.readRecordMethodError('', 'next', 0, 2), 'record has no method next');
    assert.equal(api.readRecordMethodError('int', 'next', 0, 2), 'record has no method next');
    assert.equal(api.readRecordMethodError('function', 'next', 1, 2), '.next() expects 1 arguments');
    assert.equal(api.readRecordMethodError('function', 'next', 1, 1), '');
    assert.deepEqual(copy(api.readRecordFieldInferenceStep([[97]], [97], false)), [0, 0]);
    assert.deepEqual(copy(api.readRecordFieldInferenceStep([[97]], [98], true)), [1, -1]);
    assert.deepEqual(copy(api.readRecordFieldInferenceStep([[97]], [98], false)), [2, -1]);
    const owner = { kind: 'record', fields: [] }, equalSpelling = { kind: 'record', fields: [] };
    assert.equal(api.readRecordTypeOwnerPresence(...recordOwnerArguments({ kind: 'var', id: 0 }, owner, [{ kind: 'array', element: owner }])), true);
    assert.equal(api.readRecordTypeOwnerPresence(...recordOwnerArguments(equalSpelling, owner, [])), false);
    assert.deepEqual(copy(api.readRequiredRecordName([[97]], 0)), [97]);
    for (const index of [-1, 0.5, 1, NaN, Infinity]) {
      assert.throws(() => api.readRequiredRecordName([[97]], index), /invalid required record name position/u);
    }
    assert.throws(() => api.readRequiredRecordName(new Array(1), 0), /missing required record name row/u);
  }
});

const observe = source => {
  const lines = [];
  vm.runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return lines;
};

test('complete real record programs keep overwritten and spread evaluation through text binary and source-free edits', () => {
  const prefix = '/** @param {string} label @param {bigint} value @returns {bigint} */function mark(label,value){console.log("%s",label);return value;}';
  const cases = [
    { source: prefix + 'function build(){const entry={z:mark("first",1n),a:mark("second",2n),z:mark("third",3n)};return entry;}const entry=build();console.log("%s",String(entry.z));console.log("%s",String(entry.a));', expected: ['first', 'second', 'third', '3', '2'] },
    { source: prefix + 'function build(){const base={b:mark("base-b",2n),a:mark("base-a",1n)};const entry={...base,b:mark("override-b",4n)};return entry;}const entry=build();console.log("%s",String(entry.a));console.log("%s",String(entry.b));', expected: ['base-b', 'base-a', 'override-b', '1', '4'] },
    { source: prefix + 'function copy(base){return {...base,b:mark("override-b",4n)};}function build(){const base={b:mark("base-b",2n),a:mark("base-a",1n)};return copy(base);}const entry=build();console.log("%s",String(entry.a));console.log("%s",String(entry.b));', expected: ['base-b', 'base-a', 'override-b', '1', '4'] },
  ];
  for (const entry of cases) {
    assert.deepEqual(observe(entry.source), entry.expected);
    const packet = createTypedProgramLinks(entry.source, 'JavaScript');
    for (const network of [LinkNetwork.fromLino(packet.network.toLino()), decodeLinkNetworkBinary(encodeLinkNetworkBinary(packet.network))]) {
      assert.deepEqual(observe(emitTypedProgramLinks(network, packet.root, 'JavaScript').code), entry.expected);
    }
  }
  const packet = createTypedProgramLinks(cases[0].source, 'JavaScript');
  const program = decodeTypedProgramLinks(LinkNetwork.fromLino(packet.network.toLino()), packet.root);
  const visited = new WeakSet();
  const literals = [];
  const walk = value => {
    if (!value || typeof value !== 'object' || visited.has(value)) return;
    visited.add(value);
    if (value.k === 'lit' && value.type.kind === 'int' && value.value === '3') literals.push(value);
    for (const child of value instanceof Map ? value.values() : Object.values(value)) walk(child);
  };
  walk(program);
  assert.equal(literals.length, 1);
  literals[0].value = '5';
  const edited = encodeTypedProgramLinks(program);
  const reloaded = decodeLinkNetworkBinary(encodeLinkNetworkBinary(edited.network));
  assert.deepEqual(observe(emitTypedProgramLinks(reloaded, edited.root, 'JavaScript').code), ['first', 'second', 'third', '5', '2']);
});
