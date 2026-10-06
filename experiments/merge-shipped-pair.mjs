#!/usr/bin/env node
// Merges a shipped native grammar with a vendored grammars-v4 grammar:
//   node experiments/merge-shipped-pair.mjs json JSON json '{"reconcile":true}'
import { readFileSync } from 'node:fs';
import { importAntlr, mergeGrammars, parseGrammarLinks } from '../js/src/index.js';
import { startingAt } from '../js/scripts/run-grammar-bulk-pipeline.mjs';

const [native, g4, entry, options = '{}'] = process.argv.slice(2);
const tree = parseGrammarLinks(readFileSync(`parity/grammars/native/${native}.lino`, 'utf8'));
const antlr = startingAt(importAntlr(readFileSync(`parity/grammars/reconcile/grammars-v4-7df52be-${g4}.g4`, 'utf8')), entry);
const merged = mergeGrammars([
  { id: 'native', language: g4, precedence: 0, grammar: tree },
  { id: 'grammars-v4', language: g4, precedence: 1, grammar: antlr },
], JSON.parse(options));
const decisions = merged.groups.flatMap((group) => group.decisions);
const cross = decisions.filter(({ kind, members }) => (kind === 'merged' || kind === 'reconciled') && new Set(members.map((alias) => alias.split(':')[0])).size > 1);
console.log(JSON.stringify({ native: tree.rules.size, grammarsV4: antlr.rules.size, merged: merged.groups[0].grammar.rules.size, shared: cross.length }));
for (const decision of cross) console.log(`  ${decision.kind} ${decision.name}: ${decision.members.join(' ')} [${decision.basis}]`);
