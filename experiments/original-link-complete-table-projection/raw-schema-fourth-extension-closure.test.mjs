// Bounded source provenance controls; no native planner or compilation.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {extendTypedProgramSchemaFourthStage} from '../../js/scripts/extend-typed-program-schema-fourth-stage.mjs';
import {checkTypedProgramSchemaFourthArchive, checkTypedProgramSchemaThirdArchive} from '../../js/scripts/check-typed-program-schema-history.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES} from './raw-schema-source-closure.mjs';
import {verifyOriginalLinkRawSchemaThirdExtensionClosure} from './raw-schema-third-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES,
  readOriginalLinkRawSchemaFourthContext, verifyOriginalLinkRawSchemaFourthExtensionClosure} from './raw-schema-fourth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSIONS} from './raw-schema-fourth-extension-policy.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES, readOriginalLinkRawSchemaFifthContext, verifyOriginalLinkRawSchemaFifthExtensionClosure} from './raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SIXTH_SOURCE_FILES, readOriginalLinkRawSchemaSixthContext, verifyOriginalLinkRawSchemaSixthExtensionClosure} from './raw-schema-sixth-extension-closure.mjs';

const sha256 = value => createHash('sha256').update(value).digest('hex');
const root = new URL('../../', import.meta.url);
function fixture() {
  let bytes = readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url));
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
    const sources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]));
    verifyOriginalLinkRawSchemaFifthExtensionClosure(readOriginalLinkRawSchemaFifthContext(), bytes, fifth, sources);
    bytes = Buffer.from(fifth.previous.source, 'utf8');
  }
  return {input: readOriginalLinkRawSchemaFourthContext(),
    bytes,
    fourth: readStructuredDataLino(readFileSync(new URL('./data/raw-schema-fourth-extension-closure.lino', import.meta.url), 'utf8')),
    sources: Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]))};
}
const verify = value => verifyOriginalLinkRawSchemaFourthExtensionClosure(value.input, value.bytes, value.fourth, value.sources);

test('the fourth schema successor authenticates exact third bytes all predecessors and the sole optional query plan', () => {
  const value = fixture(), proof = verify(value), before = readStructuredDataLino(value.fourth.previous.source);
  const after = readStructuredDataLino(value.bytes.toString('utf8'));
  assert.equal(sha256(value.input.thirdBytes), ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT);
  assert.equal(value.fourth.previous.bytes, 141198);
  assert.equal(value.fourth.previous.sha256, '9308bcaf5635c128e7d899a512b535555231063ea495c4ab2504626da9580e18');
  assert.equal(sha256(value.bytes), '6f8a9f0223f620a3a85ba88f4bf093963f67b69236504afcf006f376783ac30d');
  assert.equal(Object.keys(value.input.historicalSources).length, 47);
  assert.equal(value.input.receipt.generation.definitionCount, 106);
  assert.equal(value.input.receipt.generation.completeRustBytes, 351746);
  for (const property of ORIGINAL_LINK_RAW_SCHEMA_PROPERTIES) assert.deepEqual(after[property], before[property]);
  assert.deepEqual(Object.keys(after.expressionProjections), [...Object.keys(before.expressionProjections), 'optionalTypeOf']);
  assert.deepEqual(after.expressionProjections.optionalTypeOf, ORIGINAL_LINK_RAW_SCHEMA_FOURTH_EXTENSIONS.expressionProjections.optionalTypeOf);
  assert.equal(proof.sha256, sha256(value.bytes));
  assert.equal(proof.thirdExtensionReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_THIRD_EXTENSION_RECEIPT);
  const verifyThird = bytes => verifyOriginalLinkRawSchemaThirdExtensionClosure(value.input.receipt, value.input.first,
    bytes, value.input.historicalSources, value.input.second, value.input.third, value.input.thirdSources);
  assert.doesNotThrow(() => verifyThird(Buffer.from(value.fourth.previous.source, 'utf8')));
  assert.throws(() => verifyThird(value.bytes), assert.AssertionError, 'the old strict profile is never widened');
});

test('coherent fourth-stage hashes cannot rewrite old plans or the derived optional type query projection', () => {
  const original = fixture();
  for (const mutate of [
    schema => {schema.fields.k = 'counterfeit';},
    schema => {schema.expressionOperands.optionalTypeOf = ['value'];},
    schema => {delete schema.expressionProjections.lambda;},
    schema => {schema.expressionProjections.optionalTypeOf.construct = 'toString';},
    schema => {schema.expressionProjections.optionalTypeOf.fields[0][2] = 'present';},
    schema => {schema.expressionProjections.optionalTypeOf.fields.push(['present', 'copy', 'present']);},
    schema => {schema.expressionProjections = Object.fromEntries(Object.entries(schema.expressionProjections).reverse());},
    schema => {schema.expressionProjections.extra = {};},
    schema => {schema.unexpected = {};},
  ]) {
    const value = {...original, fourth: structuredClone(original.fourth)};
    const schema = readStructuredDataLino(original.bytes.toString('utf8')); mutate(schema);
    value.bytes = Buffer.from(structuredDataToLino(schema));
    value.fourth.current.bytes = value.bytes.length; value.fourth.current.sha256 = sha256(value.bytes);
    assert.throws(() => verify(value), assert.AssertionError);
  }
});

