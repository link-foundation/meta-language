// Parses candidate recovery sources for the shared embedded fixtures and
// shows whether the embedded region holds error or missing nodes.
import { LinkNetwork, LinkType } from '../src/index.js';

const candidates = [
  ['HTML', '<script>const answer = ;</script>'],
  ['HTML', '<style>.answer { color: green; </style>'],
  ['Markdown', '# Demo\n```Rust\nfn answer() -> u32 { 42\n```\n'],
  ['Markdown', '# Demo\n<section><strong>answer</strong></section>\n'],
  ['Markdown', '# Demo\n<section><strong>answer</strong></section></div>\n'],
  ['Markdown', '# Demo\n<section><strong answer=">\n'],
  ['HTML', '<p style="color: blue {">answer</p>'],
  ['HTML', '<p style="color: ;">answer</p>'],
];
for (const [language, source] of candidates) {
  const network = LinkNetwork.parse(source, language);
  const regions = network.links().filter((link) => link.metadata().linkType === LinkType.Region);
  const bad = network.links().filter((link) => {
    const metadata = link.metadata();
    return metadata.linkType === LinkType.Syntax && (metadata.flags.isError || metadata.flags.isMissing);
  }).map((link) => `${link.metadata().language}:${link.metadata().term}@${link.metadata().span.byteRange.start}`);
  console.log(JSON.stringify(source), network.verifyFullMatch().isClean(), regions.map((r) => r.metadata().language), bad);
}
