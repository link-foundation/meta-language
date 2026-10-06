// Times the native parse of every generative fixture case and run-time fuzz case of one
// language (default Lean), slowest first, to find which inputs make the generative suite
// slow. Usage: node experiments/generative-case-timing.mjs [Language] [runtimeCases] [out.json]
// With out.json, it writes every timed source there, for
// rust/experiments/generative_case_timing.rs to time in Rust.
import { readFileSync, writeFileSync } from 'node:fs';

import { LinkNetwork } from '../src/index.js';
import { RELATIONS, applyEdit, createRandom, randomEdit, seedSources } from '../tests/support/generative.js';

const language = process.argv[2] ?? 'Lean';
const runtimeCases = Number(process.argv[3] ?? 48);
const out = process.argv[4];
const root = new URL('../../parity/fixtures/issue-195-generative/', import.meta.url);
const manifest = JSON.parse(readFileSync(new URL('manifest.json', root)));
const fixture = JSON.parse(readFileSync(new URL(manifest.languages[language].file.split('/').pop(), root)));

const timings = [];
const time = (label, source) => {
  const started = performance.now();
  LinkNetwork.parse(source, language);
  timings.push({ label, ms: performance.now() - started, length: source.length, source });
};
for (const entry of fixture.cases) {
  time(`${entry.id} ${entry.kind}`, entry.source);
  if (entry.variant) time(`${entry.id} variant`, entry.variant);
  for (const [index, step] of (entry.steps ?? []).entries()) if (step.source) time(`${entry.id} step ${index}`, step.source);
}
const random = createRandom(`${manifest.seed}:${language}:runtime`);
const seeds = seedSources(language);
for (let index = 0; index < runtimeCases; index += 1) {
  let source = random.pick(seeds).source;
  for (let step = 0, total = 1 + random.int(6); step < total; step += 1) source = applyEdit(source, randomEdit(random, source, language));
  time(`runtime ${index}`, source);
  time(`runtime ${index} variant`, RELATIONS['prepend-blank-lines'].transform(source));
}
if (out) writeFileSync(out, JSON.stringify({ language, cases: timings.map(({ label, source }) => ({ label, source })) }));
timings.sort((a, b) => b.ms - a.ms);
const total = timings.reduce((sum, entry) => sum + entry.ms, 0);
console.log(`${language}: ${timings.length} parses, ${Math.round(total)} ms`);
for (const entry of timings.slice(0, 15)) {
  console.log(`${Math.round(entry.ms)} ms  ${entry.label}  (${entry.length} chars)  ${JSON.stringify(entry.source).slice(0, 120)}`);
}
