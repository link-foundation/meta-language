// Probe: a theorem statement that holds on the bounded domain of
// `--ml-check-theorems` (n in 0..6) but is false in general (n = 7) passes the
// program targets' bounded check, while the source kernel and every proof
// target kernel reject it, so a bounded check is never a general proof.
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { translateProgram } from '../js/src/index.js';

const root = new URL('../', import.meta.url);
const corpus = JSON.parse(readFileSync(new URL('parity/fixtures/four-language-conformance.json', root))).translationCorpus;
const dir = new URL(`${corpus.directory}/`, root);
const MUTATIONS = {
  Lean: ['2 * Arith.sumTo n = n * (n + 1) :=', 'Arith.sumTo n - 21 = 0 :='],
  Rocq: ['2 * Arith.sumTo n = n * (n + 1).', 'Arith.sumTo n - 21 = 0.'],
};
const tmp = mkdtempSync(path.join(tmpdir(), 'proof-bounded-'));
function exec(cmd, args, cwd) {
  try { return { ok: true, out: execFileSync(cmd, args, { cwd, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] }) }; } catch (e) { return { ok: false, out: `${e.stdout}${e.stderr}` }; }
}
function runAs(language, code) {
  if (language === 'Lean') { writeFileSync(path.join(tmp, 'p.lean'), code); const r = exec('lean', [path.join(tmp, 'p.lean')]); return { ok: r.ok && !/warning|error|sorry/iu.test(r.out), out: r.out }; }
  if (language === 'Rocq') { writeFileSync(path.join(tmp, 'p.v'), code); const r = exec('rocq', ['compile', '-q', 'p.v'], tmp); return { ok: r.ok && !/warning|error/iu.test(r.out), out: r.out }; }
  if (language === 'JavaScript') { writeFileSync(path.join(tmp, 'p.mjs'), code); return exec('node', [path.join(tmp, 'p.mjs'), '--ml-check-theorems']); }
  writeFileSync(path.join(tmp, 'p.rs'), code); const c = exec('rustc', ['--edition', '2024', '-O', '-o', path.join(tmp, 'p'), path.join(tmp, 'p.rs')]);
  if (!c.ok) return { ok: false, out: `COMPILE ${c.out}` };
  return exec(path.join(tmp, 'p'), ['--ml-check-theorems']);
}
for (const [source, [find, replace]] of Object.entries(MUTATIONS)) {
  const text = readFileSync(new URL(corpus.sources[source].file, dir), 'utf8');
  const mutated = text.replace(find, replace);
  const own = runAs(source, mutated);
  console.log(`== ${source}: source kernel ok=${own.ok} ${own.out.split('\n').find((l) => /error/u.test(l))?.slice(0, 140) ?? ''}`);
  for (const target of Object.keys(corpus.sources)) {
    if (target === source) continue;
    const t = translateProgram(mutated, source, target);
    if (t.contract.support !== 'semantic-translation') { console.log(`   ${target}: support=${t.contract.support}`); continue; }
    console.log(`   ${target} obligation: ${JSON.stringify(t.semantics.obligations.find((o) => o.source === 'sumTo_formula'))}`);
    const r = runAs(target, t.code);
    const line = r.out.split('\n').filter((l) => l.includes('sumTo') || /fails|error/iu.test(l)).slice(0, 2).join(' | ');
    console.log(`   ${target}: ok=${r.ok} ${line.slice(0, 160)}`);
  }
}
rmSync(tmp, { recursive: true, force: true });
