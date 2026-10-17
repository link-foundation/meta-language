import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import test from 'node:test';
import {readStructuredDataLino, structuredDataToLino} from '../../js/src/structured-data-links.js';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES, readOriginalLinkRawSchemaFifthContext,
  verifyOriginalLinkRawSchemaFifthExtensionClosure} from './raw-schema-fifth-extension-closure.mjs';
import {ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS} from './raw-schema-fifth-extension-policy.mjs';

const root = new URL('../../', import.meta.url);
const input = readOriginalLinkRawSchemaFifthContext();
const fifth = readStructuredDataLino(readFileSync(new URL('./data/raw-schema-fifth-extension-closure.lino', import.meta.url), 'utf8'));
const currentBytes = readFileSync(new URL('../../js/src/data/typed-program-links-schema.lino', import.meta.url));
const currentSchema = readStructuredDataLino(currentBytes.toString('utf8'));
const authoredSources = Object.fromEntries(ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES.map(file => [file, readFileSync(new URL(file, root))]));

test('the fifth source closure preserves every predecessor and exactly the two collection projections', () => {
  const result = verifyOriginalLinkRawSchemaFifthExtensionClosure(input, currentBytes, fifth, authoredSources);
  assert.equal(result.sha256, fifth.current.sha256);
  assert.deepEqual(fifth.appendedSchemaEntries, {
    expressionProjections: {
      set: {construct: 'set', fields: [['items', 'expression', 'items'], ['ty', 'copy', 'type']]},
      map: {construct: 'map', fields: [['entries', 'expression', 'entries'], ['ty', 'copy', 'type']]},
    },
  });
  const before = readStructuredDataLino(fifth.previous.source);
  assert.deepEqual(Object.keys(currentSchema.expressionProjections), [...Object.keys(before.expressionProjections), 'set', 'map']);
  for (const kind of ['set', 'map']) assert.deepEqual(currentSchema.expressionProjections[kind], ORIGINAL_LINK_RAW_SCHEMA_FIFTH_EXTENSIONS.expressionProjections[kind]);
  assert.equal(currentSchema.schemaVersion, before.schemaVersion);
  assert.equal(currentSchema.language, before.language);
  assert.equal(currentSchema.root, before.root);
  assert.deepEqual(currentSchema.expressionOperands, before.expressionOperands);
  assert.deepEqual(currentSchema.fields, before.fields);
});

test('each collection operand and complete type field must keep its exact projection instruction', () => {
  for (const kind of ['set', 'map']) {
    for (const [field, position, replacement] of [
      [0, 0, 'arg'], [0, 1, 'copy'], [0, 2, 'arg'],
      [1, 0, 'type'], [1, 1, 'expression'], [1, 2, 'ret'],
    ]) {
      const schema = structuredClone(currentSchema), receipt = structuredClone(fifth);
      schema.expressionProjections[kind].fields[field][position] = replacement;
      const bytes = Buffer.from(structuredDataToLino(schema));
      receipt.current.bytes = bytes.length;
      receipt.current.sha256 = createHash('sha256').update(bytes).digest('hex');
      assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, bytes, receipt, authoredSources),
        `${kind} field ${field} instruction ${position} cannot change even with a fresh complete byte hash`);
      receipt.appendedSchemaEntries.expressionProjections[kind] = structuredClone(schema.expressionProjections[kind]);
      assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, bytes, receipt, authoredSources),
        `${kind} cannot redefine its policy by coherently editing both the schema and receipt`);
    }
  }
});

