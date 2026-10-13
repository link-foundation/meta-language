// Build actual semantic link fixtures from independently asserted structured
// statements. Project fixture metadata uses the shared Links data codec.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { importResourceDescriptionGraph, decodeResourceDescriptionGraph, mergeResourceDescriptionGraphs } from '../src/resource-description-graphs.js';
import { serializeProjectData } from './project-data.mjs';

const identifier = value => ({ kind: 'identifier', value });
const literal = (value, datatype, language) => ({ kind: 'literal', value,
  ...(datatype === undefined ? {} : { datatype }), ...(language === undefined ? {} : { language }) });
const statement = (subject, object) => [subject, identifier('urn:property'), object];
const integer = 'http://www.w3.org/2001/XMLSchema#integer';
const inputs = [
  { sourceIdentifier: 'source A', triples: [statement({ kind: 'blank', value: 'same' }, literal('1', integer)),
    [identifier('urn:left'), identifier('http://www.w3.org/2002/07/owl#sameAs'), identifier('urn:right')],
    statement(identifier('urn:subject'), literal('same')), statement(identifier('urn:subject'), literal('same'))] },
  { sourceIdentifier: 'source B', triples: [statement({ kind: 'blank', value: 'same' }, literal('01', integer)),
    statement(identifier('urn:subject'), literal('same')), statement(identifier('urn:unicode:🦀'), literal('空\0\n', undefined, 'EN-gb'))] },
];
const graphs = inputs.map(input => importResourceDescriptionGraph(input.triples, input));
const merged = mergeResourceDescriptionGraphs(graphs);
const decoded = decodeResourceDescriptionGraph(merged.network, merged.root);
assert.equal(decoded.triples.length, 5);
assert.equal(decoded.occurrences.length, 7);
assert.deepEqual(decoded.occurrences.map(value => [value.sourceIdentifier, value.position]),
  [['source A', 0], ['source A', 1], ['source A', 2], ['source A', 3], ['source B', 0], ['source B', 1], ['source B', 2]]);
assert.deepEqual(decoded.triples.filter(triple => triple[0].kind === 'blank').map(triple =>
  [triple[0].sourceIdentifier, triple[2].value]).sort(), [['source A', '1'], ['source B', '01']]);
assert.equal(decoded.triples.find(triple => triple[1].value.endsWith('#sameAs'))[2].value, 'urn:right');
assert.equal(decoded.triples.find(triple => triple[0].value === 'urn:unicode:🦀')[2].language, 'en-gb');
const outputs = [
  ['resource-description-graph.lino', merged.network.toLino()],
  ['resource-description-graph-expectations.lino', serializeProjectData({ root: Number(merged.root.asU64()), inputs, decoded }, 'resource-description-graph-expectations.lino')],
];
for (const [name, value] of outputs) {
  const file = new URL('../../parity/fixtures/' + name, import.meta.url);
  if (process.argv.includes('--check')) assert.equal(readFileSync(file, 'utf8'), value, 'a shared graph fixture is stale');
  else writeFileSync(file, value);
}
console.log('resource description fixture: 5 exact statements, 7 ordered source occurrences');
