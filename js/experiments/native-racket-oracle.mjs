// Prints the tree-sitter-racket oracle rows of the sources given as one
// JSON array on the command line, to design parity/grammars/native/racket.lino:
//   node experiments/native-racket-oracle.mjs '["(a . b)", "#(1 #\\x)"]'
// With --brief it prints only whether the oracle recovers.
import { oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const brief = process.argv.includes('--brief');
const sources = JSON.parse(process.argv.find((arg, index) => index > 1 && arg !== '--brief') ?? '["(a b)"]');
for (const source of sources) {
  console.log(JSON.stringify(source), oracleRecovers(source, 'Racket') ? 'RECOVERS' : 'accepts');
  if (!brief) for (const row of oracleRows(source, 'Racket')) console.log('  ', JSON.stringify(row));
}
