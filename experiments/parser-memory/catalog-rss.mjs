// Parses every inventory language once and reports resident memory after each,
// to locate memory growth in the grammar-backed parser.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../../js/src/index.js';

const inventory = JSON.parse(readFileSync(new URL('../../parity/language-grammar-inventory.json', import.meta.url), 'utf8'));
const mb = () => Math.round(process.memoryUsage().rss / 1048576);
console.log('start', mb());
for (const language of inventory.languages) {
  if (!language.source) continue;
  const started = Date.now();
  try { LinkNetwork.parse(language.source, language.name); } catch (error) { console.log(language.name, 'error', error.message.slice(0, 80)); continue; }
  console.log(language.name, mb(), `${Date.now() - started}ms`);
}
console.log('done', mb(), JSON.stringify(Object.fromEntries(Object.entries(process.memoryUsage()).map(([k, v]) => [k, Math.round(v / 1048576)]))));
process.on('beforeExit', () => console.log('beforeExit', mb()));
process.on('exit', () => console.log('exit', mb(), readFileSync('/proc/self/status', 'utf8').match(/VmHWM:\s*(\d+)/)[1] >> 10, 'MB peak'));
if (process.env.EXPLICIT_EXIT) process.exit(0);
