// Prints the tree-sitter-csv oracle rows of the sources given as one JSON
// array on the command line, to design parity/grammars/native/csv.lino:
//   node experiments/native-csv-oracle.mjs '["a,1\n", "\"a\"\"b\",c"]'
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

void nativeRows;
const language = process.env.LANGUAGE_NAME ?? 'CSV';
for (const source of JSON.parse(process.argv[2] ?? '["a,1\\n"]')) {
  console.log(JSON.stringify(source), oracleRecovers(source, language) ? 'RECOVERS' : '');
  for (const row of oracleRows(source, language)) console.log('  ', JSON.stringify(row));
}
