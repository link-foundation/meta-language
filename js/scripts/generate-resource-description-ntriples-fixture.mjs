// Independent structured expectations and syntax inputs; archives are emitted
// automatically. No fixture is an observation for the full RDF/OWL obligation.
import { readFileSync } from 'node:fs';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { importResourceDescriptionNTriples, emitResourceDescriptionNTriples } from '../src/resource-description-ntriples.js';
const I = value => ({ kind: 'identifier', value });
const L = (value, datatype = 'http://www.w3.org/2001/XMLSchema#string', language = '') => ({ kind: 'literal', value, datatype, language });
const B = value => ({ kind: 'blank', value, sourceIdentifier: 'syntax-fixture' });
const cases = [
  { id: 'adjacent-identifiers', source: '<urn:s><urn:p><urn:o>.', triples: [[I('urn:s'), I('urn:p'), I('urn:o')]], canonical: '<urn:s> <urn:p> <urn:o> .\n' },
  { id: 'unicode-language-duplicates', source: '# original spelling\r\n<urn:s> <urn:p> "caf\\u00E9 \\U0001F980"@EN-us .\n<urn:s> <urn:p> "café 🦀"@en-US . # same RDF term\n',
    triples: [[I('urn:s'), I('urn:p'), L('café 🦀', 'http://www.w3.org/1999/02/22-rdf-syntax-ns#langString', 'en-us')]], canonical: '<urn:s> <urn:p> "café 🦀"@en-us .\n' },
  { id: 'scoped-blank-label', source: '_:é.中 <urn:p> _:𐀀.\n', triples: [[B('é.中'), I('urn:p'), B('𐀀')]], canonical: '_:b0 <urn:p> _:b1 .\n' },
  { id: 'literal-escapes', source: '<urn:s> <urn:p> "\\t\\b\\n\\r\\f\\\"\\\'\\\\\\u0000" .\n',
    triples: [[I('urn:s'), I('urn:p'), L('\t\b\n\r\f"\'\\\0')]], canonical: '<urn:s> <urn:p> "\t\b\\n\\r\f\\"\'\\\\\0" .\n' },
  { id: 'datatype-lexical-not-value', source: '<urn:s> <urn:p> "not an integer"^^<http://www.w3.org/2001/XMLSchema#integer> .\n',
    triples: [[I('urn:s'), I('urn:p'), L('not an integer', 'http://www.w3.org/2001/XMLSchema#integer')]], canonical: '<urn:s> <urn:p> "not an integer"^^<http://www.w3.org/2001/XMLSchema#integer> .\n' },
  { id: 'identifier-escaped-unicode', source: '<https://example.test/\\u00E9> <urn:p> <urn:é> .\n',
    triples: [[I('https://example.test/é'), I('urn:p'), I('urn:é')]], canonical: '<https://example.test/é> <urn:p> <urn:é> .\n' },
  { id: 'grandfathered-language', source: '<urn:s> <urn:p> "hello"@I-klingon .\n',
    triples: [[I('urn:s'), I('urn:p'), L('hello', 'http://www.w3.org/1999/02/22-rdf-syntax-ns#langString', 'i-klingon')]], canonical: '<urn:s> <urn:p> "hello"@i-klingon .\n' },
  { id: 'empty-comment-document', source: '\t# comment 🦀\r\n\n', triples: [], canonical: '' },
];
const malformed = [
  ['relative-identifier', '<relative> <urn:p> <urn:o> .'],
  ['surrogate-escape', '<urn:s> <urn:p> "\\uD800" .'],
  ['out-of-range-escape', '<urn:s> <urn:p> "\\U00110000" .'],
  ['truncated-escape', '<urn:s> <urn:p> "\\u12" .'],
  ['unknown-literal-escape', '<urn:s> <urn:p> "\\x" .'],
  ['iri-echar', '<urn:s\\t> <urn:p> <urn:o> .'],
  ['raw-iri-space', '<urn:s a> <urn:p> <urn:o> .'],
  ['raw-string-newline', '<urn:s> <urn:p> "a\nb" .'],
  ['missing-period', '<urn:s> <urn:p> <urn:o>'],
  ['literal-subject', '"s" <urn:p> <urn:o> .'],
  ['blank-predicate', '<urn:s> _:p <urn:o> .'],
  ['missing-object', '<urn:s> <urn:p> .'],
  ['two-statements-one-line', '<urn:s> <urn:p> <urn:o> . <urn:s> <urn:p> <urn:o> .'],
  ['line-break-inside-triple', '<urn:s>\n<urn:p> <urn:o> .'],
  ['empty-blank-label', '_: <urn:p> <urn:o> .'],
  ['blank-leading-dot', '_:.x <urn:p> <urn:o> .'],
  ['double-language', '<urn:s> <urn:p> "x"@en@fr .'],
  ['language-and-datatype', '<urn:s> <urn:p> "x"@en^^<urn:type> .'],
  ['missing-datatype', '<urn:s> <urn:p> "x"^^ .'],
  ['invalid-language', '<urn:s> <urn:p> "x"@en--US .'],
  ['langstring-without-language', '<urn:s> <urn:p> "x"^^<http://www.w3.org/1999/02/22-rdf-syntax-ns#langString> .'],
].map(([id, source]) => ({ id, source, kind: 'malformed' }));
const unsupported = [{ id: 'bracketed-address', source: '<http://[::1]/> <urn:p> <urn:o> .', kind: 'unsupported' }];
for (const item of cases) {
  const graph = importResourceDescriptionNTriples(item.source, { sourceIdentifier: 'syntax-fixture' });
  const emitted = emitResourceDescriptionNTriples(graph.network, graph.root);
  if (emitted.source !== item.canonical) throw new Error('independent canonical oracle differs: ' + item.id);
  Object.assign(item, { archive: graph.network.toLino(), root: graph.root.asU64(), syntaxRoot: graph.syntaxRoot.asU64(),
    correspondenceArchive: emitted.correspondence.network.toLino(), correspondenceRoot: emitted.correspondence.root.asU64() });
}
const fixture = { schemaVersion: 1, profile: 'RDF 1.1 N-Triples with supported absolute identifiers', cases, malformed, unsupported };
const path = new URL('../../parity/fixtures/resource-description-ntriples.lino', import.meta.url);
if (process.argv.includes('--check')) checkProjectData(path, fixture); else writeProjectData(path, fixture);
console.log('N-Triples fixture: ' + cases.length + ' source cases; ' + malformed.length + ' malformed and ' + unsupported.length + ' unsupported inputs');
