// Writes the projected native tree and parse time of every case of a
// generative fixture as JSON lines, to compare two executor versions.
//   node experiments/native-fixture-trees.mjs LANGUAGE > out.jsonl
import { readFileSync } from 'node:fs';
import { parseNative } from '../js/src/native-grammar-parser.js';

const [language = 'Lean'] = process.argv.slice(2);
const dir = new URL('../parity/fixtures/issue-195-generative/', import.meta.url);
const manifest = JSON.parse(readFileSync(new URL('manifest.json', dir), 'utf8'));
const fixture = JSON.parse(readFileSync(new URL(manifest.languages[language].file, dir), 'utf8'));
const show = ({ term, start, end, children, isError, isMissing }) => (children.length > 0
  ? `(${term}${isError ? '!' : ''} ${children.map(({ node, field }) => (field ? `${field}:` : '') + show(node)).join(' ')})`
  : `${isMissing ? 'MISSING ' : ''}${term}@${start}-${end}`);
for (const entry of fixture.cases) {
  for (const key of ['source', 'variant']) {
    if (typeof entry[key] !== 'string') continue;
    const started = performance.now();
    const tree = show(parseNative(`native-${language.toLowerCase()}`, entry[key]));
    console.log(JSON.stringify({ id: `${entry.id}/${key}`, ms: Math.round(performance.now() - started), tree }));
  }
}
