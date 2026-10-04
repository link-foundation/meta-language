// Prints the semantic (non-syntax) links meta-language reads from LiNo samples.
import { LinkNetwork } from '../js/src/index.js';
const skip = new Set(['SourceToken', 'Syntax', 'Trivia', 'Field', 'Language', 'Grammar']);
for (const src of process.argv.slice(2).map((s) => JSON.parse(`"${s}"`))) {
  const n = LinkNetwork.parse(src, 'LiNo');
  console.log(JSON.stringify(src));
  for (const l of n.links()) {
    const m = l.metadata();
    if (skip.has(m.linkType)) continue;
    console.log(' ', l.id().asU64(), m.linkType, m.term ?? '-', l.references().map((r) => r.asU64()).join(','), m.span ? `${m.span.byteRange.start}-${m.span.byteRange.end}` : '');
  }
}
