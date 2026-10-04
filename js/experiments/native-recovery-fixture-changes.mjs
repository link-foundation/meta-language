// Compares the recovered trees of parity/fixtures/native-grammars/*.json with those of a git
// revision (HEAD by default): for each changed rejection, the source and the repair sites
// (ERROR spans with the bytes they skip, MISSING leaves) before and after, to judge whether a
// change to the native executor's recovery repairs malformed input more locally:
//   node experiments/native-recovery-fixture-changes.mjs [REVISION]
import { execFileSync } from 'node:child_process';
import { readFileSync, readdirSync } from 'node:fs';

const revision = process.argv[2] ?? 'HEAD';
const directory = new URL('../../parity/fixtures/native-grammars/', import.meta.url);
const sites = (tree) => [...tree.matchAll(/(ERROR@(\d+)\.\.(\d+)|MISSING@\d+ [^)]*)/gu)].map((match) => match[1]);
const skipped = (tree) => [...tree.matchAll(/ERROR@(\d+)\.\.(\d+)/gu)].reduce((sum, match) => sum + Number(match[2]) - Number(match[1]), 0);
const totals = { better: 0, worse: 0, same: 0 };
for (const file of readdirSync(directory).filter((name) => name.endsWith('.json')).sort()) {
  const current = JSON.parse(readFileSync(new URL(file, directory), 'utf8'));
  const before = JSON.parse(execFileSync('git', ['show', `${revision}:parity/fixtures/native-grammars/${file}`], { encoding: 'utf8' }));
  const old = new Map(before.rejections.map((entry) => [entry.source, entry.recovered]));
  for (const { source, recovered } of current.rejections) {
    const previous = old.get(source);
    if (previous === undefined || previous === recovered) continue;
    const [was, now] = [skipped(previous), skipped(recovered)];
    const verdict = now < was ? 'better' : now > was ? 'worse' : 'same';
    totals[verdict] += 1;
    console.log(`${file} ${JSON.stringify(source)} [${verdict}: skipped ${was} -> ${now}]`);
    console.log(`  before: ${sites(previous).join(' ')}`);
    console.log(`  after:  ${sites(recovered).join(' ')}`);
  }
}
console.log(totals);
