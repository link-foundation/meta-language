// Issue 195 generative tests: property compositions, fuzz mutations, metamorphic pairs,
// edit sequences and kept reproducers of parity/fixtures/issue-195-generative/, generated
// from the conformance inputs by a seeded PRNG and each compared with the tree the native
// tree-sitter CLI printed for it (js/scripts/generate-issue-195-generative.mjs). On top of
// the oracle, every parse must keep the oracle-free properties of support/generative.js,
// and a run-time fuzz pass (seed ISSUE_195_GENERATIVE_SEED, default the fixture seed; count
// ISSUE_195_GENERATIVE_CASES, default 48) checks them, and the blank-line metamorphic
// relation of clean trees, on inputs no fixture contains.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';
import { documentGrammarRoots, documentOracleProblems, parseCstLines, renderCstLines } from './support/cst-lines.js';
import {
  COUNTS,
  RELATIONS,
  applyEdit,
  createRandom,
  generateInputs,
  propertyProblems,
  randomEdit,
  relationHolds,
  seedSources,
} from './support/generative.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const MANIFEST_FILE = 'parity/fixtures/issue-195-generative/manifest.json';
const fixtureRoot = new URL('../../parity/fixtures/issue-195-generative/', import.meta.url);
const repositoryRoot = new URL('../../', import.meta.url);
const read = (path) => readFileSync(new URL(path, fixtureRoot));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const manifest = JSON.parse(read('manifest.json'));
const reproducers = JSON.parse(read('reproducers.json'));
const lock = JSON.parse(readFileSync(new URL('js/src/vendor/grammars/grammar-lock.json', repositoryRoot)));
const RUNTIME_SEED = process.env.ISSUE_195_GENERATIVE_SEED ?? manifest.seed;
const RUNTIME_CASES = Number(process.env.ISSUE_195_GENERATIVE_CASES ?? 48);

const ASSERTIONS = [
  'propertyBasedCasesExecuted',
  'fuzzCasesExecuted',
  'metamorphicCasesExecuted',
  'roundTripPropertiesChecked',
  'malformedInputPropertiesChecked',
  'unicodeSpanPropertiesChecked',
  'editSequencePropertiesChecked',
  'independentOracleUsed',
];

/** Oracle and property problems of one parse of `source`, with its rendered public tree. */
function parseProblems(language, source, cst) {
  const network = LinkNetwork.parse(source, language);
  const { problems, text } = documentOracleProblems(network, language, source, cst);
  return { problems: [...problems, ...propertyProblems(network, source)], text };
}

const renderPublic = (language, source) => {
  const network = LinkNetwork.parse(source, language);
  return { network, text: renderCstLines(documentGrammarRoots(network, language), language).text };
};

test('the generative fixtures record their seed and oracle, and the seed regenerates their inputs', () => {
  assert.equal(manifest.oracle.tool, lock.treeSitterCli);
  assert.equal(manifest.inputs['reproducers.json'], sha256(read('reproducers.json')));
  assert.equal(manifest.inputs['issue-195-conformance/manifest.json'],
    sha256(readFileSync(new URL('parity/fixtures/issue-195-conformance/manifest.json', repositoryRoot))));
  assert.deepEqual(manifest.counts, COUNTS);
  for (const [language, details] of Object.entries(manifest.languages)) {
    assert.equal(sha256(read(details.file)), details.sha256, language);
    const fixture = JSON.parse(read(details.file));
    assert.equal(fixture.seed, manifest.seed);
    const seeds = seedSources(language);
    assert.equal(seeds.length, details.seedSources, language);
    const inputs = generateInputs(language, seeds, manifest.seed);
    const kept = reproducers.cases.filter((entry) => entry.language === language);
    const stored = fixture.cases.map(({ id, source, variant, steps }) => ({ id, source, variant, steps: steps?.map(({ start, end, replacement }) => ({ start, end, replacement })) }));
    const regenerated = [
      ...inputs.map(({ id, source, variant, steps }) => ({ id, source, variant, steps: steps?.map(({ start, end, replacement }) => ({ start, end, replacement })) })),
      ...kept.map((entry) => ({ id: `reproducer/${entry.id}`, source: entry.source, variant: undefined, steps: undefined })),
    ];
    assert.deepEqual(stored, regenerated, `${language} inputs are what seed ${manifest.seed} generates`);
  }
});

test('the PRNG and edits are the ones the Rust suite mirrors', () => {
  for (const [stream, vector] of Object.entries(manifest.prngVectors)) {
    const random = createRandom(stream);
    assert.deepEqual([random.next(), random.next(), random.next(), random.next()], vector, stream);
  }
  assert.equal(Object.keys(manifest.prngVectors).length, Object.keys(manifest.languages).length);
  const text = 'a𝒳b\r\n';
  const edit = randomEdit(createRandom(7), text, 'Rust');
  const bytes = Buffer.from(text, 'utf8');
  for (const offset of [edit.start, edit.end]) assert.ok(offset === bytes.length || (bytes[offset] & 0xc0) !== 0x80);
  assert.equal(applyEdit('héllo', { start: 1, end: 3, replacement: 'e' }), 'hello');
});

