// Checks whether each native grammar reproduces the tree-sitter rows of its
// language's inventory positive source, and how it repairs the recovery source.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../js/src/index.js';
import { NATIVE_GRAMMARS } from '../js/scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows } from '../js/scripts/native-grammar-rows.mjs';

const root = new URL('../', import.meta.url);
const inventory = JSON.parse(readFileSync(new URL('parity/language-grammar-inventory.json', root), 'utf8'));
const expected = JSON.parse(readFileSync(new URL('parity/fixtures/default-cst-expected.json', root), 'utf8'));
for (const entry of NATIVE_GRAMMARS) {
  const language = inventory.languages.find(({ name }) => name === entry.language);
  const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(entry.grammar, root), 'utf8')));
  const positive = parser.parseTree(language.source);
  const rows = positive.tree ? nativeRows(positive.tree, language.source, entry) : null;
  const same = JSON.stringify(rows) === JSON.stringify(expected.languages[entry.language].positive);
  console.log(entry.language, 'positive ok', positive.ok, 'rows equal oracle', same);
  if (!same) console.log(JSON.stringify(rows), '\n', JSON.stringify(expected.languages[entry.language].positive));
  const recovery = parser.parseTree(language.recoverySource, { errorRecovery: true, recovery: 'accept' });
  console.log('  recovery', JSON.stringify(language.recoverySource), recovery.ok, recovery.tree && renderSyntaxTree(recovery.tree));
}
