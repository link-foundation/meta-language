// Root's one second-stage writer. Source closure only; no generation/test
// result or committed current revision is manufactured by this command.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, writeFileSync} from 'node:fs';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData, serializeProjectData, writeProjectData} from './project-data.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES, readOriginalLinkRawHistoricalSources} from '../../experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, compareOriginalLinkRawSchemaExtension, verifyOriginalLinkRawSchemaExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_SCOPE} from '../../experiments/original-link-complete-table-projection/raw-schema-second-extension-policy.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, verifyOriginalLinkRawSchemaSecondDraftPredecessor, verifyOriginalLinkRawSchemaSecondExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES, verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';

const schemaFile = new URL('../src/data/typed-program-links-schema.lino', import.meta.url);
const base = new URL('../../experiments/original-link-complete-table-projection/', import.meta.url);
const receiptFile = new URL('data/raw-schema-source-closure.lino', base);
const firstFile = new URL('data/raw-schema-extension-closure.lino', base);
const secondFile = new URL('data/raw-schema-second-extension-closure.lino', base);
const thirdFile = new URL('data/raw-schema-third-extension-closure.lino', base);
const oracleFile = new URL('raw-native-observations.mjs', base);
const sha256 = value => createHash('sha256').update(value).digest('hex');
const receiptBytes = readFileSync(receiptFile), firstBytes = readFileSync(firstFile);
assert.equal(sha256(receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
assert.equal(sha256(firstBytes), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
const receipt = readStructuredDataLino(receiptBytes.toString('utf8'));
const first = readStructuredDataLino(firstBytes.toString('utf8'));
const sources = readOriginalLinkRawHistoricalSources(readFileSync(oracleFile, 'utf8'));
let actualBytes = readFileSync(schemaFile);
const args = process.argv.slice(2);
const check = args.length === 1 && args[0] === '--check';
const refresh = args.length === 2 && args[0] === '--refresh-draft';
assert.ok(args.length === 0 || check || refresh, 'use no arguments, --check, or --refresh-draft PRESERVED_RECEIPT_FILE');
if (existsSync(thirdFile)) {
  assert.equal(refresh, false, 'the published second stage is immutable after its third successor exists');
  const thirdBytes = readFileSync(thirdFile), third = readStructuredDataLino(thirdBytes.toString('utf8'));
  const second = readStructuredDataLino(readFileSync(secondFile, 'utf8'));
  assert.equal(sha256(thirdBytes), /^const schemaThirdExtensionReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1]);
  const authoredSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => [file, readFileSync(new URL('../../' + file, import.meta.url))]));
  verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, actualBytes, sources, second, third, authoredSources);
  checkProjectData(thirdFile, third);
  actualBytes = Buffer.from(third.previous.source, 'utf8');
}
let second;
let preservedFile, preservedBytes;
if (refresh) {
  assert.ok(existsSync(secondFile), 'a draft refresh requires an existing second-stage receipt');
  preservedFile = args[1];
  assert.ok(preservedFile.startsWith('/private/tmp/'), 'the prior draft audit must be a root-preserved private temporary file');
  preservedBytes = readFileSync(preservedFile);
  assert.deepEqual(preservedBytes, readFileSync(secondFile), 'the preserved prior receipt is byte exact');
  const oldPin = /^const schemaSecondExtensionReceiptSha256 = '([0-9a-f]{64})';$/mu.exec(readFileSync(oracleFile, 'utf8'))?.[1];
  assert.equal(sha256(preservedBytes), oldPin, 'the prior whole receipt matches its existing exact pin');
  const old = readStructuredDataLino(preservedBytes.toString('utf8'));
  verifyOriginalLinkRawSchemaSecondDraftPredecessor(receipt, first, actualBytes, sources, old);
  const before = readStructuredDataLino(actualBytes.toString('utf8'));
  const after = readStructuredDataLino(actualBytes.toString('utf8'));
  const delta = {};
  for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS)) {
    if (!Object.hasOwn(after, map)) after[map] = {};
    for (const [key, value] of Object.entries(entries)) {
      assert.notEqual(value, undefined, 'a declared final schema packet is missing: ' + map + '.' + key);
      if (Object.hasOwn(after[map], key)) assert.deepEqual(after[map][key], value, 'a draft refresh cannot replace an existing entry');
      else {
        delta[map] ??= {};
        delta[map][key] = value;
        after[map][key] = value;
      }
    }
  }
  compareOriginalLinkRawSchemaExtension(before, after, delta);
  const currentBytes = Buffer.from(serializeProjectData(after, schemaFile));
  second = {...old,
    current: {file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: currentBytes.length, sha256: sha256(currentBytes)},
    appendedSchemaEntries: ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS,
  };
  verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, currentBytes, sources, second);
  writeFileSync(schemaFile, currentBytes);
  writeProjectData(secondFile, second);
} else if (existsSync(secondFile)) {
  second = readStructuredDataLino(readFileSync(secondFile, 'utf8'));
  verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, actualBytes, sources, second);
  checkProjectData(secondFile, second);
} else {
  assert.equal(check, false, 'the second schema extension has not been generated');
  verifyOriginalLinkRawSchemaExtensionClosure(receipt, actualBytes, sources, first);
  const before = readStructuredDataLino(actualBytes.toString('utf8'));
  const after = readStructuredDataLino(actualBytes.toString('utf8'));
  for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS)) {
    if (!Object.hasOwn(after, map)) after[map] = {};
    for (const [key, value] of Object.entries(entries)) {
      assert.notEqual(value, undefined, 'a declared final schema packet is missing: ' + map + '.' + key);
      if (Object.hasOwn(after[map], key)) assert.deepEqual(after[map][key], value, 'stage two cannot replace an existing entry');
      else after[map][key] = value;
    }
  }
  compareOriginalLinkRawSchemaExtension(before, after, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS);
  const currentBytes = Buffer.from(serializeProjectData(after, schemaFile));
  second = {
    version: 1,
    predecessorReceiptSha256: ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT,
    previous: {...first.current, source: actualBytes.toString('utf8')},
    current: {file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: currentBytes.length, sha256: sha256(currentBytes)},
    consumedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES],
    appendedSchemaEntries: ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSIONS,
    scope: ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_SCOPE,
  };
  verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, currentBytes, sources, second);
  writeFileSync(schemaFile, currentBytes);
  writeProjectData(secondFile, second);
}
const oracle = readFileSync(oracleFile, 'utf8');
const pattern = /^const schemaSecondExtensionReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
assert.ok(pattern.test(oracle), 'one separate second-stage receipt pin is required');
const pin = "const schemaSecondExtensionReceiptSha256 = '" + sha256(readFileSync(secondFile)) + "';";
if (check) assert.equal(oracle.match(pattern)[0], pin);
else writeFileSync(oracleFile, oracle.replace(pattern, pin));
assert.deepEqual(readFileSync(receiptFile), receiptBytes);
assert.deepEqual(readFileSync(firstFile), firstBytes);
if (refresh) {
  assert.deepEqual(readFileSync(preservedFile), preservedBytes, 'the prior whole draft receipt remains in the private audit');
  console.log('Preserved prior draft receipt: ' + preservedFile + '; bytes ' + preservedBytes.length + '; sha256 ' + sha256(preservedBytes));
}
console.log('Typed schema stage two: exact predecessor chain and declared cell/optional/array/method/union child plans; source closure only');
