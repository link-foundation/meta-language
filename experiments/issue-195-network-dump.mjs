// Prints every link of a public parse: id, type, term, flags, span and references.
//   node experiments/issue-195-network-dump.mjs LANGUAGE 'source'
import { LinkNetwork } from '../js/src/index.js';

const [language, source] = process.argv.slice(2);
const network = LinkNetwork.parse(source, language);
for (const link of network.links()) {
  const metadata = link.metadata();
  const span = metadata.span ? `${metadata.span.byteRange.start}..${metadata.span.byteRange.end}` : '';
  const flags = Object.entries(metadata.flags ?? {}).filter(([, value]) => value).map(([key]) => key).join(',');
  console.log(link.id().value, metadata.linkType, JSON.stringify(metadata.term), metadata.named ? 'named' : '', flags, span,
    link.references().map((reference) => reference.value).join(' '));
}
