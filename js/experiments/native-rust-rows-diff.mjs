// Prints the native rows of SOURCE next to the tree-sitter oracle rows and the
// first row where they part, to trace a clean-oracle tree difference.
// Usage: SOURCE='fn f() { |(|a|b)| c; }' [GRAMMAR=c ORACLE=C] node experiments/native-rust-rows-diff.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const grammar = process.env.GRAMMAR ?? 'rust';
const language = process.env.ORACLE ?? 'Rust';
const fixture = JSON.parse(read(`parity/fixtures/native-grammars/${grammar}.json`));
const parser = compileGrammar(parseGrammarLinks(read(`parity/grammars/native/${grammar}.lino`)));
const source = process.env.SOURCE;
const expected = oracleRows(source, language);
const outcome = parser.parseTree(source);
console.log('oracle recovers', oracleRecovers(source, language), 'native ok', outcome.ok, 'ambiguities', outcome.ambiguities?.length);
const actual = outcome.ok ? nativeRows(outcome.tree, source, fixture) : [];
const at = expected.findIndex((row, index) => JSON.stringify(row) !== JSON.stringify(actual[index]));
console.log('first difference at row', at);
for (let index = Math.max(0, at - 3); at >= 0 && index < Math.min(expected.length, at + 6); index += 1) {
  console.log(JSON.stringify(expected[index]).padEnd(60), JSON.stringify(actual[index]));
}
