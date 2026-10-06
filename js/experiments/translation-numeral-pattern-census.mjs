// Lists the natural-number numeral patterns (and n + k offsets) the frontends
// produce across the translation corpus and every case set, largest first, to
// size the checker's successor-chain unfolding.
import { readFileSync, readdirSync } from 'node:fs';
import { join } from 'node:path';

import { CORPUS_DIRECTORY, FRONTENDS, loadCases } from '../scripts/translation-stage-lib.mjs';

const found = [];
const walk = (node, where) => {
  if (Array.isArray(node)) return node.forEach((item) => walk(item, where));
  if (!node || typeof node !== 'object') return;
  if (node.k === 'numLit') found.push([BigInt(node.value), where]);
  if (node.k === 'natAdd') found.push([BigInt(node.add), `${where} (n + k)`]);
  Object.values(node).forEach((value) => walk(value, where));
};
const sources = (await loadCases()).map(({ name, extension, source }) => [name, [extension, source]]);
for (const name of readdirSync(CORPUS_DIRECTORY)) {
  const extension = name.split('.').pop();
  if (FRONTENDS[extension]) sources.push([name, [extension, readFileSync(join(CORPUS_DIRECTORY, name), 'utf8')]]);
}
for (const [name, [extension, source]] of sources) {
  try { walk(FRONTENDS[extension](source), name); } catch { /* rejected by the frontend */ }
}
found.sort(([a], [b]) => (a < b ? 1 : a > b ? -1 : 0));
for (const [value, where] of found.slice(0, 15)) console.log(String(value), where);
