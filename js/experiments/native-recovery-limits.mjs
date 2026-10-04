// Parses one generative or conformance case of LANGUAGE with the native grammar and
// prints whether the parse completed, how long it took and the top-level nodes, to tell a
// recovery choice from a resource limit:
//   node experiments/native-recovery-limits.mjs LANGUAGE generative <id> [replace-from replace-to]
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { documentGrammarRoots, renderCstLines } from '../tests/support/cst-lines.js';

const [language, suite, id, from, to] = process.argv.slice(2);
const root = new URL('../../parity/fixtures/', import.meta.url);
const json = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
let source;
if (suite === 'generative') {
  const file = json('issue-195-generative/manifest.json').languages[language].file;
  source = json(`issue-195-generative/${file}`).cases.find((entry) => entry.id === id).source;
} else {
  source = id;
}
if (from !== undefined) source = source.replaceAll(JSON.parse(`"${from}"`), JSON.parse(`"${to ?? ''}"`));
const started = performance.now();
const network = LinkNetwork.parse(source, language);
const elapsed = performance.now() - started;
const text = renderCstLines(documentGrammarRoots(network, language), language).text;
console.log(`${Buffer.byteLength(source)} bytes, ${elapsed.toFixed(0)} ms, clean ${network.verifyFullMatch().isClean()}`);
console.log(text.split('\n').filter((line) => /^ {0,2}\S/u.test(line)).join('\n'));
