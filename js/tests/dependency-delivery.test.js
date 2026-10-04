import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';

import { checkDeliveredDependencies, checkInventory, collectDependencies } from '../scripts/dependency-inventory.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const fixture = 'parity/dependency-inventory.json';
const inventory = JSON.parse(readFileSync(path.join(root, fixture), 'utf8'));
const collectedOf = (items) => items.map(({ current, evidence, reason, ...item }) => item);
const current = {
  id: 'npm demo@2.0.0', category: 'npm', name: 'demo', scope: 'demo/package.json',
  declaredIn: ['demo/package.json'], pinned: '2.0.0', current: '2.0.0', compare: 'version',
  source: { type: 'npm', package: 'demo' }, role: 'direct', kind: 'runtime',
};
const consumerOf = (items) => ({ packages: {
  'node_modules/meta-language': { dependencies: Object.fromEntries(items.filter(({ category }) => category === 'npm')
    .map(({ source, pinned }) => [source.package, pinned])) },
  ...Object.fromEntries(items.filter(({ category }) => category === 'npm')
    .map(({ source, pinned }) => [`node_modules/${source.package}`, { version: pinned }])),
} });

test('delivery rejects a stale pin even when its compatibility reason passes inventory validation', () => {
  const stale = { ...current, id: 'npm demo@1.0.0', pinned: '1.0.0', reason: 'The new major version needs integration changes before this package can upgrade.' };
  const audited = { auditDate: '2026-09-29', items: [stale] };
  assert.deepEqual(checkInventory(audited, collectedOf([stale])), []);
  const problems = checkDeliveredDependencies(audited, collectedOf([stale]), { npmConsumerLock: consumerOf([stale]) });
  assert.ok(problems.some(({ kind }) => kind === 'stale-delivered-dependency'));
  assert.ok(problems.some(({ kind }) => kind === 'stale-consumer-resolution'));
});

test('delivery rejects stale transitive resolutions and generated descendants', () => {
  const stale = { ...current, id: 'npm demo@1.0.0', pinned: '1.0.0', role: 'transitive', reason: 'The upstream package has not yet updated this locked dependency.' };
  const generated = {
    id: 'generator demo.mjs@script', category: 'generator', name: 'demo.mjs',
    declaredIn: ['demo.mjs'], pinned: 'script', compare: 'derived',
    source: { type: 'none' }, dependsOn: [stale.id],
  };
  const items = [stale, generated];
  const problems = checkDeliveredDependencies({ auditDate: '2026-09-29', items }, collectedOf(items), { npmConsumerLock: consumerOf(items) });
  assert.equal(problems.filter(({ kind }) => kind === 'stale-delivered-dependency').length, 2);
  assert.match(problems[1].message, /generator demo/);
});

test('delivery keeps inventory integrity checks and accepts current dependencies', () => {
  const audited = { auditDate: '2026-09-29', items: [current] };
  assert.deepEqual(checkDeliveredDependencies(audited, collectedOf([current]), { npmConsumerLock: consumerOf([current]) }), []);
  assert.ok(checkDeliveredDependencies(audited, []).some(({ kind }) => kind === 'not-in-repository'));
});

test('current repository overrides cannot certify a missing or stale installed dependency tree', () => {
  const audited = { auditDate: '2026-09-29', items: [current] };
  assert.ok(checkDeliveredDependencies(audited, collectedOf([current]))
    .some(({ kind }) => kind === 'missing-delivery-consumer'));
  const consumer = consumerOf([current]);
  consumer.packages['node_modules/meta-language/node_modules/demo'] = { version: '1.0.0' };
  // The unrelated hoisted copy remains current. Check the version the
  // installed meta-language package actually resolves from its own scope.
  assert.equal(consumer.packages['node_modules/demo'].version, '2.0.0');
  const problems = checkDeliveredDependencies(audited, collectedOf([current]), { npmConsumerLock: consumer });
  assert.ok(problems.some(({ kind, message }) => kind === 'stale-consumer-resolution' && message.includes('1.0.0')));
});

test('delivery inspects transitive copies and rejects prereleases, missing and unaudited resolutions', () => {
  const helper = { ...current, id: 'npm helper@2.0.0', name: 'helper', source: { type: 'npm', package: 'helper' } };
  const items = [current, helper];
  const audited = { auditDate: '2026-09-29', items };
  const check = (consumer) => checkDeliveredDependencies(audited, collectedOf(items), { npmConsumerLock: consumer });
  const consumer = consumerOf(items);
  consumer.packages['node_modules/demo'].dependencies = { helper: '^1.0.0' };
  consumer.packages['node_modules/demo/node_modules/helper'] = { version: '1.0.0' };
  assert.ok(check(consumer).some(({ kind, message }) => kind === 'stale-consumer-resolution' && message.startsWith('helper:')));
  for (const version of ['2.0.0-rc.1', 'invalid', undefined]) {
    consumer.packages['node_modules/demo/node_modules/helper'].version = version;
    assert.ok(check(consumer).some(({ kind }) => kind === 'stale-consumer-resolution'));
  }
  consumer.packages['node_modules/demo/node_modules/helper'].version = '2.0.0';
  // A dependency cycle resolves to existing locations and must terminate.
  consumer.packages['node_modules/helper'].dependencies = { demo: '2.0.0' };
  assert.deepEqual(check(consumer), []);
  consumer.packages['node_modules/demo'].dependencies.unknown = '1.0.0';
  assert.ok(check(consumer).some(({ kind }) => kind === 'missing-consumer-resolution'));
  consumer.packages['node_modules/unknown'] = { version: '1.0.0' };
  assert.ok(check(consumer).some(({ kind }) => kind === 'uninventoried-consumer-dependency'));
});

test('the actual delivered inventory cannot use compatibility reasons to certify its stale items', () => {
  const collected = collectDependencies(root);
  assert.match(readFileSync(path.join(root, 'docs/vision.md'), 'utf8'),
    /A recorded compatibility reason explains a stale item but does not complete its\nupgrade/);
  // Without the verified newest compatible release and its holders, every
  // item that records a reason is stale, whatever the reason says.
  const reasonsOnly = structuredClone(inventory);
  for (const item of reasonsOnly.items) {
    delete item.compatible;
    delete item.heldBy;
  }
  const stale = checkDeliveredDependencies(reasonsOnly, collected).filter(({ kind }) => kind === 'stale-delivered-dependency');
  const explained = inventory.items.filter(({ reason }) => reason);
  assert.ok(explained.length > 0, 'the inventory records reasons for held items');
  for (const item of explained) {
    assert.ok(stale.some(({ message }) => message.startsWith(`${item.id}:`)), item.id);
  }
  const mutation = structuredClone(inventory);
  const item = mutation.items.find((entry) => entry.compare === 'version' && /^\d+\.\d+\.\d+$/.test(entry.current));
  assert.ok(item, 'a delivered versioned dependency is available for the stale-pin mutation');
  item.pinned = '0.0.0';
  item.reason = 'This intentionally stale mutation must fail delivery despite its recorded compatibility explanation.';
  assert.ok(checkDeliveredDependencies(mutation, collectedOf(mutation.items)).some(({ kind, message }) =>
    kind === 'stale-delivered-dependency' && message.startsWith(`${item.id}:`)));
});
