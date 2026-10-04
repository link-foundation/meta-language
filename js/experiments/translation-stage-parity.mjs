// Dumps every stage of the JavaScript pipeline for the translation corpus
// and the check cases, then runs the Rust pipeline on the same sources and
// reports each stage whose result differs.
// Usage: node experiments/translation-stage-parity.mjs [outdir] [--stage parse|check|emit|pipeline]
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdirSync, readdirSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, '..', '..');
const args = process.argv.slice(2);
const stageIndex = args.indexOf('--stage');
const stage = stageIndex >= 0 ? args.splice(stageIndex, 2)[1] : 'pipeline';
const outdir = args[0] ?? '/tmp/translation-stages';
mkdirSync(outdir, { recursive: true });

const corpus = join(root, 'parity', 'fixtures', 'translation-corpus');
const runs = [];
for (const name of readdirSync(corpus).filter((file) => /\.(lean|v|rs|mjs)$/u.test(file))) {
  const directory = join(outdir, 'corpus', name);
  execFileSync(process.execPath, [join(here, 'translation-stage-dump.mjs'), join(corpus, name), directory]);
  runs.push([join(corpus, name), directory]);
}
execFileSync(process.execPath, [join(here, 'translation-check-cases.mjs'), join(outdir, 'cases')]);
for (const name of readdirSync(join(outdir, 'cases'))) {
  const directory = join(outdir, 'cases', name);
  const source = readdirSync(directory).find((file) => file.startsWith('source.'));
  runs.push([join(directory, source), directory]);
}

execFileSync('cargo', ['build', '--quiet', '--example', 'translation_stage_probe'], { cwd: join(root, 'rust'), stdio: 'inherit' });
const probe = join(root, 'rust', 'target', 'debug', 'examples', 'translation_stage_probe');
let failed = 0;
let compared = 0;
for (const [source, directory] of runs) {
  const input = { parse: source, pipeline: source, check: join(directory, 'surface.json'), emit: join(directory, 'ir.json') }[stage];
  if (stage === 'check' && !readdirSync(directory).includes('surface.json')) continue;
  if (stage === 'emit' && !readdirSync(directory).includes('ir.json')) continue;
  compared += 1;
  const result = spawnSync(probe, [stage, input, directory], { encoding: 'utf8' });
  if (result.status !== 0) {
    failed += 1;
    console.log(`${directory}:\n${`${result.stdout}${result.stderr}`.split('\n').slice(0, 12).map((line) => `  ${line}`).join('\n')}`);
  }
}
console.log(`${stage}: ${compared - failed} of ${compared} programs agree`);
process.exitCode = failed ? 1 : 0;
