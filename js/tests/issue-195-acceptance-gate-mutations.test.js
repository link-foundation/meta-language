// Issue #195 gate mutations (docs/vision.md#acceptance-and-evidence): each
// directive obligation has a gate, and a mutation of the committed artifact
// that gate guards makes it fail, while the same gate passes the unmutated
// artifact. The mutations remove a grammar feature, leave an old dependency,
// bypass native execution, break reverse conversion, introduce a semantic
// duplicate and an abbreviation, drop a source requirement, weaken an oracle
// mapping and replace proof obligations with bounded checks.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { checkDeliveredDependencies } from '../scripts/dependency-inventory.mjs';
import { compareScopeBaseline, evaluateIssue195Acceptance } from '../scripts/issue-195-acceptance-lib.mjs';
import { checkConceptRecords, checkRepositoryNames, loadNamingRegisters, nameProblems } from '../scripts/issue-195-naming.mjs';
import { conformanceOracleProblems } from '../scripts/issue-195-oracle-mapping.mjs';
import { proofObligationProblems } from '../scripts/issue-195-proof-obligations.mjs';
import { ISSUE_195_SOURCES } from '../scripts/issue-195-requirements.mjs';
import { validateSourceRegister } from '../scripts/issue-195-sources.mjs';
import { loadWordNet } from '../scripts/english-vocabulary.mjs';
import {
  Grammar,
  checkGrammarLowering,
  checkGrammarReverseConversion,
  droppedGrammarFeatures,
  grammarEmitter,
  grammarImporter,
  parseGrammarLinks,
  translateProgram,
} from '../src/index.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT_ID = 'I195-ACCEPTANCE-GATE-MUTATIONS';
const rootUrl = new URL('../../', import.meta.url);
const root = fileURLToPath(rootUrl);
const readJson = (relative) => JSON.parse(readFileSync(new URL(relative, rootUrl), 'utf8'));
const manifest = readJson('parity/issue-195-requirements.json');

function record(assertion, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT_ID,
    suffix: 'behavior',
    runtime: 'tooling',
    fixtureId: `planned:repository-directive:${REQUIREMENT_ID.toLowerCase()}`,
    fixtureFile: 'docs/vision.md',
    assertions: [assertion],
    testName,
  });
}

test('issue 195 removing a grammar feature fails the lowering gate', () => {
  const [features] = readJson('parity/fixtures/grammar-importers.json').lowering;
  const grammar = parseGrammarLinks(features.links);
  const { accepts, rejects } = features;
  assert.deepEqual(checkGrammarLowering(grammar, 'gbnf', { accepts, rejects }).failures, []);
  // A lowering whose metadata loses the kind of the `sep` rule.
  const editMetadata = (metadata) => metadata.split('\n').filter((line) => !line.startsWith('(kind sep')).join('\n');
  const report = checkGrammarLowering(grammar, 'gbnf', { accepts, rejects, editMetadata });
  assert.equal(report.status, 'broken');
  assert.ok(report.failures.length > 0 && report.failures.every(({ kind }) => kind === 'feature-dropped'));
  // A grammar that loses a rule is a dropped feature too.
  assert.deepEqual(droppedGrammarFeatures(grammar, grammar), []);
  const rules = new Map([...grammar.rules].filter(([name]) => name !== 'shout'));
  assert.notDeepEqual(droppedGrammarFeatures(grammar, new Grammar(grammar.start, rules, grammar.sourceFormat)), []);
  record('removedGrammarFeatureFails', 'issue 195 removing a grammar feature fails the lowering gate');
});

