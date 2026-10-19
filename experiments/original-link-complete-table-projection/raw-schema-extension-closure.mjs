// Bounded source closure chained through the unchanged B43 receipt.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {verifyOriginalLinkRawSchemaSourceClosure, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_EXTENSION_SCOPE} from './raw-schema-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT = '4279200cc6d7f1aa36aa7090461b3d2f09d3e711c42cee6c86d29e8bf8873375';
const sha256 = value => createHash('sha256').update(value).digest('hex');

/** Retain old object identities, old keys in order and every old leaf value. */
export function compareOriginalLinkRawSchemaExtension(before, after, additions = ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS) {
  const forward = new WeakMap(), reverse = new WeakMap();
  function compare(left, right, path, extension) {
    if (!left || typeof left !== 'object') {
      assert.ok(Object.is(left, right), 'an original schema value changed: ' + path);
      return;
    }
    assert.ok(right && typeof right === 'object', 'an original schema shape changed: ' + path);
    assert.equal(Array.isArray(right), Array.isArray(left), 'an original schema container changed: ' + path);
    if (forward.has(left)) {
      assert.equal(forward.get(left), right, 'an original schema alias split: ' + path);
      return;
    }
    assert.equal(reverse.has(right), false, 'distinct original schema objects merged: ' + path);
    forward.set(left, right); reverse.set(right, left);
    const extra = Object.keys(extension ?? {}).filter(key => !Object.hasOwn(left, key));
    assert.deepEqual(Object.keys(right), [...Object.keys(left), ...extra], 'an original schema key or append order changed: ' + path);
    for (const key of Object.keys(left)) compare(left[key], right[key], path + '.' + key, extension?.[key]);
    for (const key of Object.keys(extension ?? {})) {
      if (!Object.hasOwn(left, key)) assert.deepEqual(right[key], extension[key], 'a declared appended schema entry differs: ' + path + '.' + key);
    }
  }
  compare(before, after, 'schema', additions);
  // A declaration of an existing entry must be an exact assertion, never an
  // instruction to alter that entry under an apparently additive map.
  for (const [map, entries] of Object.entries(additions)) {
    for (const [key, value] of Object.entries(entries)) if (Object.hasOwn(before[map] ?? {}, key)) {
      assert.deepEqual(before[map][key], value, 'an extension attempts to redefine an existing entry: ' + map + '.' + key);
    }
  }
}

/** Authenticate the predecessor first, then the exact declared append only. */
export function verifyOriginalLinkRawSchemaExtensionClosure(receipt, currentBytes, historicalSources, extension) {
  assert.equal(extension.version, 1);
  assert.equal(extension.predecessorReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(sha256(structuredDataToLino(receipt)), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, 'the complete predecessor receipt remains byte exact');
  assert.equal(typeof extension.previous.source, 'string');
  assert.ok(extension.previous.source.isWellFormed());
  const previousBytes = Buffer.from(extension.previous.source, 'utf8');
  assert.equal(extension.previous.commit, receipt.current.commit);
  assert.equal(extension.previous.file, receipt.current.file);
  assert.equal(extension.previous.bytes, previousBytes.length);
  assert.equal(extension.previous.sha256, sha256(previousBytes));
  assert.equal(extension.previous.sha256, receipt.current.sha256);
  const previous = verifyOriginalLinkRawSchemaSourceClosure(receipt, previousBytes, historicalSources);
  assert.deepEqual(extension.appendedSchemaEntries, ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS);
  assert.deepEqual(extension.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.equal(extension.current.file, receipt.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(extension.current.bytes, currentBytes.length);
  assert.equal(extension.current.sha256, sha256(currentBytes), 'the complete extended schema pin is exact');
  const before = readStructuredDataLino(extension.previous.source);
  const after = readStructuredDataLino(currentBytes.toString('utf8'));
  compareOriginalLinkRawSchemaExtension(before, after);
  for (const key of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(after[key], before[key], 'a raw producer input changed: ' + key);
  assert.equal(extension.scope, ORIGINAL_LINK_RAW_SCHEMA_EXTENSION_SCOPE);
  const {sourceCommit, ...retained} = previous;
  return {
    ...retained,
    sha256: extension.current.sha256,
    predecessorSourceCommit: sourceCommit,
    predecessorSha256: extension.previous.sha256,
    predecessorReceiptSha256: extension.predecessorReceiptSha256,
    appendedSchemaEntries: extension.appendedSchemaEntries,
  };
}
