// Rewrites the dependency inventory's declaredIn lists after a workflow file
// moved, without querying any registry, and re-renders the audit document.
//   node experiments/relocate-declared-in.mjs <from> <to>
import { readFileSync, writeFileSync } from 'node:fs';
import { AUDIT_DOCUMENT, INVENTORY_FILE, renderAuditDocument } from '../js/scripts/dependency-inventory.mjs';

const [from, to] = process.argv.slice(2);
const inventory = JSON.parse(readFileSync(INVENTORY_FILE, 'utf8'));
let moved = 0;
for (const item of inventory.items) {
  if (!item.declaredIn?.includes(from)) continue;
  item.declaredIn = [...new Set(item.declaredIn.map((file) => (file === from ? to : file)))].sort();
  moved += 1;
}
writeFileSync(INVENTORY_FILE, `${JSON.stringify(inventory, null, 2)}\n`);
writeFileSync(AUDIT_DOCUMENT, renderAuditDocument(inventory));
console.log(`moved ${moved} items from ${from} to ${to}`);