test('issue 195 leaving an old dependency fails the delivery gate', () => {
  const inventory = readJson('parity/dependency-inventory.json');
  const collectedOf = (items) => items.map(({ current, evidence, reason, ...item }) => item);
  const item = inventory.items.find((entry) =>
    entry.compare === 'version' && !entry.reason && entry.pinned === entry.current && /^\d+\.\d+\.\d+$/u.test(entry.current));
  assert.ok(item, 'a current versioned dependency is available for the mutation');
  const problemsOf = (candidate) => checkDeliveredDependencies(candidate, collectedOf(candidate.items))
    .filter(({ kind, message }) => kind === 'stale-delivered-dependency' && message.startsWith(`${item.id}:`));
  assert.deepEqual(problemsOf(inventory), []);
  const mutation = structuredClone(inventory);
  Object.assign(mutation.items.find(({ id }) => id === item.id), {
    pinned: '0.0.0',
    reason: 'This intentionally old pin must fail delivery despite its recorded compatibility explanation.',
  });
  assert.equal(problemsOf(mutation).length, 1);
  record('staleDependencyFails', 'issue 195 leaving an old dependency fails the delivery gate');
});

test('issue 195 bypassing native execution fails the acceptance gate', () => {
  // A translation row whose target artifact is validated by the native
  // toolchain only when the target compiler or kernel actually runs it.
  const entry = manifest.atomicRequirements.find(({ id }) => id === 'I195-TRANSLATE-javascript-to-lean');
  const cell = entry.verifications.find(({ runtime }) => runtime === 'javascript');
  assert.ok(cell.assertions.includes('nativeTargetValidation'));
  const commit = 'candidate-sha';
  const evaluate = (assertions) => {
    const result = {
      testId: cell.testId,
      outcome: 'passed',
      kind: cell.kind,
      positiveEvidence: true,
      command: 'node --test js/tests/issue-195-translation-pairs.test.js',
      toolchainVersions: { node: process.version },
      grammarVersions: { translationCorpus: 'pinned' },
      evidenceArtifacts: ['issue-195-results/observations.jsonl'],
      assertionsPassed: assertions,
      executionRecords: assertions.flatMap((assertionId) => cell.fixtureIds.map((fixtureId) => ({
        testId: cell.testId, assertionId, fixtureId, fixtureDigest: manifest.fixtureCatalog[fixtureId].sha256,
        runtime: cell.runtime, commit, outcome: 'passed', testName: 'translation pair',
      }))),
      fixtureDigests: Object.fromEntries(cell.fixtureIds.map((fixtureId) => [fixtureId, manifest.fixtureCatalog[fixtureId].sha256])),
    };
    const document = { schemaVersion: 1, issue: 195, commit, producer: 'gate-mutation', generatedAt: new Date(0).toISOString(), results: [result] };
    const single = { ...manifest, atomicRequirements: [{ ...entry, verifications: [cell] }] };
    return evaluateIssue195Acceptance(single, [document], { checkpoint: 'pre-merge', commit });
  };
  assert.equal(evaluate(cell.assertions).passed, true);
  const bypassed = evaluate(cell.assertions.filter((assertion) => assertion !== 'nativeTargetValidation'));
  assert.equal(bypassed.passed, false);
  assert.match(JSON.stringify(bypassed), /observable assertion did not execute: nativeTargetValidation/u);
  record('nativeBypassFails', 'issue 195 bypassing native execution fails the acceptance gate');
});

test('issue 195 breaking reverse conversion fails the round-trip gate', () => {
  const [fixture] = readJson('parity/fixtures/grammar-importers.json').reverse;
  const importGrammar = grammarImporter(fixture.format);
  const emitGrammar = grammarEmitter(fixture.format);
  const options = { importGrammar, accepts: fixture.accepts, rejects: fixture.rejects };
  assert.equal(checkGrammarReverseConversion(fixture.source, { ...options, emitGrammar }).status, 'equivalent');
  // An emitter that reverses every top-level sequence.
  const reversed = (grammar) => {
    const rules = new Map([...grammar.rules].map(([name, rule]) => [name, rule.expression.kind === 'seq'
      ? { ...rule, expression: { ...rule.expression, items: [...rule.expression.items].reverse() } }
      : rule]));
    return emitGrammar(new Grammar(grammar.start, rules, grammar.sourceFormat));
  };
  const report = checkGrammarReverseConversion(fixture.source, { ...options, emitGrammar: reversed });
  assert.equal(report.status, 'different');
  assert.ok(report.failures.some(({ kind }) => kind === 'rules-changed'));
  record('brokenReverseConversionFails', 'issue 195 breaking reverse conversion fails the round-trip gate');
});

