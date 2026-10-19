// Small source-closure checks only: no grammar imports, graph planning or process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';

export const ORIGINAL_LINK_RAW_SCHEMA_PRODUCER = '9123dd5d19fc26c95a9d31eff47dd9655c33da53';
export const ORIGINAL_LINK_RAW_SCHEMA_FILE = 'js/src/data/typed-program-links-schema.lino';
export const ORIGINAL_LINK_RAW_SCHEMA_BASELINE = '377101200a12b4ada15e0a34a583900e8b6d62adb1fb685f9b2d6f161cacafdc';
export const ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES = Object.freeze([
  'language', 'root', 'schemaVersion', 'containers', 'scalars', 'fields', 'references', 'referenceAliases',
]);
export const ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS = Object.freeze(['propositionOperands', 'propositionBindingForms']);
const schemaBinding = '../../' + ORIGINAL_LINK_RAW_SCHEMA_FILE;
const sha256 = value => createHash('sha256').update(value).digest('hex');

/** Read the original literal source-binding table, without importing a planner. */
export function readOriginalLinkRawHistoricalSources(source) {
  const literal = /^const sourceDigests = (\{[\s\S]*?^\});/mu.exec(source)?.[1];
  assert.ok(literal, 'the historical raw source-binding table exists');
  const sources = JSON.parse(literal);
  assert.equal(Object.keys(sources).length, 47);
  assert.equal(sources[schemaBinding], ORIGINAL_LINK_RAW_SCHEMA_BASELINE);
  for (const digest of Object.values(sources)) assert.match(digest, /^[0-9a-f]{64}$/u);
  return sources;
}

// The bijection also preserves repeated graph objects; value equality alone
// must not allow an old shared object to split or distinct objects to merge.
function compareOriginalSchema(before, after, path, forward, reverse) {
  if (!before || typeof before !== 'object') {
    assert.ok(Object.is(before, after), 'an original schema value changed: ' + path);
    return;
  }
  assert.ok(after && typeof after === 'object', 'an original schema shape changed: ' + path);
  assert.equal(Array.isArray(after), Array.isArray(before), 'an original schema container changed: ' + path);
  if (forward.has(before)) {
    assert.equal(forward.get(before), after, 'an original schema alias split: ' + path);
    return;
  }
  assert.equal(reverse.has(after), false, 'distinct original schema objects merged: ' + path);
  forward.set(before, after);reverse.set(after, before);
  assert.deepEqual(Object.keys(after), Object.keys(before), 'an original schema property or order changed: ' + path);
  for (const key of Object.keys(before)) compareOriginalSchema(before[key], after[key], path + '.' + key, forward, reverse);
}

/** Authenticate a separate additive host schema without relabeling its producer. */
export function verifyOriginalLinkRawSchemaSourceClosure(receipt, currentBytes, historicalSources) {
  assert.equal(receipt.version, 1);
  assert.equal(receipt.producerCommit, ORIGINAL_LINK_RAW_SCHEMA_PRODUCER);
  assert.deepEqual(Object.entries(receipt.historicalSources), Object.entries(historicalSources), 'all original source bindings remain exact and ordered');
  assert.equal(Object.keys(historicalSources).length, 47);
  assert.equal(historicalSources[schemaBinding], ORIGINAL_LINK_RAW_SCHEMA_BASELINE);
  for (const digest of Object.values(historicalSources)) assert.match(digest, /^[0-9a-f]{64}$/u);
  assert.deepEqual(receipt.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.deepEqual(receipt.addedUnusedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS]);
  assert.equal(receipt.baseline.commit, ORIGINAL_LINK_RAW_SCHEMA_PRODUCER);
  assert.equal(receipt.baseline.file, ORIGINAL_LINK_RAW_SCHEMA_FILE);
  assert.equal(typeof receipt.baseline.source, 'string');
  assert.ok(receipt.baseline.source.isWellFormed());
  const baselineBytes = Buffer.from(receipt.baseline.source, 'utf8');
  assert.equal(baselineBytes.length, receipt.baseline.bytes);
  assert.equal(baselineBytes.length, 101294);
  assert.equal(receipt.baseline.sha256, ORIGINAL_LINK_RAW_SCHEMA_BASELINE);
  assert.equal(sha256(baselineBytes), ORIGINAL_LINK_RAW_SCHEMA_BASELINE, 'original schema source bytes are authenticated');
  assert.match(receipt.current.commit, /^[0-9a-f]{40}$/u);
  assert.equal(receipt.current.file, ORIGINAL_LINK_RAW_SCHEMA_FILE);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(currentBytes.length, receipt.current.bytes);
  assert.equal(sha256(currentBytes), receipt.current.sha256, 'the current schema full-file pin is exact');
  const baseline = readStructuredDataLino(receipt.baseline.source);
  const current = readStructuredDataLino(currentBytes.toString('utf8'));
  assert.deepEqual(Object.keys(current), [...Object.keys(baseline), ...ORIGINAL_LINK_RAW_SCHEMA_ADDITIONS],
    'only the two declared unused additive plans may extend the original schema');
  const originalProjection = Object.fromEntries(Object.keys(baseline).map(key => [key, current[key]]));
  compareOriginalSchema(baseline, originalProjection, 'schema', new WeakMap(), new WeakMap());
  for (const key of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.ok(Object.hasOwn(baseline, key));
  assert.equal(receipt.generation.command, 'node experiments/original-link-complete-table-projection/generate-raw.mjs --native --check');
  assert.equal(receipt.generation.exitCode, 0);
  assert.equal(receipt.generation.sourceSha256, historicalSources['raw-rules.mjs']);
  assert.equal(receipt.generation.definitionCount, 106);
  assert.equal(receipt.generation.completeRustBytes, 351746);
  assert.equal(receipt.generation.completeRustSha256, '24db774177e047de0c0ea22adb2464e4e8481d03547f5a810b2d9fe317fb3046');
  assert.equal(receipt.generation.artifactSha256, historicalSources['data/raw-native-definitions.lino']);
  assert.match(receipt.scope, /source closure only; no native execution/u);
  return {
    producerCommit: receipt.producerCommit,
    baselineSha256: receipt.baseline.sha256,
    sourceCommit: receipt.current.commit,
    sha256: receipt.current.sha256,
    consumedSchemaProperties: [...receipt.consumedSchemaProperties],
    addedUnusedSchemaProperties: [...receipt.addedUnusedSchemaProperties],
  };
}
