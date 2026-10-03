import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../src/index.js';
const parser = compileGrammar(parseGrammarLinks(readFileSync('../parity/grammars/native/c.lino', 'utf8')));
const source = process.argv[2];
const out = parser.parseTree(source);
const leaves = (t) => (t.type === 'node' ? t.children.flatMap(leaves) : [t]);
console.log(out.ok, JSON.stringify(leaves(out.tree).map(({ kind, text }) => [kind, text])));
