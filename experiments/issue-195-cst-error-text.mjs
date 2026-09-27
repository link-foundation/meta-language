// Prints the source text of every outermost ERROR/missing CST node in the given files.
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType } from '../js/src/index.js';

const languages = { mjs: 'javascript', rs: 'rust', lean: 'lean', v: 'rocq' };
for (const file of process.argv.slice(2)) {
  const source = readFileSync(file, 'utf8');
  const bytes = Buffer.from(source, 'utf8');
  const network = LinkNetwork.parse(source, languages[file.split('.').pop()]);
  const ranges = network.links()
    .map((link) => link.metadata())
    .filter((metadata) => metadata.linkType === LinkType.Syntax && (metadata.term === 'ERROR' || metadata.isMissing))
    .map((metadata) => [(metadata.span ?? metadata).byteRange.start, (metadata.span ?? metadata).byteRange.end, metadata.isMissing ? 'MISSING ' + metadata.term : 'ERROR']);
  const outer = ranges.filter(([s, e]) => !ranges.some(([s2, e2]) => (s2 < s && e2 >= e) || (s2 <= s && e2 > e)));
  console.log(`${file}: ${outer.length} outermost error region(s)`);
  for (const [s, e, kind] of outer) console.log(`  ${kind} ${s}-${e}: ${JSON.stringify(bytes.subarray(s, e).toString('utf8').slice(0, 200))}`);
}
