// CI-only independent target-tool evidence; no Rust compilation is performed.
import { writeFile, mkdir } from 'node:fs/promises';
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import peggy from 'peggy';
import { checkIndependentGrammarCycle } from './grammar-interchange-tools.mjs';
import { readProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const fixture = readProjectData(new URL('parity/fixtures/grammar-interchange-tools.json', root));
const index = process.argv.indexOf('--out');
if (index < 0 || !process.argv[index + 1]) throw new Error('provide --out REPORT.json');
const output = path.resolve(process.argv[index + 1]);
const results = fixture.cases.map((entry) => {
  try { return checkIndependentGrammarCycle(entry, peggy); }
  catch (error) { return { fixture: entry.id, failures: [String(error?.stack ?? error)] }; }
});
const report = { schemaVersion: 1, commit: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
  targetTools: [{ name: 'peggy', version: peggy.VERSION }], results };
await mkdir(path.dirname(output), { recursive: true });
await writeFile(output, `${JSON.stringify(report, null, 2)}\n`);
const failures = results.flatMap((entry) => entry.failures.map((failure) => `${entry.fixture}: ${failure}`));
for (const failure of failures) console.error(failure);
console.log(`independent grammar cycles: ${results.length} fixtures, ${failures.length} failures; ${output}`);
if (failures.length) process.exitCode = 1;
