import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, verifyOriginalLinkRawSchemaSecondDraftPredecessor, verifyOriginalLinkRawSchemaSecondExtensionClosure} from './raw-schema-second-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT} from './raw-schema-third-extension-closure.mjs';

const sha256 = value => createHash('sha256').update(value).digest('hex');
function fixture() {
  const base = new URL('./data/', import.meta.url);
  const receiptBytes = readFileSync(new URL('raw-schema-source-closure.lino', base));
  const firstBytes = readFileSync(new URL('raw-schema-extension-closure.lino', base));
  const secondBytes = readFileSync(new URL('raw-schema-second-extension-closure.lino', base));
  assert.equal(sha256(secondBytes), ORIGINAL_LINK_RAW_SCHEMA_SECOND_EXTENSION_RECEIPT);
  const thirdFile = new URL('raw-schema-third-extension-closure.lino', base);
  // The successor archives the exact published stage-two schema. These tests
  // retain their original complete stage-two admission and refusal boundary.
  const bytes = existsSync(thirdFile)
    ? Buffer.from(readStructuredDataLino(readFileSync(thirdFile, 'utf8')).previous.source, 'utf8')
    : readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url));
  return {
    receiptBytes, firstBytes,
    receipt: readStructuredDataLino(receiptBytes.toString('utf8')),
    first: readStructuredDataLino(firstBytes.toString('utf8')),
    second: readStructuredDataLino(secondBytes.toString('utf8')),
    bytes,
    sources: readOriginalLinkRawHistoricalSources(readFileSync(new URL('./raw-native-observations.mjs', import.meta.url), 'utf8')),
  };
}

