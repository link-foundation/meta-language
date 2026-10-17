import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, readdirSync, renameSync, rmSync, unlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import test from 'node:test';
import { readProjectData, writeProjectData } from '../scripts/project-data.mjs';
import { readGrammarReport, writeGrammarReport } from '../scripts/grammar-report-files.mjs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const limits = { nodeLimit: 512 };
const digest = value => createHash('sha256').update(value).digest('hex');
function directory(run) {
  const root = mkdtempSync(join(tmpdir(), 'grammar-report-fragments-'));
  try { run(root); } finally { rmSync(root, { recursive: true, force: true }); }
}
function report() {
  const shared = { name: 'shared immutable definition', spelling: '\ud800', number: -0 };
  Object.defineProperty(shared, '__proto__', { value: 'plain source field', enumerable: true });
  return { schemaVersion: 1, source: 'synthetic bounded node-cap witness',
    concepts: Array.from({ length: 65 }, (_, index) => ({ identity: index, definition: shared, origins: [`source:${index}`, `rule:${index}`] })),
    mappings: { first: shared, second: shared }, proofs: [], failures: [], ambiguities: [] };
}

test('factored LiNo reloads the complete report graph under the unchanged per-file node cap', () => directory(root => {
  const original = report();
  assert.throws(() => structuredDataToLino(original, limits), /node resource limit exceeded/);
  const file = writeGrammarReport(join(root, 'concepts.lino'), original, { limits });
  const index = readProjectData(file, { limits });
  assert.equal(index.format, 'meta-language grammar report fragments');
  assert.ok(index.parts.length > 1);
  const decoded = readGrammarReport(file, { limits });
  assert.deepEqual(decoded, original);
  assert.equal(decoded.mappings.first, decoded.mappings.second);
  assert.equal(decoded.concepts[0].definition, decoded.concepts.at(-1).definition);
  assert.equal(decoded.concepts[0].definition, decoded.mappings.first);
  assert.equal(Object.is(decoded.mappings.first.number, -0), true);
  const second = writeGrammarReport(file, original, { limits });
  assert.equal(second, file);
  assert.deepEqual(readProjectData(file, { limits }), index);
}));

function alterPart(file, edit) {
  const index = readProjectData(file, { limits });
  const oldFolder = join(dirname(file), index.directory);
  const part = index.parts[0];
  const oldFile = join(oldFolder, part.file);
  const block = readProjectData(oldFile, { limits });
  edit(block);
  const text = structuredDataToLino(block, limits);
  const sha256 = digest(text);
  const name = `${sha256}.lino`;
  writeFileSync(join(oldFolder, name), text);
  unlinkSync(oldFile);
  index.parts[0] = { file: name, sha256 };
  const directory = `${file.split('/').at(-1)}.parts-${digest(index.parts.map(part => part.sha256).join('\n'))}`;
  renameSync(oldFolder, join(dirname(file), directory));
  index.directory = directory;
  writeProjectData(file, index);
}

test('a correctly hashed foreign block and cyclic or dangling graph still fail closure validation', () => {
  for (const [kind, change, expected] of [
    ['foreign', block => { block.format = 'foreign object segments'; }, /foreign report fragment/],
    ['dangling', block => { block.segments[0].entries[0][1] = { reference: 999_999 }; }, /dangling or foreign/],
    ['cyclic', block => { block.segments[0].entries[0][1] = { reference: 1 }; }, /structural cycle/],
  ]) directory(root => {
    const file = writeGrammarReport(join(root, `${kind}.lino`), report(), { limits });
    alterPart(file, change);
    assert.throws(() => readGrammarReport(file, { limits }), expected, kind);
  });
});

test('factored object fields retain an explicitly ordered integer-key view', () => directory(root => {
  const target = { 1: 'first', 2: 'second' };
  const ordered = new Proxy(target, { ownKeys: () => ['2', '1'] });
  const original = report();
  original.ordered = ordered;
  const file = writeGrammarReport(join(root, 'ordered.lino'), original, { limits });
  const decoded = readGrammarReport(file, { limits });
  assert.deepEqual(Object.keys(decoded.ordered), ['2', '1']);
  assert.deepEqual(decoded.ordered, ordered);
}));

test('typed report fragments refuse missing, unexpected, tampered and foreign objects', () => directory(root => {
  const file = writeGrammarReport(join(root, 'concepts.lino'), report(), { limits });
  const index = readProjectData(file, { limits });
  const folder = join(dirname(file), index.directory);
  const first = join(folder, index.parts[0].file);
  const bytes = readFileSync(first);
  unlinkSync(first);
  assert.throws(() => readGrammarReport(file, { limits }), /missing or unexpected/);
  writeFileSync(first, bytes);
  const unexpected = join(folder, 'unexpected.lino');
  writeFileSync(unexpected, 'foreign');
  assert.throws(() => readGrammarReport(file, { limits }), /missing or unexpected/);
  unlinkSync(unexpected);
  writeFileSync(first, `${bytes}\n`);
  assert.throws(() => readGrammarReport(file, { limits }), /checksum mismatch/);
  writeFileSync(first, bytes);
  writeProjectData(file, { ...index, directory: '../foreign' });
  assert.throws(() => readGrammarReport(file, { limits }), /foreign fragment directory/);
  assert.ok(readdirSync(folder).length > 1);
}));

test('report factorization retains cycle, depth, scalar and cap refusals', () => directory(root => {
  const cyclic = {}; cyclic.self = cyclic;
  assert.throws(() => writeGrammarReport(join(root, 'cycle.lino'), cyclic), /structural cycle/);
  assert.throws(() => writeGrammarReport(join(root, 'large-limit.lino'), {}, { limits: { nodeLimit: 1_000_001 } }), /cannot increase nodeLimit/);
  assert.throws(() => writeGrammarReport(join(root, 'depth.lino'), { a: { b: {} } }, { limits: { depthLimit: 4 } }), /depth resource limit/);
  assert.throws(() => writeGrammarReport(join(root, 'number.lino'), { value: NaN }), /finite binary64/);
}));

test('unsupported observation values retain their exact field path without replacing existing report evidence', () => directory(root => {
  const file = join(root, 'observations.lino');
  const original = { schemaVersion: 1, results: [] };
  writeGrammarReport(file, original);
  const before = readFileSync(file, 'utf8');
  for (const [kind, value] of [
    ['undefined', undefined], ['function', () => {}], ['symbol', Symbol('unsupported observation')], ['bigint', 1n],
  ]) {
    const report = { results: [{ observations: [{ native: { accepted: false, offset: value } }] }] };
    assert.throws(() => writeGrammarReport(file, report), error => error instanceof TypeError
      && error.message === `grammar report: unsupported host value (${kind}) at $["results"][0]["observations"][0]["native"]["offset"]`);
    assert.equal(Object.hasOwn(report.results[0].observations[0].native, 'offset'), true);
    assert.equal(report.results[0].observations[0].native.offset, value);
    assert.equal(readFileSync(file, 'utf8'), before);
    assert.deepEqual(readdirSync(root), ['observations.lino']);
    assert.deepEqual(readGrammarReport(file), original);
  }
}));
