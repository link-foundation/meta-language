// Checks that the native Rust grammar builds the oracle rows of each source
// given, as a `matches` case of parity/fixtures/native-grammars/rust.json
// must. Usage: node experiments/native-rust-match-check.mjs <source>...
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { NATIVE_GRAMMARS } from '../scripts/generate-native-grammar-fixtures.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const entry = NATIVE_GRAMMARS.find(({ id }) => id === 'rust');
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../../${entry.grammar}`, import.meta.url), 'utf8')));
for (const source of process.argv.slice(2)) {
  const outcome = parser.parseTree(source);
  const same = outcome.ok && JSON.stringify(nativeRows(outcome.tree, source, entry)) === JSON.stringify(oracleRows(source, entry.language));
  console.log(JSON.stringify(source), 'oracle recovers', oracleRecovers(source, entry.language), 'native', outcome.ok, 'same', same);
}
