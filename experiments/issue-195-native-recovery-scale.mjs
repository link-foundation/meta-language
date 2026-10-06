// Time and heap of a native JSON parse of `[0,1,...,9,0,... 7]` (a stray
// value before the closing bracket) with and without automatic recovery.
import { readFileSync } from 'node:fs';
import { compileGrammar, parseGrammarLinks } from '../js/src/index.js';

const parser = compileGrammar(parseGrammarLinks(readFileSync(new URL('../parity/grammars/native/json.lino', import.meta.url), 'utf8')));
const stray = process.argv[3] ?? ' 7';
for (const count of (process.argv[2] ?? '250,500,1000,2000').split(',').map(Number)) {
  const source = `[${Array.from({ length: count }, (_, index) => String(index % 10)).join(',')}${stray}]`;
  for (const options of [{}, { errorRecovery: true }]) {
    global.gc?.();
    const before = process.memoryUsage().heapUsed;
    const started = performance.now();
    const outcome = parser.parseTree(source, options);
    const ms = performance.now() - started;
    console.log(count, JSON.stringify(options), outcome.ok, outcome.rejection?.reason ?? '', `${ms.toFixed(0)} ms`, `${((process.memoryUsage().heapUsed - before) / 1e6).toFixed(1)} MB`);
  }
}
console.log('peak rss', (process.resourceUsage().maxRSS / 1024).toFixed(0), 'MB');
