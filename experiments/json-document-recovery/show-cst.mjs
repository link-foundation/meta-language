import { LinkNetwork, LinkType } from '../../js/src/index.js';

const documents = process.argv.slice(2);
for (const source of documents) {
  const network = LinkNetwork.parse(source, 'JSON');
  console.log(JSON.stringify(source));
  for (const link of network.links()) {
    const { term, linkType, flags, span } = link.metadata();
    if (linkType !== LinkType.Syntax && !flags.hasRecoveryIssue()) continue;
    const { start, end } = span.byteRange;
    console.log(link.id().asU64(), [start, end], term, flags, link.references().map(id => id.asU64()));
  }
}
