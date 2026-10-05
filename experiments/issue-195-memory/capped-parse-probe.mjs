// Parses a synthetic TypeScript input that needs error recovery, with the
// native TypeScript grammar under a memory budget, and prints the rejection
// and the peak heap: run it under `node --max-old-space-size=N` to see the
// budget turn the parse into a diagnostic instead of a heap exhaustion.
//   node --max-old-space-size=160 experiments/issue-195-memory/capped-parse-probe.mjs COPIES LIMIT
import { compileGrammar } from '../../js/src/grammar.js';
import { parseGrammarLinks } from '../../js/src/grammar-links.js';
import { nativeGrammarText } from '../../js/src/native-grammar-parser.js';

const [copies = '200', limit = '100000'] = process.argv.slice(2);
const unit = (i) => `export const table${i}: Map<string, Array<number>> = new Map([["a", [1, 2, ${i}]]]);\n`;
const source = Array.from({ length: Number(copies) }, (_, i) => unit(i)).join('');
const parser = compileGrammar(parseGrammarLinks(nativeGrammarText('native-typescript')));
const started = performance.now();
const outcome = parser.parseTree(source, { errorRecovery: true, recovery: 'accept', memoryLimit: Number(limit) });
console.log(JSON.stringify({ bytes: source.length, ok: outcome.ok, tree: outcome.tree?.type ?? null, rejection: outcome.rejection,
  ms: Math.round(performance.now() - started), heapMiB: Math.round(process.memoryUsage().heapUsed / 2 ** 20) }));
