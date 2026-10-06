import { readFileSync } from 'node:fs';
import { compileGrammar } from '../src/grammar.js';
import { parseGrammarLinks } from '../src/grammar-links.js';
const compiled = compileGrammar(parseGrammarLinks(readFileSync(process.env.LINO ?? '../parity/grammars/native/c.lino', 'utf8')));
const source = readFileSync(process.argv[2], 'utf8');
const out = compiled.parseTree(source, { errorRecovery: true, recovery: 'accept' });
console.log(out.ok, out.tree === null, JSON.stringify(out.rejection)?.slice(0, 800));
