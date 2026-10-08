// The audit of the current relative-meta-logic pull request 184
// (docs/downstream-consumers.md, "Current RML pull request 184"): the section
// pins the pull request head, links every file of that head that reaches
// meta-language, and maps each workload to ledger rows with executable
// acceptance cells. In acceptance mode the inventory is derived again from
// verified immutable GitHub blobs. --online compares the live head separately
// in the non-blocking post-merge report on main.
import assert from 'node:assert/strict';
import { test } from 'node:test';

import {
  RML_PR184_FIXTURE,
  gitBlob,
  inventoryWorkloads,
  loadRmlPr184Audit,
  parseRmlPr184Audit,
  readRmlPr184,
  validateRmlPr184Audit,
} from '../scripts/issue-195-rml-pr184.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const { fixture, audit, context } = loadRmlPr184Audit();
const acceptance = Boolean(process.env.ISSUE_195_OBSERVATION_FILE);

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: 'I195-DOWNSTREAM-RML-PR184-AUDIT',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-downstream-rml-pr184-audit',
    fixtureFile: 'docs/downstream-consumers.md',
    assertions,
    testName,
    runtime: 'tooling',
  });
}

const noProblems = {
  currentPullRequestRevisionPinned: [],
  actualWorkloadsInventoried: [],
  eachWorkloadMappedToExecutableAcceptance: [],
};
const head = 'f'.repeat(40);
const blob = (file) => `https://github.com/link-foundation/relative-meta-logic/blob/${head}/${file}`;
// An audit section over one source and one test, mapped to the ledger row `row`.
const sectionWith = ({ row = 'I195-DOWNSTREAM-RML-WORKLOADS', revision = head, scope = head, sourceLink = blob('js/src/facade.mjs') } = {}) => [
  '## Audit scope', '',
  `- [#184](https://github.com/link-foundation/relative-meta-logic/pull/184) was open, head \`${scope}\`.`, '',
  '## relative-meta-logic', '',
  '### Current RML pull request 184', '',
  `This table inspects \`${revision}\`, which pins meta-language \`^0.46.0\` and 0.58.2.`, '',
  '| Consumer usage or requirement | meta-language capability | Ledger rows | Tests | Status |',
  '|---|---|---|---|---|',
  `| Round trip | [facade][source] | \`${row}\` | [tests][test]; \`js/tests/core.test.js\` | not yet verified |`, '',
  `[source]: ${sourceLink}`,
  `[test]: ${blob('js/tests/facade.test.mjs')}`, '',
  '## formal-ai', '',
].join('\n');
const syntheticFixture = {
  repository: 'link-foundation/relative-meta-logic',
  pullRequest: 184,
  headRevision: head,
  dependencies: { javascript: '^0.46.0', rust: '0.58.2' },
  workloads: [
    { file: 'js/src/facade.mjs', kind: 'source', blob: '0'.repeat(40) },
    { file: 'js/tests/facade.test.mjs', kind: 'test', blob: '0'.repeat(40) },
    { file: 'rust/src/facade.rs', kind: 'source', blob: '0'.repeat(40) },
    { file: 'rust/tests/facade.rs', kind: 'test', blob: '0'.repeat(40) },
  ],
};
// The synthetic section also links the Rust workloads.
const syntheticSection = (options) => sectionWith(options).replace(
  '| Round trip |',
  `| Rust | [facade][rust-source], [tests][rust-test] | \`I195-DOWNSTREAM-RML-WORKLOADS\` | \`js/tests/core.test.js\` | not yet verified |\n| Round trip |`,
).replace('\n## formal-ai', `[rust-source]: ${blob('rust/src/facade.rs')}\n[rust-test]: ${blob('rust/tests/facade.rs')}\n\n## formal-ai`);
const validate = (markdown, workloads = syntheticFixture) => validateRmlPr184Audit(parseRmlPr184Audit(markdown), workloads, context);

