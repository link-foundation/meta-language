import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { format, inspect } from 'node:util';
import vm from 'node:vm';
import test from 'node:test';
import {
  formatRecordTextDraft, prepareRecordTextDraft, readRecordTextDraftLino,
  recordTextDraftToLino, recordTextModels, validateRecordTextDraft,
} from '../../experiments/issue-199-record-text-draft.mjs';
import { wrapGeneratedRustNamespace } from '../../experiments/issue-199-generated-rust-namespace.mjs';
import { retainCompilerFeedback } from '../scripts/compiler-style-feedback.mjs';
import { LinkNetwork } from '../src/network.js';
import { createTypedProgramLinks, decodeTypedProgramLinks, emitTypedProgramLinks } from '../src/typed-program-links.js';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const source = () => readFileSync(new URL('../../experiments/issue-199-record-text-rules.mjs', import.meta.url), 'utf8');
const inputs = [
  {}, { left: 1 }, { right: 1 }, { left: 1, right: 2 }, { x: -0 },
  { x: true, y: false }, { x: 12345678901234567890n },
  { child: { value: 3n, label: 'x' } }, { child: { next: { value: -1 } } },
  { 'space key': 'x', '': 'empty' }, { '1': 'first', '10': 'ten', '2': 'second', a: 'last' },
];
for (const value of ['', 'abc', "single'quote", 'double"quote', "both'\"", "all'\"`", "interpolate'\"${x}", '\\', '\n\r\t']) {
  inputs.push({ value });
  inputs.push({ [value]: 'x' });
}
inputs.push(Object.fromEntries([['__proto__', 'data']]));

function reloadAlgorithm() {
  const archive = createTypedProgramLinks(source(), 'JavaScript', { context: { referenceSemantics: false } });
  const text = archive.network.toLino();
  assert.ok(Buffer.byteLength(text) < 512 * 1024, 'the complete shared record algorithm stays bounded');
  return { network: LinkNetwork.fromLino(text), root: archive.root };
}

test('record draft distinguishes ordinary coercion from actual Node console inspection', () => {
  assert.equal(inputs.length, 30);
  for (const value of inputs) {
    const tape = prepareRecordTextDraft(value);
    assert.equal(formatRecordTextDraft(tape, recordTextModels[0]), String(value));
    assert.equal(formatRecordTextDraft(tape, recordTextModels[1]), inspect(value));
    assert.equal(formatRecordTextDraft(tape, recordTextModels[1]), format(value), 'console uses Node formatting');
  }
  assert.equal(formatRecordTextDraft(prepareRecordTextDraft({ left: 1 }), recordTextModels[0]), '[object Object]');
  assert.equal(formatRecordTextDraft(prepareRecordTextDraft({ left: 1 }), recordTextModels[1]), '{ left: 1 }');
  assert.throws(() => formatRecordTextDraft(prepareRecordTextDraft({}), 'rust-tuple-display'), /undeclared conversion model/u);
});

test('record draft reloads inspectable recursive values and complete checked rules without source objects', () => {
  const archive = reloadAlgorithm();
  const program = decodeTypedProgramLinks(archive.network, archive.root);
  assert.equal(program.declarations.size, 12, 'seven source rules and five generated loop declarations');
  for (const name of ['acceptRecordTextModel', 'acceptRecordKeyCharacter', 'acceptUnquotedRecordKey', 'quoteRecordText',
    'renderRecordKey', 'renderRecordNode', 'renderRecordText']) assert.ok(program.declarations.has(name));
  const javascript = emitTypedProgramLinks(archive.network, archive.root, 'JavaScript').code;
  const rust = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  assert.match(rust, /pub fn render_record_text/u);
  assert.match(rust, /pub fn render_record_node/u);
  const context = vm.createContext({});
  vm.runInContext(javascript, context, { timeout: 1000 });
  for (const value of inputs) {
    const prepared = prepareRecordTextDraft(value);
    const tape = readRecordTextDraftLino(recordTextDraftToLino(prepared));
    assert.deepEqual(tape, prepared);
    for (const model of recordTextModels) {
      assert.equal(context.renderRecordText(tape.kinds, tape.keys, tape.values, tape.ends, model),
        model === recordTextModels[0] ? String(value) : format(value));
    }
  }
});

