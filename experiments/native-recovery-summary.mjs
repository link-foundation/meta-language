// Summarizes parity/fixtures/native-recovery.json, or a git revision of it, by
// language and category: how a recovery change moves the recorded cases.
//   node experiments/native-recovery-summary.mjs [REVISION]
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';

const path = 'parity/fixtures/native-recovery.json';
const [revision] = process.argv.slice(2);
const text = revision ? execFileSync('git', ['show', `${revision}:${path}`], { encoding: 'utf8' }) : readFileSync(path, 'utf8');
const summary = {};
for (const record of JSON.parse(text).cases) {
  const language = (summary[record.language] ??= { cases: 0, missing: 0, errors: 0 });
  language.cases += 1;
  language[record.category] = (language[record.category] ?? 0) + 1;
  language.missing += record.native.missing.length;
  language.errors += record.native.errors.length;
}
console.log(JSON.stringify(summary, null, 1));
