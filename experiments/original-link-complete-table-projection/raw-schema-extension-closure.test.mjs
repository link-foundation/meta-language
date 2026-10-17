// Source-only controls: no grammar imports, matrix planning or native process.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync, readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {readOriginalLinkRawHistoricalSources} from './raw-schema-source-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT, compareOriginalLinkRawSchemaExtension, verifyOriginalLinkRawSchemaExtensionClosure} from './raw-schema-extension-closure.mjs';

const sha256 = value => createHash('sha256').update(value).digest('hex');
function fixture() {
  const receiptBytes = readFileSync(new URL('./data/raw-schema-source-closure.lino', import.meta.url));
  const secondFile = new URL('./data/raw-schema-second-extension-closure.lino', import.meta.url);
  // Continue testing stage one's exact boundary after a separately checked
  // stage two exists. Its authenticated previous source is stage one's schema.
  const bytes = existsSync(secondFile)
    ? Buffer.from(readStructuredDataLino(readFileSync(secondFile, 'utf8')).previous.source)
    : readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url));
  return {
    receiptBytes,
    receipt: readStructuredDataLino(receiptBytes.toString('utf8')),
    extension: readStructuredDataLino(readFileSync(new URL('./data/raw-schema-extension-closure.lino', import.meta.url), 'utf8')),
    bytes,
    sources: readOriginalLinkRawHistoricalSources(readFileSync(new URL('./raw-native-observations.mjs', import.meta.url), 'utf8')),
  };
}

test('the schema extension authenticates the unchanged complete predecessor receipt and original producer inputs', () => {
  const {receiptBytes, receipt, extension, bytes, sources} = fixture();
  assert.equal(sha256(receiptBytes), ORIGINAL_LINK_RAW_SCHEMA_PREDECESSOR_RECEIPT);
  const result = verifyOriginalLinkRawSchemaExtensionClosure(receipt, bytes, sources, extension);
  assert.equal(result.predecessorSha256, receipt.current.sha256);
  assert.equal(result.sha256, sha256(bytes));
  assert.equal(Object.keys(sources).length, 47);
  assert.equal(receipt.generation.definitionCount, 106);
  assert.equal(receipt.generation.completeRustBytes, 351746);
  assert.deepEqual(readFileSync(new URL('./data/raw-schema-source-closure.lino', import.meta.url)), receiptBytes);
  assert.match(extension.scope, /source closure only; no native execution/u);
});

test('coherent extension hashes cannot redefine remove reorder or add undeclared old schema data', () => {
  const changes = [
    schema => {schema.fields.k = 'counterfeit kind';},
    schema => {delete schema.expressionProjections.var;},
    schema => {schema.expressionOperands.referenceRead = ['value'];},
    schema => {schema.expressionProjections = Object.fromEntries(Object.entries(schema.expressionProjections).reverse());},
    schema => {schema.expressionProjections.referenceRead.operand = 'value';},
    schema => {schema.expressionOperandEntries.record.entries = 'name';},
    schema => {schema.expressionFields.record.push('extra');},
    schema => {schema.expressionProjections.extra = {construct: 'bool', fields: []};},
    schema => {delete schema.propositionOperands;},
    schema => {schema.unusedExtra = {};},
    schema => Object.fromEntries(Object.entries(schema).reverse()),
  ];
  for (const change of changes) {
    const {receipt, extension, bytes, sources} = fixture();
    const data = readStructuredDataLino(bytes.toString('utf8'));
    const counterfeit = Buffer.from(structuredDataToLino(change(data) ?? data));
    extension.current.bytes = counterfeit.length;
    extension.current.sha256 = sha256(counterfeit);
    assert.throws(() => verifyOriginalLinkRawSchemaExtensionClosure(receipt, counterfeit, sources, extension), assert.AssertionError);
  }
});

test('extension receipts cannot forge predecessor bytes counts source bindings or the declared append plan', () => {
  const changes = [
    value => {value.extension.previous.source += '\n';},
    value => {value.extension.previous.bytes--;},
    value => {value.extension.previous.sha256 = '0'.repeat(64);},
    value => {value.extension.previous.commit = '0'.repeat(40);},
    value => {value.extension.predecessorReceiptSha256 = '0'.repeat(64);},
    value => {value.extension.consumedSchemaProperties.reverse();},
    value => {delete value.extension.appendedSchemaEntries.expressionProjections.referenceRead;},
    value => {value.receipt.generation.definitionCount--;},
    value => {value.receipt.generation.stdout += 'invented';},
    value => {value.sources['policy.mjs'] = '0'.repeat(64);},
    value => {value.extension.current.bytes--;},
    value => {value.extension.current.sha256 = '0'.repeat(64);},
  ];
  for (const change of changes) {
    const value = fixture(); change(value);
    assert.throws(() => verifyOriginalLinkRawSchemaExtensionClosure(value.receipt, value.bytes, value.sources, value.extension), assert.AssertionError);
  }
});

test('schema append comparison preserves old aliases and distinguishes new identity mergers', () => {
  const shared = {value: 1};
  assert.throws(() => compareOriginalLinkRawSchemaExtension({first: shared, second: shared}, {first: {value: 1}, second: {value: 1}}, {}), /alias split/u);
  const merged = {value: 1};
  assert.throws(() => compareOriginalLinkRawSchemaExtension({first: {value: 1}, second: {value: 1}}, {first: merged, second: merged}, {}), /objects merged/u);
  const original = {plans: {old: {fields: ['value']}}};
  const additions = {plans: {next: {fields: ['body']}}};
  assert.doesNotThrow(() => compareOriginalLinkRawSchemaExtension(original, {plans: {old: {fields: ['value']}, next: {fields: ['body']}}}, additions));
  assert.throws(() => compareOriginalLinkRawSchemaExtension(original, {plans: {next: {fields: ['body']}, old: {fields: ['value']}}}, additions), /append order/u);
});
