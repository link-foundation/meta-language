// Prints the oracle problems of one conformance or generative case of a language:
//   node experiments/native-oracle-difference.mjs Rust 'conformance rust-stray-tokens'
// (ids as native-rust-recovery-discrepancies.mjs prints them; generative edit steps
// and metamorphic variants are not looked up).
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { documentOracleProblems } from '../tests/support/cst-lines.js';
import { parseCorpus } from '../tests/support/cst-sexpression.js';

const [language, id] = process.argv.slice(2);
const root = new URL('../../parity/fixtures/', import.meta.url);
const json = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
const [family, caseId] = id.split(' ');
let entry;
if (family === 'conformance') {
  const details = json('issue-195-conformance/manifest.json').languages[language];
  entry = json(`issue-195-conformance/${details.oracle}`).cases.find((candidate) => candidate.id === caseId);
  if (entry.source === undefined) {
    const [, file, index] = caseId.split('/');
    entry = { ...entry, source: parseCorpus(readFileSync(new URL(`issue-195-conformance/${details.corpus.directory}/${file}`, root), 'utf8'))[Number(index)].source };
  }
} else {
  const details = json('issue-195-generative/manifest.json').languages[language];
  entry = json(`issue-195-generative/${details.file}`).cases.find((candidate) => candidate.id === caseId);
}
const network = LinkNetwork.parse(entry.source, language);
const { problems, text } = documentOracleProblems(network, language, entry.source, entry.cst);
console.log(JSON.stringify(entry.source));
console.log(problems.join('\n'));
if (process.env.TREES) console.log(`--- native\n${text}\n--- oracle\n${entry.cst}`);
