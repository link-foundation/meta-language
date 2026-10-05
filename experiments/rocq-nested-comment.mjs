// Prints the native Rocq tree and rows of a source next to the oracle's rows.
//   node experiments/rocq-nested-comment.mjs '(* a (* b *) c *)'
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../js/src/grammar.js';
import { parseGrammarLinks } from '../js/src/grammar-links.js';
import { nativeOracleKinds } from '../js/scripts/build-language-catalog.mjs';
import { nativeRows, oracleRows } from '../js/scripts/native-grammar-rows.mjs';

const text = readFileSync(new URL('../parity/grammars/native/rocq.lino', import.meta.url), 'utf8');
const compiled = compileGrammar(parseGrammarLinks(text));
const source = process.argv[2] ?? '(*(*(**)*)*)';
const outcome = compiled.parseTree(source);
console.log(JSON.stringify(outcome.tree ?? outcome.rejection));
const options = { hidden: [], anonymous: ['unnamed_token'], extras: ['comment'], oracleKinds: nativeOracleKinds(text) };
console.log('oracle', JSON.stringify(oracleRows(source, 'Rocq')));
if (outcome.ok) console.log('native', JSON.stringify(nativeRows(outcome.tree, source, options)));