test('issue 195 a semantic duplicate or an abbreviation fails the naming gate', () => {
  const wordnet = loadWordNet();
  const context = { wordnet, vocabulary: loadNamingRegisters(root).vocabulary };
  const { records } = checkRepositoryNames(root, wordnet);
  assert.deepEqual(checkConceptRecords(records, context), []);
  // WordNet lists "token" and "item" in one noun synset.
  const item = records.find(({ id }) => id === 'grammar.item');
  const token = { ...structuredClone(item), id: 'grammar.token', phrase: 'token', definition: 'A rule for one lexical unit.' };
  const duplicates = checkConceptRecords([...records, token], context).filter(({ kind }) => kind === 'duplicate');
  assert.match(duplicates.map(({ message }) => message).join('\n'), /"token" is a synonym of "item"/u);
  record('semanticDuplicateFails', 'issue 195 a semantic duplicate or an abbreviation fails the naming gate');

  const renamed = records.map((candidate) =>
    candidate.id === 'grammar.character-class' ? { ...candidate, id: 'grammar.char-class', phrase: 'char class' } : candidate);
  const problems = checkRepositoryNames(root, wordnet, { records: renamed }).problems;
  assert.ok(problems.some(({ kind, id }) => kind === 'abbreviation' && id === 'grammar.char-class'));
  assert.deepEqual(nameProblems('program::module', context), []);
  assert.match(nameProblems('program::mod', context)[0].message, /an ambiguous abbreviation/u);
  record('abbreviationFails', 'issue 195 a semantic duplicate or an abbreviation fails the naming gate');
});

test('issue 195 dropping a source requirement fails the scope and source register gates', () => {
  assert.deepEqual(compareScopeBaseline(manifest, manifest), []);
  const dropped = { ...manifest, atomicRequirements: manifest.atomicRequirements.filter(({ id }) => id !== REQUIREMENT_ID) };
  assert.deepEqual(compareScopeBaseline(manifest, dropped), [`required scope row removed: ${REQUIREMENT_ID}`]);
  // A row that keeps its identity but loses an observable assertion is weaker too.
  const weakened = structuredClone(manifest);
  weakened.atomicRequirements.find(({ id }) => id === REQUIREMENT_ID).verifications[0].assertions.pop();
  assert.match(compareScopeBaseline(manifest, weakened).join('\n'), /assertion removed/u);
  // Dropping a requirement source from the register.
  const register = readJson('parity/issue-195-sources.json');
  assert.deepEqual(validateSourceRegister(register, manifest, ISSUE_195_SOURCES), []);
  const unregistered = structuredClone(register);
  unregistered.requirementSources = unregistered.requirementSources.filter(({ key }) => key !== 'repositoryDirective');
  assert.ok(validateSourceRegister(unregistered, manifest, ISSUE_195_SOURCES)
    .includes('ledger source ISSUE_195_SOURCES.repositoryDirective is not a registered requirement source'));
  record('droppedSourceRequirementFails', 'issue 195 dropping a source requirement fails the scope and source register gates');
});

