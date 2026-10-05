// Prints each rule's kind and top expression kind for one language's two sources.
import { bulkLanguages, grammarsV4Text, startingAt, treeSitterGrammarJson } from '../js/scripts/run-grammar-bulk-pipeline.mjs';
import { importAntlr, parseGrammarLinks } from '../js/src/index.js';
import { importTreeSitterNative, renderTreeSitterNative } from '../js/src/grammar-importers/tree-sitter-native.js';
const name = process.argv[2];
const entry = bulkLanguages().find((language) => language.name === name);
const { text, pinned } = await treeSitterGrammarJson(entry.grammars[0]);
console.log('pinned', Boolean(pinned));
const tree = pinned ? null : parseGrammarLinks(renderTreeSitterNative(importTreeSitterNative(JSON.parse(text), { wordRule: 'word_characters' })));
const v4 = await grammarsV4Text(name);
const antlr = startingAt(importAntlr(v4.text), v4.entryPoint);
for (const [label, grammar] of [['ts', tree], ['v4', antlr]]) {
  if (!grammar) continue;
  for (const rule of grammar.rules.values()) console.log(label, rule.name, rule.kind, rule.expression.kind, Object.keys(rule).join(','));
}