test('stage two authenticates both immutable predecessor receipts and every original producer input', () => {
  const value = fixture();
  assert.equal(sha256(value.receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(sha256(value.firstBytes), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  const result = verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, value.bytes, value.sources, value.second);
  assert.equal(result.sha256, sha256(value.bytes));
  assert.equal(result.firstExtensionSha256, value.first.current.sha256);
  assert.equal(Object.keys(value.sources).length, 47);
  assert.equal(value.receipt.generation.definitionCount, 106);
  assert.match(value.second.scope, /source closure only; no native execution/u);
  const historical = readStructuredDataLino(readFileSync(new URL('./data/raw-schema-second-extension-provenance.lino', import.meta.url), 'utf8'));
  assert.equal(historical.version, 1);
  assert.equal(historical.publishedCommit, '0e120757b5ee6cc677a22be97007bd5d4c0eabc7');
  assert.equal(historical.originalReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(historical.firstExtensionReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  const historicalSchemaBytes = Buffer.from(historical.schema.source, 'utf8');
  const historicalReceiptBytes = Buffer.from(historical.receipt.source, 'utf8');
  assert.equal(historical.schema.file, 'js/src/data/typed-program-links-schema.lino');
  assert.equal(historical.receipt.file, 'experiments/original-link-complete-table-projection/data/raw-schema-second-extension-closure.lino');
  assert.equal(historicalSchemaBytes.length, 134305);
  assert.equal(historicalReceiptBytes.length, 215894);
  assert.equal(historical.schema.bytes, historicalSchemaBytes.length);
  assert.equal(historical.receipt.bytes, historicalReceiptBytes.length);
  assert.equal(sha256(historicalSchemaBytes), '7705e8e0da784d2ac77c75c7eb328597de08972adff93621d3bcc9ed28671d4f');
  assert.equal(sha256(historicalReceiptBytes), '6dd312d713ae528d364ea2cd3e1e4118e70e1c9cc628e3d46fe3c78c4dc25deb');
  assert.equal(historical.schema.sha256, sha256(historicalSchemaBytes));
  assert.equal(historical.receipt.sha256, sha256(historicalReceiptBytes));
  const historicalSecond = readStructuredDataLino(historical.receipt.source);
  assert.equal(historicalSecond.current.sha256, historical.schema.sha256);
  assert.equal(historicalSecond.current.bytes, historical.schema.bytes);
  assert.doesNotThrow(() => verifyOriginalLinkRawSchemaSecondDraftPredecessor(value.receipt, value.first, historicalSchemaBytes, value.sources, historicalSecond));
});

test('coherent second-stage hashes cannot modify old plans roles key order or new declared decisions', () => {
  for (const mutate of [
    schema => {schema.fields.k = 'counterfeit';},
    schema => {schema.expressionOperands.referenceRead = ['value'];},
    schema => {schema.expressionProjections.index.fields.push(['provenInBounds', 'copy', 'provenInBounds']);},
    schema => {delete schema.expressionProjections.lambda;},
    schema => {schema.expressionProjectionAlternatives.length[0].projection = 'index';},
    schema => {schema.expressionProjections.method.fields[2][1] = 'expressions';},
    schema => {schema.typeChildPlans.union.preserveDuplicates = false;},
    schema => {schema.expressionProjections = Object.fromEntries(Object.entries(schema.expressionProjections).reverse());},
    schema => {schema.unexpectedPlan = {};},
  ]) {
    const value = fixture();
    const schema = readStructuredDataLino(value.bytes.toString('utf8'));
    mutate(schema);
    const bytes = Buffer.from(structuredDataToLino(schema));
    value.second.current.bytes = bytes.length;
    value.second.current.sha256 = sha256(bytes);
    assert.throws(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, bytes, value.sources, value.second), assert.AssertionError);
  }
});

test('stage-two receipts cannot relabel predecessor sources receipts bindings or scope', () => {
  for (const mutate of [
    value => {value.first.current.sha256 = '0'.repeat(64);},
    value => {value.second.previous.source += '\n';},
    value => {value.second.previous.bytes--;},
    value => {value.second.previous.sha256 = '0'.repeat(64);},
    value => {value.second.predecessorReceiptSha256 = '0'.repeat(64);},
    value => {value.second.consumedSchemaProperties.reverse();},
    value => {delete value.second.appendedSchemaEntries.expressionProjections.cellRead;},
    value => {value.second.scope = 'native execution';},
    value => {value.receipt.generation.definitionCount--;},
    value => {value.sources['policy.mjs'] = '0'.repeat(64);},
  ]) {
    const value = fixture(); mutate(value);
    assert.throws(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, value.bytes, value.sources, value.second), assert.AssertionError);
  }
});

test('draft predecessor admission retains old entries while the final gate still requires every declared role', () => {
  const value = fixture();
  const previous = structuredClone(value.second);
  const schema = readStructuredDataLino(value.bytes.toString('utf8'));
  delete previous.appendedSchemaEntries.declarationTypeProjections;
  delete schema.declarationTypeProjections;
  const bytes = Buffer.from(structuredDataToLino(schema));
  previous.current.bytes = bytes.length;
  previous.current.sha256 = sha256(bytes);
  assert.doesNotThrow(() => verifyOriginalLinkRawSchemaSecondDraftPredecessor(value.receipt, value.first, bytes, value.sources, previous));
  assert.throws(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, bytes, value.sources, previous), assert.AssertionError);
  previous.appendedSchemaEntries.expressionProjections.cellRead.fields[0][2] = 'value';
  assert.throws(() => verifyOriginalLinkRawSchemaSecondDraftPredecessor(value.receipt, value.first, bytes, value.sources, previous), /redefine an old draft entry/u);
});

function verifyCommittedSecondSchemaProvenance(historical, value) {
  assert.equal(historical.version, 1);
  assert.equal(historical.sourceCommit, '0112b8e364f3f2f5945dc738552cd75af0111866');
  assert.equal(Object.hasOwn(historical, 'publishedCommit'), false);
  assert.equal(historical.originalReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(historical.firstExtensionReceiptSha256, ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
  const schemaBytes = Buffer.from(historical.schema.source, 'utf8'), receiptBytes = Buffer.from(historical.receipt.source, 'utf8');
  assert.equal(historical.schema.file, 'js/src/data/typed-program-links-schema.lino');
  assert.equal(historical.receipt.file, 'experiments/original-link-complete-table-projection/data/raw-schema-second-extension-closure.lino');
  assert.equal(schemaBytes.length, 135107);assert.equal(historical.schema.bytes, schemaBytes.length);
  assert.equal(receiptBytes.length, 216801);assert.equal(historical.receipt.bytes, receiptBytes.length);
  assert.equal(sha256(schemaBytes), '0f25aad3f91f511144dc3a1f974027d10d13446e15b4bd1cb86aa61a50197ece');
  assert.equal(sha256(receiptBytes), 'e8312a7334e1a8642386e7e0bc370689879c789a33064076090e1b4872472c2b');
  assert.equal(historical.schema.sha256, sha256(schemaBytes));assert.equal(historical.receipt.sha256, sha256(receiptBytes));
  const second = readStructuredDataLino(historical.receipt.source);
  assert.equal(second.current.sha256, historical.schema.sha256);assert.equal(second.current.bytes, schemaBytes.length);
  assert.deepEqual(second.consumedSchemaProperties, ['language', 'root', 'schemaVersion', 'containers', 'scalars', 'fields', 'references', 'referenceAliases']);
  assert.equal(Object.keys(value.sources).length, 47);
  verifyOriginalLinkRawSchemaSecondDraftPredecessor(value.receipt, value.first, schemaBytes, value.sources, second);
  return {schemaBytes, receiptBytes, second};
}

test('committed stage-two provenance preserves the exact 0112 schema and receipt before optional text projections', () => {
  const value = fixture(), file = new URL('./data/raw-schema-second-extension-provenance-0112b8e3.lino', import.meta.url);
  const bytes = readFileSync(file);
  assert.equal(bytes.length, 527360);
  assert.equal(sha256(bytes), '28635e6c1c02acc5878ff38d610ceba432082901461c2801ebcf2c410fa10710');
  const historical = readStructuredDataLino(bytes.toString('utf8'));
  assert.equal(structuredDataToLino(historical), bytes.toString('utf8'), 'every historical source byte survives actual Links reading and writing');
  const {schemaBytes, second} = verifyCommittedSecondSchemaProvenance(historical, value);
  const previous = readStructuredDataLino(schemaBytes.toString('utf8')), current = readStructuredDataLino(value.bytes.toString('utf8'));
  assert.deepEqual(previous.expressionProjections.nullishNumber, {construct: 'numberValue', fields: [['arg', 'expression', 'arg']]});
  for (const name of ['optionalText', 'optionalText:false', 'optionalText:true']) {
    assert.equal(Object.hasOwn(previous.expressionProjections, name), false);
    assert.equal(Object.hasOwn(second.appendedSchemaEntries.expressionProjections, name), false);
    assert.equal(Object.hasOwn(current.expressionProjections, name), true);
  }
  for (const key of second.consumedSchemaProperties) assert.deepEqual(current[key], previous[key], 'a raw producer input remains exact: ' + key);
  assert.throws(() => verifyOriginalLinkRawSchemaSecondExtensionClosure(value.receipt, value.first, schemaBytes, value.sources, second), assert.AssertionError,
    'the narrower historical predecessor cannot replace the complete current second-stage gate');
  const oldBytes = readFileSync(new URL('./data/raw-schema-second-extension-provenance.lino', import.meta.url));
  assert.equal(oldBytes.length, 524605);
  assert.equal(sha256(oldBytes), 'e11957cb88af9d80fda7a4ce97f40ffb81434fa8e684a2f697a55f3f35c3fb03');
  assert.equal(sha256(value.receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  assert.equal(sha256(value.firstBytes), ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT);
});

test('committed stage-two provenance rejects publication relabeling and coherently changed predecessor bytes', () => {
  const value = fixture(), original = readStructuredDataLino(readFileSync(new URL('./data/raw-schema-second-extension-provenance-0112b8e3.lino', import.meta.url), 'utf8'));
  for (const mutate of [
    historical => {historical.sourceCommit = '0'.repeat(40);},
    historical => {historical.publishedCommit = historical.sourceCommit;},
    historical => {historical.originalReceiptSha256 = '0'.repeat(64);},
    historical => {historical.receipt.file = historical.schema.file;},
    historical => {historical.schema.source += '\n';historical.schema.bytes = Buffer.byteLength(historical.schema.source);historical.schema.sha256 = sha256(historical.schema.source);},
    historical => {historical.receipt.source += '\n';historical.receipt.bytes = Buffer.byteLength(historical.receipt.source);historical.receipt.sha256 = sha256(historical.receipt.source);},
  ]) {
    const changed = structuredClone(original);mutate(changed);
    assert.throws(() => verifyCommittedSecondSchemaProvenance(changed, value), assert.AssertionError);
  }
  const foreign = {...value, sources: {...value.sources, 'policy.mjs': '0'.repeat(64)}};
  assert.throws(() => verifyCommittedSecondSchemaProvenance(original, foreign), assert.AssertionError);
});
