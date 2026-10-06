// Prints, for each file, the ERROR and MISSING nodes of the pinned
// tree-sitter oracle with the text around them: where a native recovery may
// differ. Runs the oracle only, never the native parser.
//   node experiments/oracle-error-sites.mjs LANGUAGE FILE...
import { readFileSync } from 'node:fs';

import { oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, ...files] = process.argv.slice(2);
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  const bytes = Buffer.from(source);
  for (const [, , kind, , start, end, flags] of oracleRows(source, language)) {
    if (!flags.includes('E') && !flags.includes('M')) continue;
    const around = bytes.subarray(Math.max(0, start - 40), Math.min(bytes.length, end + 40)).toString();
    console.log(`${file} ${kind}${flags} ${start}-${end}: ${JSON.stringify(around)}`);
  }
}