test('the audit section maps every pull request 184 workload to executable acceptance rows', () => {
  assert.equal(fixture.pullRequest, 184);
  assert.match(fixture.headRevision, /^[0-9a-f]{40}$/u);
  assert.ok(audit.mappings.length > 0);
  assert.deepEqual(validateRmlPr184Audit(audit, fixture, context), noProblems);
  for (const kind of ['source', 'test']) {
    for (const runtime of ['js/', 'rust/']) {
      assert.ok(fixture.workloads.some((workload) => workload.kind === kind && workload.file.startsWith(runtime)), `${runtime} ${kind}`);
    }
  }
  observe(['eachWorkloadMappedToExecutableAcceptance'], 'the audit section maps every pull request 184 workload to executable acceptance rows');
});

test('the pinned pull request revision and its workloads match the GitHub tree', {
  skip: acceptance ? false : 'reads GitHub; runs in the issue 195 acceptance suite',
}, async () => {
  const live = await readRmlPr184(fixture.headRevision, { live: false });
  assert.equal(live.head, fixture.headRevision, `the GitHub tree must match the revision in ${RML_PR184_FIXTURE}`);
  assert.deepEqual(live.workloads, fixture.workloads);
  assert.deepEqual(validateRmlPr184Audit(audit, { ...fixture, workloads: live.workloads }, context), noProblems);
  observe(
    ['currentPullRequestRevisionPinned', 'actualWorkloadsInventoried'],
    'the pinned pull request revision and its workloads match the GitHub tree',
  );
});

test('the inventory follows imports from the package to sources and tests in both runtimes', () => {
  const files = new Map([
    ['js/src/facade.mjs', "import { LinkNetwork } from 'meta-language';\n"],
    ['js/src/network.mjs', "import { parse } from './facade.mjs';\n"],
    ['js/src/unrelated.mjs', "import { other } from './links.mjs';\n"],
    ['js/src/links.mjs', 'export const other = 1;\n'],
    ['js/tests/network.test.mjs', "import { network } from '../src/network.mjs';\n"],
    ['js/tests/links.test.mjs', "import { other } from '../src/links.mjs';\n"],
    ['rust/src/support.rs', 'use meta_language::{LinkNetwork};\n'],
    ['rust/src/corpus.rs', 'use crate::support::parse;\n'],
    ['rust/src/lib.rs', 'pub mod support;\npub mod corpus;\n'],
    ['rust/tests/corpus_tests.rs', 'use rml::corpus::read;\n'],
    ['rust/tests/other_tests.rs', 'use rml::links::read;\n'],
    ['docs/meta-language.md', "import 'meta-language';\n"],
  ]);
  const inventory = inventoryWorkloads(files);
  assert.deepEqual(inventory.map(({ file, kind }) => `${kind} ${file}`), [
    'source js/src/facade.mjs',
    'source js/src/network.mjs',
    'test js/tests/network.test.mjs',
    'source rust/src/corpus.rs',
    'source rust/src/support.rs',
    'test rust/tests/corpus_tests.rs',
  ]);
  assert.equal(inventory[0].blob, gitBlob("import { LinkNetwork } from 'meta-language';\n"));
  // `git hash-object` of the empty file.
  assert.equal(gitBlob(''), 'e69de29bb2d1d6434b8b29ae775ad8c2e48c5391');
});

test('the audit rejects a stale or inconsistent pin, an unlinked workload and an unmapped row', () => {
  assert.deepEqual(validate(syntheticSection()), noProblems);

  const moved = 'e'.repeat(40);
  assert.equal(validate(syntheticSection({ revision: moved })).currentPullRequestRevisionPinned.length, 1);
  assert.equal(validate(syntheticSection({ scope: moved })).currentPullRequestRevisionPinned.length, 1);
  const staleLink = validate(syntheticSection({ sourceLink: blob('js/src/facade.mjs').replace(head, moved) }));
  assert.equal(staleLink.currentPullRequestRevisionPinned.length, 1);
  assert.deepEqual(staleLink.actualWorkloadsInventoried, ['no workload of the audit links js/src/facade.mjs at the pinned head']);
  assert.equal(validate(syntheticSection(), { ...syntheticFixture, dependencies: { javascript: '^0.58.2' } })
    .currentPullRequestRevisionPinned.length, 1);

  const added = { ...syntheticFixture, workloads: [...syntheticFixture.workloads, { file: 'js/src/theory.mjs', kind: 'source', blob: '0'.repeat(40) }] };
  assert.deepEqual(validate(syntheticSection(), added).actualWorkloadsInventoried, ['no workload of the audit links js/src/theory.mjs at the pinned head']);
  const withoutRust = { ...syntheticFixture, workloads: syntheticFixture.workloads.filter(({ file }) => file.startsWith('js/')) };
  assert.equal(validate(syntheticSection(), withoutRust).actualWorkloadsInventoried.length, 2);

  assert.deepEqual(validate(syntheticSection({ row: 'I195-DOWNSTREAM-RML-MISSING' })).eachWorkloadMappedToExecutableAcceptance,
    ['Round trip maps the missing ledger row I195-DOWNSTREAM-RML-MISSING']);
  assert.deepEqual(validate(syntheticSection().replace('`js/tests/core.test.js` | not yet verified |\n|', '`js/tests/missing.test.js` | not yet verified |\n|'))
    .eachWorkloadMappedToExecutableAcceptance, ['Rust maps the missing test js/tests/missing.test.js']);
  assert.ok(Object.values(validateRmlPr184Audit(null, syntheticFixture, context)).every((problems) => problems.length === 1));
});

