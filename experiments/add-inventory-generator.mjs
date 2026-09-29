// Adds a derived generator script to parity/dependency-inventory.json and
// re-renders docs/dependency-audit.md without querying the registries.
//   node experiments/add-inventory-generator.mjs js/scripts/<script>.mjs
import { readFileSync, writeFileSync } from 'node:fs';

import { AUDIT_DOCUMENT, INVENTORY_FILE, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const inventory = JSON.parse(readFileSync(INVENTORY_FILE, 'utf8'));
for (const script of process.argv.slice(2)) {
  const id = `generator ${script}@script`;
  if (inventory.items.some((item) => item.id === id)) continue;
  inventory.items.push({ id, category: 'generator', name: script, declaredIn: [script], pinned: 'script', compare: 'derived', source: { type: 'none' }, dependsOn: [] });
}
const categoryOrder = [...new Set(inventory.items.map(({ category }) => category))];
inventory.items.sort((a, b) => categoryOrder.indexOf(a.category) - categoryOrder.indexOf(b.category) || (a.category === b.category ? a.id.localeCompare(b.id) : 0));
writeFileSync(INVENTORY_FILE, `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(AUDIT_DOCUMENT, renderAuditDocument(inventory));
