// Prints the pinned tree-sitter oracle rows of each source.
//   node experiments/oracle-rows.mjs LANGUAGE 'source' ['source' ...]
import { oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, ...sources] = process.argv.slice(2);
for (const source of sources) {
  console.log(JSON.stringify(source));
  for (const row of oracleRows(source, language)) console.log('  ' + '  '.repeat(row[0]) + `${row[1] ? row[1] + ': ' : ''}${row[2]}${row[3] ? '' : ' (anon)'} ${row[4]}-${row[5]} ${row[6]}`);
}