test('the property checks reject a network that breaks them', () => {
  const network = LinkNetwork.parse('let a = 1;\n', 'JavaScript');
  assert.deepEqual(propertyProblems(network, 'let a = 1;\n'), []);
  assert.notDeepEqual(propertyProblems(network, 'let a = 2;\n'), []);
  assert.notDeepEqual(propertyProblems(network, 'let a = 1;\n\n'), []);
  const [base, variant] = [renderPublic('JavaScript', 'let a = 1;\n').text, renderPublic('JavaScript', '\n\nlet a = 1;\n').text];
  assert.equal(relationHolds('prepend-blank-lines', base, variant), true);
  assert.equal(relationHolds('prepend-blank-lines', base, base), false);
});

for (const [language, details] of Object.entries(manifest.languages)) {
  const testName = `${language} keeps the oracle trees and properties of its property, fuzz, metamorphic, edit-sequence and reproducer cases`;
  test(testName, () => {
    const fixture = JSON.parse(read(details.file));
    assert.equal(fixture.language, language);
    const failures = [];
    const fail = (entry, where, problems, source = entry.source) => {
      if (problems.length) failures.push(`${entry.id}${where} ${JSON.stringify(source)}: ${problems.join('; ')}`);
    };
    for (const entry of fixture.cases) {
      const base = parseProblems(language, entry.source, entry.cst);
      fail(entry, '', base.problems);
      if (entry.kind === 'metamorphic') {
        const variant = parseProblems(language, entry.variant, entry.variantCst);
        fail(entry, ' (variant)', variant.problems, entry.variant);
        if (relationHolds(entry.relation, base.text, variant.text) !== entry.relationHolds) {
          fail(entry, ' (relation)', [`${entry.relation} holds for the public trees iff it holds for the oracle`]);
        }
      }
      if (entry.kind === 'edit') {
        // The JavaScript runtime has no incremental reparse: each step reparses the edited text.
        let source = entry.source;
        entry.steps.forEach((step, position) => {
          source = applyEdit(source, step);
          fail(entry, ` step ${position}`, parseProblems(language, source, step.cst).problems, source);
        });
      }
    }
    assert.deepEqual(failures, []);

    // Every family is present, and each exercises what it claims.
    const byKind = (kind) => fixture.cases.filter((entry) => entry.kind === kind);
    assert.equal(byKind('property').length, COUNTS.property);
    assert.equal(byKind('fuzz').length, COUNTS.fuzz);
    assert.equal(byKind('metamorphic').length, COUNTS.metamorphic * Object.keys(RELATIONS).length);
    assert.equal(byKind('edit').length, COUNTS.edit);
    assert.ok(byKind('edit').every((entry) => entry.steps.length === COUNTS.editSteps));
    assert.ok(byKind('metamorphic').some((entry) => entry.relationHolds));
    assert.equal(byKind('reproducer').length, reproducers.cases.filter((entry) => entry.language === language).length);
    const malformed = fixture.cases.filter((entry) => !entry.clean);
    assert.ok(malformed.length > 0 && malformed.every((entry) => parseCstLines(entry.cst).some((node) => node.error || node.missing)));
    assert.ok(fixture.cases.some((entry) => /[\u{10000}-\u{10FFFF}]/u.test(entry.source)), 'astral characters');
    assert.ok(fixture.cases.some((entry) => /\r/u.test(entry.source)), 'carriage returns');

    // Run-time fuzz: new inputs from the run seed, checked for the oracle-free properties
    // and for the blank-line relation between the public trees of an input and its variant.
    const random = createRandom(`${RUNTIME_SEED}:${language}:runtime`);
    const seeds = seedSources(language);
    const runtimeFailures = [];
    let cleanCases = 0;
    for (let index = 0; index < RUNTIME_CASES; index += 1) {
      let source = random.pick(seeds).source;
      for (let step = 0, total = 1 + random.int(6); step < total; step += 1) source = applyEdit(source, randomEdit(random, source, language));
      const { network, text } = renderPublic(language, source);
      const problems = propertyProblems(network, source);
      const variant = renderPublic(language, RELATIONS['prepend-blank-lines'].transform(source));
      problems.push(...propertyProblems(variant.network, RELATIONS['prepend-blank-lines'].transform(source)));
      const clean = network.verifyFullMatch().isClean();
      if (clean) cleanCases += 1;
      if (RELATIONS['prepend-blank-lines'].applies(source, clean) && !relationHolds('prepend-blank-lines', text, variant.text)) {
        problems.push('prepend-blank-lines does not hold');
      }
      if (problems.length) {
        runtimeFailures.push(`seed ${JSON.stringify(RUNTIME_SEED)} case ${index} source ${JSON.stringify(source)}: ${problems.join('; ')} (keep it in parity/fixtures/issue-195-generative/reproducers.json)`);
      }
    }
    assert.deepEqual(runtimeFailures, []);
    assert.ok(cleanCases > 0 && cleanCases < RUNTIME_CASES, `${cleanCases} of ${RUNTIME_CASES} run-time cases are clean`);

    recordIssue195Observations({
      requirementId: `I195-GENERATIVE-${language.toUpperCase()}`,
      suffix: 'positive-and-negative',
      fixtureId: `planned:generative:${language}`,
      fixtureFile: MANIFEST_FILE,
      assertions: ASSERTIONS,
      testName,
    });
  });
}
