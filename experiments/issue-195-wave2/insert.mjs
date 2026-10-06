// Inserts the wave 2 rows of rows.mjs into parity/language-grammar-inventory.json
// after the Dart row, in the file's one-row-per-line style.
//   node experiments/issue-195-wave2/insert.mjs
import { readFileSync, writeFileSync } from 'node:fs';
import { ROWS } from './rows.mjs';

const path = new URL('../../parity/language-grammar-inventory.json', import.meta.url);
let text = readFileSync(path, 'utf8');
const json = (value) => (Array.isArray(value) ? `[${value.map(json).join(', ')}]` : JSON.stringify(value));
const dispatch = ROWS.map(([name, , , , extensions]) => `    ${json(name)}: ${json(extensions)},`).join('\n');
const rows = ROWS.map(([name, aliases, family, grammar, , source, recoverySource]) => {
  const backend = `tree-sitter-${grammar}`;
  return `    { "name": ${json(name)}, "aliases": ${json(aliases)}, "family": ${json(family)}, "grammars": [${json(grammar)}], "source": ${json(source)}, "recoverySource": ${json(recoverySource)}, "javascript": { "status": "grammar-cst", "backend": ${json(backend)} }, "rust": { "status": "grammar-cst", "backend": ${json(backend)} } },`;
}).join('\n');
const insertAfter = (marker, block) => {
  const at = text.indexOf(marker);
  if (at < 0) throw new Error(`missing ${marker}`);
  const end = text.indexOf('\n', at) + 1;
  text = `${text.slice(0, end)}${block}\n${text.slice(end)}`;
};
insertAfter('    "Dart": [".dart"],', dispatch);
insertAfter('    { "name": "Dart",', rows);
JSON.parse(text);
writeFileSync(path, text);
