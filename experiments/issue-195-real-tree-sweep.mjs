// Parses every file with the given extension under DIR through the public parser and prints
// the files that are not clean (lossless reconstruction is also checked).
//   node experiments/issue-195-real-tree-sweep.mjs LANGUAGE EXT DIR [MAX_BYTES]
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join } from 'node:path';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const [language, extension, directory, maxBytes = '400000'] = process.argv.slice(2);
const walk = (dir) => readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
  const full = join(dir, entry.name);
  if (entry.isDirectory()) return walk(full);
  return entry.name.endsWith(extension) ? [full] : [];
});
let clean = 0;
let dirty = 0;
for (const file of walk(directory).sort()) {
  if (statSync(file).size > Number(maxBytes)) continue;
  const source = readFileSync(file, 'utf8');
  const network = LinkNetwork.parse(source, language);
  if (network.reconstructText() !== source) console.log(`LOSSY ${file}`);
  if (network.verifyFullMatch().isClean()) { clean += 1; continue; }
  dirty += 1;
  const errors = network.links().filter((link) => link.metadata().linkType === LinkType.Syntax &&
    (link.metadata().flags.isError || link.metadata().flags.isMissing));
  const line = errors[0]?.metadata().span?.start.row;
  console.log(`DIRTY ${file}:${line + 1} ${source.split('\n')[line]?.trim().slice(0, 100)}`);
}
console.log(`${language}: ${clean} clean, ${dirty} dirty`);
