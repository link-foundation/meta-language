// One serialized append-only schema writer. This performs bounded source
// checks only; JavaScript tests and native generation are separate commands.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {existsSync, readFileSync, writeFileSync} from 'node:fs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData, serializeProjectData, writeProjectData} from './project-data.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES, readOriginalLinkRawHistoricalSources, verifyOriginalLinkRawSchemaSourceClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_EXTENSION_SCOPE} from '../../experiments/original-link-complete-table-projection/raw-schema-extension-policy.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, compareOriginalLinkRawSchemaExtension, verifyOriginalLinkRawSchemaExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaSecondExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES, verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';

const root = new URL('../../', import.meta.url);
const schemaFile = new URL('../src/data/typed-program-links-schema.lino', import.meta.url);
const receiptFile = new URL('../../experiments/original-link-complete-table-projection/data/raw-schema-source-closure.lino', import.meta.url);
const extensionFile = new URL('../../experiments/original-link-complete-table-projection/data/raw-schema-extension-closure.lino', import.meta.url);
const secondFile = new URL('../../experiments/original-link-complete-table-projection/data/raw-schema-second-extension-closure.lino', import.meta.url);
const thirdFile = new URL('../../experiments/original-link-complete-table-projection/data/raw-schema-third-extension-closure.lino', import.meta.url);
const oracleFile = new URL('../../experiments/original-link-complete-table-projection/raw-native-observations.mjs', import.meta.url);
const sha256 = value => createHash('sha256').update(value).digest('hex');
const receiptBytes = readFileSync(receiptFile);
assert.equal(sha256(receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, 'the original B43 receipt bytes remain unchanged');
const receipt = readStructuredDataLino(receiptBytes.toString('utf8'));
const historicalSources = readOriginalLinkRawHistoricalSources(readFileSync(oracleFile, 'utf8'));
const actualBytes = readFileSync(schemaFile);
const check = process.argv.includes('--check');
assert.ok(process.argv.slice(2).every(argument => argument === '--check'), 'use no arguments or --check');
if (existsSync(secondFile)) {
  assert.equal(check, true, 'stage one is immutable after a separate second extension exists');
  const first = readStructuredDataLino(readFileSync(extensionFile, 'utf8'));
  const second = readStructuredDataLino(readFileSync(secondFile, 'utf8'));
  if (existsSync(thirdFile)) {
    const thirdBytes = readFileSync(thirdFile), third = readStructuredDataLino(thirdBytes.toString('utf8'));
    assert.equal(sha256(thirdBytes), /^const schemaThirdExtensionReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1]);
    const authoredSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]));
    verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, actualBytes, historicalSources, second, third, authoredSources);
    checkProjectData(thirdFile, third);
  } else verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, actualBytes, historicalSources, second);
  checkProjectData(extensionFile, first);
  checkProjectData(secondFile, second);
  assert.equal(sha256(readFileSync(extensionFile)), /^const schemaExtensionReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1]);
  assert.equal(sha256(readFileSync(secondFile)), /^const schemaSecondExtensionReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1]);
  console.log('Typed schema stage one remains exact within the separately authenticated second extension');
  process.exit(0);
}

let extension;
if (existsSync(extensionFile)) {
  extension = readStructuredDataLino(readFileSync(extensionFile, 'utf8'));
  verifyOriginalLinkRawSchemaExtensionClosure(receipt, actualBytes, historicalSources, extension);
  checkProjectData(extensionFile, extension);
} else {
  assert.equal(check, false, 'the coordinated schema extension receipt has not been generated');
  verifyOriginalLinkRawSchemaSourceClosure(receipt, actualBytes, historicalSources);
  const committed = execFileSync('git', ['show', receipt.current.commit + ':' + ORIGINAL_LINK_RAW_SCHEMA_FILE], {cwd: root, maxBuffer: 1024 * 1024});
  assert.deepEqual(committed, actualBytes, 'the complete predecessor schema is captured from its authenticated immutable commit');
  const before = readStructuredDataLino(actualBytes.toString('utf8'));
  const after = readStructuredDataLino(actualBytes.toString('utf8'));
  for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS)) {
    if (!Object.hasOwn(after, map)) after[map] = {};
    for (const [key, value] of Object.entries(entries)) {
      if (Object.hasOwn(after[map], key)) assert.deepEqual(after[map][key], value, 'a coordinated append cannot replace an existing entry');
      else after[map][key] = value;
    }
  }
  compareOriginalLinkRawSchemaExtension(before, after);
  const currentBytes = Buffer.from(serializeProjectData(after, schemaFile));
  extension = {
    version: 1,
    predecessorReceiptSha256: ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT,
    previous: {...receipt.current, source: actualBytes.toString('utf8')},
    current: {file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: currentBytes.length, sha256: sha256(currentBytes)},
    consumedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES],
    appendedSchemaEntries: ORIGINAL_LINK_RAW_SCHEMA_EXTENSIONS,
    scope: ORIGINAL_LINK_RAW_SCHEMA_EXTENSION_SCOPE,
  };
  verifyOriginalLinkRawSchemaExtensionClosure(receipt, currentBytes, historicalSources, extension);
  writeFileSync(schemaFile, currentBytes);
  writeProjectData(extensionFile, extension);
}

const oracle = readFileSync(oracleFile, 'utf8');
const pin = /^const schemaExtensionReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
assert.ok(pin.test(oracle), 'one separate extension receipt pin is required');
const expected = "const schemaExtensionReceiptSha256 = '" + sha256(readFileSync(extensionFile)) + "';";
if (check) assert.equal(oracle.match(pin)[0], expected, 'the complete extension receipt pin is exact');
else writeFileSync(oracleFile, oracle.replace(pin, expected));
assert.deepEqual(readFileSync(receiptFile), receiptBytes, 'the writer preserves every byte of the old receipt');
console.log('Typed schema append: immutable predecessor + exact ordered reference/callable/record additions; source closure only');
