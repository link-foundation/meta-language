// Issue 195 four-language conformance: the upstream tree-sitter corpora, real-project
// files and hand-authored construct, Unicode, malformed and mixed-language inputs of
// parity/fixtures/issue-195-conformance/, each compared with the tree the native
// tree-sitter CLI printed for it (js/scripts/generate-issue-195-conformance.mjs):
// structure, kinds, fields, spans, error and missing nodes, trivia, diagnostics and
// exact reconstruction. Where the upstream authors' expected tree agrees with the
// CLI, the public tree must equal it too.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { LinkNetwork } from '../src/index.js';
import {
  cstLinesToSexp,
  diagnosticProblems,
  documentOracleProblems,
  firstDifference,
  parseCstLines,
  regionGrammarRoots,
  renderCstLines,
  triviaProblems,
} from './support/cst-lines.js';
import { normalize, parseCorpus, stripFields } from './support/cst-sexpression.js';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const MANIFEST_FILE = 'parity/fixtures/issue-195-conformance/manifest.json';
const fixtureRoot = new URL('../../parity/fixtures/issue-195-conformance/', import.meta.url);
const repositoryRoot = new URL('../../', import.meta.url);
const read = (path) => readFileSync(new URL(path, fixtureRoot));
const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
const manifest = JSON.parse(read('manifest.json'));
const inputs = JSON.parse(read('cases.json'));
const errorRecovery = JSON.parse(read('error-recovery.json'));
const lock = JSON.parse(readFileSync(new URL('js/src/vendor/grammars/grammar-lock.json', repositoryRoot)));

const ASSERTIONS = [
  'claimedConstructInventoryMapped',
  'upstreamCorpusExecuted',
  'representativeRealProjectExecuted',
  'externalCorpusProvenanceRecorded',
  'malformedRecoveryCasesExecuted',
  'unicodeCasesExecuted',
  'mixedLanguageCasesExecuted',
];

/** The source of every oracle case, taken from the vendored inputs. */
function caseSources(language, details) {
  const sources = new Map();
  for (const file of Object.keys(details.corpus.files)) {
    parseCorpus(read(`${details.corpus.directory}/${file}`).toString('utf8'))
      .forEach((corpusCase, index) => sources.set(`corpus/${file}/${index}`, corpusCase));
  }
  for (const project of details.projects) {
    sources.set(`project/${project.file.split('/').pop()}`, { source: read(project.file).toString('utf8') });
  }
  for (const host of inputs.mixed) sources.set(`host/${host.id}`, { source: host.source });
  return sources;
}

/** Problems of the public tree of one non-mixed case against its oracle tree. */
function documentProblems(language, entry, source, corpusCase) {
  const network = LinkNetwork.parse(source, language);
  const { problems, text } = documentOracleProblems(network, language, source, entry.cst);
  if (entry.upstream === 'match') {
    const expected = normalize(corpusCase.expected);
    let actual = normalize(cstLinesToSexp(text));
    if (!/\w: /u.test(expected)) actual = stripFields(actual);
    if (actual !== expected) problems.push(`upstream expected tree differs:\n  actual   ${actual}\n  expected ${expected}`);
  }
  if (entry.upstream === 'error-expected' && network.verifyFullMatch().isClean()) {
    problems.push('the upstream corpus expects an error, but the network verifies clean');
  }
  return problems;
}

/** Problems of the public region tree of one mixed-language case against its oracle tree. */
function regionProblems(language, entry, host) {
  const network = LinkNetwork.parse(host.source, host.host);
  const region = regionGrammarRoots(network).find((candidate) => candidate.language === language &&
    candidate.span.byteRange.start === entry.startByte && candidate.span.byteRange.end === entry.endByte);
  if (!region) return [`no ${language} region at ${entry.startByte}..${entry.endByte} of the ${host.host} host`];
  const { text, rendered } = renderCstLines(region, language);
  const problems = [];
  if (text !== entry.cst) problems.push(`region CST differs at ${firstDifference(text, entry.cst)}`);
  if (network.reconstructText() !== host.source) problems.push('reconstruction differs from the host source');
  problems.push(...triviaProblems(network, host.source, entry.cst, [entry.startByte, entry.endByte]));
  problems.push(...diagnosticProblems(network, rendered, entry.cst, { checkClean: false }));
  return problems;
}

