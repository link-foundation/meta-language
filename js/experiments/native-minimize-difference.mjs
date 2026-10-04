// Shrinks SOURCE, word by word (white-space-separated runs), to a smallest
// source the tree-sitter oracle parses clean and the native grammar parses
// to different rows, a minimal case of a clean-oracle tree difference.
// Usage: SOURCE='fn f() { ... }' [GRAMMAR=rust ORACLE=Rust] node experiments/native-minimize-difference.mjs
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { nativeRows, oracleRecovers, oracleRows } from '../scripts/native-grammar-rows.mjs';

const read = (relative) => readFileSync(new URL(`../../${relative}`, import.meta.url), 'utf8');
const grammar = process.env.GRAMMAR ?? 'rust';
const language = process.env.ORACLE ?? 'Rust';
const fixture = JSON.parse(read(`parity/fixtures/native-grammars/${grammar}.json`));
const parser = compileGrammar(parseGrammarLinks(read(`parity/grammars/native/${grammar}.lino`)));

function differs(source) {
  if (oracleRecovers(source, language)) return false;
  const outcome = parser.parseTree(source);
  if (!outcome.ok) return true;
  return JSON.stringify(nativeRows(outcome.tree, source, fixture)) !== JSON.stringify(oracleRows(source, language));
}

let words = process.env.SOURCE.split(/(\s+)/u);
if (!differs(words.join(''))) throw new Error('SOURCE does not differ');
for (let changed = true; changed;) {
  changed = false;
  for (let index = 0; index < words.length; index += 1) {
    if (/^\s*$/u.test(words[index])) continue;
    const candidate = [...words.slice(0, index), ...words.slice(index + 2)];
    if (differs(candidate.join(''))) {
      words = candidate;
      changed = true;
      index -= 1;
    }
  }
}
console.log(JSON.stringify(words.join('')));
