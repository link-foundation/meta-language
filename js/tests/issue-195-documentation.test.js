// The documentation check (docs/vision.md#documentation-consistency): every
// documentation file, package README, changelog fragment and case study, the
// generated ledger, the pull request description and the release reports agree
// with the requirement ledger. Future work, approximate round trips, external
// production parsers and an incomplete scope are never presented as the
// finished contract, case studies carry the historical label, and no text
// reports completion or carries an issue-closing directive before every
// requirement is verified and delivered.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';

import {
  auditDocumentation,
  auditText,
  CASE_STUDIES,
  completionState,
  documentationFiles,
  documentKind,
  DOCUMENTATION_FIXTURES,
  HISTORICAL_LABEL,
  unlabeledCaseStudies,
} from '../scripts/issue-195-documentation.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const root = fileURLToPath(new URL('../../', import.meta.url));
const manifest = JSON.parse(readFileSync(path.join(root, 'parity/issue-195-requirements.json'), 'utf8'));
const fixtures = JSON.parse(readFileSync(path.join(root, DOCUMENTATION_FIXTURES), 'utf8'));
const current = completionState(manifest);

function observe(requirementId, assertions, testName) {
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: DOCUMENTATION_FIXTURES,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

/** The ledger with every row implemented and an acceptance report over every checkpoint that passed them all. */
function deliveredState() {
  const rows = manifest.atomicRequirements.map((row) => ({
    ...row,
    implementationEntryPoints: Object.fromEntries(Object.keys(row.implementationEntryPoints ?? {}).map((runtime) => [runtime, ['entry']])),
  }));
  const report = {
    checkpoint: 'all',
    passed: true,
    summary: { requirements: rows.length },
    requirements: rows.map(({ id }) => ({ id, passed: true })),
  };
  return completionState({ ...manifest, atomicRequirements: rows }, report);
}

const claims = (text, kind = 'document', state = current) => auditText(text, { file: 'fixture.md', kind, state }).map(({ claim }) => claim);

/** A temporary git repository holding the given Markdown files. */
function repositoryWith(t, files) {
  const directory = mkdtempSync(path.join(tmpdir(), 'documentation-'));
  t.after(() => rmSync(directory, { recursive: true, force: true }));
  for (const [file, text] of Object.entries(files)) {
    mkdirSync(path.dirname(path.join(directory, file)), { recursive: true });
    writeFileSync(path.join(directory, file), text);
  }
  execFileSync('git', ['init', '-q'], { cwd: directory });
  execFileSync('git', ['add', '.'], { cwd: directory });
  return directory;
}

test('the ledger is open, so completion claims are contradictions today', () => {
  assert.equal(current.requirements, manifest.atomicRequirements.length);
  assert.equal(current.complete, false);
  assert.equal(current.verified, false, 'no acceptance report, no verification');
  assert.ok(current.unimplemented.length > 0);
  // A report over a single checkpoint, or over fewer rows, never verifies the work.
  const partial = { checkpoint: 'push', passed: true, summary: { requirements: current.requirements }, requirements: [] };
  assert.equal(completionState(manifest, partial).verified, false);
  const failing = completionState(manifest, { checkpoint: 'all', passed: false, summary: { requirements: current.requirements }, requirements: [{ id: 'I195-X', passed: false }] });
  assert.deepEqual(failing.failing, ['I195-X']);
  assert.ok(failing.open.has('I195-X'));
  assert.equal(deliveredState().complete, true);
});

test('every tracked document is audited and the repository documentation agrees with the ledger', () => {
  const files = documentationFiles(root);
  const tracked = execFileSync('git', ['ls-files', '--', '*.md'], { cwd: root, encoding: 'utf8' }).split('\n').filter(Boolean);
  const ours = tracked.filter((file) => !/(^|\/)(vendor|node_modules)\//u.test(file) && !file.startsWith('rust/benches/corpora/'));
  assert.deepEqual(files, [...ours].sort());
  for (const file of ['README.md', 'docs/vision.md', 'docs/issue-195-requirement-ledger.md', 'rust/CHANGELOG.md', 'AGENTS.md']) {
    assert.ok(files.includes(file), `${file} is audited`);
  }
  assert.ok(files.some((file) => file.startsWith('rust/changelog.d/')), 'changelog fragments are audited');
  assert.ok(files.some((file) => file.startsWith(`${CASE_STUDIES}/`)), 'case studies are audited');
  assert.equal(documentKind('docs/vision.md'), 'contract');
  assert.equal(documentKind('docs/issue-195-requirement-ledger.md'), 'ledger');
  assert.equal(documentKind(`${CASE_STUDIES}/issue-10/README.md`), 'case-study');
  assert.equal(documentKind('README.md'), 'document');
  const { problems } = auditDocumentation(root, current);
  assert.deepEqual(problems, [], 'the repository documentation agrees with the open ledger');
  observe('I195-VISION-CONTRADICTION-AUDIT', ['everyDocumentAudited'], 'every tracked document is audited and the repository documentation agrees with the ledger');
});

test('future work, approximate round trips, external parsers and an incomplete scope are rejected as delivered', () => {
  for (const { kind, claim, text } of fixtures.rejected) {
    assert.ok(claims(text, kind).includes(claim), `${kind} ${JSON.stringify(text)} is rejected as ${claim}`);
  }
  for (const { kind, text } of fixtures.accepted) {
    assert.deepEqual(claims(text, kind), [], `${kind} ${JSON.stringify(text)} is accepted`);
  }
  // Once every row is delivered and verified, the same claims are true, except
  // an approximate round trip, which contradicts the specification whatever
  // the ledger says.
  const delivered = deliveredState();
  for (const { kind, claim, text } of fixtures.rejected) {
    const expected = claim === 'approximate-round-trips' ? [claim] : [];
    assert.deepEqual(claims(text, kind, delivered).filter((found) => found !== 'full-pass-count'), expected, JSON.stringify(text));
  }
  observe(
    'I195-VISION-CONTRADICTION-AUDIT',
    ['futureWorkNotPresentedAsDelivered', 'contradictionFixturesRejected'],
    'future work, approximate round trips, external parsers and an incomplete scope are rejected as delivered',
  );
});

test('every case study carries the historical label', (t) => {
  const files = documentationFiles(root);
  assert.deepEqual(unlabeledCaseStudies(root, files), []);
  const repository = repositoryWith(t, {
    [`${CASE_STUDIES}/issue-1/README.md`]: `# Issue 1\n\n${HISTORICAL_LABEL} Written when issue #1 was open.\n`,
    [`${CASE_STUDIES}/issue-2/README.md`]: '# Issue 2\n\nThe investigation.\n',
    [`${CASE_STUDIES}/issue-3/notes.md`]: '# Notes without a README\n',
  });
  const { problems } = auditDocumentation(repository, current);
  assert.deepEqual(
    problems.filter(({ claim }) => claim === 'historical-label').map(({ file }) => file),
    [`${CASE_STUDIES}/issue-2/README.md`, `${CASE_STUDIES}/issue-3/README.md`],
  );
  observe('I195-VISION-CONTRADICTION-AUDIT', ['caseStudiesLabeledHistorical'], 'every case study carries the historical label');
});

test('the README and the vision agree with the ledger', (t) => {
  for (const file of ['README.md', 'docs/vision.md', 'docs/issue-195-requirement-ledger.md']) {
    const text = readFileSync(path.join(root, file), 'utf8');
    assert.deepEqual(auditText(text, { file, kind: documentKind(file), state: current }), [], file);
  }
  const readme = readFileSync(path.join(root, 'README.md'), 'utf8');
  const repository = repositoryWith(t, {
    'README.md': `${readme}\nMeta-language now fully implements the vision.\n`,
    'docs/vision.md': '# Vision\n\nThe whole specification is delivered.\n',
  });
  const found = auditDocumentation(repository, current).problems.map(({ file, claim }) => `${file} ${claim}`);
  assert.deepEqual(found, ['README.md fully-implemented', 'docs/vision.md overall-completion']);
  observe('I195-DOCUMENTATION-RECONCILED', ['readmeConsistentWithLedger'], 'the README and the vision agree with the ledger');
});

test('a pull request description reports no completion and no closing directive before delivery', () => {
  const body = [
    '## Summary',
    '',
    `Acceptance: ${current.requirements}/${current.requirements} requirements passing.`,
    '',
    'Fixes #195',
  ].join('\n');
  const external = [{ name: 'pull request #196 description', text: body }];
  const open = auditDocumentation(root, current, { external, files: [] }).problems;
  assert.deepEqual(open.map(({ line, claim }) => `${line} ${claim}`), ['3 full-pass-count', '5 closing-directive']);
  assert.deepEqual(auditDocumentation(root, deliveredState(), { external, files: [] }).problems, [], 'true once delivered');
  const honest = `Acceptance: ${current.requirements - current.open.size}/${current.requirements} requirements passing; the rest remain open.\n\nIssue #195 stays open until every requirement is verified and delivered.\n`;
  assert.deepEqual(auditText(honest, { file: 'body', state: current }), []);
  // A pass count over fewer rows than the ledger is stale even when delivered.
  assert.deepEqual(claims('Acceptance: 219/219 requirements passing.', 'document', deliveredState()), ['full-pass-count']);
  observe(
    'I195-DOCUMENTATION-RECONCILED',
    ['pullRequestBodyConsistentWithLedger', 'noClosingDirectiveBeforeDelivery'],
    'a pull request description reports no completion and no closing directive before delivery',
  );
});

test('release reports, changelogs and the generated ledger agree with the ledger', (t) => {
  const release = { name: 'release v1.0.0', text: 'meta-language 1.0.0\n\nIssue #195 is complete, with native parsers for every language.' };
  const found = auditDocumentation(root, current, { external: [release], files: [] }).problems.map(({ claim }) => claim);
  assert.deepEqual(found, ['overall-completion']);
  const repository = repositoryWith(t, {
    'rust/CHANGELOG.md': '# Changelog\n\n## 1.0.0\n\n- Every catalog language is parsed by native merged grammars.\n',
    'rust/changelog.d/20260929_000000_merge.md': '### Added\n\n- The pipeline automatically merges every source grammar.\n',
    'docs/issue-195-requirement-ledger.md': '# Ledger\n\nAcceptance result: **PASS**\n',
  });
  const changelogs = auditDocumentation(repository, current).problems.map(({ file, claim }) => `${file} ${claim}`);
  assert.deepEqual(changelogs, [
    'docs/issue-195-requirement-ledger.md ledger-pass',
    'rust/CHANGELOG.md external-production-parsers',
    'rust/changelog.d/20260929_000000_merge.md automatic-merge',
  ]);
  observe('I195-DOCUMENTATION-RECONCILED', ['releaseReportsConsistentWithLedger'], 'release reports, changelogs and the generated ledger agree with the ledger');
});
