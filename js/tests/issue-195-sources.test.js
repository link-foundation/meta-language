// The issue #195 source register (parity/issue-195-sources.json): every
// requirement source is registered with its content hash, an edited, deleted
// or unregistered comment is reported, and the register and the ledger agree.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

import { ISSUE_195_SOURCES } from '../scripts/issue-195-requirements.mjs';
import {
  compareWithLiveDiscussion,
  contentHash,
  isAutomationComment,
  refreshRegister,
  revisionOf,
  validateSourceRegister,
} from '../scripts/issue-195-sources.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REGISTER_FILE = 'parity/issue-195-sources.json';
const repositoryFile = (relative) => new URL(`../../${relative}`, import.meta.url);
const register = JSON.parse(readFileSync(repositoryFile(REGISTER_FILE), 'utf8'));
const manifest = JSON.parse(readFileSync(repositoryFile('parity/issue-195-requirements.json'), 'utf8'));

function observeSourceRegister(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-VISION-SOURCE-REGISTER',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-vision-source-register',
    fixtureFile: REGISTER_FILE,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

// A two-comment discussion and the register reconciled against it.
function syntheticDiscussion() {
  const issue = {
    number: 195, user: { login: 'author' }, body: '# Issue\nrequirements',
    created_at: '2026-01-01T00:00:00Z', updated_at: '2026-01-01T00:00:00Z',
  };
  const comment = (id, body, updated = '2026-01-02T00:00:00Z') => ({
    id, body, user: { login: 'author' }, created_at: '2026-01-02T00:00:00Z', updated_at: updated,
    html_url: `https://github.com/link-foundation/meta-language/pull/196#issuecomment-${id}`,
  });
  const comments = [comment(1, '## Directive\nmore requirements'), comment(2, '## Status\nprogress report')];
  const entry = (id, kind, live, extra) => ({
    id: String(id), kind,
    url: kind === 'issue'
      ? 'https://github.com/link-foundation/meta-language/issues/195'
      : `https://github.com/link-foundation/meta-language/pull/196#issuecomment-${id}`,
    title: 'title', ...revisionOf(live), ...extra,
  });
  const synthetic = {
    schemaVersion: 1,
    repository: 'link-foundation/meta-language',
    requirementSources: [
      entry(195, 'issue', issue, { key: 'issue', role: 'issue body', ledgerCoverage: ['I195-'] }),
      entry(1, 'comment', comments[0], { key: 'directive', role: 'directive', ledgerCoverage: ['I195-'] }),
    ],
    statusReports: [entry(2, 'comment', comments[1], { role: 'status report' })],
  };
  return { issue, comments, register: synthetic };
}

test('the committed source register matches the ledger and every ledger source', () => {
  assert.deepEqual(validateSourceRegister(register, manifest, ISSUE_195_SOURCES), []);
  for (const entry of [...register.requirementSources, ...register.statusReports]) {
    assert.match(entry.sha256, /^[0-9a-f]{64}$/u, `${entry.url} has a sha256 content hash`);
    assert.ok(entry.bytes > 0 && entry.updatedAt, `${entry.url} records its revision`);
  }
  const registeredUrls = new Set(register.requirementSources.map(({ url }) => url));
  for (const row of manifest.atomicRequirements) {
    assert.ok(registeredUrls.has(row.source), `${row.id} cites a registered source`);
  }
  for (const entry of register.requirementSources) {
    for (const prefix of entry.ledgerCoverage) {
      assert.ok(manifest.atomicRequirements.some(({ id }) => id.startsWith(prefix)), `${entry.key} covers ${prefix}`);
    }
  }
  observeSourceRegister(
    ['everySourceRegisteredWithHash', 'everyRowSourceRegistered', 'everySourceCoversLedgerRows'],
    'the committed source register matches the ledger and every ledger source',
  );
});

test('the register check rejects unregistered row sources and uncovered prefixes', () => {
  const rogueSource = 'https://github.com/link-foundation/meta-language/pull/196#issuecomment-1';
  const rogueRow = { ...manifest.atomicRequirements[0], id: 'I195-ROGUE-ROW', source: rogueSource };
  const withRogueRow = { ...manifest, atomicRequirements: [...manifest.atomicRequirements, rogueRow] };
  assert.ok(validateSourceRegister(register, withRogueRow, ISSUE_195_SOURCES)
    .includes(`I195-ROGUE-ROW cites unregistered source ${rogueSource}`));

  const uncovered = structuredClone(register);
  uncovered.requirementSources.find(({ key }) => key === 'cacheCleanup').ledgerCoverage = ['I195-NOTHING-'];
  const errors = validateSourceRegister(uncovered, manifest, ISSUE_195_SOURCES);
  assert.ok(errors.includes('requirement source cacheCleanup maps to I195-NOTHING-, which matches no ledger row'));
  assert.ok(errors.some((error) => /I195-CACHE-CLEANUP-ENTRY-POINT is not covered/u.test(error)));

  const unhashed = structuredClone(register);
  unhashed.requirementSources[0].sha256 = 'edited by hand';
  assert.ok(validateSourceRegister(unhashed, manifest, ISSUE_195_SOURCES)
    .includes('source issue has no sha256 content hash'));

  const missingLedgerSource = structuredClone(register);
  missingLedgerSource.requirementSources = missingLedgerSource.requirementSources
    .filter(({ key }) => key !== 'repositoryDirective');
  assert.ok(validateSourceRegister(missingLedgerSource, manifest, ISSUE_195_SOURCES)
    .includes('ledger source ISSUE_195_SOURCES.repositoryDirective is not a registered requirement source'));
  observeSourceRegister(
    ['everySourceRegisteredWithHash', 'everyRowSourceRegistered', 'everySourceCoversLedgerRows'],
    'the register check rejects unregistered row sources and uncovered prefixes',
  );
});

test('an edited, deleted or unregistered comment is reported and automation output is ignored', () => {
  const { issue, comments, register: synthetic } = syntheticDiscussion();
  assert.deepEqual(compareWithLiveDiscussion(synthetic, { issue, comments }), []);

  const edited = [{ ...comments[0], body: `${comments[0].body}\nand one more`, updated_at: '2026-02-01T00:00:00Z' }, comments[1]];
  const editErrors = compareWithLiveDiscussion(synthetic, { issue, comments: edited });
  assert.equal(editErrors.length, 1);
  assert.match(editErrors[0], /issuecomment-1 was edited at 2026-02-01T00:00:00Z/u);
  // A silent body change with an unchanged timestamp is still caught by the hash.
  const silent = [{ ...comments[0], body: 'rewritten' }, comments[1]];
  assert.match(compareWithLiveDiscussion(synthetic, { issue, comments: silent })[0], /was edited/u);
  assert.match(compareWithLiveDiscussion(synthetic, { issue: { ...issue, body: 'new scope' }, comments })[0], /issues\/195 was edited/u);

  const deleted = compareWithLiveDiscussion(synthetic, { issue, comments: [comments[0]] });
  assert.deepEqual(deleted, ['registered status-report https://github.com/link-foundation/meta-language/pull/196#issuecomment-2 no longer exists']);

  const newcomer = { ...comments[0], id: 3, body: '## New requirement\ndo more', html_url: 'https://example.test/3' };
  const automation = [
    '<!-- hive-mind session -->\nlog', '## 🤖 Solution Draft Log\n...', '🤖 **AI Work Session Started**',
    '## 🔄 Auto-restart 1/3\n...', '## ✅ Ready to merge\n...', '## ⏳ Usage Limit Reached\n...',
    '⏰ **Auto Resume (on limit reset)**\n...', '## ⏰ Auto Resume (on limit reset) 1/5 Log\n...',
  ].map((body, index) => ({ ...comments[0], id: 10 + index, body }));
  const unregistered = compareWithLiveDiscussion(synthetic, { issue, comments: [...comments, newcomer, ...automation] });
  assert.deepEqual(unregistered, [
    'comment https://example.test/3 (New requirement) is not registered; classify it as a requirement source or a status report',
  ]);
  assert.ok(automation.every(({ body }) => isAutomationComment(body)));
  assert.ok(!isAutomationComment('## Repository-wide delivery directive'));

  const refreshed = refreshRegister(synthetic, { issue, comments: edited });
  assert.equal(refreshed.requirementSources[1].sha256, contentHash(edited[0].body));
  assert.deepEqual(compareWithLiveDiscussion(refreshed, { issue, comments: edited }), []);
  observeSourceRegister(
    ['editedSourceDetected', 'deletedSourceDetected', 'unregisteredCommentDetected', 'automationCommentsIgnored'],
    'an edited, deleted or unregistered comment is reported and automation output is ignored',
  );
});