test('record draft rejects effectful objects, cycles, unsupported primitives and layouts without invoking hooks', () => {
  let calls = 0;
  const getter = Object.defineProperty({}, 'value', { enumerable: true, get() { calls += 1; return 1; } });
  const proxy = new Proxy({}, { getPrototypeOf() { calls += 1; return Object.prototype; } });
  const cycle = {}; cycle.self = cycle;
  const shared = {};
  const invalid = [getter, proxy, cycle, { a: shared, b: shared }, Object.create(null), [], new Date(0),
    { [Symbol.for('nodejs.util.inspect.custom')]: () => { calls += 1; return 'custom'; } },
    { toString: () => { calls += 1; return 'custom'; } }, { valueOf: 1 },
    Object.defineProperty({}, 'hidden', { value: 1 }), { value: null }, { value: undefined },
    { value: Symbol('x') }, { value: () => 1 }, { value: 0.1 }, { value: Infinity },
    { value: Number.MAX_SAFE_INTEGER + 1 }, { value: '\0' }, { value: 'λ' },
    { value: 'a'.repeat(80) }, { a: { b: { c: {} } } },
    Object.fromEntries(Array.from({ length: 32 }, (_, index) => ['a' + index, 1])),
  ];
  for (const value of invalid) assert.throws(() => prepareRecordTextDraft(value), /record text draft/u);
  assert.equal(calls, 0, 'admission does not execute getter, proxy, coercion or inspection hooks');
});

test('reloaded record tapes reject counterfeit structure and disconnected or noncanonical values', () => {
  const original = prepareRecordTextDraft({ left: { value: 1 }, right: true });
  const changed = mutate => { const tape = readStructuredDataLino(structuredDataToLino(original)); mutate(tape); return tape; };
  const corrupt = [
    tape => { tape.ends[0] -= 1; }, tape => { tape.ends[1] = 1; }, tape => { tape.ends[1] = 5; },
    tape => { tape.ends[0] = 1.5; }, tape => { tape.keys.pop(); }, tape => { tape.keys[0] = 'root'; },
    tape => { tape.values[1] = 'opaque'; }, tape => { tape.kinds[2] = 'callback'; },
    tape => { tape.values[2] = '01'; }, tape => { tape.values[3] = 'yes'; },
    tape => { tape.keys[3] = 'left'; }, tape => { tape.keys[1] = 'toString'; },
    tape => { tape.rendered = '{ left: 1 }'; },
  ];
  for (const mutate of corrupt) {
    const tape = changed(mutate);
    assert.throws(() => validateRecordTextDraft(tape), /record text draft/u);
    assert.throws(() => readRecordTextDraftLino(structuredDataToLino(tape)), /record text draft/u);
  }
  const indexed = prepareRecordTextDraft({ 1: 1, 2: 2, tail: 3 });
  [indexed.keys[1], indexed.keys[2]] = [indexed.keys[2], indexed.keys[1]];
  assert.throws(() => validateRecordTextDraft(indexed), /array-index property order/u);
  const lateIndex = prepareRecordTextDraft({ 1: 1, tail: 3 });
  [lateIndex.keys[1], lateIndex.keys[2]] = [lateIndex.keys[2], lateIndex.keys[1]];
  assert.throws(() => readRecordTextDraftLino(structuredDataToLino(lateIndex)), /array-index property order/u);
});

test('direct tape admission rejects effects before reading root or array fields', () => {
  let calls = 0;
  const inert = prepareRecordTextDraft({ value: 1 });
  const getter = Object.defineProperty({ ...inert }, 'kinds', { enumerable: true, get() { calls += 1; return inert.kinds; } });
  const proxy = new Proxy(inert, { ownKeys() { calls += 1; return Reflect.ownKeys(inert); } });
  const arrayGetter = { ...inert, values: inert.values.slice() };
  Object.defineProperty(arrayGetter.values, 0, { enumerable: true, get() { calls += 1; return ''; } });
  const arrayProxy = { ...inert, kinds: new Proxy(inert.kinds, { get() { calls += 1; return ''; } }) };
  const sparse = { ...inert, kinds: inert.kinds.slice() }; delete sparse.kinds[1];
  for (const tape of [getter, proxy, arrayGetter, arrayProxy, sparse]) assert.throws(() => validateRecordTextDraft(tape), /record text draft/u);
  assert.equal(calls, 0);
});

