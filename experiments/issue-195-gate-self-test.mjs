// Synthesizes a passing result for every pre-merge cell and evaluates the
// whole ledger: what does the evaluator still demand beyond the results?
import path from 'node:path';
import { buildIssue195Manifest, evaluateIssue195Acceptance } from '../js/scripts/issue-195-acceptance-lib.mjs';

const root = path.resolve(new URL('..', import.meta.url).pathname);
const manifest = await buildIssue195Manifest(root);
const commit = 'c'.repeat(40);
const results = [];
for (const entry of manifest.atomicRequirements) {
  for (const cell of entry.verifications.filter(({ checkpoint }) => checkpoint === 'pre-merge')) {
    results.push({
      testId: cell.testId, outcome: 'passed', kind: cell.kind, positiveEvidence: true,
      command: 'synthetic', toolchainVersions: { node: process.version }, grammarVersions: { corpus: 'pinned' },
      evidenceArtifacts: ['issue-195-results/observations.jsonl'], assertionsPassed: cell.assertions,
      executionRecords: cell.assertions.flatMap((assertionId) => cell.fixtureIds.map((fixtureId) => ({
        testId: cell.testId, assertionId, fixtureId, fixtureDigest: manifest.fixtureCatalog[fixtureId].sha256,
        runtime: cell.runtime, commit, outcome: 'passed', testName: 'synthetic',
      }))),
      fixtureDigests: Object.fromEntries(cell.fixtureIds.map((id) => [id, manifest.fixtureCatalog[id].sha256])),
    });
  }
}
const document = { schemaVersion: 1, issue: 195, commit, producer: 'self-test', generatedAt: new Date(0).toISOString(), results };
const report = evaluateIssue195Acceptance(manifest, [document], { checkpoint: 'pre-merge', commit });
console.log('passed', report.passed, 'rows', report.requirements.length, 'cells', results.length);
const reasons = new Map();
for (const row of report.requirements.filter(({ passed }) => !passed)) {
  for (const reason of [...(row.reasons ?? []), ...row.cells.flatMap((cell) => cell.reasons)]) {
    reasons.set(reason.replace(/I195-[A-Z0-9-]+|[0-9a-f]{40}/g, '*'), (reasons.get(reason.replace(/I195-[A-Z0-9-]+|[0-9a-f]{40}/g, '*')) ?? 0) + 1);
  }
}
console.log([...reasons].sort((a, b) => b[1] - a[1]).slice(0, 30));
console.log(Object.keys(report).join(' '), JSON.stringify(report.gateErrors ?? report.errors ?? null).slice(0, 500));
