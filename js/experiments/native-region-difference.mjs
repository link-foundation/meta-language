// Prints the public region tree and the oracle tree of the mixed-language conformance
// cases of LANGUAGE whose region differs from the oracle.
//   node experiments/native-region-difference.mjs [LANGUAGE]
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { firstDifference, regionGrammarRoots, renderCstLines } from '../tests/support/cst-lines.js';

const language = process.argv[2] ?? 'Rust';
const root = new URL('../../parity/fixtures/issue-195-conformance/', import.meta.url);
const json = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
const inputs = json('cases.json');
const oracle = json(json('manifest.json').languages[language].oracle);
for (const entry of oracle.cases.filter((candidate) => candidate.kind === 'mixed')) {
  const host = inputs.mixed.find((candidate) => candidate.id === entry.hostCase);
  const network = LinkNetwork.parse(host.source, host.host);
  const region = regionGrammarRoots(network).find((candidate) => candidate.language === language &&
    candidate.span.byteRange.start === entry.startByte && candidate.span.byteRange.end === entry.endByte);
  const text = region ? renderCstLines(region, language).text : null;
  if (text === entry.cst) continue;
  console.log(`${entry.id} ${JSON.stringify(host.source.slice(entry.startByte, entry.endByte))}`);
  if (!text) { console.log('  no region'); continue; }
  console.log(firstDifference(text, entry.cst));
  console.log(`native:\n${text}\noracle:\n${entry.cst}`);
}
