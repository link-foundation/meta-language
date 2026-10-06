// Prints whether the tree-sitter-c oracle finds ERROR or MISSING nodes in each file.
import { readFileSync } from 'node:fs';
import { oracleRows } from '../scripts/native-grammar-rows.mjs';
for (const file of process.argv.slice(2)) {
  const rows = oracleRows(readFileSync(file, 'utf8'), 'c');
  const bad = rows.filter((row) => row[2] === 'ERROR' || String(row[2]).startsWith('MISSING'));
  console.log(`${bad.length > 0 ? 'ERRORS' : 'clean '} ${file} ${JSON.stringify(bad.slice(0, 3))}`);
}
