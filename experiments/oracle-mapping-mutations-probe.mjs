// Probe: the problems the conformance oracle gate reports for each weakened
// oracle mapping of js/tests/issue-195-acceptance-gate-mutations.test.js.
import { readFileSync } from 'node:fs';
import { conformanceOracleProblems } from '../js/scripts/issue-195-oracle-mapping.mjs';

const rootUrl = new URL('../', import.meta.url);
const fixtureRoot = new URL('parity/fixtures/issue-195-conformance/', rootUrl);
const read = (path) => readFileSync(new URL(path, fixtureRoot));
const readRepository = (path) => readFileSync(new URL(path, rootUrl));
const conformance = JSON.parse(read('manifest.json'));
const lock = JSON.parse(readRepository('js/src/vendor/grammars/grammar-lock.json'));
const mutations = {
  unmutated: () => {},
  'another language oracle': ({ languages }) => Object.assign(languages.JavaScript, { oracle: languages.Rust.oracle, oracleSha256: languages.Rust.oracleSha256 }),
  'another grammar': ({ languages }) => Object.assign(languages.JavaScript.grammar, { id: 'typescript', parserSha256: lock.grammars.typescript.parserSha256 }),
  'no real projects': ({ languages }) => { languages.Lean.projects = []; },
  'another oracle tool': ({ oracle }) => { oracle.tool = 'tree-sitter 0.20.0'; },
  'an unpinned oracle': ({ languages }) => { languages.Rocq.oracleSha256 = '0'.repeat(64); },
};
for (const [name, change] of Object.entries(mutations)) {
  const candidate = structuredClone(conformance);
  change(candidate);
  console.log(name, JSON.stringify(conformanceOracleProblems(candidate, { lock, read, readRepository })));
}
