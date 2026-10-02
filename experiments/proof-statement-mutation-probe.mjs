// Probe: a false theorem statement in the Lean or Rocq source must be rejected
// by the source kernel, by the target kernel of every proof target, and by the
// bounded check of every program target.
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { translateProgram } from '../js/src/index.js';

const root = new URL('../', import.meta.url);
const corpus = JSON.parse(readFileSync(new URL('parity/fixtures/four-language-conformance.json', root))).translationCorpus;
const dir = new URL(`${corpus.directory}/`, root);
const MUTATIONS = {
  Lean: {
    fact_five: ['Arith.fact 5 = 120 :=', 'Arith.fact 5 = 121 :='],
    sumTo_formula: ['2 * Arith.sumTo n = n * (n + 1) :=', '2 * Arith.sumTo n = n * (n + 2) :='],
    mirror_mirror: ['Tree.mirror (Tree.mirror t) = t :=', 'Tree.mirror t = t :='],
    size_mirror: ['Tree.size (Tree.mirror t) = Tree.size t :=', 'Tree.size (Tree.mirror t) = Tree.size t + 1 :='],
  },
  Rocq: {
    fact_five: ['Arith.fact 5 = 120%N.', 'Arith.fact 5 = 121%N.'],
    sumTo_formula: ['2 * Arith.sumTo n = n * (n + 1).', '2 * Arith.sumTo n = n * (n + 2).'],
    mirror_mirror: ['Tree.mirror (Tree.mirror t) = t.', 'Tree.mirror t = t.'],
    size_mirror: ['Tree.size (Tree.mirror t) = Tree.size t.', 'Tree.size (Tree.mirror t) = Tree.size t + 1.'],
  },
};
const tmp = mkdtempSync(path.join(tmpdir(), 'proof-mutation-'));
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
const ONLY = process.argv[2]; const FROM = process.argv[3];
let started = !FROM;
for (const source of ONLY ? [ONLY] : ["Lean", "Rocq"]) {
  const text = readFileSync(new URL(corpus.sources[source].file, dir), 'utf8');
  for (const [theorem, [find, replace]] of Object.entries(MUTATIONS[source])) {
    if (theorem === FROM) started = true;
    if (!started) continue;
    if (text.split(find).length !== 2) { console.log(`!! ${source} ${theorem} site not unique`); continue; }
    const mutated = text.replace(find, replace);
    let t1 = Date.now();
    const own = runAs(source, mutated);
    console.log(`== ${source} ${theorem}: source kernel ok=${own.ok} ${(Date.now() - t1) / 1000}s`);
    for (const target of Object.keys(corpus.sources)) {
      if (target === source) continue;
      let t;
      try { t = translateProgram(mutated, source, target); } catch (e) { console.log(`   ${target}: THROW ${e.message}`); continue; }
      if (t.contract.support !== 'semantic-translation') { console.log(`   ${target}: support=${t.contract.support}`); continue; }
      t1 = Date.now();
      const r = runAs(target, t.code);
      const line = r.out.split('\n').filter((l) => l.includes(theorem) || /fails|error/iu.test(l)).slice(0, 2).join(' | ');
      console.log(`   ${target}: ok=${r.ok} ${(Date.now() - t1) / 1000}s ${line.slice(0, 160)}`);
    }
  }
}
rmSync(tmp, { recursive: true, force: true });
