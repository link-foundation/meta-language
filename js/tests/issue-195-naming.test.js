// The readable-naming check (docs/vision.md#readable-english-names): every
// canonical name the sources define has a concept record, every record is an
// unabbreviated English noun phrase (concepts) or verb phrase (operations)
// checked against WordNet 3.1, and a mutation that introduces an abbreviation,
// an ambiguity, a semantic duplicate or an unrecorded name fails the gate.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';

import { areSynonyms, loadWordNet, partsOfSpeech } from '../scripts/english-vocabulary.mjs';
import {
  checkConceptRecords,
  checkNamingFixtures,
  checkRepositoryNames,
  extractNameInventory,
  extractSourceDescriptions,
  FORMER_NAME_TABLES,
  loadNamingRegisters,
  NAME_INVENTORIES,
  NAMING_FIXTURES,
  nameProblems,
} from '../scripts/issue-195-naming.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const wordnet = loadWordNet();
const registers = loadNamingRegisters(root);
const context = { wordnet, vocabulary: registers.vocabulary };
const { fixtures } = JSON.parse(readFileSync(new URL(`../../${NAMING_FIXTURES}`, import.meta.url), 'utf8'));
const repository = checkRepositoryNames(root, wordnet);

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-NAMING-CI-ENFORCEMENT',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-naming-ci-enforcement',
    fixtureFile: NAMING_FIXTURES,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

const record = (id) => structuredClone(repository.records.find((candidate) => candidate.id === id));
const kindsOf = (problems) => [...new Set(problems.map((problem) => problem.kind))].sort();
// The problems of the repository with records or source names mutated in memory.
const mutated = (override) => checkRepositoryNames(root, wordnet, override).problems;

test('the vocabulary is WordNet 3.1 plus registered technical terms, not a letter pattern', () => {
  assert.equal(wordnet.source, 'WordNet 3.1 (wordnet-db)');
  assert.ok(wordnet.lemmas.noun.size > 100_000);
  assert.deepEqual(partsOfSpeech(wordnet, 'repetitions'), ['noun']);
  assert.deepEqual(partsOfSpeech(wordnet, 'frobnicator'), []);
  assert.ok(areSynonyms(wordnet, 'argument', 'parameter', 'noun'));
  assert.equal(areSynonyms(wordnet, 'loop', 'branch', 'noun'), false);
  // Every technical term is outside WordNet in the part of speech it registers
  // and cites the reference that defines it.
  for (const [word, term] of registers.vocabulary.technicalTerms) {
    assert.ok(term.partsOfSpeech.some((part) => !partsOfSpeech(wordnet, word).includes(part)), word);
    assert.match(term.source, /\S/u, word);
    assert.match(term.definition, /^\p{Lu}.*\.$/u, word);
  }
  // Letters-only words outside the vocabulary are rejected.
  assert.deepEqual(kindsOf(nameProblems('xyzzy_rule', context)), ['vocabulary']);
  assert.deepEqual(kindsOf(mutated({ names: [...repository.names, { inventory: 'inferred rule names', file: 'rust/src/grammar/inference/advisor.rs', name: 'frobnicated_value', record: null }] })), ['vocabulary']);
  observe(['vocabularyChecked'], 'the vocabulary is WordNet 3.1 plus registered technical terms, not a letter pattern');
});

test('the positive and negative naming fixtures are judged as they expect', () => {
  assert.deepEqual(checkNamingFixtures(fixtures, context), []);
  const expected = new Set(fixtures.flatMap((fixture) => fixture.expected));
  for (const kind of ['record', 'vocabulary', 'abbreviation', 'phrase-role', 'ambiguous', 'duplicate']) assert.ok(expected.has(kind), kind);
  assert.ok(fixtures.some((fixture) => fixture.expected.length === 0));
  // A fixture judged differently than it expects is itself a problem.
  const [accepted] = fixtures.filter((fixture) => fixture.expected.length === 0);
  assert.equal(checkNamingFixtures([{ ...accepted, expected: ['abbreviation'] }], context)[0].kind, 'fixture');
});

