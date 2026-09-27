// Prints how the JavaScript BNF/EBNF importers read quote and backslash edge cases.
import * as ml from '../js/src/index.js';
import { renderGrammarRule } from '../js/tests/support/render-grammar-expression.js';
import { cases } from './issue-195-quote-probe-cases.mjs';
for (const [format, source] of cases) {
  const importer = format === 'bnf' ? ml.importBnf : ml.importEbnf;
  try { console.log(format, JSON.stringify(source), '=>', renderGrammarRule(importer(source).rule('a'))); }
  catch (error) { console.log(format, JSON.stringify(source), '=> ERROR', error.message); }
}
