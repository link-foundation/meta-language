// Adds a published crate include entry to parity/dependency-inventory.json and
// re-renders docs/dependency-audit.md without querying the registries.
//   node experiments/add-inventory-crate-include.mjs src/data/<file>.json
import { readFileSync, writeFileSync } from 'node:fs';

import { AUDIT_DOCUMENT, INVENTORY_FILE, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const inventory = JSON.parse(readFileSync(INVENTORY_FILE, 'utf8'));
for (const name of process.argv.slice(2)) {
  const id = `published crate ${name}@include`;
  if (inventory.items.some((item) => item.id === id)) continue;
  inventory.items.push({ id, category: 'published', scope: 'crate', name, declaredIn: ['rust/Cargo.toml'], pinned: 'include', compare: 'unversioned', source: { type: 'none' } });
}
const categoryOrder = [...new Set(inventory.items.map(({ category }) => category))];
inventory.items.sort((a, b) => categoryOrder.indexOf(a.category) - categoryOrder.indexOf(b.category) || (a.category === b.category ? a.id.localeCompare(b.id) : 0));
writeFileSync(INVENTORY_FILE, `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(AUDIT_DOCUMENT, renderAuditDocument(inventory));
