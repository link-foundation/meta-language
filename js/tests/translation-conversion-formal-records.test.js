import assert from 'node:assert/strict';
import { test } from 'node:test';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';

import {
  createClosedRecordConversionDefinition, proveClosedRecordConversionEquivalence,
  readClosedRecordConversionDefinition, verifyClosedRecordConversionEquivalence,
} from '../../experiments/issue-199-conversion-formal-records.mjs';
import { prepareRecordTextDraft, recordTextModels } from '../../experiments/issue-199-record-text-draft.mjs';
import { Grammar } from '../src/grammar.js';
import { createGlobalConceptSpace, globalConceptSpaceToLinks, readGlobalConceptSpaceLinks } from '../src/global-concept-space.js';
import { LinkNetwork } from '../src/network.js';
import { emitTypedProgramLinks } from '../src/typed-program-links.js';

const reload = ({ network, root }) => ({ links: network.toLino(), root });
const networkArchive = value => ({ network: value.network ?? LinkNetwork.fromLino(value.links), root: value.root });
const define = (record, model) => createClosedRecordConversionDefinition(prepareRecordTextDraft(record), model);
function evaluate(value) {
  const { network, root } = networkArchive(value);
  const body = network.link(root).references()[0];
  const scope = {};
  runInNewContext(emitTypedProgramLinks(network, body, 'JavaScript').code, scope, { timeout: 1_000 });
  return scope.readRecordConversionMeaning();
}
function mutateScalar(value, from, to) {
  const result = networkArchive(reload(value));
  const links = result.network.fieldRelations().filter(({ label, child }) => label === 'value' && result.network.link(child).metadata().definition === from);
  assert.ok(links.length, `missing semantic scalar ${from}`);
  for (const { child } of links) {
    const link = result.network.link(child);
    link.setMetadata(link.metadata().withDefinition(to));
  }
  return result;
}

test('closed recursive record definitions retain distinct executable model premises after source-free reload', () => {
  const records = [{ left: 1 }, { right: 1 }];
  const definitions = records.map(record => recordTextModels.map(model => define(record, model)));
  for (const [index, record] of records.entries()) for (const [modelIndex, model] of recordTextModels.entries()) {
    const definition = definitions[index][modelIndex];
    const reloaded = reload(definition);
    assert.deepEqual(readClosedRecordConversionDefinition(reloaded), { model, tape: prepareRecordTextDraft(record) });
    assert.equal(evaluate(reloaded), modelIndex === 0 ? String(record) : format(record));
    assert.ok(definition.network.fieldRelations().some(({ label, child }) => label === 'declaration reference'
      && definition.network.fieldRelations().some(relation => relation.parent.equals(child)
        && relation.label === 'name' && definition.network.link(relation.child).metadata().definition === 'renderRecordNode')),
    'the recursive renderer has actual declaration references');
    assert.ok(!reloaded.links.includes('export function'), 'the definition stores semantic links rather than source text');
  }
  assert.equal(evaluate(reload(definitions[0][0])), evaluate(reload(definitions[1][0])), 'ordinary object coercion does not inspect the field names');
  assert.notEqual(evaluate(reload(definitions[0][1])), evaluate(reload(definitions[1][1])), 'Node inspection exposes the different public field names');
  assert.equal(proveClosedRecordConversionEquivalence(definitions[0][0], definitions[0][1]), null, 'a model substitution requires a new proof');
  assert.equal(proveClosedRecordConversionEquivalence(definitions[0][0], definitions[1][0]), null,
    'this structural verifier keeps an unknown result for observationally equal definitions with different closed inputs');
});

test('closed model and algorithm premises constrain checked correspondence certificates', () => {
  const definition = define({ left: 1 }, recordTextModels[1]);
  const reloaded = reload(definition);
  const proof = proveClosedRecordConversionEquivalence(definition, reloaded);
  assert.ok(proof);
  assert.equal(verifyClosedRecordConversionEquivalence(reloaded, reloaded, reload(proof)), true);
  const changedModel = mutateScalar(definition, recordTextModels[1], recordTextModels[0]);
  assert.equal(evaluate(changedModel), '[object Object]');
  assert.equal(verifyClosedRecordConversionEquivalence(definition, changedModel, proof), false);
  const unknown = mutateScalar(definition, recordTextModels[1], 'undeclared record interpretation');
  assert.throws(() => readClosedRecordConversionDefinition(unknown), /declared conversion model/u);
  assert.equal(proveClosedRecordConversionEquivalence(unknown, unknown), null);
  assert.equal(verifyClosedRecordConversionEquivalence(unknown, unknown, proof), false);
  const missing = networkArchive(reload(definition));
  const relations = missing.network.fieldRelations();
  const declaration = relations.find(({ label, child }) => label === 'name'
    && missing.network.link(child).metadata().definition === 'readRecordConversionMeaning').parent;
  const body = relations.find(({ parent, label }) => parent.equals(declaration) && label === 'body').child;
  const argumentsLink = missing.network.link(relations.find(({ parent, label }) => parent.equals(body) && label === 'arguments').child);
  argumentsLink.setReferences(argumentsLink.references().slice(0, -1));
  assert.throws(() => readClosedRecordConversionDefinition(missing), /wrong number of arguments/u);
  assert.equal(proveClosedRecordConversionEquivalence(missing, missing), null, 'a removed model premise leaves no complete conversion definition');
  const alteredAlgorithm = mutateScalar(definition, '[object Object]', '[counterfeit Object]');
  assert.throws(() => readClosedRecordConversionDefinition(alteredAlgorithm), /supported recursive algorithm/u);
  assert.equal(proveClosedRecordConversionEquivalence(alteredAlgorithm, alteredAlgorithm), null,
    'identical counterfeit algorithms do not establish the declared conversion basis');
});

test('global concept identity requires the complete formal conversion premise, beyond equal grammar syntax', () => {
  const ordinary = define({ left: 1 }, recordTextModels[0]);
  const console = define({ left: 1 }, recordTextModels[1]);
  const source = id => ({ id, language: id, grammar: new Grammar('conversion', [
    ['conversion', { kind: 'normal', expression: { kind: 'literal', value: 'record' } }],
  ]) });
  const space = createGlobalConceptSpace(['ordinary-first', 'ordinary-second', 'console'].map(source), {
    records: [], formalDefinitions: {
      'ordinary-first:conversion': ordinary, 'ordinary-second:conversion': reload(ordinary), 'console:conversion': console,
    },
    equivalences: [['ordinary-first:conversion', 'ordinary-second:conversion'], ['ordinary-first:conversion', 'console:conversion']],
  });
  assert.equal(space.mappings['ordinary-first:conversion'], space.mappings['ordinary-second:conversion']);
  assert.notEqual(space.mappings['ordinary-first:conversion'], space.mappings['console:conversion']);
  const links = globalConceptSpaceToLinks(space);
  const reloaded = readGlobalConceptSpaceLinks(links, { records: [] });
  assert.deepEqual(reloaded, space);
  assert.equal(globalConceptSpaceToLinks(reloaded), links);
  assert.equal(reloaded.proofs.length, 1);
  assert.equal(reloaded.failures.length, 1, 'the unproved cross-model candidate remains inspectable');
});