test('fourth-stage receipts retain predecessor bytes every source binding actual author sources and scope', () => {
  const original = fixture();
  for (const mutate of [
    value => {value.input.receipt.generation.definitionCount--;},
    value => {value.input.historicalSources['policy.mjs'] = '0'.repeat(64);},
    value => {value.input.third.current.bytes--;},
    value => {value.input.thirdBytes = Buffer.concat([value.input.thirdBytes, Buffer.from('\n')]);},
    value => {value.fourth.previous.source += '\n';},
    value => {value.fourth.previous.bytes--;},
    value => {value.fourth.previous.sha256 = '0'.repeat(64);},
    value => {value.fourth.predecessorReceiptSha256 = '0'.repeat(64);},
    value => {value.fourth.consumedSchemaProperties.reverse();},
    value => {delete value.fourth.appendedSchemaEntries.expressionProjections.optionalTypeOf;},
    value => {value.fourth.authoring.command += ' --repair';},
    value => {value.fourth.authoring.sourceFiles.reverse();},
    value => {value.fourth.authoring.sourceFiles[0].sha256 = '0'.repeat(64);},
    value => {value.fourth.scope = 'native execution';},
    value => {const file = Object.keys(value.input.thirdSources)[0]; value.input.thirdSources[file] = Buffer.concat([value.input.thirdSources[file], Buffer.from('\n')]);},
    value => {const file = ORIGINAL_LINK_RAW_SCHEMA_FOURTH_SOURCE_FILES[0]; value.sources[file] = Buffer.concat([value.sources[file], Buffer.from('\n')]);},
  ]) {
    const value = {input: {...original.input, receipt: structuredClone(original.input.receipt),
      first: structuredClone(original.input.first), second: structuredClone(original.input.second), third: structuredClone(original.input.third),
      historicalSources: {...original.input.historicalSources}, thirdSources: {...original.input.thirdSources}, thirdBytes: Buffer.from(original.input.thirdBytes)},
      bytes: original.bytes, fourth: structuredClone(original.fourth), sources: {...original.sources}};
    mutate(value); assert.throws(() => verify(value), assert.AssertionError);
  }
});

test('the live fourth freshness check and archived third check leave every protected input unchanged', () => {
  const files = ['js/src/data/typed-program-links-schema.lino',
    ...['raw-schema-source-closure', 'raw-schema-extension-closure', 'raw-schema-second-extension-closure',
      'raw-schema-third-extension-closure', 'raw-schema-fourth-extension-closure', 'raw-schema-second-extension-provenance',
      'raw-schema-second-extension-provenance-0112b8e3'].map(name => 'experiments/original-link-complete-table-projection/data/' + name + '.lino'),
    'experiments/original-link-complete-table-projection/raw-native-observations.mjs',
    'experiments/original-link-complete-table-projection/raw-schema-third-extension-policy.mjs',
    'experiments/original-link-complete-table-projection/raw-schema-third-extension-closure.mjs',
    'js/scripts/extend-typed-program-schema-third-stage.mjs'];
  const before = files.map(file => readFileSync(new URL(file, root)));
  let result;
  if (existsSync(new URL('./data/raw-schema-fifth-extension-closure.lino', import.meta.url))) {
    assert.throws(() => extendTypedProgramSchemaFourthStage({check: true}), assert.AssertionError,
      'the frozen strict fourth writer refuses the live fifth profile');
    result = checkTypedProgramSchemaFourthArchive();
  } else result = extendTypedProgramSchemaFourthStage({check: true});
  assert.equal(result.check, true); assert.equal(result.sourceClosureOnly, true);
  assert.equal(result.appendedProjectionKeys, 1); assert.equal(result.preservedProducerBindings, 47);
  const third = checkTypedProgramSchemaThirdArchive();
  assert.equal(third.schemaSha256, '9308bcaf5635c128e7d899a512b535555231063ea495c4ab2504626da9580e18');
  assert.equal(third.appendedProjectionKeys, 3); assert.equal(third.appendedAlternativeKeys, 1);
  files.forEach((file, index) => assert.deepEqual(readFileSync(new URL(file, root)), before[index]));
});
