// Parses JavaScript snippets with the native grammar
// (parity/grammars/native/javascript.lino) and the tree-sitter-javascript
// oracle and prints SAME or DIFF with the first differing rows; the snippets
// are the cases the native scanner port of automatic semicolons, html
// comments and ternary question marks must decide as the C scanner does.
//   node experiments/native-javascript-snippets.mjs [SNIPPET...]
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
import { nativeOracleKinds } from '../scripts/build-language-catalog.mjs';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../../parity/grammars/native/javascript.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment', 'html_comment'], oracleKinds: nativeOracleKinds(text) };
const snippets = process.argv.length > 2 ? process.argv.slice(2) : [
  'x // c\n(y)', 'return // c\nx', 'x // c\ny', 'let x // c\ny', 'x /* c */ y', 'x /* c\n */ (y)',
  'break /* c\n */ (y)', 'a\n/* c */ +b', 'a ? b : c', 'a?.b', 'a ? .5 : c', '`a${b}c`', 'x\n++y', 'x\n.5',
  'if (n-->0){}', '/<!--/;', '<!-- c\ny * z;\n--> d\n', 'a\nin b', 'f(a, b)\n[c]', '<a>t {x} u</a>',
  'x\n!y', 'x\n!= y', 'return\nx',
];
for (const source of snippets) {
  const started = performance.now();
  const recovers = oracleRecovers(source, 'JavaScript');
  const oracle = oracleRows(source, 'JavaScript');
  const outcome = compiled.parseTree(source);
  const ms = Math.round(performance.now() - started);
  if (!outcome.ok) {
    console.log(`REJECT ${JSON.stringify(source)} (${ms} ms) ${JSON.stringify(outcome.rejection)}${recovers ? ' [oracle recovers]' : ''}`);
    continue;
  }
  const native = nativeRows(outcome.tree, source, options);
  const same = JSON.stringify(oracle) === JSON.stringify(native);
  console.log(`${same ? 'SAME' : 'DIFF'} ${JSON.stringify(source)} (${ms} ms)${recovers ? ' [oracle recovers]' : ''}`);
  if (!same) {
    const rows = Math.max(oracle.length, native.length);
    for (let index = 0; index < rows; index += 1) {
      const [o, n] = [JSON.stringify(oracle[index]), JSON.stringify(native[index])];
      console.log(`  ${o === n ? ' ' : '!'} ${o}   ${o === n ? '' : n}`);
    }
  }
}
