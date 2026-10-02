// Probe: obligations of every directed translation and what the target or
// source kernel reports about each theorem (`#print axioms`, `Print Assumptions`).
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { translateProgram } from '../js/src/index.js';

const root = new URL('../', import.meta.url);
const corpus = JSON.parse(readFileSync(new URL('parity/fixtures/four-language-conformance.json', root))).translationCorpus;
const dir = new URL(`${corpus.directory}/`, root);
const L = Object.keys(corpus.sources);
const tmp = mkdtempSync(path.join(tmpdir(), 'proof-probe-'));
function kernel(language, code, names) {
  if (language === 'Lean') {
    const file = path.join(tmp, 'p.lean');
    writeFileSync(file, `${code}\n${names.map((n) => `#print axioms ${n}`).join('\n')}\n`);
    try { return execFileSync('lean', [file], { encoding: 'utf8' }); } catch (e) { return `FAIL ${e.stdout}${e.stderr}`; }
  }
  const file = path.join(tmp, 'p.v');
  writeFileSync(file, `${code}\n${names.map((n) => `Print Assumptions ${n}.`).join('\n')}\n`);
  try { return execFileSync('rocq', ['compile', '-q', 'p.v'], { cwd: tmp, encoding: 'utf8' }); } catch (e) { return `FAIL ${e.stdout}${e.stderr}`; }
}
for (const source of L) {
  const text = readFileSync(new URL(corpus.sources[source].file, dir), 'utf8');
  for (const target of L) {
    if (source === target) continue;
    const t = translateProgram(text, source, target);
    console.log(`== ${source} -> ${target}`);
    for (const o of t.semantics.obligations) console.log('  ', JSON.stringify(o));
    if (corpus.targets[target].proof) {
      const out = kernel(target, t.code, t.semantics.obligations.map((o) => o.target));
      console.log(out.split('\n').filter((l) => /axiom|Closed|FAIL|depends|Axioms|error/u.test(l)).slice(0, 12).map((l) => `   | ${l}`).join('\n'));
    }
  }
}
for (const source of ['Lean', 'Rocq']) {
  const text = readFileSync(new URL(corpus.sources[source].file, dir), 'utf8');
  console.log(`== source kernel ${source}`);
  console.log(kernel(source, text, corpus.sources[source].theorems).split('\n').filter((l) => l.trim()).slice(-14).map((l) => `   | ${l}`).join('\n'));
}
rmSync(tmp, { recursive: true, force: true });
