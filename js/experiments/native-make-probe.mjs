// Prints the tree-sitter-make oracle rows and the native rows of each source
// given on the command line (JSON strings), for comparing them by hand.
//   node experiments/native-make-probe.mjs '"foo = bar\n"' ...
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(process.env.LINO ?? new URL('../../parity/grammars/native/make.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token', 'raw_line'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
for (const argument of process.argv.slice(2)) {
  const source = JSON.parse(argument);
  console.log(`== ${JSON.stringify(source)} oracle recovers: ${oracleRecovers(source, 'Make')}`);
  const oracle = oracleRows(source, 'Make');
  const outcome = compiled.parseTree(source);
  const native = outcome.ok ? nativeRows(outcome.tree, source, options) : [];
  if (!outcome.ok) console.log('  native rejects', JSON.stringify(outcome.rejection).slice(0, 300));
  for (let index = 0; index < Math.max(oracle.length, native.length); index += 1) {
    const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
    console.log(`${o === n ? ' ' : '!'} ${o}${o === n ? '' : `   ${n}`}`);
  }
}
