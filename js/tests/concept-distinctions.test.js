// Concept distinctions: two languages' spellings share a concept only under a
// one-to-one correspondence of meaning that the records justify, required
// distinctions (ordered and unordered choice, lexical and syntactic
// precedence, binding and assignment, the foundation models) never merge,
// and lookalike spellings with different meanings stay distinct
// (requirement I195-GRAMMAR-CONCEPT-DISTINCTIONS). The Rust twin is
// rust/tests/unit/concept_distinctions.rs.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  CONCEPT_RECORDS,
  REQUIRED_CONCEPT_DISTINCTIONS,
  REQUIRED_FOUNDATION_DISTINCTIONS,
  checkConceptDistinctions,
  conceptCorrespondence,
  grammarExprConceptId,
  grammarPrecedenceConcepts,
  importAbnf,
  importBnf,
  importPest,
  importTreeSitterJson,
  sourceMeanings,
} from '../src/index.js';
import { FOUNDATION_MODELS } from '../src/foundation-models.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-GRAMMAR-CONCEPT-DISTINCTIONS';
const FIXTURE = 'parity/fixtures/concept-distinctions.json';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const fixture = JSON.parse(readFileSync(path.join(root, FIXTURE), 'utf8'));
const IMPORTERS = { pest: importPest, bnf: importBnf, abnf: importAbnf };

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: FIXTURE,
    assertions,
    testName,
  });
}

const records = () => structuredClone(CONCEPT_RECORDS);
const register = () => structuredClone(FOUNDATION_MODELS);
const kinds = (conceptRecords, foundationRegister = FOUNDATION_MODELS) =>
  checkConceptDistinctions(conceptRecords, foundationRegister).map(({ kind }) => kind);
const record = (list, id) => list.find((entry) => entry.id === id);

test('every fixture correspondence relates the spellings as recorded, and shared spellings carry a justification', () => {
  assert.deepEqual(checkConceptDistinctions(), []);
  for (const expected of fixture.correspondences) {
    const actual = conceptCorrespondence(expected.first, expected.second, { within: expected.within });
    const label = `${expected.first.source} ${expected.first.name} / ${expected.second.source} ${expected.second.name}`;
    assert.equal(actual.relation, expected.relation, label);
    assert.equal(actual.shared, expected.shared, label);
    assert.equal(actual.justification !== null, expected.justified, label);
    assert.equal(actual.correspondence, expected.correspondence, label);
    if (expected.meanings) assert.deepEqual([...actual.first, ...actual.second], expected.meanings, label);
    if (actual.relation === 'shared') {
      assert.deepEqual(actual.first, [actual.shared], label);
      assert.deepEqual(actual.second, [actual.shared], label);
      assert.match(actual.justification, /\S+ \S+ \S+.*\.$/u, label);
    }
    // An ambiguous spelling justifies no correspondence: it is never reported as shared.
    if (actual.relation === 'ambiguous') {
      assert.ok(actual.first.length > 1 || actual.second.length > 1, label);
      assert.equal(actual.shared, null, label);
    }
  }
  assert.deepEqual(sourceMeanings({ source: 'Python', name: '=' }), ['binding', 'assignment']);
  observe(['sharedOnlyWithJustifiedCorrespondence'], 'every fixture correspondence relates the spellings as recorded, and shared spellings carry a justification');
});

