// Prints every case of `cases.json` imported by the JavaScript Lark importer in
// the same format as `src/main.rs`, so the two outputs can be diffed.
import { readFileSync } from 'node:fs';

import { importLark } from '../../js/src/grammar-importers/lark.js';
import { renderGrammarExpression } from '../../js/tests/support/render-grammar-expression.js';

const path = process.argv[2] ?? new URL('./cases.json', import.meta.url);
const cases = JSON.parse(readFileSync(path, 'utf8'));
const lines = [];
for (const [name, source] of cases) {
  lines.push(`== ${name}`);
  let grammar;
  try {
    grammar = importLark(source);
  } catch (error) {
    if (error.name !== 'GrammarImportError') throw error;
    lines.push(`error ${error.kind}: ${error.message}`);
    continue;
  }
  lines.push(`format ${grammar.sourceFormat ?? 'none'}`);
  lines.push(`start ${grammar.start ?? 'none'}`);
  for (const rule of grammar.rules.values()) {
    lines.push(`rule ${rule.name} = ${rule.kind} ${renderGrammarExpression(rule.expression)}`);
    const doc = grammar.ruleDocs.get(rule.name);
    if (doc !== undefined) lines.push(`  doc ${JSON.stringify(doc)}`);
  }
  const undefinedNames = grammar.undefinedNonterminals().map((name) => JSON.stringify(name));
  lines.push(`undefined {${undefinedNames.join(', ')}}`);
}
process.stdout.write(`${lines.join('\n')}\n`);
