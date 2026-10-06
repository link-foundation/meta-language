// Adds the declared dependencies that parity/dependency-inventory.json lacks,
// offline: each new item takes its release data (current, evidence, ...) from
// an inventoried item with the same source, so the audit date and the other
// items stay as they are. Use the networked `--refresh` for a real audit.
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { AUDIT_DOCUMENT, INVENTORY_FILE, collectDependencies, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const inventory = JSON.parse(readFileSync(path.join(root, INVENTORY_FILE), 'utf8'));
const known = new Set(inventory.items.map(({ id }) => id));
const collected = collectDependencies(root);
const order = new Map(collected.map(({ id }, index) => [id, index]));
const sourceKey = ({ source }) => JSON.stringify(source);
for (const item of collected.filter(({ id }) => !known.has(id))) {
  const donor = inventory.items.find((candidate) => sourceKey(candidate) === sourceKey(item) && candidate.current !== undefined);
  if (!donor) throw new Error(`${item.id}: no inventoried item with source ${sourceKey(item)}; run the networked --refresh`);
  const { current, evidence, latest } = donor;
  inventory.items.push({ ...item, current, evidence, ...(latest === undefined ? {} : { latest }) });
  console.log(`added ${item.id} (release data from ${donor.id})`);
}
inventory.items.sort((a, b) => order.get(a.id) - order.get(b.id));
writeFileSync(path.join(root, INVENTORY_FILE), `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(path.join(root, AUDIT_DOCUMENT), renderAuditDocument(inventory));
