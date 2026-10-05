#!/usr/bin/env node
// Imports one language's tree-sitter and grammars-v4 grammars as the bulk
// pipeline does (without compiling or parsing) and prints what the merge
// shares: node --max-old-space-size=1024 experiments/merge-pair.mjs JSON
import { bulkLanguages, grammarsV4Text, startingAt, treeSitterGrammarJson } from '../js/scripts/run-grammar-bulk-pipeline.mjs';
import { importAntlr, mergeGrammars, parseGrammarLinks } from '../js/src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';

const name = process.argv[2];
const options = JSON.parse(process.argv[3] ?? '{}');
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
const merged = mergeGrammars([
  { id: 'tree-sitter', language: name, precedence: 0, grammar: tree },
  { id: 'grammars-v4', language: name, precedence: 1, grammar: antlr },
], options);
const decisions = merged.groups.flatMap((group) => group.decisions);
const cross = decisions.filter(({ kind, members }) => (kind === 'merged' || kind === 'reconciled') && new Set(members.map((alias) => alias.split(':')[0])).size > 1);
const rules = merged.groups.reduce((sum, group) => sum + group.grammar.rules.size, 0);
console.log(JSON.stringify({ language: name, treeSitter: tree.rules.size, grammarsV4: antlr.rules.size, merged: rules, shared: cross.length }));
for (const decision of cross) console.log(`  ${decision.kind} ${decision.name}: ${decision.members.join(' ')} [${decision.basis}]`);
if (process.env.VERBOSE) {
  console.log('tree-sitter:', [...tree.rules.keys()].join(' '));
  console.log('grammars-v4:', [...antlr.rules.keys()].join(' '));
}
if (process.env.DEFS) {
  const { normalizedRuleDefinition } = await import('../js/src/index.js');
  for (const [label, grammar] of [['tree-sitter', tree], ['grammars-v4', antlr]]) {
    console.log(`== ${label}`);
    for (const rule of grammar.rules.values()) console.log(`  ${rule.name} = ${normalizedRuleDefinition(rule).slice(0, 400)}`);
  }
}
