// Authenticate the immutable third stage before interpreting its one successor.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {readStructuredDataLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES, readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {compareOriginalLinkRawSchemaExtension} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES, verifyOriginalLinkRawSchemaThirdExtensionClosure} from './raw-schema-third-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_SCOPE} from './raw-schema-fourth-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT = '7ec40c9f63e4ea4bd266387a624dd4d5315a41f8a544428eae17e8e2eb97cde9';
export const ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES = Object.freeze([
  'experiments/original-link-complete-table-projection/raw-schema-fourth-extension-policy.mjs',
  'experiments/original-link-complete-table-projection/raw-schema-fourth-extension-closure.mjs',
  'js/scripts/extend-typed-program-schema-fourth-stage.mjs',
]);
export const ORIGINAL_LINK_RAW_SCHEMA_FOURTH_COMMAND = 'node js/scripts/extend-typed-program-schema-fourth-stage.mjs';
const root = new URL('../../', import.meta.url);
const sha256 = value => createHash('sha256').update(value).digest('hex');

/** Read actual receipt and source bytes without loading the live typed schema. */
export function readOriginalLinkRawSchemaFourthContext(read = readFileSync) {
  const files = {receipt: 'raw-schema-source-closure', first: 'raw-schema-extension-closure',
    second: 'raw-schema-second-extension-closure', third: 'raw-schema-third-extension-closure'};
  const bytes = Object.fromEntries(Object.entries(files).map(([key, name]) => [key, read(new URL('./data/' + name + '.lino', import.meta.url))]));
  return {
    ...Object.fromEntries(Object.entries(bytes).map(([key, value]) => [key, readStructuredDataLino(value.toString('utf8'))])),
    thirdBytes: bytes.third,
    historicalSources: readOriginalLinkRawHistoricalSources(read(new URL('./raw-native-observations.mjs', import.meta.url)).toString('utf8')),
    thirdSources: Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => [file, read(new URL(file, root))])),
  };
}

/** Validate the actual third receipt, every prior premise and the sole append. */
export function verifyOriginalLinkRawSchemaFourthExtensionClosure(input, currentBytes, fourth, authoredSources) {
  const {receipt, first, second, third, thirdBytes, historicalSources, thirdSources} = input;
  assert.deepEqual(Object.keys(fourth), ['version', 'predecessorReceiptSha256', 'previous', 'current',
    'consumedSchemaProperties', 'appendedSchemaEntries', 'authoring', 'scope']);
  assert.equal(fourth.version, 1);
  assert.equal(fourth.predecessorReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT);
  assert.ok(Buffer.isBuffer(thirdBytes));
  assert.equal(sha256(thirdBytes), ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT);
  assert.deepEqual(readStructuredDataLino(thirdBytes.toString('utf8')), third);
  assert.deepEqual(Object.keys(fourth.previous), ['file', 'bytes', 'sha256', 'source']);
  assert.equal(typeof fourth.previous.source, 'string');
  assert.ok(fourth.previous.source.isWellFormed());
  const before = Buffer.from(fourth.previous.source, 'utf8');
  assert.equal(fourth.previous.file, third.current.file);
  assert.equal(fourth.previous.bytes, third.current.bytes);
  assert.equal(fourth.previous.bytes, before.length);
  assert.equal(fourth.previous.sha256, third.current.sha256);
  assert.equal(fourth.previous.sha256, sha256(before));
  const previous = verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, before,
    historicalSources, second, third, thirdSources);
  assert.deepEqual(fourth.appendedSchemaEntries, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSIONS);
  assert.deepEqual(fourth.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.deepEqual(Object.keys(fourth.current), ['file', 'bytes', 'sha256']);
  assert.equal(fourth.current.file, third.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(fourth.current.bytes, currentBytes.length);
  assert.equal(fourth.current.sha256, sha256(currentBytes));
  const beforeSchema = readStructuredDataLino(fourth.previous.source), afterSchema = readStructuredDataLino(currentBytes.toString('utf8'));
  compareOriginalLinkRawSchemaExtension(beforeSchema, afterSchema, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSIONS);
  for (const property of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(afterSchema[property], beforeSchema[property]);
  assert.deepEqual(Object.keys(fourth.authoring), ['command', 'sourceFiles']);
  assert.equal(fourth.authoring.command, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_COMMAND);
  assert.deepEqual(Object.keys(authoredSources), [...ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES]);
  assert.deepEqual(fourth.authoring.sourceFiles.map(row => row.file), [...ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES]);
  for (const row of fourth.authoring.sourceFiles) {
    assert.deepEqual(Object.keys(row), ['file', 'bytes', 'sha256']);
    const bytes = authoredSources[row.file];
    assert.ok(Buffer.isBuffer(bytes));
    assert.equal(row.bytes, bytes.length);
    assert.equal(row.sha256, sha256(bytes));
  }
  assert.equal(fourth.scope, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_SCOPE);
  return {...previous, sha256: fourth.current.sha256, fourthPredecessorSha256: fourth.previous.sha256,
    thirdExtensionReceiptSha256: fourth.predecessorReceiptSha256,
    appendedFourthSchemaEntries: fourth.appendedSchemaEntries};
}
