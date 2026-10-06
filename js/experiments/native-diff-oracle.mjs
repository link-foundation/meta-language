// Prints the tree-sitter-diff oracle rows of a few unified diffs, to design
// parity/grammars/native/diff.lino.
import { oracleRows } from '../scripts/native-grammar-rows.mjs';

const language = process.env.LANGUAGE_NAME ?? 'Diff';
const sources = process.argv.length > 2 ? process.argv.slice(2).map((s) => JSON.parse(`"${s}"`)) : [
  'diff --git a/x b/x\nindex 1234567..89abcde 100644\n--- a/x\n+++ b/x\n@@ -1,2 +1,2 @@ f\n a\n-b\n+c\n',
  '+a\n', '-a\n', ' a\n', '# c\n', 'x', '\n', '--- a\n+++ b\n',
];
for (const source of sources) {
  console.log(JSON.stringify(source));
  for (const row of oracleRows(source, language)) console.log('  ', JSON.stringify(row));
}
