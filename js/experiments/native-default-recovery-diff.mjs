// Compares the recorded native default-CST recovery rows of the given
// languages with a fresh native repair, without running any oracle.
// Usage: node experiments/native-default-recovery-diff.mjs JavaScript TypeScript
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { NATIVE_GRAMMARS } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows } from '../scripts/native-grammar-rows.mjs';

const root = resolve(import.meta.dirname, '../..');
const inventory = JSON.parse(readFileSync(resolve(root, 'parity/language-grammar-inventory.json'), 'utf8'));
const expected = JSON.parse(readFileSync(resolve(root, 'parity/fixtures/native-default-cst-expected.json'), 'utf8'));
for (const name of process.argv.slice(2)) {
  for (const entry of NATIVE_GRAMMARS.filter((candidate) => candidate.language === name)) {
    const language = inventory.languages.find((candidate) => candidate.name === name);
    const parser = compileGrammar(parseGrammarLinks(readFileSync(resolve(root, entry.grammar), 'utf8')));
    const repaired = parser.parseTree(language.recoverySource, { errorRecovery: true, recovery: 'accept' });
    const rows = nativeRows(repaired.tree, language.recoverySource, entry);
    const same = JSON.stringify(rows) === JSON.stringify(expected.languages[name]?.recovery);
    console.log(name, entry.id, same ? 'SAME' : 'CHANGED');
  }
}
