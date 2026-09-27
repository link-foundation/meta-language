// Regenerates parity/fixtures/grammar-literal-quoting.json from the JavaScript
// runtime. The Rust unit test grammar_literal_quoting.rs checks the same
// records, so any divergence between the runtimes fails one side.
import { writeFileSync } from 'node:fs';
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';

const importers = { bnf: ml.importBnf, ebnf: ml.importEbnf };
const emitters = { bnf: ml.emitBnf, ebnf: ml.emitEbnf };
const imports = [
  ['bnf:backslash-is-literal', 'bnf', '<a> ::= "x\\\\y"\n'],
  ['bnf:backslash-quote-unterminated', 'bnf', '<a> ::= "\\""\n'],
  ['bnf:single-quoted-double-quote', 'bnf', "<a> ::= '\"'\n"],
  ['bnf:double-quoted-single-quote', 'bnf', '<a> ::= "\'"\n'],
  ['bnf:comment-marker-inside-literal', 'bnf', '<a> ::= "x;y" ; comment\n'],
  ['bnf:empty-terminal-alternative', 'bnf', '<a> ::= "x" | ""\n'],
  ['bnf:only-empty-terminals', 'bnf', '<a> ::= "" | \'\'\n'],
  ['bnf:empty-terminal-in-sequence', 'bnf', '<a> ::= "x" "" "y"\n'],
  ['ebnf:escaped-backslash', 'ebnf', 'a = "x\\\\y" ;\n'],
  ['ebnf:escaped-quote', 'ebnf', 'a = "\\"" ;\n'],
  ['ebnf:control-and-solidus-escapes', 'ebnf', 'a = "\\t\\/\\b\\f\\n\\r" ;\n'],
  ['ebnf:unknown-escape', 'ebnf', 'a = "\\q" ;\n'],
  ['ebnf:concatenation-binds-tighter', 'ebnf', 'a = "x" , [ "y" ] | "z" , "w" ;\n'],
  ['ebnf:juxtaposition-concatenates', 'ebnf', 'a = "x" "y" | "z" ;\n'],
  ['ebnf:empty-terminal-alternative', 'ebnf', 'a = "x" | "" ;\n'],
  ['ebnf:only-empty-terminals', 'ebnf', 'a = "" | \'\' ;\n'],
].map(([id, format, source]) => {
  try {
    return { id, format, source, rule: renderGrammarRule(importers[format](source).rule('a')) };
  } catch (error) {
    return { id, format, source, error: error.constructor.name };
  }
});
const emits = [
  ['bnf:plain', 'bnf', 'x y'],
  ['bnf:double-quote', 'bnf', 'say "hi"'],
  ['bnf:backslash', 'bnf', 'x\\y'],
  ['bnf:both-quotes', 'bnf', `it's "x"`],
  ['ebnf:double-quote', 'ebnf', 'say "hi"'],
  ['ebnf:both-quotes', 'ebnf', `it's "x"`],
  ['ebnf:backslash-and-controls', 'ebnf', 'x\\y\n\t'],
].map(([id, format, value]) => {
  const grammar = new ml.GrammarBuilder('a').rule('a', ml.GrammarBuilder.literal(value)).build();
  const { source, report } = emitters[format](grammar);
  const reimported = renderGrammarRule(importers[format](source).rule('a'));
  return { id, format, value, source, lossy: report.lossy, reimported };
});
writeFileSync(
  new URL('../parity/fixtures/grammar-literal-quoting.json', import.meta.url),
  `${JSON.stringify({ schemaVersion: 1, imports, emits }, null, 2)}\n`,
);
