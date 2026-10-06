// Prints every link of a parsed network: id, type, term, refs, span, flags.
//   node experiments/dump-network.mjs <language> <source>
import { LinkNetwork } from '../src/index.js';

const [language, source] = process.argv.slice(2);
const network = LinkNetwork.parse(source.replaceAll('\\n', '\n'), language);
for (const link of network.links()) {
  const m = link.metadata();
  const span = m.span ? `${m.span.byteRange.start}-${m.span.byteRange.end}` : '';
  const flags = Object.entries(m.flags ?? {}).filter(([, v]) => v).map(([k]) => k).join(',');
  console.log(link.id().toString(), m.linkType, JSON.stringify(m.term), m.named ? 'N' : '', `[${link.references().map(String).join(' ')}]`, span, flags, m.language ?? '');
}
