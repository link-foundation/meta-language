// Prints the tree-sitter-ini oracle rows of a few INI sources, to design
// parity/grammars/native/ini.lino.
import { oracleRows } from '../scripts/native-grammar-rows.mjs';

const sources = process.argv.length > 2 ? process.argv.slice(2).map((s) => JSON.parse(`"${s}"`)) : [
  'a=1\n', '[s]\nk = v\n', '; c\n[s]\n', 'a=1', '\n\n[x]\n\nk=v w \n', '# c\nk=\n', '[a b]\r\nk=v\r\n', 'k = v ; not comment\n',
];
for (const source of sources) {
  console.log(JSON.stringify(source));
  for (const row of oracleRows(source, 'INI')) console.log('  ', JSON.stringify(row));
}
