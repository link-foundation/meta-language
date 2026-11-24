// The dependency inventory: every dependency, toolchain, action, image and
// downloaded asset the repository declares is in the dependency inventory
// with its current stable release, the audit is dated, and a retained item
// behind that release without a recorded compatibility reason fails the check
// (requirement I195-DEPENDENCY-INVENTORY).
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import {
  AUDIT_DOCUMENT,
  CATEGORIES,
  INVENTORY_FILE,
  checkInventory,
  collectDependencies,
  comparePypiVersions,
  couplingResolvers,
  deliveredStatuses,
  ownStatus,
  pinnedPipPackages,
  refreshInventory,
  reconcileLocalDeclarations,
  registryHttp,
  releaseResolvers,
  renderAuditDocument,
  statuses,
} from '../scripts/dependency-inventory.mjs';
import { readProjectData } from '../scripts/project-data.mjs';
import { checkDependencyFreshness, refreshDependencyBlockers } from '../scripts/dependency-freshness.mjs';
import { parseWorkflow } from '../../scripts/check-cache-policy.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-DEPENDENCY-INVENTORY';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const inventory = readProjectData(path.join(root, INVENTORY_FILE));
const TODAY = '2026-09-29';

function observe(assertions, testName) {
  recordIssue195Observations({
    requirementId: REQUIREMENT,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${REQUIREMENT.toLowerCase()}`,
    fixtureFile: INVENTORY_FILE,
    assertions,
    testName,
    runtime: 'tooling',
  });
}

const kinds = (problems) => problems.map(({ kind }) => kind);

test('dependency inventory retains runner declarations from inactive native and consumer platforms', () => {
  const workflows = ['.github/workflows/ci.yml', '.github/workflows/rust.yml'];
  const collected = collectDependencies(root);
  for (const name of ['macos-latest', 'windows-latest', 'ubuntu-latest']) {
    const runner = collected.find(item => item.category === 'runner' && item.name === name);
    assert.ok(runner, name);
    assert.equal(runner.compare, 'floating', name);
    for (const workflow of workflows) assert.ok(runner.declaredIn.includes(workflow), `${name}: ${workflow}`);
  }
});

test('main dependency refresh retains failed live findings and delivers their outcome to repair pull requests', () => {
  const text = readFileSync(path.join(root, '.github/workflows/dependency-refresh.yml'), 'utf8');
  const job = parseWorkflow(text).find(entry => entry.id === 'refresh');
  assert.match(text, /^defaults:\n  run:\n    shell: bash$/mu);
  const triggers = /^on:\n((?: {2,}.*\n)+)/mu.exec(text)[1];
  assert.deepEqual([...triggers.matchAll(/^ {2}(\w+):/gmu)].map((match) => match[1]), ['schedule', 'workflow_dispatch']);
  assert.match(job.header, /github\.ref == 'refs\/heads\/main'/u);
  const audit = job.steps.find(step => step.name === 'Refresh the dependency audit from the registries');
  const lockfiles = job.steps.find(step => step.name === 'Update the lockfiles within the declared ranges');
  const outdated = job.steps.find(step => step.name === 'Report available updates from npm and Cargo');
  const retain = job.steps.find(step => step.name === 'Retain the live dependency report');
  const summary = job.steps.find(step => step.name === 'Report the live audit outcome');
  const regenerate = job.steps.find(step => step.name === 'Regenerate dependency history and acceptance fixtures');
  const publish = job.steps.find(step => step.name === 'Open or update the refresh pull request');
  const reportFailure = job.steps.find(step => step.name === 'Fail the unresolved live audit');
  assert.ok(reportFailure, 'the scheduled job must fail for an unresolved live audit');
  assert.match(audit.text, /id: audit/u);assert.match(audit.text, /continue-on-error: true/u);
  assert.deepEqual(job.steps.filter(step => /continue-on-error:/u.test(step.text)), [lockfiles, outdated, audit]);
  assert.ok(retain.afterFailure && summary.afterFailure);
  for (const file of ['js/dependency-refresh.log', 'js/dependency-update.log', 'js/dependency-outdated.log', 'parity/dependency-inventory.lino', 'docs/dependency-audit.md']) assert.ok(retain.text.includes(file));
  assert.match(retain.text, /if-no-files-found: error/u);
  for (const step of [summary, publish]) assert.ok(step.text.includes('AUDIT_OUTCOME: ${{ steps.audit.outcome }}'));
  assert.doesNotMatch(publish.text, /continue-on-error/u);
  assert.match(job.header, /group: dependency-refresh-\$\{\{ github\.repository \}\}-write\n\s+cancel-in-progress: false/u);
  const script = step => step.run.replace(/^\|\s*\n/u, '').replace(/^ {10}/gmu, '');
  assert.ok(job.steps.indexOf(regenerate) > job.steps.indexOf(summary));
  assert.ok(job.steps.indexOf(regenerate) < job.steps.indexOf(publish));
  assert.doesNotMatch(regenerate.text, /continue-on-error/u);
  assert.ok(job.steps.indexOf(reportFailure) > job.steps.indexOf(publish));
  assert.ok(reportFailure.afterFailure);
  assert.match(reportFailure.text, /steps\.audit\.outcome == 'failure'/u);
  assert.doesNotMatch(reportFailure.text, /continue-on-error/u);
  for (const step of [lockfiles, audit, outdated, summary, regenerate, publish, reportFailure]) {
    const syntax = spawnSync('bash', ['-n', '-c', script(step)], {encoding: 'utf8'});
    assert.equal(syntax.status, 0, syntax.stderr);
  }
  const directory = mkdtempSync(path.join(os.tmpdir(), 'meta-language-live-audit-contract-'));
  try {
    const regenerateCommands = `
      npm() { printf 'npm %s\\n' "$*"; }
      node() { printf 'node %s\\n' "$*"; }
    `;
    const install = 'npm --prefix js ci --ignore-scripts';
    const acceptanceCommands = [
      'node js/scripts/generate-dependency-workflow-source-history.mjs',
      'npm --prefix js run acceptance:issue-195:refresh',
      'node js/scripts/check-issue-195-acceptance.mjs --scope-baseline HEAD',
    ];
    for (const [diffStatus, nativeCommands] of [
      [0, []],
      [1, [
        'node experiments/grammar-outcome-network/native-transport/diagnostic-order/generate.mjs --write',
        'node js/scripts/generate-ci-integration-source-history.mjs',
      ]],
    ]) {
      const git = `git() { test "$*" = 'diff --quiet HEAD -- rust/Cargo.lock rust/Cargo.toml' || exit 2; return ${diffStatus}; }\n`;
      const regeneration = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', regenerateCommands + git + script(regenerate)],
        { cwd: directory, encoding: 'utf8' });
      assert.equal(regeneration.status, 0, regeneration.stderr);
      assert.deepEqual(regeneration.stdout.trim().split('\n'), [install, ...nativeCommands, ...acceptanceCommands]);
    }
    const failure = spawnSync('bash', ['-e', '-o', 'pipefail', '-c',
      "npm() { printf '23 dependency inventory problem(s)\\n' >&2; return 1; }\n" + script(audit)],
    {cwd: directory, encoding: 'utf8'});
    assert.equal(failure.status, 1, 'the real audit command keeps its failure status under pipefail');
    assert.equal(readFileSync(path.join(directory, 'dependency-refresh.log'), 'utf8'), '23 dependency inventory problem(s)\n');
    const jobFailure = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', script(reportFailure)],
      {cwd: directory, encoding: 'utf8'});
    assert.equal(jobFailure.status, 1, jobFailure.stderr);
    assert.match(jobFailure.stdout, /::error::The live dependency audit failed.*retained audit/u);
    mkdirSync(path.join(directory, 'js'));
    const probe = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', `
      npm() { printf '{"example":{"latest":"2.0.0"}}\\n'; return 1; }
      cargo() { printf '%s\\n    Locking 0 packages\\n' "$*"; }
    ` + script(outdated)], {cwd: directory, encoding: 'utf8'});
    assert.equal(probe.status, 0, probe.stderr);
    const updates = readFileSync(path.join(directory, 'js/dependency-outdated.log'), 'utf8');
    assert.match(updates, /"latest":"2\.0\.0"/u);
    assert.match(updates, /install cargo-outdated --locked/u);
    assert.match(updates, /outdated --root-deps-only --exit-code 1 --manifest-path rust\/Cargo.toml/u);
    assert.match(updates, /outdated --root-deps-only --exit-code 1 --manifest-path rust\/web\/Cargo.toml/u);
    assert.match(updates, /update --dry-run --manifest-path rust\/Cargo.toml/u);
    assert.match(updates, /update --dry-run --manifest-path rust\/web\/Cargo.toml/u);
    const outdatedCargo = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', `
      npm() { printf '{}\\n'; }
      cargo() {
        printf '%s\\n' "$*"
        if [ "$1" = outdated ]; then return 1; fi
        return 0
      }
    ` + script(outdated)], {cwd: directory, encoding: 'utf8'});
    assert.equal(outdatedCargo.status, 1, outdatedCargo.stderr);
    assert.match(outdatedCargo.stdout, /update --dry-run --manifest-path rust\/web\/Cargo.toml/u);
    const stub = `git() {
      if [ "$1" = diff ]; then return 1; fi
      printf '%s\\n' "$*" >> "$CALLS"
      if [ "$1" = ls-remote ] && [ -z "$EXISTING" ]; then return 2; fi
      return 0
    }
    gh() {
      if [ "$1 $2" = 'pr list' ]; then printf '%s' "$EXISTING"; return 0; fi
      printf '%s\\n' "$*" >> "$CALLS"
      while [ "$#" -gt 0 ]; do
        if [ "$1" = --body-file ]; then cp "$2" "$CAPTURED_BODY"; return 0; fi
        shift
      done
      return 1
    }
    `;
    for (const outcome of ['success', 'failure']) for (const existing of ['', '227']) {
      const selected = path.join(directory, outcome + (existing || '-new'));mkdirSync(selected);
      const calls = path.join(selected, 'calls.log'), body = path.join(selected, 'captured.md'), report = path.join(selected, 'summary.md');
      const environment = {...process.env, AUDIT_OUTCOME: outcome, EXISTING: existing, RUNNER_TEMP: selected,
        BRANCH: 'automation/dependency-refresh', CALLS: calls, CAPTURED_BODY: body, GITHUB_STEP_SUMMARY: report};
      writeFileSync(calls, '');
      const note = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', script(summary)], {cwd: selected, env: environment, encoding: 'utf8'});
      assert.equal(note.status, 0, note.stderr);
      assert.ok(readFileSync(report, 'utf8').includes('**' + outcome + '**'));
      assert.equal(note.stdout.includes('::warning::'), outcome === 'failure');
      const result = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', stub + script(publish)], {cwd: selected, env: environment, encoding: 'utf8'});
      assert.equal(result.status, 0, result.stderr);
      assert.ok(readFileSync(body, 'utf8').includes('Live audit outcome: **' + outcome + '**'));
      assert.match(readFileSync(body, 'utf8'), /Required offline PR checks and current-dependency delivery gates remain unchanged/u);
      const invocations = readFileSync(calls, 'utf8');
      assert.ok(invocations.includes(existing ? 'pr edit 227 --body-file ' : 'pr create --base main --head automation/dependency-refresh'));
      assert.ok(invocations.includes('push origin HEAD:refs/heads/automation/dependency-refresh'));
      assert.doesNotMatch(invocations, /--force/u);
      assert.equal(invocations.includes('merge --strategy=ours --no-edit FETCH_HEAD'), Boolean(existing));
    }
  } finally {rmSync(directory, {recursive: true, force: true});}
  recordIssue195Observations({
    requirementId: 'I195-ISSUE-197-FRESHNESS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-issue-197-freshness',
    fixtureFile: 'js/tests/dependency-inventory.test.js',
    assertions: ['nativeLockRefreshRegeneratesDiagnosticProfile'],
    testName: 'main dependency refresh retains failed live findings and delivers their outcome to repair pull requests',
    runtime: 'tooling',
  });
});

test('scheduled refresh fast-forwards a diverged automation branch under the existing push rule', (context) => {
  const directory = mkdtempSync(path.join(os.tmpdir(), 'meta-language-refresh-branch-'));
  context.after(() => rmSync(directory, { recursive: true, force: true }));
  const origin = path.join(directory, 'origin.git');
  const author = path.join(directory, 'author');
  const checkout = path.join(directory, 'checkout');
  const branch = 'automation/dependency-refresh';
  const git = (cwd, ...args) => {
    const result = spawnSync('git', ['-C', cwd, ...args], { encoding: 'utf8' });
    assert.equal(result.status, 0, result.stderr);
    return result.stdout.trim();
  };
  git(directory, 'init', '--bare', '--initial-branch=main', origin);
  git(origin, 'config', 'receive.denyNonFastForwards', 'true');
  git(directory, 'clone', origin, author);
  git(author, 'config', 'user.name', 'Dependency Test');
  git(author, 'config', 'user.email', 'dependency-test@example.com');
  const audit = path.join(author, 'audit.txt');
  writeFileSync(audit, 'base\n');
  git(author, 'add', 'audit.txt');
  git(author, 'commit', '-m', 'Record base audit');
  git(author, 'switch', '--create', branch);
  writeFileSync(audit, 'previous refresh\n');
  git(author, 'commit', '-am', 'Previous refresh');
  git(author, 'push', 'origin', `HEAD:refs/heads/${branch}`);
  const oldRefresh = git(origin, 'rev-parse', branch);
  git(author, 'switch', 'main');
  writeFileSync(audit, 'main update\n');
  git(author, 'commit', '-am', 'Update main independently');
  git(author, 'push', 'origin', 'main');
  const main = git(origin, 'rev-parse', 'main');
  git(directory, 'clone', origin, checkout);

  const rejected = spawnSync('git', ['push', '--force', 'origin', `HEAD:refs/heads/${branch}`], { cwd: checkout, encoding: 'utf8' });
  assert.notEqual(rejected.status, 0);
  assert.match(rejected.stderr, /denying non-fast-forward/u);
  assert.equal(git(origin, 'rev-parse', branch), oldRefresh);

  writeFileSync(path.join(checkout, 'audit.txt'), 'current refresh\n');
  const workflow = readFileSync(path.join(root, '.github/workflows/dependency-refresh.yml'), 'utf8');
  const { run } = parseWorkflow(workflow).find(job => job.id === 'refresh').steps
    .find(step => step.name === 'Open or update the refresh pull request');
  const publish = run.replace(/^\|\s*\n/u, '').replace(/^ {10}/gmu, '');
  const result = spawnSync('bash', ['-e', '-o', 'pipefail', '-c', 'gh() { return 0; }\n' + publish], {
    cwd: checkout, encoding: 'utf8', env: { ...process.env, AUDIT_OUTCOME: 'success', BRANCH: branch, RUNNER_TEMP: directory },
  });
  assert.equal(result.status, 0, result.stderr);
  git(origin, 'merge-base', '--is-ancestor', oldRefresh, branch);
  git(origin, 'merge-base', '--is-ancestor', main, branch);
  assert.equal(git(origin, 'show', `${branch}:audit.txt`), 'current refresh');
  assert.equal(git(origin, 'rev-parse', 'main'), main);
});

test('outdated direct pins require an audited open issue attached to each declaration', async (context) => {
  const fixture = mkdtempSync(path.join(os.tmpdir(), 'dependency-freshness-'));
  context.after(() => rmSync(fixture, { recursive: true, force: true }));
  const url = 'https://github.com/upstream/project/issues/42';
  const cargoManifest = 'rust/Cargo.toml';
  const npmManifest = 'js/package.json';
  mkdirSync(path.join(fixture, 'rust'));
  mkdirSync(path.join(fixture, 'js'));
  const cargo = (annotation, requirement = '=1.0.0') => writeFileSync(path.join(fixture, cargoManifest),
    `[dependencies]\nalias = { package = "example", version = "${requirement}" } ${annotation}\n`);
  const npm = (requirement) => writeFileSync(path.join(fixture, npmManifest), JSON.stringify({
    devDependencies: { example: '1.0.0' },
    dependencyBlockers: { devDependencies: { example: { requirement, issue: url } } },
  }));
  const direct = { name: 'example', compare: 'version', role: 'direct', pinned: '1.0.0', current: '2.0.0' };
  const fixtureAudit = { items: [
    { ...direct, id: 'example crate', category: 'crate', source: { type: 'crate', crate: 'example' },
      scope: 'rust/Cargo.lock', declaredIn: [cargoManifest] },
    { ...direct, id: 'example npm package', category: 'npm', source: { type: 'npm', package: 'example' },
      scope: 'js/package-lock.json', declaredIn: [npmManifest] },
    { ...direct, id: 'transitive example', category: 'crate', role: 'transitive', scope: 'rust/Cargo.lock' },
  ] };
  cargo('');
  npm('^1.0.0');
  assert.deepEqual(kinds(checkDependencyFreshness(fixture, fixtureAudit)), ['stale-direct-dependency', 'stale-direct-dependency']);
  cargo(`# blocked: ${url}`);
  npm('1.0.0');
  const routes = [];
  const refresh = (issue, upstream = 'upstream/project') => refreshDependencyBlockers(fixture, fixtureAudit, {
    repository: async () => upstream,
    github: async (route) => {
      routes.push(route);
      return issue && { html_url: url, ...issue };
    },
  });
  const audited = await refresh({ state: 'open' });
  assert.deepEqual(checkDependencyFreshness(fixture, audited), []);
  assert.ok(audited.items[0].reason.includes(url));
  assert.deepEqual(routes, ['repos/upstream/project/issues/42']);
  cargo(`# blocked: ${url}`, '^1.0.0');
  assert.equal(checkDependencyFreshness(fixture, audited).length, 1);
  cargo(`# blocked: ${url}`);
  assert.equal(checkDependencyFreshness(fixture, await refresh({ state: 'closed' })).length, 2);
  assert.equal(checkDependencyFreshness(fixture, await refresh({ state: 'open', pull_request: {} })).length, 2);
  assert.equal(checkDependencyFreshness(fixture, await refresh(null)).length, 2);
  assert.equal(checkDependencyFreshness(fixture, await refresh({ state: 'open' }, 'another/repository')).length, 2);
  assert.equal(checkDependencyFreshness(fixture, await refresh({ state: 'open', html_url: `${url}0` })).length, 2);
  recordIssue195Observations({
    requirementId: 'I195-ISSUE-197-FRESHNESS',
    suffix: 'behavior',
    fixtureId: 'planned:repository-directive:i195-issue-197-freshness',
    fixtureFile: 'js/tests/dependency-inventory.test.js',
    assertions: ['staleDirectPinRejected', 'openBlockerAccepted', 'closedOrWrongBlockerRejected'],
    testName: 'outdated direct pins require an audited open issue attached to each declaration',
    runtime: 'tooling',
  });
});

test('workflow pip pins use normalized PyPI names and retain the declared exact versions', () => {
  assert.deepEqual(pinnedPipPackages(`
    # python3 -m pip install ignored==9.0.0
    python3 -m pip install --disable-pip-version-check --no-cache-dir lark==1.3.1
    pip3 install 'Example_Package[extra]==2.0.1' \\
      Another.Package==1.2.0.post1 # comment==0.1.0
  `), [
    { name: 'lark', pinned: '1.3.1' },
    { name: 'example-package', pinned: '2.0.1' },
    { name: 'another-package', pinned: '1.2.0.post1' },
  ]);
  assert.deepEqual(pinnedPipPackages('pip install unpinned range>=1.0 wildcard==1.*'), []);
  const actual = collectDependencies(root).find((item) => item.id === 'toolchain pypi lark@1.3.1');
  assert.ok(actual);
  assert.deepEqual(actual.source, { type: 'pypi', package: 'lark' });
  assert.deepEqual(actual.declaredIn, ['.github/workflows/js.yml']);
  assert.throws(() => reconcileLocalDeclarations({ auditDate: TODAY, items: [] }, [actual]), /live dependency refresh/u);
});

test('the primary PyPI resolver excludes yanked and prerelease distributions and caches its one registry request', async () => {
  const requests = [];
  const resolvers = releaseResolvers({ today: TODAY, github: () => assert.fail('no GitHub calls'), http: async (url, headers) => {
    requests.push({ url, headers });
    return JSON.stringify({ info: { name: 'Example_Package', version: '9.0.0rc1' }, releases: {
      '1.0.0': [{ yanked: false }], '2.0.0': [{ yanked: false }], '2.0.0.post1': [{ yanked: false }],
      '3.0.0': [{ yanked: true }], '8.0.0': [], '9.0.0rc1': [{ yanked: false }],
      '8.0.0.dev1': [{ yanked: false }], '1!9.0rc1': [{ yanked: false }],
    } });
  } });
  const source = { package: 'example-package' };
  const result = await resolvers.pypi(source);
  assert.equal(result.current, '2.0.0.post1');
  assert.equal((await resolvers.pypi(source)).current, result.current);
  assert.deepEqual(requests, [{ url: 'https://pypi.org/pypi/example-package/json', headers: { accept: 'application/json' } }]);
  assert.equal(ownStatus({ compare: 'version', pinned: '2.0.0', current: result.current, source: { type: 'pypi' } }), 'behind');
  assert.equal(comparePypiVersions('1!1.0', '999.0.0'), 1);
  assert.equal(comparePypiVersions('1.0.0.1', '1.0.0'), 1);
  assert.equal(comparePypiVersions('1.0', '1.0.0'), 0);
  assert.equal(comparePypiVersions('999999999999999999999999', '999999999999999999999998'), 1);
});

test('PyPI resolver fails closed on missing projects, outages and unusable registry metadata', async () => {
  const resolver = (http) => releaseResolvers({ today: TODAY, github: () => assert.fail('no GitHub calls'), http }).pypi({ package: 'lark' });
  await assert.rejects(resolver(async () => { throw new Error('HTTP 503'); }), /HTTP 503/u);
  await assert.rejects(resolver(async () => { throw new Error('HTTP 404'); }), /HTTP 404/u);
  await assert.rejects(resolver(async () => 'not JSON'));
  await assert.rejects(resolver(async () => JSON.stringify({ info: { name: 'other' }, releases: {} })), /inconsistent project/u);
  await assert.rejects(resolver(async () => JSON.stringify({ info: { name: 'lark' }, releases: { '2.0.0rc1': [{ yanked: false }], '1.0.0': [{ yanked: true }] } })), /no available stable release/u);
  await assert.rejects(resolver(async () => JSON.stringify({ info: { name: 'lark' }, releases: { 'unknown-stable': [{ yanked: false }] } })), /unsupported stable PyPI version/u);
});

test('local declaration reconciliation retains audited releases and rejects unaudited external pins', () => {
  const external = { id: 'npm example@1', category: 'npm', name: 'example', pinned: '1',
    source: { type: 'npm', package: 'example' }, declaredIn: ['package.json'] };
  const local = { id: 'generator rules@script', category: 'generator', name: 'rules', pinned: 'script',
    source: { type: 'none' }, declaredIn: ['rules.mjs'] };
  const audit = { auditDate: '2026-10-06', items: [{ ...external, current: '2', evidence: 'registry receipt',
    reason: 'verified compatibility constraint', compatible: '1' }] };
  const updated = reconcileLocalDeclarations(audit, [{ ...external, declaredIn: ['package.json', 'consumer.json'] }, local]);
  assert.equal(updated.auditDate, audit.auditDate);
  assert.equal(updated.items[0].current, '2');
  assert.equal(updated.items[0].compatible, '1');
  assert.equal(updated.items[0].evidence, 'registry receipt');
  assert.deepEqual(updated.items[0].declaredIn, ['package.json', 'consumer.json']);
  assert.deepEqual(updated.items[1].dependsOn, []);
  assert.deepEqual(audit.items[0].declaredIn, ['package.json']);
  assert.throws(() => reconcileLocalDeclarations(audit, [{ ...external, id: 'npm example@3', pinned: '3' }]), /live dependency refresh/u);
  assert.throws(() => reconcileLocalDeclarations(audit, [{ ...external, source: { type: 'npm', package: 'different' } }]), /release source changed/u);

  const collected = collectDependencies(root);
  const cache = collected.find(({ id }) => id === 'action actions/cache@v6');
  assert.ok(cache);
  assert.equal(cache.name, 'actions/cache');
  assert.equal(cache.pinned, 'v6');
  assert.equal(cache.compare, 'major');
  assert.deepEqual(cache.source, { type: 'github-release', repository: 'actions/cache' });
  const auditedCache = inventory.items.find(({ id }) => id === cache.id);
  for (const field of ['name', 'pinned', 'compare', 'source', 'declaredIn']) assert.deepEqual(cache[field], auditedCache[field]);
  for (const [action, expectedOwners] of [
    ['actions/cache/restore', ['.github/workflows/ci.yml', '.github/workflows/rust.yml']],
    ['actions/cache/save', ['.github/workflows/ci.yml']],
  ]) {
    const child = collected.find(({ id }) => id === `action ${action}@v6`);
    assert.ok(child, action);
    assert.equal(child.category, 'action');
    assert.equal(child.name, action, 'the complete child action retains its identity');
    assert.equal(child.pinned, cache.pinned);
    assert.equal(child.compare, cache.compare);
    assert.deepEqual(child.source, cache.source, 'nested actions share only the actual repository release source');
    assert.deepEqual(child.declaredIn, expectedOwners);
    assert.throws(() => reconcileLocalDeclarations({ auditDate: TODAY, items: [auditedCache] }, [child]), /live dependency refresh/u);
  }
  const checkout = collected.find(({ id }) => id === 'action actions/checkout@v7');
  assert.deepEqual(checkout.source, { type: 'github-release', repository: 'actions/checkout' });
  assert.equal(checkout.compare, 'major');
  const rust = collected.find(({ id }) => id === 'action dtolnay/rust-toolchain@1.100.0');
  assert.deepEqual(rust.source, { type: 'github-release', repository: 'rust-lang/rust' });
  assert.equal(rust.compare, 'version');
});

/** A toolchain item as the collector records it, with its resolved current release. */
function toolchain(name, pinned, current, extra = {}) {
  return {
    id: `toolchain ${name}@${pinned}`,
    category: 'toolchain',
    name,
    declaredIn: ['.github/workflows/demo.yml'],
    pinned,
    compare: 'version',
    source: { type: 'github-release', repository: `demo/${name}` },
    current,
    ...extra,
  };
}

const collectedOf = (items) => items.map(({ current, evidence, reason, reasonHeld, note, ...collected }) => collected);

test('the committed inventory lists every dependency the repository declares, in every category', () => {
  const collected = collectDependencies(root);
  const inventoried = new Set(inventory.items.map(({ id }) => id));
  const missing = collected.filter(({ id }) => !inventoried.has(id)).map(({ id }) => id);
  assert.deepEqual(missing, [], 'every declared dependency is inventoried');
  const present = new Set(inventory.items.map(({ category }) => category));
  for (const { id } of CATEGORIES) assert.ok(present.has(id), `the inventory has ${id} items`);
  const npmKinds = new Set(inventory.items.filter(({ category }) => category === 'npm').map(({ kind }) => kind));
  assert.ok(npmKinds.has('runtime') && npmKinds.has('development'), 'runtime and development npm packages');
  const crateRoles = new Set(inventory.items.filter(({ category }) => category === 'crate').map(({ role }) => role));
  assert.ok(crateRoles.has('direct') && crateRoles.has('transitive'), 'direct crates and lockfile resolutions');
  assert.equal(readFileSync(path.join(root, AUDIT_DOCUMENT), 'utf8'), renderAuditDocument(inventory), `${AUDIT_DOCUMENT} is the inventory's rendering`);
  observe(['everyDependencyInventoried'], 'the committed inventory lists every dependency the repository declares, in every category');
});

test('direct Cargo and npm dependencies satisfy the audited current releases', () => {
  const declarations = collectDependencies(root);
  const actual = new Map(inventory.items.map((item) => [item.id, item]));
  for (const item of declarations.filter(({ category, role }) => ['crate', 'npm'].includes(category) && role === 'direct')) {
    assert.equal(actual.get(item.id)?.pinned, item.pinned, item.id);
    if (item.compare === 'version') assert.ok(actual.get(item.id).current, item.id);
  }
  assert.deepEqual(checkDependencyFreshness(root, inventory), []);
  const requirementId = 'I195-ISSUE-197-DEPENDENCIES';
  recordIssue195Observations({
    requirementId,
    suffix: 'behavior',
    fixtureId: `planned:repository-directive:${requirementId.toLowerCase()}`,
    fixtureFile: INVENTORY_FILE,
    assertions: ['directPackageUpdatesAudited'],
    testName: 'direct Cargo and npm dependencies satisfy the audited current releases',
    runtime: 'tooling',
  });
});

test('the committed inventory is dated and every retained item is current or records its compatibility reason', () => {
  assert.match(inventory.auditDate, /^\d{4}-\d{2}-\d{2}$/u);
  assert.ok(inventory.auditDate <= new Date().toISOString().slice(0, 10), 'the audit date is not in the future');
  assert.match(renderAuditDocument(inventory), new RegExp(inventory.auditDate, 'u'), 'the audit document carries the date');
  assert.deepEqual(checkInventory(inventory, collectDependencies(root)), [], 'the committed inventory passes the check');
  for (const item of inventory.items) {
    if (!['floating', 'unversioned', 'derived'].includes(item.compare)) assert.ok(item.current, `${item.id} records its current stable release`);
    if (ownStatus(item) === 'behind') assert.ok(item.reason?.length > 20, `${item.id} is behind ${item.current} and records why`);
  }
  observe(['auditDateRecorded', 'retainedItemsAtCurrentStableRelease'], 'the committed inventory is dated and every retained item is current or records its compatibility reason');
});

test('the native assertion range selects the newest admitted stable Rust release', async () => {
  const declarations = collectDependencies(root);
  const holder = declarations.find(item => item.name === 'rust/src/data/declaration-dependency-rules.lino');
  const action = declarations.find(item => item.name === 'dtolnay/rust-toolchain' && item.pinned === '1.99.1');
  assert.ok(holder && action);
  assert.equal(holder.requirement, '>=1.90.0 <1.100.0');
  const releases = ['1.100.0', '1.99.2', '1.99.1', '1.99.0', '1.90.0'].map(tag_name => ({
    tag_name, draft: tag_name === '1.99.2', prerelease: false,
  }));
  const resolve = couplingResolvers({ github: async route => {
    assert.equal(route, 'repos/rust-lang/rust/releases?per_page=100');
    return releases;
  } })['action dtolnay/rust-toolchain'];
  const pinned = { ...action, current: '1.100.0' };
  const find = (category, name) => declarations.find(item => item.category === category && item.name === name);
  const profile = await resolve(pinned, find);
  assert.equal(profile.compatible, '1.99.1');
  assert.deepEqual(profile.heldBy.map(({ id, requirement }) => ({ id, requirement })), [{ id: holder.id, requirement: holder.requirement }]);
  assert.equal(deliveredStatuses([holder, { ...pinned, ...profile }]).get(action.id).status, 'compatible');
  assert.deepEqual(await resolve({ ...pinned, declaredIn: ['.github/workflows/dependency-refresh.yml'] }, find), {});
  const previous = { ...pinned, pinned: '1.99.0' };
  assert.equal(deliveredStatuses([holder, { ...previous, ...profile }]).get(action.id).status, 'behind');
  assert.equal(deliveredStatuses([holder, { ...pinned, ...profile, pinned: '1.98.0' }]).get(action.id).status, 'behind');
  releases.find(release => release.tag_name === '1.99.1').draft = true;
  const priorProfile = await resolve(previous, find);
  assert.equal(priorProfile.compatible, '1.99.0');
  assert.equal(deliveredStatuses([holder, { ...previous, ...priorProfile }]).get(action.id).status, 'compatible');
  releases.pop();
  await assert.rejects(resolve(pinned, find), /does not cover the full native assertion range/u);
});

test('the complete test jobs share the newest stable Node release in the recorded assertion range', async () => {
  const declarations = collectDependencies(root);
  const find = (category, name) => declarations.find(item => item.category === category && item.name === name);
  const holder = find('toolchain', 'js/src/translation/data/ascii-text-predicate-rules.lino');
  const node = declarations.find(item => item.name === 'node' && item.pinned === '24.22.3');
  assert.ok(holder && node);
  assert.equal(holder.requirement, '>=22.15.0 <23.0.0 || >=24.0.0 <25.0.0');
  const releases = ['26.13.1', '25.8.0', '24.22.4-rc.1', '24.22.3', '24.22.0', '22.23.5', '22.15.0']
    .map(version => ({ version: `v${version}` }));
  const resolvers = releaseResolvers({ http: async url => {
    assert.equal(url, 'https://nodejs.org/dist/index.json');
    return JSON.stringify(releases);
  } });
  const pinned = { ...node, current: '26.13.1' };
  const resolve = resolvers.couplings['toolchain node'];
  const profile = await resolve(pinned, find);
  assert.equal(profile.compatible, '24.22.3');
  assert.deepEqual(profile.heldBy.map(({ id, requirement }) => ({ id, requirement })), [{ id: holder.id, requirement: holder.requirement }]);
  assert.equal(deliveredStatuses([holder, { ...pinned, ...profile }]).get(node.id).status, 'compatible');
  for (const older of ['22.23.5', '24.22.0']) {
    const previous = { ...pinned, pinned: older, ...profile };
    assert.equal(deliveredStatuses([holder, previous]).get(node.id).status, 'behind');
  }
  assert.deepEqual(await resolve({ ...pinned, declaredIn: ['.github/workflows/ci.yml'] }, find), {});
  assert.deepEqual(await resolve({ ...pinned, source: { type: 'node-maintained' } }, find), {});
  await assert.rejects(couplingResolvers({ nodeIndex: async () => releases.slice(0, -1) })['toolchain node'](pinned, find),
    /does not cover the full JavaScript assertion range/u);
});

test('the check rejects a stale item without a reason, an undated audit and an uninventoried dependency', () => {
  const current = toolchain('fresh', '2.0.0', '2.0.0');
  const stale = toolchain('stale', '1.0.0', '2.0.0');
  const collected = collectedOf([current, stale]);
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [current, stale] }, collected, { today: TODAY })), ['behind-without-reason']);
  const placeholder = { ...stale, reason: 'TODO' };
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [current, placeholder] }, collected, { today: TODAY })), ['behind-without-reason']);
  const explained = { ...stale, reason: 'The 2.x line drops the platform the release job builds on.' };
  assert.deepEqual(checkInventory({ auditDate: TODAY, items: [current, explained] }, collected, { today: TODAY }), []);
  const needless = { ...current, reason: 'Held back.' };
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [needless, explained] }, collected, { today: TODAY })), ['stale-reason']);
  // A moving minor tag such as `5.4` is behind once a newer minor line is released.
  const minor = toolchain('line', '5.4', '5.5.1', { compare: 'minor' });
  assert.equal(ownStatus(minor), 'behind');
  assert.equal(ownStatus({ ...minor, current: '5.4.3' }), 'current');
  // A generated asset is behind when the tool it is built from is behind.
  const generated = { id: 'generator demo.mjs@script', category: 'generator', name: 'demo.mjs', declaredIn: ['demo.mjs'], pinned: 'script', compare: 'derived', source: { type: 'none' }, dependsOn: [stale.id] };
  assert.equal(statuses([stale, generated]).get(generated.id), 'behind');
  assert.equal(statuses([current, { ...generated, dependsOn: [current.id] }]).get(generated.id), 'current');
  for (const auditDate of [undefined, '', 'yesterday', '2026-13-45', '2099-01-01']) {
    assert.deepEqual(kinds(checkInventory({ auditDate, items: [current, explained] }, collected, { today: TODAY })), ['audit-date'], JSON.stringify(auditDate));
  }
  const added = toolchain('added', '1.0.0', '1.0.0');
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [current, explained] }, collectedOf([current, stale, added]), { today: TODAY })), ['not-inventoried']);
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [current, explained, added] }, collected, { today: TODAY })), ['not-in-repository']);
  const moved = toolchain('stale', '1.5.0', '2.0.0');
  assert.deepEqual(kinds(checkInventory({ auditDate: TODAY, items: [current, explained] }, collectedOf([current, moved]), { today: TODAY })), ['not-inventoried', 'not-in-repository']);
  observe(['staleDependencyRejected', 'auditDateRecorded'], 'the check rejects a stale item without a reason, an undated audit and an uninventoried dependency');
});

