// Shows one issue 195 conformance case: its source, the oracle CST, the JS runtime's
// canonical CST and the problems the conformance test reports for it.
// Usage: node experiments/issue-195-conformance-case.mjs LANGUAGE CASE_ID
import { readFileSync } from 'node:fs';

import { LinkNetwork, LinkType } from '../js/src/index.js';
import { diagnosticProblems, documentGrammarRoots, renderCstLines, triviaProblems } from '../js/tests/support/cst-lines.js';
import { parseCorpus } from '../js/tests/support/cst-sexpression.js';

const [language, id] = process.argv.slice(2);
const root = new URL('../parity/fixtures/issue-195-conformance/', import.meta.url);
const manifest = JSON.parse(readFileSync(new URL('manifest.json', root)));
const details = manifest.languages[language];
const oracle = JSON.parse(readFileSync(new URL(details.oracle, root)));
const entry = oracle.cases.find((candidate) => candidate.id === id);
let source = entry.source;
if (source === undefined && id.startsWith('corpus/')) {
  const [, file, index] = id.split('/');
  source = parseCorpus(readFileSync(new URL(`${details.corpus.directory}/${file}`, root), 'utf8'))[Number(index)].source;
} else if (source === undefined) {
  source = readFileSync(new URL(details.projects.find((p) => p.file.endsWith(id.split('/')[1])).file, root), 'utf8');
}
console.log('source', JSON.stringify(source));
const network = LinkNetwork.parse(source, language);
const { text, rendered } = renderCstLines(documentGrammarRoots(network, language), language);
const left = text.split('\n');
const right = entry.cst.split('\n');
for (let i = 0; i < Math.max(left.length, right.length); i += 1) {
  console.log(`${left[i] === right[i] ? ' ' : '!'} ${(left[i] ?? '').padEnd(60)} | ${right[i] ?? ''}`);
}
for (const link of network.links()) {
  const m = link.metadata();
  if (m.linkType === LinkType.Trivia || (m.linkType === LinkType.SourceToken && m.flags?.isExtra)) {
    console.log(m.linkType, m.term, JSON.stringify(m.term), m.span?.byteRange);
  }
}
console.log('trivia', triviaProblems(network, source, entry.cst));
console.log('diagnostics', diagnosticProblems(network, rendered, entry.cst));
