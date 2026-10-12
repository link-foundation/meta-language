// The dependency inventory: every dependency, toolchain, action, image and
// vendored asset the repository declares is in parity/dependency-inventory.json
// with its current stable release, the audit is dated, and a retained item
// behind that release without a recorded compatibility reason fails the check
// (requirement I195-DEPENDENCY-INVENTORY).
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
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
  ownStatus,
  pinnedPipPackages,
  refreshInventory,
  reconcileLocalDeclarations,
  registryHttp,
  releaseResolvers,
  renderAuditDocument,
  statuses,
} from '../scripts/dependency-inventory.mjs';
import { recordIssue195Observations } from './support/issue-195-observations.js';

const REQUIREMENT = 'I195-DEPENDENCY-INVENTORY';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const inventory = JSON.parse(readFileSync(path.join(root, INVENTORY_FILE), 'utf8'));
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
