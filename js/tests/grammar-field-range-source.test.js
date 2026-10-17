import assert from 'node:assert/strict';
import test from 'node:test';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeGrammarText } from '../src/native-grammar-parser.js';
import { readGrammarSelectionFieldRangeView } from '../src/grammar-runtime/field-range-source.js';
import { loadProgram } from '../src/grammar-runtime/load.js';
import { Executor, memoryBudget, stepBudget, NestingTooDeep,
  StepLimitReached, MemoryBudgetReached, readChainChildren } from '../src/grammar-runtime/executor.js';

let jsonParser;
function fieldRangeSourceTree(source) {
  jsonParser ??= compileGrammar(parseGrammarLinks(nativeGrammarText('native-json')));
  const outcome = jsonParser.parseTree(source);
  assert.equal(outcome.ok, true, JSON.stringify(outcome.rejection));
  return outcome.tree;
}
function fieldRangeSourceControl(source, options = {}) {
  const bytes = new TextEncoder().encode(source);
  return { bytes, budget: stepBudget(options, bytes.length, memoryBudget(options)),
    step: Executor.prototype.step, retain: Executor.prototype.retain };
}
function fieldRangeSourceSelector(first = 0, count = 1, mode = 'include') {
  return { kind: 'source-field-selector', label: 'value', mode, first, count };
}
function fieldRangeSourceBytes(node) {
  if (node.type === 'missing') return [];
  if (node.type === 'token' || node.type === 'error') return node.text === null
    ? [...Buffer.from(node.hex, 'hex')] : [...new TextEncoder().encode(node.text)];
  return (node.sourceChildren ?? node.children ?? [node.root]).flatMap(fieldRangeSourceBytes);
}
function fieldRangeSourceFrozen(value) {
  if (!value || typeof value !== 'object') return;
  assert.equal(Object.isFrozen(value), true);
  for (const child of Object.values(value)) fieldRangeSourceFrozen(child);
}

test('actual JSON fields preserve independent byte spans and topmost occurrence paths', () => {
  const cases = [
    { source: '{"k":1}', bytes: [123, 34, 107, 34, 58, 49, 125], span: [5, 6] },
    { source: '{"k":{"x":23}}', bytes: [123, 34, 107, 34, 58, 123, 34, 120, 34, 58, 50, 51, 125, 125], span: [5, 13] },
    { source: '{"é":"β"}', bytes: [123, 34, 195, 169, 34, 58, 34, 206, 178, 34, 125], span: [6, 10] },
  ];
  for (const { source, bytes, span: [start, end] } of cases) {
    const tree = fieldRangeSourceTree(source), control = fieldRangeSourceControl(source);
    assert.deepEqual([...control.bytes], bytes);
    const view = readGrammarSelectionFieldRangeView(control, 0, bytes.length,
      fieldRangeSourceSelector(), tree.children, 32);
    assert.deepEqual(view.ranges, [[start, end]]);
    assert.deepEqual(view.occurrences, [{ path: ['children', 0, 'children', 1, 'children', 2], start, end }]);
    assert.deepEqual(view.witness, { start: 0, end: bytes.length, children: tree.children });
    assert.deepEqual(view.witness.children.flatMap(fieldRangeSourceBytes), bytes);
    fieldRangeSourceFrozen(view);
    assert.ok(control.budget.steps > 0 && control.budget.memory.cells > bytes.length);
  }
});

test('actual sibling field windows preserve occurrence order and include or exclude exact bytes', () => {
  const source = '{"a":1,"b":2}', bytes = [123, 34, 97, 34, 58, 49, 44, 34, 98, 34, 58, 50, 125];
  const tree = fieldRangeSourceTree(source);
  for (const [first, count, mode, ranges, spans] of [
    [0, 2, 'include', [[5, 6], [11, 12]], [[5, 6], [11, 12]]],
    [1, 1, 'include', [[11, 12]], [[11, 12]]],
    [1, 1, 'exclude', [[0, 11], [12, 13]], [[11, 12]]],
    [0, 2, 'exclude', [[0, 5], [6, 11], [12, 13]], [[5, 6], [11, 12]]],
  ]) {
    const view = readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, bytes.length,
      fieldRangeSourceSelector(first, count, mode), tree.children, 32);
    assert.deepEqual(view.ranges, ranges);
    assert.deepEqual(view.occurrences, spans.map(([start, end]) => ({
      path: ['children', 0, 'children', start === 5 ? 1 : 3, 'children', 2], start, end,
    })));
    assert.deepEqual(view.witness.children.flatMap(fieldRangeSourceBytes), bytes);
  }
});

test('field view failures preserve existing depth step and memory budget errors', () => {
  const source = '{"k":1}', tree = fieldRangeSourceTree(source);
  const empty = fieldRangeSourceTree('{}');
  assert.deepEqual(readGrammarSelectionFieldRangeView(fieldRangeSourceControl('{}'), 0, 2,
    fieldRangeSourceSelector(), empty.children, 32), { reason: 'invalidEmbeddingMapping', offset: 0 });
  for (const selector of [fieldRangeSourceSelector(1), fieldRangeSourceSelector(0, 2),
    fieldRangeSourceSelector(0, 0), fieldRangeSourceSelector(0, 1, 'unknown'),
    { ...fieldRangeSourceSelector(), label: 'absent' }]) {
    assert.equal(readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
      selector, tree.children, 32).reason, 'invalidEmbeddingMapping');
  }
  const invalid = structuredClone(tree.children);
  invalid[0].end -= 1;
  assert.equal(readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
    fieldRangeSourceSelector(), invalid, 32).reason, 'invalidEmbeddingMapping');
  assert.throws(() => readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
    fieldRangeSourceSelector(), tree.children, 0), NestingTooDeep);
  for (const [options, error] of [[{ stepLimit: 0 }, StepLimitReached], [{ memoryLimit: 0 }, MemoryBudgetReached]]) {
    assert.throws(() => readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source, options), 0, 7,
      fieldRangeSourceSelector(), tree.children, 32), error);
  }
});

