// Times the native parse of every case of a generative fixture, slowest
// first, in the JavaScript executor.
//   node experiments/lean-generative-timing.mjs [LANGUAGE] [ID...]
import { readFileSync } from 'node:fs';
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language = 'Lean', ...ids] = process.argv.slice(2);
const dir = new URL('../parity/fixtures/issue-195-generative/', import.meta.url);
const manifest = JSON.parse(readFileSync(new URL('manifest.json', dir), 'utf8'));
const fixture = JSON.parse(readFileSync(new URL(manifest.languages[language].file, dir), 'utf8'));
const grammar = `native-${language.toLowerCase()}`;
const timings = [];
for (const entry of fixture.cases) {
  if (ids.length > 0 && !ids.includes(entry.id)) continue;
  for (const key of ['source', 'variant']) {
    if (typeof entry[key] !== 'string') continue;
    const started = performance.now();
    const root = parseNative(grammar, entry[key]);
    timings.push([Math.round(performance.now() - started), entry.id, key, !root.hasError, entry[key].length]);
  }
}
timings.sort((a, b) => b[0] - a[0]);
console.log(`total ${timings.reduce((sum, [ms]) => sum + ms, 0)} ms over ${timings.length} parses`);
for (const timing of timings.slice(0, 12)) console.log(timing.join(' '));
