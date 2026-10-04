import { readFile } from 'node:fs/promises';
import { parsePdfCst } from '../src/pdf-grammar.js';

const { cases } = JSON.parse(await readFile(new URL('../../parity/fixtures/pdf-grammar-cases.json', import.meta.url), 'utf8'));
const counts = { clean: 0, error: 0, oracleNull: 0, oracleOkCstError: 0 };
const terms = new Map();
const count = (node) => { terms.set(node.term, (terms.get(node.term) ?? 0) + 1); node.children.forEach(count); };
for (const { source, objects } of cases) {
  const tree = parsePdfCst(source);
  if (tree.hasError) counts.error += 1; else { counts.clean += 1; count(tree); }
  if (objects === null) counts.oracleNull += 1;
  else if (tree.hasError) counts.oracleOkCstError += 1;
}
console.log(counts);
console.log(Object.fromEntries([...terms].sort()));
