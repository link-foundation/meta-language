// Prints every case of cases.txt and the Rust ANTLR fixtures imported by the
// JavaScript importAntlr, in the same text as the Rust `src/main.rs`.
import { readFileSync } from 'node:fs';

import { importAntlr } from '../../js/src/grammar-importers/antlr.js';
import { renderGrammarRule } from '../../js/tests/support/render-grammar-expression.js';

const read = (path) => readFileSync(new URL(path, import.meta.url), 'utf8');

const cases = ['arithmetic.g4', 'covering.g4', 'lexer-mode.g4', 'case-insensitive.g4'].map((file) =>
  [`fixture ${file}`, read(`../../rust/tests/fixtures/grammar/antlr/${file}`)]);
let current = null;
for (const line of read('./cases.txt').split('\n')) {
  if (line.startsWith('### ')) {
    if (current) cases.push([current[0], current[1].join('\n')]);
    current = [line.slice(4), []];
  } else if (current) {
    current[1].push(line);
  }
}
if (current) cases.push([current[0], current[1].join('\n')]);

for (const [name, source] of cases) {
  console.log(`### ${name}`);
  let grammar;
  try {
    grammar = importAntlr(source);
  } catch (error) {
    console.log(`error: ${error.message}`);
    continue;
  }
  console.log(`start: ${grammar.startRule()?.name ?? '-'}`);
  for (const rule of grammar.rules.values()) {
    const doc = rule.doc === undefined ? '-' : JSON.stringify(rule.doc);
    console.log(`rule ${rule.name}: ${renderGrammarRule(rule)} | doc: ${doc}`);
  }
}
