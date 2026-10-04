// The concept records the runtime ships (docs/vision.md#readable-english-names):
// every canonical concept has a stable identity, a readable English phrase, a
// definition, constraints and source aliases; assigning and importing them
// renames former identities and keeps them as aliases, and round trips through
// the source formats are unaffected.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';

import {
  CONCEPT_RECORDS,
  FORMER_CONCEPT_ID_VOCABULARY,
  FORMER_CONCEPT_IDS,
  LinkNetwork,
  LinkType,
  conceptRecord,
  conceptRecords,
  conceptRecordsForSourceName,
  currentConceptId,
  deserializeGrammar,
  emitEbnf,
  importEbnf,
  serializeGrammar,
} from '../src/index.js';
import { loadWordNet } from '../scripts/english-vocabulary.mjs';
import {
  CANONICAL_CONCEPTS,
  checkConceptRecords,
  FORMER_NAME_TABLES,
  loadNamingRegisters,
  phraseOf,
} from '../scripts/issue-195-naming.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const registers = loadNamingRegisters(root);
const context = { wordnet: loadWordNet(), vocabulary: registers.vocabulary };

function observe(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: CANONICAL_CONCEPTS,
    assertions,
    testName,
  });
}

// The alias links of the concept `id` in `network`, as `vocabulary name` strings.
function aliasesOf(network, id) {
  const concept = network.findTerm(id);
  return network.links()
    .filter((link) => {
      const references = link.references();
      return link.metadata().linkType === LinkType.Semantic && references.length === 2 &&
        references[0].equals(concept) && network.link(references[1]).metadata().linkType === LinkType.Type;
    })
    .map((link) => `${link.metadata().language} ${link.metadata().term}`);
}

test('the runtime records are the register: identity, phrase, definition, constraints and source aliases', () => {
  assert.equal(conceptRecords(), CONCEPT_RECORDS);
  assert.equal(CONCEPT_RECORDS.length, registers.records.length);
  for (const [index, registered] of registers.records.entries()) {
    const record = CONCEPT_RECORDS[index];
    assert.deepEqual(record, registered);
    assert.equal(conceptRecord(record.id), record, `${record.id} is a stable identity`);
    assert.equal(record.phrase, phraseOf(record.id), `${record.id} spells its phrase`);
    assert.ok(record.definition.length > 0 && record.constraints.length > 0, `${record.id} records its meaning`);
    for (const { source, name } of record.sourceAliases) {
      assert.ok(conceptRecordsForSourceName(source, name).includes(record), `${source} names ${record.id} ${name}`);
    }
  }
  assert.equal(new Set(CONCEPT_RECORDS.map(({ id }) => id)).size, CONCEPT_RECORDS.length);
  assert.ok(Object.isFrozen(CONCEPT_RECORDS[0].sourceAliases));
  assert.deepEqual(conceptRecordsForSourceName('ebnf', '=').map(({ id }) => id), ['grammar.rule']);
  assert.equal(conceptRecord('no such concept'), undefined);
  observe(
    'I195-NAMING-CONCEPT-RECORDS',
    ['stableIdentity', 'readablePhrase', 'definitionRecorded', 'constraintsRecorded', 'sourceAliasesRecorded'],
    'the runtime records are the register: identity, phrase, definition, constraints and source aliases',
  );
});

test('every runtime concept is an unabbreviated noun phrase and every operation a verb phrase', () => {
  assert.deepEqual(checkConceptRecords(CONCEPT_RECORDS, context), []);
  const roles = new Set(CONCEPT_RECORDS.map(({ role }) => role));
  assert.deepEqual([...roles].sort(), ['concept', 'operation']);
  // The same check rejects an abbreviated concept and a noun named as an operation.
  const abbreviated = { ...CONCEPT_RECORDS.find(({ id }) => id === 'grammar.character-class') };
  Object.assign(abbreviated, { id: 'grammar.char-class', phrase: 'char class' });
  const [problem, ...others] = checkConceptRecords([abbreviated], context);
  assert.deepEqual(others, []);
  assert.equal(problem.kind, 'abbreviation');
  assert.match(problem.message, /"char" abbreviates "character"/u);
  const nounOperation = { ...CONCEPT_RECORDS.find(({ id }) => id === 'operation.parse'), id: 'operation.parser', phrase: 'parser' };
  assert.deepEqual(checkConceptRecords([nounOperation], context).map(({ kind }) => kind), ['phrase-role']);
  const verbConcept = { ...CONCEPT_RECORDS.find(({ id }) => id === 'heading'), id: 'emphasize', phrase: 'emphasize' };
  assert.deepEqual(checkConceptRecords([verbConcept], context).map(({ kind }) => kind), ['phrase-role']);
  observe(
    'I195-NAMING-CONVENTION',
    ['conceptsAreNounPhrases', 'operationsAreVerbPhrases', 'noAbbreviations'],
    'every runtime concept is an unabbreviated noun phrase and every operation a verb phrase',
  );
});