test('a refresh dates the audit, keeps a reason only for the unchanged pin and follows a moved tool', async () => {
  const kept = toolchain('kept', '1.0.0', '1.0.0', { reason: 'Held for the release job.' });
  const moved = toolchain('moved', '1.0.0', '1.0.0', { reason: 'Held for the old platform.' });
  const generator = { id: 'generator demo.mjs@script', category: 'generator', name: 'demo.mjs', declaredIn: ['demo.mjs'], pinned: 'script', compare: 'derived', source: { type: 'none' }, dependsOn: [moved.id] };
  const previous = { auditDate: '2026-01-01', items: [kept, moved, generator] };
  const bumped = toolchain('moved', '1.5.0', '2.0.0');
  // The collector does not know which tools a generator runs; the refresh carries dependsOn over.
  const { dependsOn, ...collectedGenerator } = generator;
  const collected = collectedOf([kept, bumped, collectedGenerator]);
  const releases = { 'demo/kept': '2.0.0', 'demo/moved': '2.0.0' };
  const resolvers = {
    'github-release': async (source) => ({ current: releases[source.repository], evidence: `GitHub release, ${source.repository}` }),
    none: async () => ({}),
  };
  const refreshed = await refreshInventory({ root, previous, collected, resolvers, cargoMetadata: () => assert.fail('no crates'), today: TODAY });
  assert.equal(refreshed.auditDate, TODAY);
  const byId = new Map(refreshed.items.map((item) => [item.id, item]));
  assert.equal(byId.get(kept.id).current, '2.0.0');
  assert.equal(byId.get(kept.id).reason, kept.reason, 'the unchanged pin keeps its reason');
  assert.equal(byId.get(bumped.id).reason, undefined, 'a new pin must be re-justified');
  assert.deepEqual(byId.get(generator.id).dependsOn, [bumped.id], 'the generator follows the tool to its new pin');
  assert.deepEqual(kinds(checkInventory(refreshed, collected, { today: TODAY })), ['behind-without-reason']);
  observe(['staleDependencyRejected', 'auditDateRecorded'], 'a refresh dates the audit, keeps a reason only for the unchanged pin and follows a moved tool');
});

