// Parses a file with a native grammar (no recovery) and prints the rejection,
// the time and the memo cells the parse kept. Bound the heap:
//   node --max-old-space-size=2048 experiments/issue-195-memory/ts-reject.mjs native-typescript FILE [bytes]
import { readFileSync } from 'node:fs';

import { compileGrammar } from '../../js/src/grammar.js';
import { parseGrammarLinks } from '../../js/src/grammar-links.js';
import { nativeGrammarText } from '../../js/src/native-grammar-parser.js';

const [id, file, bytes] = process.argv.slice(2);
let source = readFileSync(file, 'utf8');
if (bytes) source = source.slice(0, Number(bytes));
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText(id)));
const start = performance.now();
const outcome = parser.parseTree(source, { memoryLimit: Number(process.env.LIMIT ?? 2_000_000) });
const seconds = ((performance.now() - start) / 1000).toFixed(2);
const rejection = outcome.rejection;
const at = rejection?.offset ?? rejection?.position;
console.log(JSON.stringify({ bytes: source.length, ok: outcome.ok, seconds, rejection, context: at == null ? undefined : source.slice(Math.max(0, at - 80), at + 40) }));
