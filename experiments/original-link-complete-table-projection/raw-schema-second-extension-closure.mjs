// Authenticate stage one as a whole before allowing this separately pinned
// append. Neither predecessor receipt is regenerated or relabeled.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {compareOriginalLinkRawSchemaExtension, verifyOriginalLinkRawSchemaExtensionClosure} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_SCOPE} from './raw-schema-second-extension-policy.mjs';

export const ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT = '80c27d9bdef086925f24ac06de026cce376e44fdf02239e6b6d5cfecdaf2e811';
const sha256 = value => createHash('sha256').update(value).digest('hex');

function verifySecondSchemaSource(receipt, first, currentBytes, historicalSources, second, additions) {
  assert.equal(second.version, 1);
  assert.equal(second.predecessorReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  assert.equal(sha256(structuredDataToLino(first)), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, 'the whole first extension receipt remains byte exact');
  assert.equal(typeof second.previous.source, 'string');
  assert.ok(second.previous.source.isWellFormed());
  const previousBytes = Buffer.from(second.previous.source, 'utf8');
  assert.equal(second.previous.file, first.current.file);
  assert.equal(second.previous.bytes, previousBytes.length);
  assert.equal(second.previous.sha256, sha256(previousBytes));
  assert.equal(second.previous.sha256, first.current.sha256);
  const previous = verifyOriginalLinkRawSchemaExtensionClosure(receipt, previousBytes, historicalSources, first);
  assert.deepEqual(second.appendedSchemaEntries, additions);
  assert.deepEqual(second.consumedSchemaProperties, [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES]);
  assert.equal(second.current.file, first.current.file);
  assert.ok(Buffer.isBuffer(currentBytes));
  assert.equal(second.current.bytes, currentBytes.length);
  assert.equal(second.current.sha256, sha256(currentBytes), 'the complete second-stage schema pin is exact');
  const before = readStructuredDataLino(second.previous.source);
  const after = readStructuredDataLino(currentBytes.toString('utf8'));
  compareOriginalLinkRawSchemaExtension(before, after, additions);
  for (const key of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(after[key], before[key], 'a raw producer input changed in stage two: ' + key);
  assert.equal(second.scope, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_SCOPE);
  return {
    ...previous,
    sha256: second.current.sha256,
    firstExtensionSha256: first.current.sha256,
    firstExtensionReceiptSha256: second.predecessorReceiptSha256,
    appendedSecondSchemaEntries: second.appendedSchemaEntries,
  };
}

export function verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, currentBytes, historicalSources, second) {
  return verifySecondSchemaSource(receipt, first, currentBytes, historicalSources, second, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS);
}

/** Only authoring uses this narrower predecessor admission. The ordinary
 * verifier still requires the complete final packet. The writer separately
 * authenticates the preserved prior receipt bytes against their exact pin. */
export function verifyOriginalLinkRawSchemaSecondDraftPredecessor(receipt, first, currentBytes, historicalSources, second) {
  const old = second.appendedSchemaEntries;
  const current = ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS;
  assert.deepEqual(Object.keys(current).slice(0, Object.keys(old).length), Object.keys(old), 'old draft maps remain an ordered prefix of the reviewed final packet');
  for (const [map, entries] of Object.entries(old)) {
    assert.deepEqual(Object.keys(current[map]).slice(0, Object.keys(entries).length), Object.keys(entries), 'old draft entries remain an ordered prefix: ' + map);
    for (const [key, value] of Object.entries(entries)) assert.deepEqual(current[map][key], value, 'the final packet cannot redefine an old draft entry: ' + map + '.' + key);
  }
  return verifySecondSchemaSource(receipt, first, currentBytes, historicalSources, second, old);
}
