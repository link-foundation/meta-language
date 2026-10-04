// Parses each argument with the native TypeScript grammar (GRAMMAR=tsx for
// TSX) and its tree-sitter-typescript oracle and prints both trees' rows, or
// the native rejection, so a corpus difference can be cut down to a snippet.
//   node experiments/native-typescript-snippets.mjs '<C<D>>e.f;'
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const grammar = process.env.GRAMMAR ?? 'typescript';
const language = grammar === 'tsx' ? 'TSX' : 'TypeScript';
const text = readFileSync(new URL(`../../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment', 'html_comment'], oracleKinds: nativeOracleKinds(text) };
for (const source of process.argv.slice(2)) {
  const oracle = oracleRows(source, language);
  const outcome = compiled.parseTree(source);
  const native = outcome.ok ? nativeRows(outcome.tree, source, options) : [];
  console.log(`== ${JSON.stringify(source)} ${outcome.ok ? (JSON.stringify(oracle) === JSON.stringify(native) ? 'SAME' : 'DIFF') : 'REJECT'}`);
  if (!outcome.ok) console.log(JSON.stringify(outcome.rejection).slice(0, 400));
  for (let index = 0; index < Math.max(oracle.length, native.length); index += 1) {
    const [o, n] = [JSON.stringify(oracle[index] ?? null), JSON.stringify(native[index] ?? null)];
    console.log(`${o === n ? ' ' : '!'} ${o}${o === n ? '' : `   ${n}`}`);
  }
}
