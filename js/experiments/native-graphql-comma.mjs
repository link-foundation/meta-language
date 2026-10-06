// Prints the native GraphQL rows and the tree-sitter-graphql oracle rows of
// one source, to compare where a comma, an extra that rules also name, goes.
//   node experiments/native-graphql-comma.mjs 'query Q($a: Int, $b: Int) { a }'
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/graphql.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comma', 'comment'], oracleKinds: nativeOracleKinds(text) };
const source = process.argv[2] ?? 'query Q($a: Int, $b: Int) { a }';
const outcome = compiled.parseTree(source);
const native = outcome.ok ? nativeRows(outcome.tree, source, options) : [];
const oracle = oracleRows(source, 'GraphQL');
console.log(outcome.ok ? `native ok, ${outcome.ambiguities?.length ?? 0} ambiguities` : `native rejects ${JSON.stringify(outcome.rejection).slice(0, 200)}`);
for (let index = 0; index < Math.max(native.length, oracle.length); index += 1) {
  const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
  console.log(`${o === n ? ' ' : '!'} ${o}   ${o === n ? '' : n}`);
}
