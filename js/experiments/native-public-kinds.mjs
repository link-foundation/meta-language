// Prints the named syntax kinds of the public network of a source and of the
// pinned tree-sitter oracle rows, and the kinds only one of them has.
//   node experiments/native-public-kinds.mjs LANGUAGE 'source' | -f FILE
import { readFileSync } from 'node:fs';

import { LinkNetwork, LinkType } from '../src/index.js';
import { oracleRows } from '../scripts/native-grammar-rows.mjs';

const [language, flag, file] = process.argv.slice(2);
const source = flag === '-f' ? readFileSync(file, 'utf8') : flag;
const network = LinkNetwork.parse(source, language);
const pub = new Map();
for (const link of network.links()) {
  const m = link.metadata();
  if (m.linkType === LinkType.Syntax && m.named) {
    const key = `${m.term}${m.flags.isMissing ? ' (missing)' : ''}${m.flags.isError ? ' (error)' : ''}`;
    pub.set(key, (pub.get(key) ?? 0) + 1);
  }
}
const oracle = new Map();
for (const row of oracleRows(source, language)) {
  if (row[3]) oracle.set(row[2], (oracle.get(row[2]) ?? 0) + 1);
}
console.log('public only:', [...pub].filter(([k]) => !oracle.has(k)));
console.log('oracle only:', [...oracle].filter(([k]) => !pub.has(k)));
if (process.env.ALL) console.log(pub, oracle);
