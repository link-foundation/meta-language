#!/usr/bin/env node
// Prints how reconciliation sees every rule of one language's tree-sitter and
// grammars-v4 grammars: whether it is lexical, its role and its signature.
//   node --max-old-space-size=1024 experiments/reconcile-trace.mjs JSON
import { bulkLanguages, grammarsV4Text, startingAt, treeSitterGrammarJson } from '../js/scripts/run-grammar-bulk-pipeline.mjs';
import { importAntlr, parseGrammarLinks } from '../js/src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';
import { reconcileClasses } from '../js/src/grammar-reconcile.js';

const name = process.argv[2];
const entry = bulkLanguages().find((language) => language.name === name);
const { text, pinned } = await treeSitterGrammarJson(entry.grammars[0]);
let tree;
if (pinned) {
  const { NAME_EXPANSIONS, importSource } = await import('../js/scripts/import-native-grammars.mjs');
  const { readFileSync } = await import('node:fs');
  const naming = JSON.parse(readFileSync(new URL(`../${NAME_EXPANSIONS}`, import.meta.url), 'utf8'));
  const words = new Map(naming.words.map(({ word, replacement }) => [word, replacement]));
  tree = parseGrammarLinks(importSource(pinned, words, naming.grammars[pinned.language]).text);
} else {
  tree = parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(JSON.parse(text), { wordRule: 'word_characters' })));
}
const v4 = await grammarsV4Text(name);
const antlr = startingAt(importAntlr(v4.text), v4.entryPoint);
const sources = [{ id: 'tree-sitter', grammar: tree }, { id: 'grammars-v4', grammar: antlr }];
const nodes = [];
const index = new Map();
for (const source of sources) {
  for (const rule of source.grammar.rules.values()) {
    index.set(`${source.id}:${rule.name}`, nodes.length);
    nodes.push({ alias: `${source.id}:${rule.name}`, source, name: rule.name, kind: rule.kind, expression: rule.expression, rule });
  }
}
const label = (source, internal) => (ref) => (source.grammar.rules.has(ref) ? internal(ref) : `ext(${ref})`);
reconcileClasses(nodes, index, nodes.map((_, position) => position), label, (row) => {
  console.log(`${row.key.padEnd(5)} ${row.alias} ${row.token ? "LEX" : "parser"} ${row.role ?? ''}\n      ${row.signature.slice(0, 300)}`);
});