test('issue 195 weakening an oracle mapping fails the conformance oracle gate', () => {
  const fixtureRoot = new URL('parity/fixtures/issue-195-conformance/', rootUrl);
  const read = (path) => readFileSync(new URL(path, fixtureRoot));
  const readRepository = (path) => readFileSync(new URL(path, rootUrl));
  const conformance = JSON.parse(read('manifest.json'));
  const lock = readJson('js/src/vendor/grammars/grammar-lock.json');
  const problemsOf = (change) => {
    const candidate = structuredClone(conformance);
    change(candidate);
    return conformanceOracleProblems(candidate, { lock, read, readRepository });
  };
  assert.deepEqual(problemsOf(() => {}), []);
  const mutations = [
    // JavaScript compared with the Rust oracle, digest and all.
    [({ languages }) => Object.assign(languages.JavaScript, {
      oracle: languages.Rust.oracle, oracleSha256: languages.Rust.oracleSha256,
    }), /JavaScript oracle rust\/oracle\.json is the Rust oracle/u],
    // JavaScript mapped to the pinned TypeScript grammar.
    [({ languages }) => Object.assign(languages.JavaScript.grammar, {
      id: 'typescript', parserSha256: lock.grammars.typescript.parserSha256,
    }), /JavaScript is mapped to the grammar typescript/u],
    [({ languages }) => { languages.Lean.projects = []; }, /Lean has no real projects/u],
    [({ oracle }) => { oracle.tool = 'tree-sitter 0.20.0'; }, /the oracle tool tree-sitter 0\.20\.0 is not the pinned/u],
    [({ languages }) => { languages.Rocq.oracleSha256 = '0'.repeat(64); }, /Rocq oracle does not match its digest/u],
  ];
  for (const [change, expected] of mutations) assert.match(problemsOf(change).join('\n'), expected);
  record('weakenedOracleMappingFails', 'issue 195 weakening an oracle mapping fails the conformance oracle gate');
});

test('issue 195 replacing proof obligations with bounded checks fails the proof obligation gate', () => {
  const corpus = readJson('parity/fixtures/four-language-conformance.json').translationCorpus;
  const source = readFileSync(new URL(`${corpus.directory}/${corpus.sources.Lean.file}`, rootUrl), 'utf8');
  const relabelled = (translation, change) => ({
    ...translation,
    semantics: { ...translation.semantics, obligations: translation.semantics.obligations.map(change) },
  });
  const proved = translateProgram(source, 'Lean', 'Rocq');
  assert.equal(proved.semantics.obligations.length, corpus.sources.Lean.theorems.length);
  assert.deepEqual(proofObligationProblems(proved, { proofTarget: true }), []);
  // Into Rocq, a theorem relabelled as checked on a bounded domain instead of
  // discharged by the Rocq kernel, with or without keeping the discharger.
  for (const substitute of [{ discharge: 'source-kernel', check: 'bounded' }, { discharge: 'target-kernel', check: 'bounded' }]) {
    const problems = proofObligationProblems(relabelled(proved, (obligation) => ({ ...obligation, ...substitute })), { proofTarget: true });
    assert.equal(problems.length, corpus.sources.Lean.theorems.length, JSON.stringify(substitute));
  }
  // Into Rust a theorem stays proved by the source kernel; a bounded check
  // relabelled as a target proof, or as a runtime assertion, is reported.
  const bounded = translateProgram(source, 'Lean', 'Rust');
  assert.deepEqual(proofObligationProblems(bounded, { proofTarget: false }), []);
  for (const substitute of [{ discharge: 'target-kernel', check: null }, { discharge: 'runtime-assertion', check: null }]) {
    assert.notDeepEqual(proofObligationProblems(relabelled(bounded, (obligation) => ({ ...obligation, ...substitute })), { proofTarget: false }), []);
  }
  // The ledger keeps the kernel discharge as its own observable assertion, so
  // replacing it with the bounded-check assertion reduces the required scope.
  const weakened = structuredClone(manifest);
  const cell = weakened.atomicRequirements.find(({ id }) => id === 'I195-SEMANTICS-PROOF-PRESERVATION').verifications[0];
  cell.assertions = cell.assertions.filter((assertion) => assertion !== 'obligationsDischargedByTarget');
  assert.match(compareScopeBaseline(manifest, weakened).join('\n'), /assertion removed/u);
  record('boundedProofSubstituteFails', 'issue 195 replacing proof obligations with bounded checks fails the proof obligation gate');
});
