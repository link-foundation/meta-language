// Prints the ambiguities the native Go grammar reports on one source.
//   node experiments/native-go-ambiguity.mjs 'package main'
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';

const compiled = compileGrammar(parseGrammarLinks(readFileSync(new URL('../../parity/grammars/native/go.lino', import.meta.url), 'utf8')));
const source = process.argv[2].replace(/\\n/gu, '\n');
const outcome = compiled.parseTree(source);
console.log(JSON.stringify({ ok: outcome.ok, rejection: outcome.rejection, ambiguities: outcome.ambiguities }, null, 1).slice(0, 4000));
