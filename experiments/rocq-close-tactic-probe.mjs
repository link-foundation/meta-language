// Probe: which branch of `ml_close` makes Rocq search for minutes on the false
// Lean `mirror_mirror` restatement `Tree.mirror t = t` translated into Rocq.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { translateProgram } from '../js/src/index.js';

const text = readFileSync(new URL('../parity/fixtures/translation-corpus/project.lean', import.meta.url), 'utf8');
const mutated = text.replace('Tree.mirror (Tree.mirror t) = t :=', 'Tree.mirror t = t :=');
const { code } = translateProgram(mutated, 'Lean', 'Rocq');
const close = 'Ltac ml_close := first [reflexivity | lia | nia | congruence].';
const variant = process.argv[2] ?? 'first [reflexivity | lia | nia | congruence]';
writeFileSync('/tmp/rq/p.v', code.replace(close, `Ltac ml_close := ${variant}.`));
const start = Date.now();
try {
  execFileSync('timeout', ['120', 'rocq', 'compile', '-q', 'p.v'], { cwd: '/tmp/rq', encoding: 'utf8', stdio: 'pipe' });
  console.log(`${variant}: accepted ${(Date.now() - start) / 1000}s`);
} catch (error) {
  console.log(`${variant}: status ${error.status} ${(Date.now() - start) / 1000}s ${`${error.stdout}${error.stderr}`.split('\n').filter((l) => /Error|rror/u.test(l)).slice(0, 2).join(' | ')}`);
}