test('physical witness snapshots retain attributes lexical data and independent frozen ownership', () => {
  const source = '{"k":1}', children = structuredClone(fieldRangeSourceTree(source).children);
  const value = children[0].children[1].children[2];
  value.attributes = { note: 'π', items: [2, -0, true, null, { nested: 'β' }] };
  const token = value.type === 'token' ? value : value.children.find(child => child.type === 'token');
  assert.ok(token);
  token.lexical = { lexer: 'Actual', rule: 'number', type: 'NUMBER', channel: 'DEFAULT', disposition: 'emit' };
  const view = readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
    fieldRangeSourceSelector(), children, 32);
  assert.deepEqual(view.witness.children, children);
  assert.notEqual(view.witness.children[0].children[1].children[2].attributes, value.attributes);
  fieldRangeSourceFrozen(view);
  value.attributes.items[4].nested = 'changed';
  assert.equal(view.witness.children[0].children[1].children[2].attributes.items[4].nested, 'β');
});

test('mapped witnesses retain only physical children while identity roots remain opaque to field discovery', () => {
  const source = '{"k":1}', tree = fieldRangeSourceTree(source);
  const mapped = { type: 'embed', language: 'Mapped', start: 0, end: 7,
    sourceChildren: tree.children, root: { type: 'unsupported-decoded-root', start: 90, end: 100 },
    program: { private: true }, selection: { alternatives: [{ private: true }] } };
  const view = readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
    fieldRangeSourceSelector(), [mapped], 32);
  assert.deepEqual(view.occurrences, [{ path: ['children', 0, 'sourceChildren', 0, 'children', 1, 'children', 2], start: 5, end: 6 }]);
  assert.deepEqual(Object.keys(view.witness.children[0]), ['type', 'language', 'start', 'end', 'sourceChildren']);
  assert.deepEqual(view.witness.children.flatMap(fieldRangeSourceBytes), [123, 34, 107, 34, 58, 49, 125]);
  const siblings = '{"a":1,"b":2}', children = structuredClone(fieldRangeSourceTree(siblings).children);
  const first = children[0].children[1].children[2];
  children[0].children[1].children[2] = { type: 'embed', language: 'Opaque', start: 5, end: 6, root: first };
  const opaque = readGrammarSelectionFieldRangeView(fieldRangeSourceControl(siblings), 0, 13,
    fieldRangeSourceSelector(), children, 32);
  assert.deepEqual(opaque.ranges, [[11, 12]]);
  assert.deepEqual(opaque.occurrences, [{ path: ['children', 0, 'children', 3, 'children', 2], start: 11, end: 12 }]);
  assert.deepEqual(opaque.witness.children[0].children[1].children[2].root, first);
  fieldRangeSourceFrozen(opaque);
});

test('unrepresented embed error and missing capture metadata refuses instead of disappearing', () => {
  const source = '{"a":1,"b":2}', original = fieldRangeSourceTree(source).children;
  for (const type of ['embed', 'error', 'missing']) for (const extra of [{ field: 'value' }, { trivia: true }, { trivia: false }]) {
    const children = structuredClone(original);
    if (type === 'missing') children.unshift({ type, kind: 'omitted', start: 0, end: 0, ...extra });
    else if (type === 'error') children[0].children[1].children[2] = { type, start: 5, end: 6, text: '1', ...extra };
    else children[0].children[1].children[2] = { type, language: 'Opaque', start: 5, end: 6,
      root: children[0].children[1].children[2], ...extra };
    assert.equal(readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 13,
      fieldRangeSourceSelector(), children, 32).reason, 'invalidEmbeddingMapping');
  }
});

test('field views admit the real executor lazy children owner and refuse arbitrary getters without invocation', () => {
  const source = '{"k":1}', control = fieldRangeSourceControl(source);
  const program = loadProgram(parseGrammarLinks(nativeGrammarText('native-json')));
  const executor = new Executor(program, control.bytes, 0, 7, {}, stepBudget({}, 7), 32);
  const outcome = executor.run(program.start);
  assert.equal(outcome.ok, true);
  assert.equal(Object.getOwnPropertyDescriptor(outcome.root.children[0], 'children').get, readChainChildren);
  const view = readGrammarSelectionFieldRangeView(control, 0, 7,
    fieldRangeSourceSelector(), outcome.root.children, 32);
  assert.deepEqual(view.ranges, [[5, 6]]);
  assert.deepEqual(view.witness.children, fieldRangeSourceTree(source).children);
  const arbitrary = structuredClone(fieldRangeSourceTree(source).children);
  let reads = 0;
  Object.defineProperty(arbitrary[0], 'children', { enumerable: true, get() { reads += 1; return []; } });
  assert.equal(readGrammarSelectionFieldRangeView(fieldRangeSourceControl(source), 0, 7,
    fieldRangeSourceSelector(), arbitrary, 32).reason, 'invalidEmbeddingMapping');
  assert.equal(reads, 0);
});
