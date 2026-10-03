// Time and peak heap of a native JSON parse as the input grows.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const grammar = process.argv[2] ?? 'json';
const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL(`../parity/grammars/native/${grammar}.lino`, import.meta.url), 'utf8')));
for (const count of (process.argv[3] ?? '100,500,1000,2000').split(',').map(Number)) {
  const source = `[${Array.from({ length: count }, (_, i) => `{"k${i}": [${i}, true, null, "s"]}`).join(',\n')}]`;
  global.gc?.();
  const before = process.memoryUsage().heapUsed;
  const started = performance.now();
  const outcome = parser.parseTree(source, { errorRecovery: true });
  const ms = performance.now() - started;
  const heap = (process.memoryUsage().heapUsed - before) / 1e6;
  console.log(count, source.length, 'bytes', outcome.ok, outcome.rejection?.reason ?? '', `${ms.toFixed(0)} ms`, `${heap.toFixed(1)} MB`, `${(heap * 1e6 / source.length).toFixed(0)} B/byte`);
}
console.log('peak rss', (process.resourceUsage().maxRSS / 1024).toFixed(0), 'MB');
