// Dumps syntax mappings (term, byte start, byte end) as the Rust twin does.
import { readFileSync } from 'node:fs';
import { LinkNetwork, LinkType } from '../js/src/index.js';
const [language, ...files] = process.argv.slice(2);
for (const path of files) {
  const source = readFileSync(path, 'utf8');
  const network = LinkNetwork.parse(source, language);
  console.log(`== ${path} clean=${network.verifyFullMatch().isClean()}`);
  for (const link of network.links()) {
    const m = link.metadata();
    if (m.linkType !== LinkType.Syntax || !m.span || !m.term) continue;
    console.log(`${m.term} ${m.span.byteRange.start} ${m.span.byteRange.end}`);
  }
}
