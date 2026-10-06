// Prints the tree-sitter-json5-orchard oracle rows of the sources given as one
// JSON array on the command line, to design parity/grammars/native/json5.lino:
//   node experiments/native-json5-oracle.mjs '["{a: 1,}", "[0x1F, .5]"]'
// With --brief it prints only whether the oracle recovers.
import { oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const brief = process.argv.includes('--brief');
const sources = JSON.parse(process.argv.find((arg, index) => index > 1 && arg !== '--brief') ?? '["{a: 1}"]');
for (const source of sources) {
  console.log(JSON.stringify(source), oracleRecovers(source, 'JSON5') ? 'RECOVERS' : 'accepts');
  if (!brief) for (const row of oracleRows(source, 'JSON5')) console.log('  ', JSON.stringify(row));
}