test('original and former names stay aliases that decode to the canonical concept', () => {
  const rustTable = FORMER_NAME_TABLES.find(({ table }) => table === 'FORMER_CONCEPT_IDS');
  const rustPairs = rustTable.extract(readFileSync(new URL(`../../${rustTable.file}`, import.meta.url), 'utf8'));
  assert.deepEqual(FORMER_CONCEPT_IDS.map((pair) => [...pair]), rustPairs, 'both runtimes rename the same identities');
  for (const [former, current] of FORMER_CONCEPT_IDS) {
    assert.equal(currentConceptId(former), current);
    assert.equal(conceptRecord(former).id, current);
  }
  assert.equal(currentConceptId('grammar.character-class'), 'grammar.character-class');

  const network = new LinkNetwork();
  const report = network.seedConceptRecords();
  assert.equal(report.concepts, CONCEPT_RECORDS.length);
  assert.equal(network.seedConceptRecords().links, 0, 'assigning the records again adds nothing');
  for (const record of CONCEPT_RECORDS) {
    const concept = network.link(network.findTerm(record.id)).metadata();
    assert.equal(concept.linkType, LinkType.Concept);
    assert.equal(concept.definition, record.definition);
    assert.equal(network.reconstructConcept(record.id, 'en'), record.phrase);
    const aliases = aliasesOf(network, record.id);
    for (const { source, name } of record.sourceAliases) assert.ok(aliases.includes(`${source} ${name}`), `${record.id} keeps ${source} ${name}`);
    for (const former of record.formerNames) {
      assert.ok(aliases.includes(`${FORMER_CONCEPT_ID_VOCABULARY} ${former}`), `${record.id} keeps ${former}`);
      assert.equal(conceptRecord(former), record);
    }
  }
  observe(
    'I195-NAMING-CONVENTION',
    ['originalNamesKeptAsAliases'],
    'original and former names stay aliases that decode to the canonical concept',
  );
  observe(
    'I195-NAMING-CONCEPT-RECORDS',
    ['stableIdentity', 'sourceAliasesRecorded'],
    'original and former names stay aliases that decode to the canonical concept',
  );
});

test('import assigns records and renames former identities; source round trips are unaffected', () => {
  const older = new LinkNetwork();
  older.internConcept('grammar.char-class', 'An older definition.');
  older.insertConceptExpression('grammar.char-class', 'ru', 'класс символов');
  older.insertConceptAlias(older.findTerm('grammar.char-class'), 'wikidata', 'Q5089612');
  older.internConcept('project.custom-concept', 'A concept without a record.');

  const merged = new LinkNetwork();
  const report = merged.importConceptOntology(older);
  assert.deepEqual(report, { concepts: 2, assigned: 1, renamed: 1, aliasLinks: 1, syntaxMappings: 1 });
  assert.equal(merged.findTerm('grammar.char-class'), undefined, 'the former identity is not a concept');
  const record = conceptRecord('grammar.character-class');
  assert.equal(merged.link(merged.findTerm(record.id)).metadata().definition, record.definition);
  assert.equal(merged.reconstructConcept(record.id, 'ru'), 'класс символов');
  assert.ok(aliasesOf(merged, record.id).includes('wikidata Q5089612'));
  assert.ok(aliasesOf(merged, record.id).includes(`${FORMER_CONCEPT_ID_VOCABULARY} grammar.char-class`));
  assert.equal(merged.link(merged.findTerm('project.custom-concept')).metadata().definition, 'A concept without a record.');
  const size = merged.len();
  merged.importConceptOntology(older);
  assert.equal(merged.len(), size, 'merging the same ontology again is idempotent');

  // Assigning the records to a parsed network leaves its source reconstruction intact.
  const source = 'const answer = 42;\nfunction twice(value) { return value * 2; }\n';
  const parsed = LinkNetwork.parse(source, 'JavaScript');
  parsed.seedConceptRecords();
  assert.equal(parsed.reconstructText(), source);
  // A grammar written with the source notations the records alias still round-trips.
  const ebnf = 'number = digit , { digit } ;\ndigit = "0" | "1" ;\n';
  assert.deepEqual(conceptRecordsForSourceName('ebnf', '{ }').map(({ id }) => id), [
    'grammar.counted-repetition',
    'grammar.zero-or-more-repetition',
  ]);
  const grammar = importEbnf(ebnf);
  assert.equal(emitEbnf(grammar).source, ebnf);
  assert.equal(serializeGrammar(deserializeGrammar(serializeGrammar(grammar))), serializeGrammar(grammar));
  observe(
    'I195-NAMING-CONCEPT-RECORDS',
    ['stableIdentity', 'definitionRecorded', 'sourceAliasesRecorded', 'roundTripsUnaffected'],
    'import assigns records and renames former identities; source round trips are unaffected',
  );
});
