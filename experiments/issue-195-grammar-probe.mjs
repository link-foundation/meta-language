// Probes which Lean/Rocq snippets the bundled tree-sitter grammars parse without ERROR/missing nodes.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from '../js/src/index.js';

const [language, file] = process.argv.slice(2);
const chunks = readFileSync(file, 'utf8').split(/^=====\n/m);
for (const chunk of chunks) {
  if (!chunk.trim()) continue;
  const clean = LinkNetwork.parse(chunk, language).verifyFullMatch().isClean();
  console.log(clean ? 'CLEAN' : 'ERROR', '|', chunk.trim().split('\n').join(' ⏎ ').slice(0, 150));
}