test('the registry client retries dropped connections and transient answers, and names the URL when it gives up', async () => {
  const dropped = () => Object.assign(new TypeError('fetch failed'), { cause: Object.assign(new Error('read ECONNRESET'), { code: 'ECONNRESET' }) });
  const answer = (status, body = '') => ({ ok: status < 400, status, text: async () => body });
  const scripted = (outcomes) => {
    const calls = [];
    const fetch = async (url, options) => {
      calls.push({ url, options });
      const outcome = outcomes.shift();
      if (outcome instanceof Error) throw outcome;
      return outcome;
    };
    return { calls, fetch };
  };
  const delays = [];
  const sleep = async (ms) => void delays.push(ms);

  const recovering = scripted([dropped(), answer(503), answer(429), answer(200, 'ok')]);
  assert.equal(await registryHttp({ fetch: recovering.fetch, sleep })('https://registry.example/a', { accept: 'x' }), 'ok');
  assert.equal(recovering.calls.length, 4, 'a dropped connection, a 503 and a 429 are retried');
  assert.equal(recovering.calls[0].options.headers.accept, 'x');
  assert.equal(recovering.calls[0].options.headers['user-agent'], 'meta-language-dependency-audit');
  assert.deepEqual(delays, [1000, 2000, 3000]);

  const missing = scripted([answer(404)]);
  await assert.rejects(registryHttp({ fetch: missing.fetch, sleep })('https://registry.example/missing'), { message: 'https://registry.example/missing: HTTP 404' });
  assert.equal(missing.calls.length, 1, 'a definite answer is not retried');

  const offline = scripted([dropped(), dropped(), dropped(), dropped()]);
  await assert.rejects(registryHttp({ fetch: offline.fetch, sleep })('https://registry.example/b'), { message: 'https://registry.example/b: fetch failed (ECONNRESET)' });
  assert.equal(offline.calls.length, 4, 'the client gives up after its attempts');
});

test('private oracle manifests and their shared lock dependencies are inventoried', () => {
  const collected = collectDependencies(root);
  const manifest = 'rust/oracles/cmake-source-oracle/Cargo.toml';
  const local = collected.find(({ name, scope }) => name === 'cmake-source-oracle' && scope === 'rust/Cargo.toml');
  assert.equal(local.kind, 'development');
  assert.equal(local.pinned, 'oracles/cmake-source-oracle');
  assert.equal(local.source.type, 'none');
  for (const name of ['cc', 'flate2', 'tree-sitter-language']) {
    const item = collected.find((entry) => entry.category === 'crate' && entry.scope === 'rust/Cargo.lock' && entry.name === name);
    assert.ok(item.declaredIn.includes(manifest), name);
    assert.equal(item.source.type, 'crate');
    assert.ok(item.requirement, name);
  }
});
