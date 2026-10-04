// Prints the native Lean tree and the oracle rows of each source given on
// the command line, with the rows that differ marked, to study one case of
// experiments/native-lean-corpus.mjs.
//   node experiments/native-lean-case.mjs '#check foo 2 3'
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/lean.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
for (const source of process.argv.slice(2)) {
  const oracle = oracleRows(source, 'Lean');
  const outcome = compiled.parseTree(source);
  console.log(JSON.stringify(source));
  if (!outcome.ok) {
    console.log(`  REJECT ${JSON.stringify(outcome.rejection).slice(0, 300)}`);
    for (const row of oracle) console.log(`  o ${JSON.stringify(row)}`);
    continue;
  }
  const native = nativeRows(outcome.tree, source, options);
  for (let index = 0; index < Math.max(oracle.length, native.length); index += 1) {
    const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
    console.log(o === n ? `  = ${o}` : `  ! ${o}   ${n}`);
  }
}