test('default inspector line-width boundaries remain explicit supported and rejected cases', () => {
  let accepted = 0, rejected = 0;
  for (let count = 1; count <= 8; count += 1) {
    for (let width = 0; width <= 10; width += 1) {
      const record = Object.fromEntries(Array.from({ length: count }, (_, index) => ['key' + index, 'a'.repeat(width)]));
      for (const value of [record, { outer: record }]) {
        let tape;
        try { tape = prepareRecordTextDraft(value); } catch (error) {
          assert.match(error.message, /multiline layout/u);
          assert.ok(inspect(value).includes('\n'), 'a rejected layout has actual multiline Node output');
          rejected += 1; continue;
        }
        assert.equal(formatRecordTextDraft(tape, recordTextModels[1]), inspect(value));
        accepted += 1;
      }
    }
  }
  assert.equal(accepted, 86);
  assert.equal(rejected, 90);
});

// Native compilation is intentionally CI-only. Both source-JavaScript models
// retain their declared meaning even though Rust stores products as tuples.
test('CI executes automatically generated record text rules against both JavaScript source oracles', {
  skip: process.env.TRANSLATION_CHECK_RUST !== '1' ? 'native record formatter verification runs only in CI' : false,
}, () => {
  assert.equal(process.env.CI, 'true', 'never compile Rust locally');
  const directory = mkdtempSync(join(tmpdir(), 'meta-record-text-'));
  const path = join(directory, 'record.rs'), executable = join(directory, 'record');
  const archive = reloadAlgorithm();
  const emitted = emitTypedProgramLinks(archive.network, archive.root, 'Rust').code;
  const module = wrapGeneratedRustNamespace(emitted, 'ml_record_text_draft');
  // All admitted strings are ASCII; the JSON string-literal escapes used here
  // are also valid Rust escapes in this bounded thin source-language boundary.
  // No JSON data file or JSON payload is stored in the program or value graph.
  const literal = value => JSON.stringify(value).replaceAll('\\u0009', '\\t');
  const strings = values => `vec![${values.map(value => `Text::from_text(${literal(value)})`).join(', ')}]`;
  const calls = inputs.flatMap(value => {
    const tape = readRecordTextDraftLino(recordTextDraftToLino(prepareRecordTextDraft(value)));
    return recordTextModels.map(model => `println!("{}", render_record_text(${strings(tape.kinds)}, ${strings(tape.keys)}, ${strings(tape.values)}, vec![${tape.ends.map(end => `${end}.0`).join(', ')}], Text::from_text(${literal(model)})));`);
  });
  const host = `fn main() { use ml_record_text_draft::{ml_string::Text, render_record_text};\n${calls.join('\n')}\n}\n`;
  try {
    writeFileSync(path, module + host);
    const compiled = spawnSync('rustc', ['--edition', '2024', '-O', '-D', 'warnings', '--error-format=json', path, '-o', executable],
      { encoding: 'utf8', timeout: 120_000, maxBuffer: 8 * 1024 * 1024 });
    retainCompilerFeedback({ root, group: 'record-text-draft', compiler: 'rustc', round: 0, stderr: compiled.stderr ?? '',
      inputs: [{ path, relativePath: 'generated/record-text-draft.rs' }] });
    assert.equal(compiled.status, 0, compiled.error?.message ?? compiled.stderr);
    const executed = spawnSync(executable, [], { encoding: 'utf8', timeout: 30_000, maxBuffer: 1024 * 1024 });
    assert.equal(executed.status, 0, executed.error?.message ?? executed.stderr);
    assert.deepEqual(executed.stdout.trimEnd().split('\n'), inputs.flatMap(value => [String(value), format(value)]));
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
