import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { readProjectData } from '../scripts/project-data.mjs';
import { LinkNetwork, LinkType, agreeWithNumber, analyzeMorphology, inflect } from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const fixtureFile = 'parity/fixtures/natural-language-morphology.lino';
const fixture = readProjectData(new URL(`../../${fixtureFile}`, import.meta.url));

function hasReading(readings, [lemma, upos, feature]) {
  return readings.some(reading => reading.lemma === lemma && reading.upos === upos &&
    (feature === '_' ? reading.features.length === 0 : reading.features.includes(feature)));
}

test('word readings, ambiguity, suffix prediction, and inflection retain their source evidence', (context) => {
  const { text, language, words, ambiguous, unknown, inflections, numberAgreement } = fixture;
  const network = LinkNetwork.parse(text, language);
  const bytes = Buffer.from(text);
  assert.equal(network.reconstructText(), text);
  assert.ok(network.verifyFullMatch().isClean());
  const readings = network.links().filter(link => link.metadata().term?.startsWith('morphology:'));
  const surfaces = new Map();
  for (const link of readings) {
    const [token, ...children] = link.references().map(id => network.link(id));
    const span = link.metadata().span.byteRange;
    const spelling = bytes.subarray(span.start, span.end).toString();
    assert.equal(token.metadata().linkType, LinkType.SourceToken);
    assert.equal(token.metadata().term, spelling);
    const terms = children.map(child => {
      assert.deepEqual(child.metadata().span.byteRange, span);
      return child.metadata().term;
    });
    assert.ok(terms.some(term => term.startsWith('lemma:')));
    assert.ok(terms.some(term => term.startsWith('upos:')));
    assert.ok(terms.some(term => term.startsWith('ufeat:')));
    if (!surfaces.has(spelling)) surfaces.set(spelling, []);
    surfaces.get(spelling).push(terms);
  }
  assert.deepEqual([...surfaces.keys()], words.map(([word]) => word));
  for (const [word, lemma, upos, feature] of words) {
    assert.ok(hasReading(analyzeMorphology(word, language), [lemma, upos, feature]), word);
    assert.ok(surfaces.get(word).some(terms => [`lemma:${lemma}`, `upos:${upos}`, `ufeat:${feature}`].every(term => terms.includes(term))), word);
  }
  const alternatives = analyzeMorphology(ambiguous.word, language);
  for (const reading of ambiguous.readings) assert.ok(hasReading(alternatives, reading));
  assert.ok(alternatives.every(({ score }, index) => score >= 0 && score <= 1 && (index === 0 || score <= alternatives[index - 1].score)));
  assert.ok(Math.abs(alternatives.reduce((total, { score }) => total + score, 0) - 1) < 0.00001);
  assert.ok(hasReading(analyzeMorphology(unknown[0], language).filter(({ source }) => source === 'suffix'), unknown.slice(1)));
  assert.equal(analyzeMorphology('елка', language)[0].lemma, 'ёлка');
  for (const [lemma, features, form, owner] of inflections) assert.equal(inflect(lemma, features, owner), form);
  assert.equal(inflect('инженер', { Case: 'Unsupported' }), undefined);
  for (const [lemma, number, form, owner] of numberAgreement) assert.equal(agreeWithNumber(lemma, number, owner), form);
  assert.equal(agreeWithNumber('год', Number.POSITIVE_INFINITY), undefined);
  recordIssue195Observations({
    requirementId: 'I195-I236-OPEN-VOCABULARY-MORPHOLOGY', suffix: 'behavior',
    fixtureId: 'i195-i236-open-vocabulary-morphology', fixtureFile,
    assertions: ['everyWordHasLinkedReading', 'ambiguousReadingsAreScored', 'unseenWordsHaveSuffixReadings', 'inflectionAndNumberAgreement'],
    testName: context.name,
  });
});

test('a configured dictionary opens only when its language is used', (context) => {
  const fixtureFile = 'parity/fixtures/natural-language-morphology-lazy-loading.lino';
  const fixtureUrl = new URL(`../../${fixtureFile}`, import.meta.url);
  const moduleUrl = new URL('../src/index.js', import.meta.url);
  const child = spawnSync(process.execPath, ['--input-type=module', '--eval', `
    import assert from 'node:assert/strict';
    import { join } from 'node:path';
    import { LinkNetwork, analyzeMorphology } from ${JSON.stringify(moduleUrl.href)};
    const { language, word, lemma, dictionaryLanguage, dictionaryWord } = JSON.parse(process.argv[1]);
    const network = LinkNetwork.parse(word, language);
    assert.equal(network.reconstructText(), word);
    assert.ok(network.verifyFullMatch().isClean());
    assert.ok(network.links().some(link => link.metadata().term === 'lemma:' + lemma));
    assert.throws(() => analyzeMorphology(dictionaryWord, dictionaryLanguage),
      error => error.path === join(process.env.META_LANGUAGE_MORPHOLOGY_DATA, 'ru', 'meta.json'));
  `, JSON.stringify(readProjectData(fixtureUrl))], {
    env: { ...process.env, META_LANGUAGE_MORPHOLOGY_DATA: fileURLToPath(fixtureUrl) },
    encoding: 'utf8',
  });
  assert.ifError(child.error);
  assert.equal(child.status, 0, child.stderr);
  recordIssue195Observations({
    requirementId: 'I195-I236-OPEN-VOCABULARY-MORPHOLOGY-LAZY-DICTIONARY', suffix: 'behavior',
    fixtureId: 'i195-i236-open-vocabulary-morphology-lazy-dictionary', fixtureFile,
    assertions: ['configuredDictionaryLoadsOnlyForItsLanguage'],
    testName: context.name,
  });
});
