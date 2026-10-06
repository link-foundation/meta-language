// Writes every translation case set (js/tests/fixtures/translation-cases) to
// <outdir>/<name>/source.<ext> and dumps its stages with
// translation-stage-dump.mjs, so the Rust pipeline can be compared with the
// JavaScript one case by case.
// Usage: node experiments/translation-check-cases.mjs <outdir> [set]
import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadCases } from '../scripts/translation-stage-lib.mjs';

const dump = fileURLToPath(new URL('./translation-stage-dump.mjs', import.meta.url));
const [outdir, only] = process.argv.slice(2);

const cases = (await loadCases()).filter((entry) => !only || entry.set === only);
for (const { name, extension, source } of cases) {
  const directory = join(outdir, name);
  mkdirSync(directory, { recursive: true });
  const file = join(directory, `source.${extension}`);
  writeFileSync(file, source);
  execFileSync(process.execPath, [dump, file, directory]);
}
console.log(`${cases.length} cases`);
