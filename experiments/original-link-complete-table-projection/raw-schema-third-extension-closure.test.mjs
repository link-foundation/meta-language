// Bounded source provenance and schema controls; no native execution or planning.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {extendTypedProgramSchemaThirdStage} from '../../js/scripts/extend-typed-program-schema-third-stage.mjs';
import {checkTypedProgramSchemaThirdArchive} from '../../js/scripts/check-typed-program-schema-history.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES, readOriginalLinkRawSchemaFourthContext, verifyOriginalLinkRawSchemaFourthExtensionClosure} from './raw-schema-fourth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES, readOriginalLinkRawSchemaFifthContext, verifyOriginalLinkRawSchemaFifthExtensionClosure} from './raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES, readOriginalLinkRawSchemaSixthContext, verifyOriginalLinkRawSchemaSixthExtensionClosure} from './raw-schema-sixth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES, readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, verifyOriginalLinkRawSchemaSecondExtensionClosure} from './raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT, ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES, verifyOriginalLinkRawSchemaThirdExtensionClosure} from './raw-schema-third-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS} from './raw-schema-third-extension-policy.mjs';

const sha256 = value => createHash('sha256').update(value).digest('hex');
function fixture() {
  const read = file => readFileSync(new URL(file, import.meta.url));
  const receiptBytes = read('./data/raw-schema-source-closure.lino');
  const firstBytes = read('./data/raw-schema-extension-closure.lino');
  const secondBytes = read('./data/raw-schema-second-extension-closure.lino');
  const thirdBytes = read('./data/raw-schema-third-extension-closure.lino');
  const fourthFile = new URL('./data/raw-schema-fourth-extension-closure.lino', import.meta.url);
  let bytes = read('../../js/src/data/typed-program-links-schema.lino');
  const sixthFile = new URL('./data/raw-schema-sixth-extension-closure.lino', import.meta.url);
  if (existsSync(sixthFile)) {
    const sixth = readStructuredDataLino(readFileSync(sixthFile, 'utf8'));
    const sixthSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, new URL('../../', import.meta.url)))]));
    verifyOriginalLinkRawSchemaSixthExtensionClosure(readOriginalLinkRawSchemaSixthContext(), bytes, sixth, sixthSources);
    bytes = Buffer.from(sixth.previous.source, 'utf8');
  }
  const fifthFile = new URL('./data/raw-schema-fifth-extension-closure.lino', import.meta.url);
  if (existsSync(fifthFile)) {
    const fifth = readStructuredDataLino(readFileSync(fifthFile, 'utf8'));
    const sources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file => [file, read('../../' + file)]));
    verifyOriginalLinkRawSchemaFifthExtensionClosure(readOriginalLinkRawSchemaFifthContext(), bytes, fifth, sources);
    bytes = Buffer.from(fifth.previous.source, 'utf8');
  }
  if (existsSync(fourthFile)) {
    const fourth = readStructuredDataLino(readFileSync(fourthFile, 'utf8'));
    const sources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES.map(file => [file, read('../../' + file)]));
    verifyOriginalLinkRawSchemaFourthExtensionClosure(readOriginalLinkRawSchemaFourthContext(), bytes, fourth, sources);
    bytes = Buffer.from(fourth.previous.source, 'utf8');
  }
  return {receiptBytes, firstBytes, secondBytes, thirdBytes,
    receipt: readStructuredDataLino(receiptBytes.toString('utf8')),
    first: readStructuredDataLino(firstBytes.toString('utf8')),
    second: readStructuredDataLino(secondBytes.toString('utf8')),
    third: readStructuredDataLino(thirdBytes.toString('utf8')),
    bytes,
    sources: readOriginalLinkRawHistoricalSources(read('./raw-native-observations.mjs').toString('utf8')),
    authoredSources: Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES.map(file => [file, read('../../' + file)]))};
}
const verify = value => verifyOriginalLinkRawSchemaThirdExtensionClosure(value.receipt, value.first, value.bytes,
  value.sources, value.second, value.third, value.authoredSources);

