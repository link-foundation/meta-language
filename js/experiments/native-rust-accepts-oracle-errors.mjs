// Lists the Rust generative and conformance inputs the native default grammar
// parses clean although the tree-sitter CLI oracle reports an error, printing
// the oracle's error ranges and the source text they cover.
import { readFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { applyEdit } from '../tests/support/generative.js';

const root = new URL('../../parity/fixtures/', import.meta.url);
const read = (path) => JSON.parse(readFileSync(new URL(path, root)));
const manifest = read('issue-195-generative/manifest.json');
const fixture = read(`issue-195-generative/${manifest.languages.Rust.file}`);
const cases = [];
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
for (const entry of cases) {
  if (!/ERROR|MISSING/u.test(entry.cst)) continue;
  const network = LinkNetwork.parse(entry.source, 'Rust');
  if (!network.verifyFullMatch().isClean()) continue;
  const lines = entry.source.split('\n');
  const errors = entry.cst.split('\n').filter((line) => /ERROR|MISSING/u.test(line)).slice(0, 3)
    .map((line) => {
      const match = /(\d+):(\d+)-(\d+):(\d+)/u.exec(line);
      return `${line.trim()}  ${match ? JSON.stringify(lines[Number(match[1])]) : ''}`;
    });
  console.log(`${entry.id}\n  ${errors.join('\n  ')}`);
}
