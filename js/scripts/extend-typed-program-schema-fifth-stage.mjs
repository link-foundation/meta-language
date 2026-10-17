// One immutable fifth-stage author. This records source closure, not execution.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync, unlinkSync, writeFileSync} from 'node:fs';
import {resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {readStructuredDataLino} from '../src/structured-data-links.js';
import {checkProjectData, serializeProjectData} from './project-data.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FILE, ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from '../../experiments/original-link-complete-table-projection/raw-schema-source-closure.mjs';
import {compareOriginalLinkRawSchemaExtension} from '../../experiments/original-link-complete-table-projection/raw-schema-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaThirdExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs';
import {verifyOriginalLinkRawSchemaFourthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fourth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES,
  ORIGINAL_LINK_RAW_SCHEMA_FIFTH_COMMAND, readOriginalLinkRawSchemaFifthContext,
  verifyOriginalLinkRawSchemaFifthExtensionClosure} from '../../experiments/original-link-complete-table-projection/raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_SCOPE} from '../../experiments/original-link-complete-table-projection/raw-schema-fifth-extension-policy.mjs';

const root = new URL('../../', import.meta.url);
const schemaFile = new URL('../src/data/typed-program-links-schema.lino', import.meta.url);
const base = new URL('../../experiments/original-link-complete-table-projection/', import.meta.url);
const fifthFile = new URL('data/raw-schema-fifth-extension-closure.lino', base);
const oracleFile = new URL('raw-native-observations.mjs', base);
const sha256 = value => createHash('sha256').update(value).digest('hex');

export function extendTypedProgramSchemaFifthStage({check = false} = {}) {
  const input = readOriginalLinkRawSchemaFifthContext();
  const retainedFiles = ['raw-schema-source-closure', 'raw-schema-extension-closure', 'raw-schema-second-extension-closure',
    'raw-schema-third-extension-closure', 'raw-schema-fourth-extension-closure',
    'raw-schema-second-extension-provenance', 'raw-schema-second-extension-provenance-0112b8e3']
    .map(name => new URL('data/' + name + '.lino', base));
  const retainedBytes = retainedFiles.map(file => readFileSync(file));
  const authoredSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]));
  const oracle = readFileSync(oracleFile, 'utf8'), actualBytes = readFileSync(schemaFile);
  let fifth, fifthBytes, currentBytes = actualBytes;
  if (existsSync(fifthFile)) {
    fifthBytes = readFileSync(fifthFile); fifth = readStructuredDataLino(fifthBytes.toString('utf8'));
    verifyOriginalLinkRawSchemaFifthExtensionClosure(input, actualBytes, fifth, authoredSources);
    checkProjectData(fifthFile, fifth);
  } else {
    assert.equal(check, false, 'the fifth schema successor has not been authored');
    assert.equal(sha256(input.fourthBytes), ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT);
    verifyOriginalLinkRawSchemaFourthExtensionClosure(input, actualBytes, input.fourth, input.fourthSources);
    const before = readStructuredDataLino(actualBytes.toString('utf8')), after = readStructuredDataLino(actualBytes.toString('utf8'));
    for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS)) {
      assert.ok(Object.hasOwn(after, map), 'the declared append uses an existing map');
      for (const [key, entry] of Object.entries(entries)) {
        assert.equal(Object.hasOwn(after[map], key), false, 'the immutable successor cannot redefine an existing key');
        after[map][key] = entry;
      }
    }
    compareOriginalLinkRawSchemaExtension(before, after, ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS);
    currentBytes = Buffer.from(serializeProjectData(after, schemaFile));
    fifth = {version: 1, predecessorReceiptSha256: ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSION_RECEIPT,
      previous: {...input.fourth.current, source: actualBytes.toString('utf8')},
      current: {file: ORIGINAL_LINK_RAW_SCHEMA_FILE, bytes: currentBytes.length, sha256: sha256(currentBytes)},
      consumedSchemaProperties: [...ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES], appendedSchemaEntries: ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS,
      authoring: {command: ORIGINAL_LINK_RAW_SCHEMA_FIFTH_COMMAND, sourceFiles: ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES
        .map(file => ({file, bytes: authoredSources[file].length, sha256: sha256(authoredSources[file])}))},
      scope: ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSION_SCOPE};
    verifyOriginalLinkRawSchemaFifthExtensionClosure(input, currentBytes, fifth, authoredSources);
    fifthBytes = Buffer.from(serializeProjectData(fifth, fifthFile));
  }
  const pattern = /^const schemaFifthExtensionReceiptSha256 = '(?:PENDING|[0-9a-f]{64})';$/mu;
  assert.equal([...oracle.matchAll(new RegExp(pattern.source, 'gmu'))].length, 1, 'one separate fifth-receipt pin is required');
  const previousPin = oracle.match(pattern)[0], pin = "const schemaFifthExtensionReceiptSha256 = '" + sha256(fifthBytes) + "';";
  if (check || existsSync(fifthFile)) assert.equal(previousPin, pin, 'the immutable whole fifth receipt pin remains exact');
  else {
    const changedOracle = oracle.replace(pattern, pin);
    assert.equal(changedOracle.replace(pin, previousPin), oracle, 'only the separate fifth receipt pin may change');
    let created = false;
    try {
      writeFileSync(fifthFile, fifthBytes, {flag: 'wx'}); created = true;
      writeFileSync(schemaFile, currentBytes); writeFileSync(oracleFile, changedOracle);
    } catch (error) {
      if (created) {writeFileSync(schemaFile, actualBytes); writeFileSync(oracleFile, oracle); unlinkSync(fifthFile);}
      throw error;
    }
  }
  retainedFiles.forEach((file, index) => assert.deepEqual(readFileSync(file), retainedBytes[index]));
  return {schemaSha256: fifth.current.sha256, predecessorSchemaSha256: fifth.previous.sha256,
    predecessorReceiptSha256: fifth.predecessorReceiptSha256, appendedProjectionKeys: 2,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check};
}

