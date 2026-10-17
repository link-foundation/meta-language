// One immutable third-stage schema writer; this records source provenance only.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, unlinkSync, writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData, serializeProjectData} from './project-data.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES, readOriginalLinkRawHistoricalSources} from '../../experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, compareOriginalLinkRawSchemaExtension} from '../../experiments/original-link-complete-table-projection/raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, verifyOriginalLinkRawSchemaSecondExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT, ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES, ORIGINAL_LINK_RAW_SCHEMA_THIRD_COMMAND, verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_SCOPE} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-policy.mjs';

const root = new URL('../../', import.meta.url);
const schemaFile = new URL('../src/data/typed-program-links-schema.lino', import.meta.url);
const base = new URL('../../experiments/original-link-complete-table-projection/', import.meta.url);
const receiptFile = new URL('data/raw-schema-source-closure.lino', base);
const firstFile = new URL('data/raw-schema-extension-closure.lino', base);
const secondFile = new URL('data/raw-schema-second-extension-closure.lino', base);
const thirdFile = new URL('data/raw-schema-third-extension-closure.lino', base);
const oracleFile = new URL('raw-native-observations.mjs', base);
const sha256 = value => createHash('sha256').update(value).digest('hex');

export function extendTypedProgramSchemaThirdStage({check = false} = {}) {
  const receiptBytes = readFileSync(receiptFile), firstBytes = readFileSync(firstFile), secondBytes = readFileSync(secondFile);
  assert.equal(sha256(receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(sha256(firstBytes), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  assert.equal(sha256(secondBytes), ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT);
  const receipt = readStructuredDataLino(receiptBytes.toString('utf8')), first = readStructuredDataLino(firstBytes.toString('utf8')), second = readStructuredDataLino(secondBytes.toString('utf8'));
  const oracle = readFileSync(oracleFile, 'utf8'), historicalSources = readOriginalLinkRawHistoricalSources(oracle);
  const authoredSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]));
  const actualBytes = readFileSync(schemaFile);
  let third, currentBytes = actualBytes, thirdBytes;
  if (existsSync(thirdFile)) {
    thirdBytes = readFileSync(thirdFile); third = readStructuredDataLino(thirdBytes.toString('utf8'));
    verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, actualBytes, historicalSources, second, third, authoredSources);
    checkProjectData(thirdFile, third);
  } else {
    assert.equal(check, false, 'the third schema successor has not been authored');
    verifyOriginalLinkRawSchemaSecondExtensionClosure(receipt, first, actualBytes, historicalSources, second);
    const before = readStructuredDataLino(actualBytes.toString('utf8')), after = readStructuredDataLino(actualBytes.toString('utf8'));
    for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS)) {
      assert.ok(Object.hasOwn(after, map), 'the declared append uses an existing map');
      for (const [key, value] of Object.entries(entries)) {
        assert.equal(Object.hasOwn(after[map], key), false, 'the immutable successor cannot redefine an existing key');
        after[map][key] = value;
      }
    }
    compareOriginalLinkRawSchemaExtension(before, after, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS);
    currentBytes = Buffer.from(serializeProjectData(after, schemaFile));
    third = {version: 1, predecessorReceiptSha256: ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT,
      previous: {...second.current, source: actualBytes.toString('utf8')},
      current: {file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: currentBytes.length, sha256: sha256(currentBytes)},
      consumedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES], appendedSchemaEntries: ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS,
      authoring: {command: ORIGINAL_LINK_RAW_SCHEMA_THIRD_COMMAND,
        sourceFiles: ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => ({file, bytes: authoredSources[file].length, sha256: sha256(authoredSources[file])}))},
      scope: ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_SCOPE};
    verifyOriginalLinkRawSchemaThirdExtensionClosure(receipt, first, currentBytes, historicalSources, second, third, authoredSources);
    thirdBytes = Buffer.from(serializeProjectData(third, thirdFile));
  }
  const pattern = /^const schemaThirdExtensionReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
  assert.equal([...oracle.matchAll(new RegExp(pattern.source, 'gmu'))].length, 1, 'one separate third-receipt pin is required');
  const pin = "const schemaThirdExtensionReceiptSha256 = '" + sha256(thirdBytes) + "';";
  if (check || existsSync(thirdFile)) assert.equal(oracle.match(pattern)[0], pin, 'the immutable whole third receipt pin remains exact');
  else {
    const changedOracle = oracle.replace(pattern, pin);
    assert.equal(changedOracle.replace(pin, oracle.match(pattern)[0]), oracle, 'all producer source bytes except the new pin stay exact');
    let created = false;
    try {
      writeFileSync(thirdFile, thirdBytes, {flag: 'wx'}); created = true;
      writeFileSync(schemaFile, currentBytes); writeFileSync(oracleFile, changedOracle);
    } catch (error) {
      if (created) { writeFileSync(schemaFile, actualBytes); writeFileSync(oracleFile, oracle); unlinkSync(thirdFile); }
      throw error;
    }
  }
  assert.deepEqual(readFileSync(receiptFile), receiptBytes);assert.deepEqual(readFileSync(firstFile), firstBytes);assert.deepEqual(readFileSync(secondFile), secondBytes);
  return {schemaSha256: third.current.sha256, predecessorSchemaSha256: third.previous.sha256,
    predecessorReceiptSha256: third.predecessorReceiptSha256, appendedProjectionKeys: 3, appendedAlternativeKeys: 1,
    preservedProducerBindings: Object.keys(historicalSources).length, sourceClosureOnly: true, check};
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);assert.ok(args.length === 0 || args.length === 1 && args[0] === '--check', 'use no arguments or --check');
  console.log(JSON.stringify(extendTypedProgramSchemaThirdStage({check: args[0] === '--check'}), null, 2));
}
