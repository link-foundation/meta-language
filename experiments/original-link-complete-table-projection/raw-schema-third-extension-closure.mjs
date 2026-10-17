// A separate successor authenticates the complete immutable published stage two.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {compareOriginalLinkRawSchemaExtension} from './raw-schema-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaSecondExtensionClosure} from './raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_SCOPE} from './raw-schema-third-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT = 'c42beef3515ce9f2e2a030c07879630a1ef55acbfde0684f6ff25ea5d394c21f';
export const ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES = Object.freeze([
  'experiments/original-link-complete-table-projection/raw-schema-third-extension-policy.mjs',
  'experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs',
  'js/scripts/extend-typed-program-schema-third-stage.mjs',
]);
export const ORIGINAL_LINK_RAW_SCHEMA_THIRD_COMMAND = 'node js/scripts/extend-typed-program-schema-third-stage.mjs';
const sha256 = value => createHash('sha256').update(value).digest('hex');

/** Authenticate all predecessors before the four exact Boolean projection additions. */
export function verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, currentBytes, historicalSources, second, third, authoredSources) {
  assert.deepEqual(Object.keys(third), ['version', 'predecessorReceiptSha256', 'previous', 'current', 'consumedSchemaProperties', 'appendedSchemaEntries', 'authoring', 'scope']);
  assert.equal(third.version, 1);
  assert.equal(third.predecessorReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT);
  assert.equal(sha256(structuredDataToLino(second)), ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT, 'the whole published stage-two receipt stays byte exact');
  assert.deepEqual(Object.keys(third.previous), ['file', 'bytes', 'sha256', 'source']);
  assert.equal(typeof third.previous.source, 'string');
  assert.ok(third.previous.source.isWellFormed());
  const previousBytes = Buffer.from(third.previous.source, 'utf8');
  assert.equal(third.previous.file, second.current.file);
  assert.equal(third.previous.bytes, previousBytes.length);
  assert.equal(third.previous.bytes, second.current.bytes);
  assert.equal(third.previous.sha256, sha256(previousBytes));
  assert.equal(third.previous.sha256, second.current.sha256);
  const previous = verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, previousBytes, historicalSources, second);
  assert.deepEqual(third.appendedSchemaEntries, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS);
  assert.deepEqual(third.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.deepEqual(Object.keys(third.current), ['file', 'bytes', 'sha256']);
  assert.equal(third.current.file, second.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(third.current.bytes, currentBytes.length);
  assert.equal(third.current.sha256, sha256(currentBytes), 'the whole live successor schema is pinned');
  const before = readStructuredDataLino(third.previous.source), after = readStructuredDataLino(currentBytes.toString('utf8'));
  compareOriginalLinkRawSchemaExtension(before, after, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS);
  for (const property of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(after[property], before[property], 'an original raw producer input changed: ' + property);
  assert.deepEqual(Object.keys(third.authoring), ['command', 'sourceFiles']);
  assert.equal(third.authoring.command, ORIGINAL_LINK_RAW_SCHEMA_THIRD_COMMAND);
  assert.deepEqual(Object.keys(authoredSources), [...ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES]);
  assert.deepEqual(third.authoring.sourceFiles.map(row => row.file), [...ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES]);
  for (const row of third.authoring.sourceFiles) {
    assert.deepEqual(Object.keys(row), ['file', 'bytes', 'sha256']);
    const bytes = authoredSources[row.file];
    assert.ok(Buffer.isBuffer(bytes));
    assert.equal(row.bytes, bytes.length);
    assert.equal(row.sha256, sha256(bytes), 'the actual successor authoring source is pinned: ' + row.file);
  }
  assert.equal(third.scope, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_SCOPE);
  return {...previous, sha256: third.current.sha256,
    secondExtensionSha256: second.current.sha256,
    secondExtensionReceiptSha256: third.predecessorReceiptSha256,
    appendedThirdSchemaEntries: third.appendedSchemaEntries};
}
