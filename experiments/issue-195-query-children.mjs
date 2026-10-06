// Prints the link shape around a node kind, to compare query structural children.
// Usage: node experiments/issue-195-query-children.mjs <language> <source> <kind> [query]
import { LinkNetwork, LinkQuery } from '../js/src/index.js';

const [language, source, kind, query] = process.argv.slice(2);
const network = LinkNetwork.parse(source, language);
for (const link of network.links()) {
  const m = link.metadata();
  console.log(link.id().asU64(), m.linkType, JSON.stringify(m.term), m.named ? 'named' : '', link.references().map((id) => id.asU64()).join(' '));
}
if (query) {
  for (const match of network.queryMatches(LinkQuery.fromSexpression(query))) {
    console.log('match', match.linkId.asU64(), [...match.captures].map(([n, id]) => `${n}=${id.asU64()}`).join(' '));
  }
}
