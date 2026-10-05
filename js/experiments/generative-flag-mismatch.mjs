// Reproduces the run-time fuzz cases where verifyFullMatch() is not clean but
// no link of the network is flagged with an error or a MISSING leaf.
import { LinkNetwork } from '../src/index.js';
import { propertyProblems } from '../tests/support/generative.js';

const sources = [
  "\n\"\";\n'';\n\"\\\"\";\n𝒳\"a\\\"b\";\n'\\'';\n'a\\'b';\n\"it's a tiny tiny world\";\n'\"hello\"';\nll\r\t;\n🦀",
  "\n\"hello\\\nworld\";rldd\";rld\";\n\n'hello\\\nworld';\n",
  process.argv[2],
].filter(Boolean);
for (const source of sources) {
  const network = LinkNetwork.parse(source, 'JavaScript');
  const report = network.verifyFullMatch();
  console.log(JSON.stringify(source), 'clean', report.isClean(), propertyProblems(network, source));
  console.log(JSON.stringify(report, null, 1).slice(0, 1500));
}

// Whether every Syntax link with a flagged child has an error itself.
for (const source of sources) {
  const network = LinkNetwork.parse(source, 'JavaScript');
  const links = [...network.links()];
  const byId = new Map(links.map((link) => [link.id().value, link]));
  const lost = [];
  for (const link of links) {
    const { flags, linkType, term } = link.metadata();
    if (linkType !== 'syntax' && String(linkType) !== 'Syntax' && linkType?.toString?.() !== 'Syntax') continue;
    for (const reference of link.references()) {
      const child = byId.get(reference.value)?.metadata();
      if (child?.flags && (child.flags.isError || child.flags.isMissing || child.flags.hasError) && !flags.hasError) lost.push(`${term} over ${child.term}`);
    }
  }
  console.log('lost', lost);
}
