// Reduces a Rust source the native Rust grammar rejects while the
// tree-sitter oracle parses it clean to a smaller one with the same
// property, removing chunks of lines (delta debugging) down to single
// lines, then chunks of tokens down to single tokens. Prints the reduced source and writes it next to the input as
// `.reduced.rs`.
// Usage: node experiments/native-rust-reduce.mjs <file.rs>
import { readFileSync, writeFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks } from '../src/index.js';
import { oracleRecovers } from '../scripts/native-grammar-rows.mjs';

const grammar = new URL('../../parity/grammars/native/rust.lino', import.meta.url);
const parser = compileGrammar(parseGrammarLinks(readFileSync(grammar, 'utf8')));
const file = process.argv[2];
let lines = readFileSync(file, 'utf8').split('\n');
const failing = (candidate) => {
  const source = candidate.join('\n');
  return !oracleRecovers(source, 'Rust') && !parser.parseTree(source).ok;
};
if (!failing(lines)) throw new Error('the input does not show the discrepancy');
const reduce = (parts, join, unit) => {
  for (let size = Math.ceil(parts.length / 2); size >= 1; size = size === 1 ? 0 : Math.ceil(size / 2)) {
    for (let start = 0; start < parts.length;) {
      const candidate = [...parts.slice(0, start), ...parts.slice(start + size)];
      if (candidate.length > 0 && failing(join(candidate))) {
        parts = candidate;
        writeFileSync(file.replace(/\.rs$/u, '.reduced.rs'), join(parts).join('\n'));
        console.error(`size ${size}: ${parts.length} ${unit}`);
      } else start += size;
    }
  }
  return parts;
};
lines = reduce(lines, (parts) => parts, 'lines');
const tokens = lines.join('\n').match(/\s+|\w+|'(?:\\.|[^'\\])'|"(?:\\.|[^"\\])*"|./gsu);
const reduced = reduce(tokens, (parts) => parts.join('').split('\n'), 'tokens');
console.log(reduced.join(''));
