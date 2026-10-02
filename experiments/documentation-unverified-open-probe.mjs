// Probe: which repository texts would the documentation audit reject if every
// row stayed open until an acceptance report verifies it?
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { auditDocumentation, completionState } from '../js/scripts/issue-195-documentation.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const manifest = JSON.parse(readFileSync(path.join(root, 'parity/issue-195-requirements.json'), 'utf8'));
const state = completionState(manifest);
const unverified = { ...state, open: new Set(state.ids) };
for (const [name, s] of [['current', state], ['unverified-open', unverified]]) {
  const { problems } = auditDocumentation(root, s, { external: [] });
  console.log(`${name}: ${problems.length} problems`);
  for (const p of problems) console.log(`  ${p.file}:${p.line} [${p.claim}] ${p.message}`);
}
