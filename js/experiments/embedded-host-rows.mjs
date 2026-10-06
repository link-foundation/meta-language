// Dumps the host grammar tree (kind, field, span, text) of an HTML or Markdown
// source and the regions the public network records, to design and check
// grammar-based embedded region detection.
//   node experiments/embedded-host-rows.mjs HTML '<script type="module">x</script>'
import { LinkNetwork, LinkType } from '../src/index.js';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';

const [language, source] = process.argv.slice(2);
const parsed = parseProgrammingLanguage(source, language);
const bytes = new TextEncoder().encode(source);
const text = (span) => new TextDecoder().decode(bytes.slice(span.byteRange.start, span.byteRange.end));
const walk = (node, depth) => {
  console.log(`${'  '.repeat(depth)}${node.field ? `${node.field}: ` : ''}${node.term} ${node.span.byteRange.start}-${node.span.byteRange.end} ${JSON.stringify(text(node.span))}`);
  for (const child of node.children) walk(child, depth + 1);
};
walk(parsed.tree, 0);
const network = LinkNetwork.parse(source, language);
for (const link of network.links().filter((l) => l.metadata().linkType === LinkType.Region)) {
  const m = link.metadata();
  console.log('region', m.language, m.span.byteRange.start, m.span.byteRange.end, JSON.stringify(text(m.span)));
}