test('missing reordered and extra collection projections cannot disguise an append', () => {
  for (const kind of ['set', 'map']) {
    for (const change of ['missing-projection', 'missing-operand', 'missing-type', 'reordered-fields']) {
      const schema = structuredClone(currentSchema), receipt = structuredClone(fifth);
      if (change === 'missing-projection') delete schema.expressionProjections[kind];
      else if (change === 'missing-operand') schema.expressionProjections[kind].fields.splice(0, 1);
      else if (change === 'missing-type') schema.expressionProjections[kind].fields.splice(1, 1);
      else schema.expressionProjections[kind].fields.reverse();
      const bytes = Buffer.from(structuredDataToLino(schema));
      receipt.current.bytes = bytes.length;
      receipt.current.sha256 = createHash('sha256').update(bytes).digest('hex');
      assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, bytes, receipt, authoredSources), `${kind}: ${change}`);
    }
  }
  for (const change of ['reordered-appends', 'extra-projection']) {
    const schema = structuredClone(currentSchema), receipt = structuredClone(fifth);
    if (change === 'reordered-appends') {
      const {set, map, ...before} = schema.expressionProjections;
      schema.expressionProjections = {...before, map, set};
    } else schema.expressionProjections.extraCollection = structuredClone(schema.expressionProjections.set);
    const bytes = Buffer.from(structuredDataToLino(schema));
    receipt.current.bytes = bytes.length;
    receipt.current.sha256 = createHash('sha256').update(bytes).digest('hex');
    assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, bytes, receipt, authoredSources), change);
  }
});

test('the finite inverse rejects altered old plans and model identities despite coherent new hashes', () => {
  for (const change of ['old-plan', 'old-plan-order', 'schema-version', 'language', 'root']) {
    const schema = structuredClone(currentSchema), receipt = structuredClone(fifth);
    if (change === 'old-plan') schema.expressionProjections.optionalTypeOf.construct = 'var';
    else if (change === 'old-plan-order') {
      const entries = Object.entries(schema.expressionProjections);
      [entries[0], entries[1]] = [entries[1], entries[0]];
      schema.expressionProjections = Object.fromEntries(entries);
    } else if (change === 'schema-version') schema.schemaVersion += 1;
    else if (change === 'language') schema.language += ' changed';
    else schema.root += ' changed';
    const bytes = Buffer.from(structuredDataToLino(schema));
    receipt.current.bytes = bytes.length;
    receipt.current.sha256 = createHash('sha256').update(bytes).digest('hex');
    assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, bytes, receipt, authoredSources), change);
  }
});

test('predecessor receipts author source bindings and closure identity remain independently authenticated', () => {
  for (const change of ['predecessor-pin', 'previous-source', 'previous-hash', 'current-hash', 'version', 'scope', 'command', 'author-order', 'author-path', 'author-bytes', 'author-hash']) {
    const receipt = structuredClone(fifth);
    if (change === 'predecessor-pin') receipt.predecessorReceiptSha256 = '0'.repeat(64);
    else if (change === 'previous-source') receipt.previous.source += '\n';
    else if (change === 'previous-hash') receipt.previous.sha256 = '0'.repeat(64);
    else if (change === 'current-hash') receipt.current.sha256 = '0'.repeat(64);
    else if (change === 'version') receipt.version += 1;
    else if (change === 'scope') receipt.scope += ' changed';
    else if (change === 'command') receipt.authoring.command += ' --unreviewed';
    else if (change === 'author-order') receipt.authoring.sourceFiles.reverse();
    else if (change === 'author-path') receipt.authoring.sourceFiles[0].file += '.changed';
    else if (change === 'author-bytes') receipt.authoring.sourceFiles[0].bytes += 1;
    else receipt.authoring.sourceFiles[0].sha256 = '0'.repeat(64);
    assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, currentBytes, receipt, authoredSources), change);
  }
  const changedSources = {...authoredSources};
  const first = ORIGINAL_LINK_RAW_SCHEMA_FIFTH_SOURCE_FILES[0];
  changedSources[first] = Buffer.concat([authoredSources[first], Buffer.from('\n')]);
  assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(input, currentBytes, fifth, changedSources));
  const changedInput = {...input, fourthBytes: Buffer.concat([input.fourthBytes, Buffer.from('\n')])};
  assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(changedInput, currentBytes, fifth, authoredSources));
  const changedFourth = {...input, fourth: structuredClone(input.fourth)};
  changedFourth.fourth.current.sha256 = '0'.repeat(64);
  assert.throws(() => verifyOriginalLinkRawSchemaFifthExtensionClosure(changedFourth, currentBytes, fifth, authoredSources));
});
