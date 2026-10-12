// Prints every case of `cases.lino` imported by the JavaScript Lark importer in
// the same format as `src/main.rs`, so the two outputs can be diffed.
import { readProjectData } from '../../js/scripts/project-data.mjs';

import { importLark } from '../../js/src/grammar-importers/lark.js';
import { renderGrammarExpression } from '../../js/tests/support/render-grammar-expression.js';

const path = process.argv[2] ?? new URL('./cases.lino', import.meta.url);
const cases = readProjectData(path);
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
