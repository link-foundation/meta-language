// Applies the offline-collectable changes of moving the native languages'
// tree-sitter oracles to development dependencies to the committed
// dependency inventory (renamed oracle wasm items, crate kinds, the dropped
// published CSV parser, the root-anchored crate LICENSE), keeping every
// audited registry field, and re-renders docs/dependency-audit.md. No network: the audited releases do not change.
import { readFileSync, writeFileSync } from 'node:fs';
import { checkInventory, collectDependencies, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const root = new URL('../', import.meta.url);
const path = new URL('parity/dependency-inventory.json', root);
const inventory = JSON.parse(readFileSync(path, 'utf8'));
const collected = collectDependencies(new URL('.', root).pathname);
const live = new Map(collected.map((item) => [item.id, item]));
const items = [];
for (const item of inventory.items) {
  if (live.has(item.id)) {
    const fresh = live.get(item.id);
    items.push({ ...item, ...(fresh.kind ? { kind: fresh.kind } : {}) });
    continue;
  }
  const anchored = item.category === 'published' && live.get(`published crate /${item.name}@include`);
  if (anchored) {
    const { id, name, ...rest } = item;
    items.push({ id: anchored.id, name: anchored.name, ...rest });
    console.log(`renamed ${id} -> ${anchored.id}`);
    continue;
  }
  const moved = item.category === 'vendored-grammar'
    && collected.find((fresh) => fresh.category === 'vendored-grammar' && fresh.name.endsWith(`/${item.name.split('/').pop()}`));
  if (moved) {
    const { id, name, ...rest } = item;
    items.push({ id: moved.id, name: moved.name, ...rest, declaredIn: moved.declaredIn });
    console.log(`renamed ${id} -> ${moved.id}`);
  } else {
    console.log(`dropped ${item.id}`);
  }
}
// In the order the refresh writes: the collected (compareItems) order.
const order = new Map(collected.map((item, index) => [item.id, index]));
items.sort((a, b) => order.get(a.id) - order.get(b.id));
const next = { ...inventory, items };
writeFileSync(path, `${JSON.stringify(next, null, 2)}\n`);
writeFileSync(new URL('docs/dependency-audit.md', root), renderAuditDocument(next));
console.log(checkInventory(next, collected, { today: next.auditDate }));