test('reading the live pull request rejects contents that do not match the tree', async () => {
  const text = "import { LinkNetwork } from 'meta-language';\n";
  const responses = (sha) => ({
    'https://api.github.com/repos/link-foundation/relative-meta-logic/pulls/184': { head: { sha: head }, state: 'open' },
    [`https://api.github.com/repos/link-foundation/relative-meta-logic/git/trees/${head}?recursive=1`]: {
      truncated: false,
      tree: [{ type: 'blob', path: 'js/src/facade.mjs', sha }, { type: 'blob', path: 'README.md', sha: '0'.repeat(40) }],
    },
    [`https://raw.githubusercontent.com/link-foundation/relative-meta-logic/${head}/js/src/facade.mjs`]: text,
  });
  const fetchFrom = (table) => async (url) => {
    assert.ok(url in table, url);
    const body = table[url];
    return {
      ok: true,
      json: async () => body,
      arrayBuffer: async () => new TextEncoder().encode(body).buffer,
    };
  };
  const live = await readRmlPr184(head, { fetch: fetchFrom(responses(gitBlob(text))) });
  assert.deepEqual(live, { head, state: 'open', workloads: [{ file: 'js/src/facade.mjs', kind: 'source', blob: gitBlob(text) }] });
  await assert.rejects(readRmlPr184(head, { fetch: fetchFrom(responses('1'.repeat(40))) }), /does not match its tree blob/u);
});

test('the pre-merge inventory uses immutable blobs even when the live pull request has moved', async () => {
  const text = "import { LinkNetwork } from 'meta-language';\n";
  const requests = [];
  const moved = 'e'.repeat(40);
  const fetchPinned = async (url) => {
    requests.push(url);
    if (url.endsWith('/pulls/184')) {
      return { ok: true, json: async () => ({ head: { sha: moved }, state: 'open' }) };
    }
    if (url.includes('/git/trees/')) {
      assert.ok(url.includes(head));
      return { ok: true, json: async () => ({ truncated: false, tree: [
        { type: 'blob', path: 'js/src/facade.mjs', sha: gitBlob(text) },
      ] }) };
    }
    assert.equal(url, `https://raw.githubusercontent.com/link-foundation/relative-meta-logic/${head}/js/src/facade.mjs`);
    return { ok: true, arrayBuffer: async () => new TextEncoder().encode(text).buffer };
  };
  assert.deepEqual(await readRmlPr184(head, { fetch: fetchPinned, live: false }), {
    head, workloads: [{ file: 'js/src/facade.mjs', kind: 'source', blob: gitBlob(text) }],
  });
  assert.equal(requests.length, 2);
  assert.ok(requests.every((url) => !url.endsWith('/pulls/184')));
  const live = await readRmlPr184(head, { fetch: fetchPinned });
  assert.equal(live.head, moved, 'the post-merge reader still exposes a moved live head');
  assert.notEqual(live.head, head);
  const corrupted = async (url) => url.includes('/git/trees/')
    ? { ok: true, json: async () => ({ truncated: false, tree: [
      { type: 'blob', path: 'js/src/facade.mjs', sha: '1'.repeat(40) },
    ] }) }
    : fetchPinned(url);
  await assert.rejects(readRmlPr184(head, { fetch: corrupted, live: false }), /does not match its tree blob/u);
});
