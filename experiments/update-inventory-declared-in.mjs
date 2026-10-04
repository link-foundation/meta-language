// Offline: rewrites only the declaredIn lists of inventoried items from the
// repository's current declarations (no registry queries), and re-renders the
// audit document. Used when a workflow file is added that reuses existing pins.
import { readFileSync, writeFileSync } from 'node:fs';
import { collectDependencies, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const root = new URL('..', import.meta.url).pathname;
const inventory = JSON.parse(readFileSync(`${root}parity/dependency-inventory.json`, 'utf8'));
const collected = collectDependencies(root);
const declared = new Map();
for (const item of collected.items ?? collected) declared.set(item.id, item.declaredIn);
let changed = 0;
for (const item of inventory.items) {
  const next = declared.get(item.id);
  if (next && JSON.stringify(next) !== JSON.stringify(item.declaredIn)) { item.declaredIn = next; changed += 1; }
}
writeFileSync(`${root}parity/dependency-inventory.json`, `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(`${root}docs/dependency-audit.md`, renderAuditDocument(inventory));
console.log(`updated declaredIn of ${changed} items`);