test('checkConceptDistinctions rejects records that merge or fail to justify a required distinction', () => {
  assert.deepEqual(
    REQUIRED_CONCEPT_DISTINCTIONS.map(({ concepts }) => concepts.join(' / ')),
    [
      'grammar.ordered-choice / grammar.unordered-choice',
      'grammar.lexical-precedence / grammar.syntactic-precedence',
      'binding / assignment',
      'grammar.list / grammar.linked-list',
      'grammar.string / grammar.symbol',
      'grammar.member / grammar.ini.setting',
      'grammar.object / grammar.racket.hash-table',
      'grammar.value / grammar.datum',
      'grammar.identifier / grammar.identifier-name',
      'grammar.document / grammar.program',
      'grammar.boolean-value / grammar.true-value',
      'grammar.program / grammar.module',
      'grammar.true-constant / grammar.true-value',
      'grammar.symbol / grammar.keyword',
      'grammar.comment / grammar.block-comment',
      'grammar.null-keyword / grammar.nil-keyword',
      'grammar.collection-comprehension / grammar.for-expression',
    ],
  );
  assert.equal(REQUIRED_FOUNDATION_DISTINCTIONS.length, 6);

  const missing = records().filter(({ id }) => id !== 'grammar.ordered-choice');
  assert.deepEqual(kinds(missing), ['unknown-concept']);

  const merged = records();
  record(merged, 'grammar.lexical-precedence').definition = record(merged, 'grammar.syntactic-precedence').definition;
  assert.deepEqual(kinds(merged), ['indistinct-concepts']);

  const conflated = records();
  record(conflated, 'assignment').formerNames.push('binding');
  assert.deepEqual(kinds(conflated), ['conflated-distinction']);

  const represented = records();
  record(represented, 'grammar.unordered-choice').represents = 'grammar.ordered-choice';
  assert.deepEqual(kinds(represented), ['conflated-distinction']);

  const unrecorded = register();
  unrecorded.distinctions = unrecorded.distinctions.filter(
    ({ models }) => !models.includes('universe-model.cumulative-type-hierarchy'),
  );
  assert.deepEqual(kinds(CONCEPT_RECORDS, unrecorded), ['unrecorded-distinction']);

  const sameProperties = register();
  const natural = record(sameProperties.models, 'integer-model.natural-number');
  natural.properties = structuredClone(record(sameProperties.models, 'integer-model.unbounded-integer').properties);
  assert.deepEqual(kinds(CONCEPT_RECORDS, sameProperties), ['indistinct-concepts']);

  const unknownModel = register();
  unknownModel.models = unknownModel.models.filter(({ id }) => id !== 'logic-model.classical-propositions');
  assert.ok(kinds(CONCEPT_RECORDS, unknownModel).includes('unknown-model'));

  const unjustified = records();
  record(unjustified, 'grammar.sequence').definition = 'sequence';
  assert.deepEqual(kinds(unjustified), ['unjustified-sharing']);

  const unconstrained = records();
  record(unconstrained, 'grammar.unordered-choice').constraints = [];
  assert.deepEqual(kinds(unconstrained), ['unjustified-sharing']);

  const unjustifiedModel = register();
  record(unjustifiedModel.models, 'integer-model.fixed-width-integer').definition = 'fixed width';
  assert.deepEqual(kinds(CONCEPT_RECORDS, unjustifiedModel), ['unjustified-sharing']);
  observe(['requiredDistinctionsPreserved'], 'checkConceptDistinctions rejects records that merge or fail to justify a required distinction');
});

test('lookalike spellings import as distinct concepts and precedence splits into lexical and syntactic', () => {
  for (const { format, source, concept } of fixture.lookalikeGrammars) {
    assert.equal(grammarExprConceptId(IMPORTERS[format](source).rule('a').expression), concept, format);
  }
  const grammar = importTreeSitterJson(JSON.stringify(fixture.precedence.grammar));
  assert.deepEqual(grammarPrecedenceConcepts(grammar), fixture.precedence.uses);

  // A spelling that gains a second meaning makes every correspondence through it ambiguous, never shared.
  const pestBar = { source: 'pest', name: '|' };
  const bnfBar = { source: 'bnf', name: '|' };
  const widened = records();
  record(widened, 'grammar.unordered-choice').sourceAliases.push(pestBar);
  assert.equal(conceptCorrespondence(pestBar, bnfBar, { records: widened }).relation, 'ambiguous');
  assert.equal(conceptCorrespondence(pestBar, bnfBar).relation, 'distinct');

  for (const lookalike of fixture.correspondences.filter((entry) => entry.lookalike)) {
    assert.notEqual(conceptCorrespondence(lookalike.first, lookalike.second, { within: lookalike.within }).relation, 'shared');
  }
  observe(['lookalikeConceptsKeptDistinct'], 'lookalike spellings import as distinct concepts and precedence splits into lexical and syntactic');
});