test('the third schema successor authenticates the complete published stage two and all original source inputs', () => {
  const value = fixture(), result = verify(value);
  assert.equal(sha256(value.receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(sha256(value.firstBytes), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  assert.equal(sha256(value.secondBytes), ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT);
  assert.equal(value.third.previous.bytes, 137291);
  assert.equal(value.third.previous.sha256, '4c80d479c0b04a7c1edae33bd9bc26a5ea82e3044ab9303ee157db74dfa14414');
  assert.equal(Object.keys(value.sources).length, 47);
  assert.equal(value.receipt.generation.definitionCount, 106);
  assert.equal(value.receipt.generation.completeRustBytes, 351746);
  const before = readStructuredDataLino(value.third.previous.source), after = readStructuredDataLino(value.bytes.toString('utf8'));
  for (const property of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(after[property], before[property]);
  for (const [map, entries] of Object.entries(ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSIONS)) {
    assert.deepEqual(Object.keys(after[map]), [...Object.keys(before[map]), ...Object.keys(entries)]);
    for (const [key, entry] of Object.entries(entries)) assert.deepEqual(after[map][key], entry);
  }
  assert.deepEqual(after.expressionProjections.strictEqual, {});
  assert.equal(after.expressionProjectionAlternatives.strictEqual[0].value, false);
  assert.equal(after.expressionProjectionAlternatives.strictEqual[1].value, true);
  assert.equal(result.sha256, sha256(value.bytes));
  assert.equal(result.secondExtensionReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT);
  const archived = Buffer.from(value.third.previous.source, 'utf8');
  assert.doesNotThrow(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, archived, value.sources, value.second));
  assert.throws(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, value.bytes, value.sources, value.second), assert.AssertionError);
});

test('coherent third-stage hashes cannot change old schema data or the exact Boolean projection alternatives', () => {
  const original = fixture();
  for (const mutate of [
    schema => {schema.fields.k = 'counterfeit';},
    schema => {schema.expressionOperands.referenceRead = ['value'];},
    schema => {delete schema.expressionProjections.lambda;},
    schema => {schema.expressionProjections.strictEqual.construct = 'binary';},
    schema => {schema.expressionProjections['strictEqual:false'].fields[0][2] = 'ne';},
    schema => {schema.expressionProjectionAlternatives.strictEqual[0].value = 'false';},
    schema => {schema.expressionProjectionAlternatives.strictEqual[1].value = 'true';},
    schema => {schema.expressionProjectionAlternatives.strictEqual.reverse();},
    schema => {schema.expressionProjections = Object.fromEntries(Object.entries(schema.expressionProjections).reverse());},
    schema => {schema.expressionProjections.extra = {};},
    schema => {schema.unexpected = {};},
  ]) {
    const value = {...original, third: structuredClone(original.third)};
    const schema = readStructuredDataLino(original.bytes.toString('utf8')); mutate(schema);
    value.bytes = Buffer.from(structuredDataToLino(schema));
    value.third.current.bytes = value.bytes.length; value.third.current.sha256 = sha256(value.bytes);
    assert.throws(() => verify(value), assert.AssertionError);
  }
});

test('third-stage receipts cannot replace predecessor bindings source provenance or scope', () => {
  const original = fixture();
  for (const mutate of [
    value => {value.receipt.generation.definitionCount--;},
    value => {value.first.current.sha256 = '0'.repeat(64);},
    value => {value.second.current.sha256 = '0'.repeat(64);},
    value => {value.third.previous.source += '\n';},
    value => {value.third.previous.bytes--;},
    value => {value.third.previous.sha256 = '0'.repeat(64);},
    value => {value.third.predecessorReceiptSha256 = '0'.repeat(64);},
    value => {value.third.consumedSchemaProperties.reverse();},
    value => {delete value.third.appendedSchemaEntries.expressionProjections.strictEqual;},
    value => {value.third.authoring.command += ' --repair';},
    value => {value.third.authoring.sourceFiles.reverse();},
    value => {value.third.authoring.sourceFiles[0].bytes--;},
    value => {value.third.authoring.sourceFiles[0].sha256 = '0'.repeat(64);},
    value => {value.third.scope = 'native execution';},
    value => {value.sources['policy.mjs'] = '0'.repeat(64);},
    value => {const file = ORIGINAL_LINK_RAW_SCHEMA_THIRD_SOURCE_FILES[0]; value.authoredSources[file] = Buffer.concat([value.authoredSources[file], Buffer.from('\n')]);},
  ]) {
    const value = {...original, receipt: structuredClone(original.receipt), first: structuredClone(original.first), second: structuredClone(original.second),
      third: structuredClone(original.third), sources: {...original.sources}, authoredSources: {...original.authoredSources}};
    mutate(value); assert.throws(() => verify(value), assert.AssertionError);
  }
});

test('the actual immutable third-stage check leaves every schema receipt and producer byte unchanged', () => {
  const files = ['../../js/src/data/typed-program-links-schema.lino', './data/raw-schema-source-closure.lino',
    './data/raw-schema-extension-closure.lino', './data/raw-schema-second-extension-closure.lino',
    './data/raw-schema-third-extension-closure.lino', './data/raw-schema-second-extension-provenance.lino',
    './data/raw-schema-second-extension-provenance-0112b8e3.lino', './raw-native-observations.mjs'];
  const before = files.map(file => readFileSync(new URL(file, import.meta.url)));
  let result;
  if (existsSync(new URL('./data/raw-schema-fourth-extension-closure.lino', import.meta.url))) {
    assert.throws(() => extendTypedProgramSchemaThirdStage({check: true}), assert.AssertionError,
      'the frozen strict third writer refuses the live fourth profile');
    result = checkTypedProgramSchemaThirdArchive();
  } else result = extendTypedProgramSchemaThirdStage({check: true});
  assert.equal(result.check, true); assert.equal(result.sourceClosureOnly, true);
  assert.equal(result.appendedProjectionKeys, 3); assert.equal(result.appendedAlternativeKeys, 1);
  assert.equal(result.preservedProducerBindings, 47);
  files.forEach((file, index) => assert.deepEqual(readFileSync(new URL(file, import.meta.url)), before[index]));
});