test('concepts must be noun phrases and operations verb phrases', () => {
  for (const concept of repository.records.filter(({ role }) => role === 'concept')) assert.ok(concept.phrase.length > 0, concept.id);
  assert.ok(repository.records.some(({ role }) => role === 'operation'));
  const problems = mutated({
    records: [
      ...repository.records,
      { ...record('loop'), id: 'grammar.parse-quickly', phrase: 'parse quickly', definition: 'Parsing done fast.' },
      { ...record('operation.parse'), id: 'operation.parser', phrase: 'parser', definition: 'A program that parses text.' },
    ],
  }).filter((problem) => problem.kind === 'phrase-role');
  assert.deepEqual(problems.map((problem) => problem.id).sort(), ['grammar.parse-quickly', 'operation.parser']);
  observe(['phraseRoleChecked'], 'concepts must be noun phrases and operations verb phrases');
});

test('an abbreviation fails the gate, including one WordNet lists with another meaning', () => {
  for (const abbreviation of ['char', 'ref', 'id', 'min', 'max']) {
    assert.ok(partsOfSpeech(wordnet, abbreviation).length > 0, `${abbreviation} is a WordNet lemma`);
    assert.ok(registers.vocabulary.abbreviations.has(abbreviation), abbreviation);
  }
  const renamed = repository.records.map((candidate) =>
    candidate.id === 'grammar.character-class' ? { ...candidate, id: 'grammar.char-class', phrase: 'char class' } : candidate);
  const problems = mutated({ records: renamed });
  assert.ok(problems.some((problem) => problem.kind === 'abbreviation' && problem.id === 'grammar.char-class'));
  // Inferred rule names are generated at run time; their words are checked too.
  const inferred = mutated({ names: [...repository.names, { inventory: 'inferred rule names', file: 'rust/src/grammar/inference/advisor.rs', name: 'seq_2', record: null }] });
  assert.deepEqual(inferred.map((problem) => problem.message), ['rust/src/grammar/inference/advisor.rs (inferred rule names) seq_2: "seq" abbreviates "sequence"']);
  const ambiguous = nameProblems('program::mod', context);
  assert.match(ambiguous[0].message, /an ambiguous abbreviation/u);
  observe(['abbreviationsRejected'], 'an abbreviation fails the gate, including one WordNet lists with another meaning');
});

test('a phrase naming two concepts or an identity reusing a former name fails the gate', () => {
  const second = { ...record('grammar.string'), id: 'text.strings', phrase: 'strings', definition: 'A sequence of characters in a document.', sourceAliases: [], formerNames: [] };
  const inflected = mutated({ records: [...repository.records, second] }).filter((problem) => problem.kind === 'ambiguous');
  assert.deepEqual(inflected.map((problem) => problem.message), ['text.strings: "strings" also names grammar.string; each phrase names one concept']);
  const former = { ...record('grammar.sequence'), id: 'sequence', definition: 'An ordered list of things.', formerNames: [] };
  const problems = checkConceptRecords([...repository.records, former], context);
  assert.deepEqual(problems.map((problem) => problem.message), [
    'sequence: "sequence" also names grammar.sequence; each phrase names one concept',
    'sequence: the identity is a former name of sequential-composition',
  ]);
  observe(['ambiguousNamesRejected'], 'a phrase naming two concepts or an identity reusing a former name fails the gate');
});