test('the conformance fixtures record their provenance and match their pinned inputs', () => {
  assert.equal(manifest.oracle.tool, lock.treeSitterCli);
  assert.equal(manifest.inputs['cases.json'], sha256(read('cases.json')));
  assert.equal(manifest.inputs['error-recovery.json'], sha256(read('error-recovery.json')));
  for (const [language, details] of Object.entries(manifest.languages)) {
    assert.equal(details.grammar.parserSha256, lock.grammars[details.grammar.id].parserSha256, language);
    assert.match(details.grammar.revision, /^[0-9a-f]{40}$/u, language);
    assert.ok(readFileSync(new URL(details.grammar.license, repositoryRoot)).length > 0, `${language} grammar license`);
    assert.equal(sha256(read(details.oracle)), details.oracleSha256, `${language} oracle`);
    for (const [file, digest] of Object.entries(details.corpus.files)) {
      assert.equal(sha256(read(`${details.corpus.directory}/${file}`)), digest, `${language} ${file}`);
    }
    assert.ok(details.projects.length > 0, `${language} has real projects`);
    for (const project of details.projects) {
      assert.match(project.commit, /^[0-9a-f]{40}$/u);
      assert.ok(project.url.includes(project.commit), project.url);
      assert.equal(sha256(read(project.file)), project.sha256, project.file);
      assert.ok(read(project.licenseFile).length > 0, project.licenseFile);
    }
  }
});

test('the comparison rejects a tree that differs from the oracle', () => {
  const language = 'JavaScript';
  const oracle = JSON.parse(read(manifest.languages[language].oracle));
  const malformed = oracle.cases.find((entry) => entry.kind === 'malformed');
  const source = malformed.source;
  const cleanOracle = malformed.cst.replaceAll('•', '').replace(/ERROR /u, 'program ');
  assert.notDeepEqual(documentProblems(language, { ...malformed, cst: cleanOracle }, source), []);
  const shifted = malformed.cst.replace(/(\d+):(\d+)$/mu, (_, row, column) => `${row}:${Number(column) + 1}`);
  assert.notDeepEqual(documentProblems(language, { ...malformed, cst: shifted }, source), []);
});

for (const [language, details] of Object.entries(manifest.languages)) {
  const testName = `${language} matches the native CLI trees of its upstream corpus, real projects, Unicode, malformed and mixed-language cases`;
  test(testName, () => {
    const oracle = JSON.parse(read(details.oracle));
    assert.equal(oracle.language, language);
    const sources = caseSources(language, details);
    const counts = {};
    const failures = [];
    for (const entry of oracle.cases) {
      counts[entry.kind] = (counts[entry.kind] ?? 0) + 1;
      let problems;
      if (entry.kind === 'mixed') {
        const host = inputs.mixed.find((candidate) => candidate.id === entry.hostCase);
        assert.equal(sha256(host.source), entry.sourceSha256, entry.id);
        problems = regionProblems(language, entry, host);
      } else {
        const corpusCase = sources.get(entry.id);
        const source = entry.source ?? corpusCase.source;
        assert.equal(sha256(source), entry.sourceSha256, entry.id);
        problems = documentProblems(language, entry, source, corpusCase);
      }
      if (problems.length) failures.push(`${entry.id}${entry.name ? ` (${entry.name})` : ''}: ${problems.join('; ')}`);
    }
    assert.deepEqual(failures, []);
    assert.deepEqual(counts, details.cases);

    // Every case family is present, and each exercises what it claims.
    const byKind = (kind) => oracle.cases.filter((entry) => entry.kind === kind);
    assert.ok(byKind('corpus').length >= Object.keys(details.corpus.files).length);
    assert.ok(byKind('corpus').some((entry) => entry.upstream === 'match'), 'upstream expected trees are checked');
    assert.equal(byKind('project').length, details.projects.length);
    assert.ok(byKind('malformed').length > 0 && byKind('malformed').every((entry) => !entry.clean));
    const unicode = byKind('unicode');
    assert.ok(unicode.length > 0 && unicode.every((entry) => /[\u{10000}-\u{10FFFF}]/u.test(entry.source)));
    assert.ok(byKind('mixed').length > 0 && byKind('mixed').some((entry) => entry.clean));
    assert.ok(byKind('construct').every((entry) => entry.clean));
    const expectedInline = [...inputs.constructs, ...inputs.unicode, ...inputs.malformed]
      .filter((entry) => entry.language === language).length +
      errorRecovery.cases.filter((entry) => entry.language === language).length;
    assert.equal(byKind('construct').length + byKind('unicode').length + byKind('malformed').length, expectedInline);

    // The claimed construct inventory: every visible named kind of the grammar occurs in a checked tree.
    const seen = new Set(oracle.cases.flatMap((entry) => parseCstLines(entry.cst).map((node) => node.kind)));
    assert.deepEqual(details.inventory.filter((kind) => !seen.has(kind)), [], `${language} kinds without a conformance case`);

    recordIssue195Observations({
      requirementId: `I195-CONFORMANCE-${language.toUpperCase()}`,
      suffix: 'positive-and-negative',
      fixtureId: `planned:conformance:${language}`,
      fixtureFile: MANIFEST_FILE,
      assertions: ASSERTIONS,
      testName,
    });
  });
}
