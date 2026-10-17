import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT} from './raw-schema-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIRST_EXTENSION_RECEIPT, verifyOriginalLinkRawSchemaSecondDraftPredecessor, verifyOriginalLinkRawSchemaSecondExtensionClosure} from './raw-schema-second-extension-closure.mjs';

const sha256 = value => createHash('sha256').update(value).digest('hex');
function fixture() {
  const base = new URL('./data/', import.meta.url);
  const receiptBytes = readFileSync(new URL('raw-schema-source-closure.lino', base));
  const firstBytes = readFileSync(new URL('raw-schema-extension-closure.lino', base));
  return {
    receiptBytes, firstBytes,
    receipt: readStructuredDataLino(receiptBytes.toString('utf8')),
    first: readStructuredDataLino(firstBytes.toString('utf8')),
    second: readStructuredDataLino(readFileSync(new URL('raw-schema-second-extension-closure.lino', base), 'utf8')),
    bytes: readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url)),
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
