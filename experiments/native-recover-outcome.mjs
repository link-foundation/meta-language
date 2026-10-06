// Prints the outcome of a native recovering parse of a source: its rejection
// and the ERROR and MISSING leaves of its tree.
//   node experiments/native-recover-outcome.mjs LANGUAGE FILE
import { readFileSync } from 'node:fs';

import { compileGrammar, parseGrammarLinks, renderSyntaxTree } from '../js/src/index.js';

const [language, file] = process.argv.slice(2);
const grammar = readFileSync(new URL(`../parity/grammars/native/${language}.lino`, import.meta.url), 'utf8');
const parser = compileGrammar(parseGrammarLinks(grammar));
const source = readFileSync(file, 'utf8');
const started = Date.now();
const outcome = parser.parseTree(source, { errorRecovery: true });
console.log(JSON.stringify({ ms: Date.now() - started, rejection: outcome.rejection }));
if (outcome.tree) console.log(renderSyntaxTree(outcome.tree).match(/ERROR@\d+\.\.\d+|MISSING@\d+ \S+/gu));
