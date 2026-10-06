// Prints the tree-sitter-scheme oracle rows of the sources given as one
// JSON array on the command line, to design parity/grammars/native/scheme.lino:
//   node experiments/native-scheme-oracle.mjs '["(a . b)", "#(1 #\\x)"]'
// With --brief it prints only whether the oracle recovers.
import { oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const brief = process.argv.includes('--brief');
const sources = JSON.parse(process.argv.find((arg, index) => index > 1 && arg !== '--brief') ?? '["(a b)"]');
for (const source of sources) {
  console.log(JSON.stringify(source), oracleRecovers(source, 'Scheme') ? 'RECOVERS' : 'accepts');
  if (!brief) for (const row of oracleRows(source, 'Scheme')) console.log('  ', JSON.stringify(row));
}
