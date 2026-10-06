// Lists the Rust conformance and generative cases whose public native tree differs
// from the tree-sitter CLI oracle, with whether the oracle and the native tree are
// clean: a clean oracle that differs is a grammar or executor bug; a malformed one
// is a recovery discrepancy.
//   node experiments/native-rust-recovery-discrepancies.mjs [LANGUAGE]
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { documentOracleProblems, parseCstLines } from '../tests/support/cst-lines.js';
import { applyEdit } from '../tests/support/generative.js';
import { parseCorpus } from '../tests/support/cst-sexpression.js';

const language = process.argv[2] ?? 'Rust';
const root = new URL('../../parity/fixtures/', import.meta.url);
const json = (path) => JSON.parse(readFileSync(new URL(path, root), 'utf8'));
const malformed = (cst) => parseCstLines(cst).some((node) => node.hasError || node.error || node.missing);
const cases = [];
const conformance = json('issue-195-conformance/manifest.json').languages[language];
const corpus = new Map();
for (const file of Object.keys(conformance.corpus.files)) {
  parseCorpus(readFileSync(new URL(`issue-195-conformance/${conformance.corpus.directory}/${file}`, root), 'utf8'))
    .forEach((entry, index) => corpus.set(`corpus/${file}/${index}`, entry.source));
}
for (const entry of json(`issue-195-conformance/${conformance.oracle}`).cases) {
  if (entry.kind === 'mixed' || entry.kind === 'project') continue;
  cases.push({ id: `conformance ${entry.id}`, source: entry.source ?? corpus.get(entry.id), cst: entry.cst });
}
const generative = json('issue-195-generative/manifest.json').languages[language];
for (const entry of json(`issue-195-generative/${generative.file}`).cases) {
  cases.push({ id: `generative ${entry.id}`, source: entry.source, cst: entry.cst });
  if (entry.kind === 'metamorphic') cases.push({ id: `generative ${entry.id} (variant)`, source: entry.variant, cst: entry.variantCst });
  let source = entry.source;
  for (const [position, step] of (entry.steps ?? []).entries()) {
    source = applyEdit(source, step);
    cases.push({ id: `generative ${entry.id} step ${position}`, source, cst: step.cst });
  }
}
let differing = 0;
for (const entry of cases) {
  const network = LinkNetwork.parse(entry.source, language);
  const { problems } = documentOracleProblems(network, language, entry.source, entry.cst);
  if (!problems.length) continue;
  differing += 1;
  const nativeClean = network.verifyFullMatch().isClean();
  console.log(`${entry.id}\toracle ${malformed(entry.cst) ? 'malformed' : 'CLEAN'}\tnative ${nativeClean ? 'clean' : 'malformed'}\t${JSON.stringify(entry.source).slice(0, 80)}`);
}
console.log(`${differing} of ${cases.length} differ`);
