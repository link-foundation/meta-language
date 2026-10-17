// Generate the pinned SQL scanner semantics as shared grammar data.
import { readFileSync, writeFileSync } from 'node:fs';
import { scannerFamilies } from '../scripts/scanner-families.mjs';
const tagPattern = '\\$[^$\\s]*\\$';
export const descriptors = [
  { family: 'remembered-delimiter', name: 'dollar_function', startToken: 'dollar_quote_opening_tag', contentToken: 'dollar_function_content', endToken: 'dollar_quote_closing_tag', tagPattern },
  { family: 'remembered-literal', name: 'dollar_literal', token: 'dollar_quoted_string', startToken: 'dollar_literal_opening_tag', contentToken: 'dollar_literal_content', endToken: 'dollar_literal_closing_tag', tagPattern, excludedLabels: 'dollar_function_labels' },
];
export const listing = scannerFamilies(descriptors);
const output = new URL('../../parity/grammars/scanners/sql.lino', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== listing) throw Error('SQL scanner generation drift');
} else {
  writeFileSync(output, listing);
  console.log('generated SQL scanner grammar data');
}
