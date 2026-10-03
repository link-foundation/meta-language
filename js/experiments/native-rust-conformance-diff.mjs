// Lists the Rust conformance and generative cases whose native default tree
// differs from the tree-sitter CLI oracle, split by whether the oracle tree is
// clean (a native parsing defect) or has errors (a recovery difference).
// Usage: node experiments/native-rust-conformance-diff.mjs [conformance|generative]
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { documentOracleProblems } from '../tests/support/cst-lines.js';
import { parseCorpus } from '../tests/support/cst-sexpression.js';
import { applyEdit } from '../tests/support/generative.js';

const language = 'Rust';
const which = process.argv[2] ?? 'conformance';
const root = new URL('../../parity/fixtures/', import.meta.url);
const read = (path) => readFileSync(new URL(path, root));
const cases = [];
if (which === 'conformance') {
  const manifest = JSON.parse(read('issue-195-conformance/manifest.json'));
  const details = manifest.languages[language];
  const oracle = JSON.parse(read(`issue-195-conformance/${details.oracle}`));
  const sources = new Map();
  for (const file of Object.keys(details.corpus.files)) {
    parseCorpus(read(`issue-195-conformance/${details.corpus.directory}/${file}`).toString('utf8'))
      .forEach((corpusCase, index) => sources.set(`corpus/${file}/${index}`, corpusCase.source));
  }
  for (const project of details.projects) sources.set(`project/${project.file.split('/').pop()}`, read(`issue-195-conformance/${project.file}`).toString('utf8'));
  for (const entry of oracle.cases) {
    if (entry.kind === 'mixed') continue;
    cases.push({ id: entry.id, source: entry.source ?? sources.get(entry.id), cst: entry.cst });
  }
} else {
  const manifest = JSON.parse(read('issue-195-generative/manifest.json'));
  const fixture = JSON.parse(read(`issue-195-generative/${manifest.languages[language].file}`));
  for (const entry of fixture.cases) {
    cases.push({ id: entry.id, source: entry.source, cst: entry.cst });
    if (entry.kind === 'metamorphic') cases.push({ id: `${entry.id} (variant)`, source: entry.variant, cst: entry.variantCst });
    if (entry.kind === 'edit') {
      let source = entry.source;
      entry.steps.forEach((step, position) => {
        source = applyEdit(source, step);
        cases.push({ id: `${entry.id} step ${position}`, source, cst: step.cst });
      });
    }
  }
}
const out = { clean: [], error: [] };
for (const entry of cases) {
  const network = LinkNetwork.parse(entry.source, language);
  const { problems } = documentOracleProblems(network, language, entry.source, entry.cst);
  if (!problems.length) continue;
  const oracleClean = !/ERROR|MISSING|•/u.test(entry.cst);
  out[oracleClean ? 'clean' : 'error'].push({ id: entry.id, problem: problems[0].slice(0, 400) });
}
console.log(`${cases.length} cases; ${out.clean.length} differ with a clean oracle, ${out.error.length} with an error oracle`);
for (const kind of ['clean', 'error']) for (const { id, problem } of out[kind]) console.log(`[${kind}] ${id}: ${problem}`);
