// Refresh the declarations added by the frontend self-translation generator
// without changing the recorded registry audit for existing dependencies.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

import {
  AUDIT_DOCUMENT,
  INVENTORY_FILE,
  collectDependencies,
  renderAuditDocument,
} from '../js/scripts/dependency-inventory.mjs';

const root = fileURLToPath(new URL('..', import.meta.url));
const inventory = JSON.parse(readFileSync(new URL(`../${INVENTORY_FILE}`, import.meta.url), 'utf8'));
const generator = 'generator js/scripts/generate-frontend-rules.mjs@script';
const formatter = 'action dtolnay/rust-toolchain@stable';
const recorded = new Map(inventory.items.map((item) => [item.id, item]));
inventory.items = collectDependencies(root).map((item) => {
  if (item.id === generator) return { ...item, dependsOn: [formatter] };
  const previous = recorded.get(item.id);
  assert.ok(previous, `unexpected uninventoried dependency: ${item.id}`);
  return item.id === formatter ? { ...previous, ...item } : previous;
});
writeFileSync(new URL(`../${INVENTORY_FILE}`, import.meta.url), `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(new URL(`../${AUDIT_DOCUMENT}`, import.meta.url), renderAuditDocument(inventory));
