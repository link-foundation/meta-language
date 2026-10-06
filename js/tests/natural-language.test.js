import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { LinkMetadata, LinkNetwork, LinkType } from '../src/index.js';
import {
  LANGUAGE_IDENTIFIER_TERM,
  identifyLanguage,
} from '../src/language-identification.js';
import {
  NATURAL_LANGUAGE_GRAMMAR_FIXTURES,
  bidiDirection,
  canonicalNaturalLanguage,
} from '../src/natural-language.js';

const jsRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');

function semanticTerms(network) {
  return network.links()
    .filter((link) => link.metadata().linkType === LinkType.Semantic)
    .map((link) => link.metadata().term);
}

test('trigram identifier matches the Rust runtime verdicts', () => {
  // Same table as rust/src/language_identification.rs.
  for (const [text, expected] of [
    ['Hawaii is a state.', 'English'],
    ['Hawaii est un etat.\n', 'French'],
    ['Hawaii e um estado.\n', 'Portuguese'],
    ['Гавайи это штат.', 'Russian'],
    ['مرحبا.\n', 'Urdu'],
    ['سلام۔\n', 'Modern Standard Arabic'],
    ['我喜欢学习。', 'Mandarin Chinese'],
    ['আমি বাড়ি যাই।', 'Bengali'],
    ['12345', undefined],
    ['ok', undefined],
    ['Ωμέγα', undefined],
    ['', undefined],
  ]) {
    assert.equal(identifyLanguage(text), expected, JSON.stringify(text));
  }
});

test('trigram identifier data is generated from the pinned franc-min', () => {
  const output = execFileSync(
    process.execPath,
    ['scripts/build-language-identification.mjs', '--check'],
    { cwd: jsRoot, encoding: 'utf8' },
  );
  assert.match(output, /language identification data is current/u);
});

test('natural-language parse records identifier, Unicode and bidi annotations', () => {
  const english = LinkNetwork.parse('Hawaii is a state.\n', 'English');
  const englishTerms = semanticTerms(english);
  assert.ok(englishTerms.includes(LANGUAGE_IDENTIFIER_TERM));
  assert.ok(englishTerms.includes('bidi:ltr'));
  assert.ok(englishTerms.includes('normalization:nfc:stable'));
  assert.equal(english.reconstructText(), 'Hawaii is a state.\n');

  const arabic = LinkNetwork.parse('مرحبا بالعالم.\n', 'Arabic');
  assert.ok(semanticTerms(arabic).includes('bidi:rtl'));
  assert.equal(bidiDirection('abc مرحبا'), 'ltr');
  assert.equal(bidiDirection('مرحبا abc'), 'rtl');
  assert.equal(canonicalNaturalLanguage('ar'), 'Modern Standard Arabic');
});

test('grammar fixtures separate accepted sentences from reported errors', () => {
  for (const fixture of NATURAL_LANGUAGE_GRAMMAR_FIXTURES) {
    const accepted = LinkNetwork.parse(fixture.grammaticalSource, fixture.language);
    assert.ok(accepted.verifyFullMatch().isClean(), `${fixture.language} grammatical`);
    const rejected = LinkNetwork.parse(fixture.ungrammaticalSource, fixture.language);
    assert.ok(
      rejected.links().some((link) => link.metadata().term === 'natural-language:error:grammar'),
      `${fixture.language} ungrammatical`,
    );
    assert.equal(rejected.reconstructText(), fixture.ungrammaticalSource);
  }
});

test('statehood worked example links both languages to shared concepts', () => {
  for (const [source, language] of [
    ['Hawaii is a state.\n', 'English'],
    ['Гавайи это штат.\n', 'Russian'],
  ]) {
    const network = LinkNetwork.parse(source, language);
    const proposition = network.links()
      .find((link) => link.metadata().term === 'proposition:statehood');
    assert.ok(proposition, language);
    const concepts = proposition.references()
      .map((reference) => network.link(reference).metadata().term);
    assert.deepEqual(concepts, ['statehood', 'Q782', 'Q35657']);
    assert.equal(network.reconstructConcept('Q782', 'English'), 'Hawaii');
    assert.equal(network.reconstructConcept('Q782', 'Russian'), 'Гавайи');
  }
});

test('concept interning reuses only exact ids', () => {
  // Mirrors rust/tests/unit/concept_ontology.rs.
  const network = new LinkNetwork();
  const bank = network.internConcept('bank:river', 'first definition');
  assert.equal(network.internConcept('bank:river', 'river edge').asU64(), bank.asU64());
  assert.equal(network.definitionFor(bank), 'river edge');
  for (const variant of ['Bank:river', 'bank:rivér', 'bank:finance']) {
    assert.notEqual(network.internConcept(variant, variant).asU64(), bank.asU64(), variant);
  }
  assert.equal(network.findTerm('bank:river').asU64(), bank.asU64());
});

test('concept expressions share one language-free concept', () => {
  const network = new LinkNetwork();
  const english = network.insertConceptExpression('water', 'English', 'water');
  const spanish = network.insertConceptExpression('water', 'Spanish', 'agua');
  const concept = network.findTerm('water');
  for (const expression of [english, spanish]) {
    assert.equal(network.link(expression).references()[0].asU64(), concept.asU64());
  }
  assert.equal(network.reconstructConcept('water', 'English'), 'water');
  assert.equal(network.reconstructConcept('water', 'Spanish'), 'agua');
  assert.equal(
    network.insertConceptExpression('water', 'Spanish', 'agua').asU64(),
    spanish.asU64(),
  );
});

test('external concept aliases are queryable without becoming concept ids', () => {
  const network = new LinkNetwork();
  const concept = network.internConcept('state', 'A constituent political entity.');
  const alias = network.insertConceptAlias(concept, 'wikidata', 'Q89');
  assert.equal(network.insertConceptAlias(concept, 'wikidata', 'Q89').asU64(), alias.asU64());
  assert.equal(network.findTerm('Q89'), undefined);
  const matches = network.links().filter((link) => {
    const metadata = link.metadata();
    return metadata.linkType === LinkType.Semantic &&
      metadata.language === 'wikidata' &&
      metadata.term === 'Q89';
  });
  assert.equal(matches.length, 1);
  assert.equal(matches[0].references()[0].asU64(), concept.asU64());
  assert.ok(LinkMetadata.new());
});
