// Authenticate the exact fourth stage before interpreting its two-key successor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {compareOriginalLinkRawSchemaExtension} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES, readOriginalLinkRawSchemaFourthContext,
  verifyOriginalLinkRawSchemaFourthExtensionClosure} from './raw-schema-fourth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_SCOPE} from './raw-schema-fifth-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT = '8a25266db3397c7b012d1d8ca9c79e43996819f967695644df306669f71f6fa8';
export const ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES = Object.freeze([
  'experiments/original-link-complete-table-projection/raw-schema-fifth-extension-policy.mjs',
  'experiments/original-link-complete-table-projection/raw-schema-fifth-extension-closure.mjs',
  'js/scripts/extend-typed-program-schema-fifth-stage.mjs',
]);
export const ORIGINAL_LINK_RAW_SCHEMA_FIFTH_COMMAND = 'node js/scripts/extend-typed-program-schema-fifth-stage.mjs';
const root = new URL('../../', import.meta.url);
const sha256 = value => createHash('sha256').update(value).digest('hex');

/** Read exact predecessor receipts and sources without loading the live schema. */
export function readOriginalLinkRawSchemaFifthContext(read = readFileSync) {
  const fourthBytes = read(new URL('./data/raw-schema-fourth-extension-closure.lino', import.meta.url));
  return {
    ...readOriginalLinkRawSchemaFourthContext(read),
    fourth: readStructuredDataLino(fourthBytes.toString('utf8')),
    fourthBytes,
    fourthSources: Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES.map(file => [file, read(new URL(file, root))])),
  };
}

/** Revalidate every predecessor and invert exactly the two declared projections. */
export function verifyOriginalLinkRawSchemaFifthExtensionClosure(input, currentBytes, fifth, authoredSources) {
  const {fourth, fourthBytes, fourthSources} = input;
  assert.deepEqual(Object.keys(fifth), ['version', 'predecessorReceiptSha256', 'previous', 'current',
    'consumedSchemaProperties', 'appendedSchemaEntries', 'authoring', 'scope']);
  assert.equal(fifth.version, 1);
  assert.equal(fifth.predecessorReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT);
  assert.ok(Buffer.isBuffer(fourthBytes));
  assert.equal(sha256(fourthBytes), ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT);
  assert.deepEqual(readStructuredDataLino(fourthBytes.toString('utf8')), fourth);
  assert.deepEqual(Object.keys(fifth.previous), ['file', 'bytes', 'sha256', 'source']);
  assert.equal(typeof fifth.previous.source, 'string');
  assert.ok(fifth.previous.source.isWellFormed());
  const before = Buffer.from(fifth.previous.source, 'utf8');
  assert.equal(fifth.previous.file, fourth.current.file);
  assert.equal(fifth.previous.bytes, fourth.current.bytes);
  assert.equal(fifth.previous.bytes, before.length);
  assert.equal(fifth.previous.sha256, fourth.current.sha256);
  assert.equal(fifth.previous.sha256, sha256(before));
  const previous = verifyOriginalLinkRawSchemaFourthExtensionClosure(input, before, fourth, fourthSources);
  assert.deepEqual(fifth.appendedSchemaEntries, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS);
  assert.deepEqual(fifth.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.deepEqual(Object.keys(fifth.current), ['file', 'bytes', 'sha256']);
  assert.equal(fifth.current.file, fourth.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(fifth.current.bytes, currentBytes.length);
  assert.equal(fifth.current.sha256, sha256(currentBytes));
  const beforeSchema = readStructuredDataLino(fifth.previous.source);
  const afterSchema = readStructuredDataLino(currentBytes.toString('utf8'));
  assert.equal(Object.hasOwn(beforeSchema.expressionProjections, 'set'), false);
  assert.equal(Object.hasOwn(beforeSchema.expressionProjections, 'map'), false);
  compareOriginalLinkRawSchemaExtension(beforeSchema, afterSchema, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS);
  const inverse = readStructuredDataLino(currentBytes.toString('utf8'));
  assert.equal(delete inverse.expressionProjections.set, true);
  assert.equal(delete inverse.expressionProjections.map, true);
  compareOriginalLinkRawSchemaExtension(beforeSchema, inverse, {});
  for (const property of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(afterSchema[property], beforeSchema[property]);
  assert.deepEqual(Object.keys(fifth.authoring), ['command', 'sourceFiles']);
  assert.equal(fifth.authoring.command, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_COMMAND);
  assert.deepEqual(Object.keys(authoredSources), [...ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES]);
  assert.deepEqual(fifth.authoring.sourceFiles.map(row => row.file), [...ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES]);
  for (const row of fifth.authoring.sourceFiles) {
    assert.deepEqual(Object.keys(row), ['file', 'bytes', 'sha256']);
    const bytes = authoredSources[row.file];
    assert.ok(Buffer.isBuffer(bytes));
    assert.equal(row.bytes, bytes.length);
    assert.equal(row.sha256, sha256(bytes));
  }
  assert.equal(fifth.scope, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_SCOPE);
  return {...previous, sha256: fifth.current.sha256, fifthPredecessorSha256: fifth.previous.sha256,
    fourthExtensionReceiptSha256: fifth.predecessorReceiptSha256,
    appendedFifthSchemaEntries: fifth.appendedSchemaEntries};
}
