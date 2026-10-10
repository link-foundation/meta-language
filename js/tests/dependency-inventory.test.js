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
  ownStatus,
  refreshInventory,
  registryHttp,
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