test('a repeated identity, definition, synonym or representation name fails the gate', () => {
  const duplicates = (extra) => checkConceptRecords([...repository.records, ...extra], context).filter((problem) => problem.kind === 'duplicate');
  assert.equal(duplicates([record('loop')])[0].message, 'loop: the identity is recorded twice');
  assert.equal(duplicates([{ ...record('loop'), id: 'program.cycle', phrase: 'cycle' }])[0].message, 'program.cycle: has the definition of loop; one concept has one name');
  // A semantic duplicate: WordNet lists "token" and "item" in one noun synset.
  const token = { ...record('grammar.item'), id: 'grammar.token', phrase: 'token', definition: 'A rule for one lexical unit.' };
  assert.match(duplicates([token])[0].message, /grammar\.token: "token" is a synonym of "item" \(grammar\.item\)/u);
  // The committed records state why the synonyms they keep name different concepts.
  const withoutDistinction = repository.records.map((candidate) => (candidate.id === 'parameter' ? { ...candidate, distinctFrom: [] } : candidate));
  assert.match(checkConceptRecords(withoutDistinction, context).map((problem) => problem.message).join('\n'), /argument: "argument" is a synonym of "parameter"/u);
  const renamed = { ...record('grammar.value.string'), id: 'grammar.value.text', phrase: 'text' };
  assert.match(duplicates([renamed])[0].message, /names the concept grammar\.string "text" instead of "string"/u);
  observe(['duplicatesRejected'], 'a repeated identity, definition, synonym or representation name fails the gate');
});

test('every name the sources define has a concept record and every record is used', () => {
  assert.deepEqual(repository.problems, []);
  const inventories = new Set(repository.names.map(({ inventory }) => inventory));
  assert.deepEqual([...inventories].sort(), [...new Set(NAME_INVENTORIES.map(({ inventory }) => inventory))].sort());
  assert.equal(new Set(repository.formers.map(({ table }) => table)).size, FORMER_NAME_TABLES.length);
  // Every described inventory describes each name it defines.
  const descriptions = extractSourceDescriptions(root);
  const exempt = (name) => repository.exemptions.some(({ pattern }) => new RegExp(pattern, 'u').test(name));
  for (const inventory of new Set(descriptions.map((entry) => entry.inventory))) {
    const files = new Set(descriptions.filter((entry) => entry.inventory === inventory).map((entry) => entry.file));
    const defined = new Set(extractNameInventory(root).filter((entry) => entry.inventory === inventory && files.has(entry.file) && !exempt(entry.name)).map((entry) => entry.name));
    const described = new Set(descriptions.filter((entry) => entry.inventory === inventory).map((entry) => entry.name));
    assert.deepEqual([...described].sort(), [...defined].sort(), inventory);
  }
  // A source name without a record, a record without a source name, a former
  // name the record does not list and a source definition changed in the record
  // each fail the gate.
  const unrecorded = mutated({ names: [...repository.names, { inventory: 'grammar constructs', file: 'rust/src/grammar/fidelity.rs', name: 'word-boundary', record: 'grammar.word-boundary' }] });
  assert.deepEqual(unrecorded.map((problem) => problem.message), ['rust/src/grammar/fidelity.rs (grammar constructs) defines word-boundary without a concept record']);
  const unused = mutated({ records: [...repository.records, { ...record('loop'), id: 'program.iteration-count', phrase: 'iteration count', definition: 'How many times a loop body runs.' }] });
  assert.deepEqual(unused.map((problem) => problem.message), ['program.iteration-count: no inventory defines the recorded name']);
  const forgotten = repository.records.map((candidate) => (candidate.id === 'block-quote' ? { ...candidate, formerNames: [] } : candidate));
  assert.deepEqual(mutated({ records: forgotten }).map((problem) => problem.message), ['rust/src/concept_ontology.rs (FORMER_CONCEPT_IDS) decodes blockquote as block-quote, but block-quote does not list it as a former name']);
  const redefined = repository.records.map((candidate) => (candidate.id === 'loop' ? { ...candidate, definition: 'A cycle.' } : candidate));
  assert.match(mutated({ records: redefined }).map((problem) => problem.message).join('\n'), /loop: the definition differs from rust\/src\/concept_ontology\.rs/u);
  const stillUsed = mutated({ names: [...repository.names, { inventory: 'grammar constructs', file: 'rust/src/grammar/fidelity.rs', name: 'blockquote', record: 'grammar.blockquote' }] });
  assert.match(stillUsed.map((problem) => problem.message).join('\n'), /still uses blockquote, a former name of block-quote/u);
  observe(['fullInventoryCovered'], 'every name the sources define has a concept record and every record is used');
});