/** Check the exact archived fourth profile after authenticating its live successor. */
export function checkTypedProgramSchemaFourthArchive() {
  extendTypedProgramSchemaFifthStage({check: true});
  const input = readOriginalLinkRawSchemaFifthContext();
  const fifth = readStructuredDataLino(readFileSync(fifthFile, 'utf8'));
  verifyOriginalLinkRawSchemaFourthExtensionClosure(input, Buffer.from(fifth.previous.source, 'utf8'), input.fourth, input.fourthSources);
  return {schemaSha256: input.fourth.current.sha256, predecessorSchemaSha256: input.fourth.previous.sha256,
    predecessorReceiptSha256: input.fourth.predecessorReceiptSha256, appendedProjectionKeys: 1,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}

/** Check the exact archived third profile after authenticating all successors. */
export function checkTypedProgramSchemaThirdArchive() {
  extendTypedProgramSchemaFifthStage({check: true});
  const input = readOriginalLinkRawSchemaFifthContext();
  verifyOriginalLinkRawSchemaThirdExtensionClosure(input.receipt, input.first, Buffer.from(input.fourth.previous.source, 'utf8'),
    input.historicalSources, input.second, input.third, input.thirdSources);
  return {schemaSha256: input.third.current.sha256, predecessorSchemaSha256: input.third.previous.sha256,
    predecessorReceiptSha256: input.third.predecessorReceiptSha256,
    appendedProjectionKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjections).length,
    appendedAlternativeKeys: Object.keys(input.third.appendedSchemaEntries.expressionProjectionAlternatives).length,
    preservedProducerBindings: Object.keys(input.historicalSources).length, sourceClosureOnly: true, check: true};
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length === 0 || args.length === 1 && args[0] === '--check', 'use no arguments or --check');
  console.log(JSON.stringify(extendTypedProgramSchemaFifthStage({check: args[0] === '--check'}), null, 2));
}
