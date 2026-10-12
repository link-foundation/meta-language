// Runs the actual independent Lark tool in CI. This is authoring/oracle work;
// ordinary installed parsing never launches Python or imports a grammar engine.
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir } from 'node:fs/promises';
import { execFileSync, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { grammarEmitter, grammarImporter } from '../src/grammar-interchange.js';
import { parseWithGrammar } from '../src/grammar.js';

const expectedVersion = '1.3.1';
const root = new URL('../../', import.meta.url);
const fixtureBytes = await readFile(new URL('parity/fixtures/grammar-interchange-tools.json', root));
const fixture = JSON.parse(fixtureBytes);
const index = process.argv.indexOf('--out');
if (index < 0 || !process.argv[index + 1]) throw new Error('provide --out REPORT.json');
const output = path.resolve(process.argv[index + 1]);
const digest = (source) => createHash('sha256').update(source).digest('hex');

function nativeTree(tree, source) {
  if (tree.type !== 'node') return null;
  return { kind: tree.kind, start: tree.start, end: tree.end,
    text: Buffer.from(source).subarray(tree.start, tree.end).toString(),
    children: tree.children.map((child) => nativeTree(child, source)).filter(Boolean) };
}

function nativeOutcome(grammar, source) {
  try { return { accepted: true, tree: nativeTree(parseWithGrammar(grammar, source), source) }; }
  catch (error) { return { accepted: false, offset: error.offset, reason: error.reason }; }
}

const results = [];
for (const entry of fixture.cases) {
  const failures = [];
  try {
    const original = grammarImporter(entry.format)(entry.source);
    const emitted = grammarEmitter('lark')(original);
    const replayGrammar = grammarImporter('lark')(emitted.source);
    const start = replayGrammar.startRule().name;
    const literal = /"(?:[^"\\]|\\.)+"/u.exec(emitted.source);
    if (!literal) throw new Error('the target has no literal for the mutation witness');
    const changed = emitted.source.slice(0, literal.index) + '"independent-target-mutation"' + emitted.source.slice(literal.index + literal[0].length);
    const changedGrammar = grammarImporter('lark')(changed);
    const samples = [...entry.accepts.map((sample) => ({ ...sample, accepted: true })),
      ...entry.rejects.map((sample) => ({ ...sample, accepted: false }))];
    const command = spawnSync('python3', [fileURLToPath(new URL('./grammar-interchange-lark.py', import.meta.url))], {
      input: JSON.stringify({ source: emitted.source, start, changed, witness: entry.accepts[0].source,
        samples: samples.map(({ source }) => source) }), encoding: 'utf8', timeout: 60_000, maxBuffer: 2 ** 20,
    });
    if (command.error || command.status !== 0) throw new Error(`the target tool did not execute: ${command.error ?? command.stderr}`);
    const target = JSON.parse(command.stdout);
    if (target.version !== expectedVersion) failures.push(`independent tool version ${target.version} differs from ${expectedVersion}`);
    if (target.error) throw new Error(target.error);
    if (target.observations.length !== samples.length) failures.push('the independent target omitted a sample');
    const observations = samples.map((sample, index) => {
      const native = nativeOutcome(original, sample.source);
      const replay = nativeOutcome(replayGrammar, sample.source);
      const independent = target.observations[index];
      for (const [name, observed] of Object.entries({ native, replay, target: independent })) {
        if (observed?.accepted !== sample.accepted) failures.push(`${sample.source}: ${name} acceptance differs`);
        if (!sample.accepted && observed?.offset !== sample.offset) failures.push(`${sample.source}: ${name} diagnostic offset differs`);
        if (sample.tree && JSON.stringify(observed?.tree) !== JSON.stringify(sample.tree)) failures.push(`${sample.source}: ${name} parse structure differs`);
      }
      if (sample.accepted && (JSON.stringify(native.tree) !== JSON.stringify(independent?.tree) || JSON.stringify(native.tree) !== JSON.stringify(replay.tree))) failures.push(`${sample.source}: the native/target/replay trees differ`);
      return { source: sample.source, native, replay, target: independent };
    });
    const mutation = { targetRejected: !target.mutation.accepted,
      nativeReplayRejected: !nativeOutcome(changedGrammar, entry.accepts[0].source).accepted };
    if (!mutation.targetRejected || !mutation.nativeReplayRejected) failures.push('target mutation did not change both executions');
    results.push({ fixture: entry.id, sourceFormat: entry.format, targetTool: 'lark', targetVersion: target.version,
      sourceSha256: digest(entry.source), targetSha256: digest(emitted.source), loweringNotes: emitted.report.lossy,
      observations, mutation, failures });
  } catch (error) {
    results.push({ fixture: entry.id, failures: [...failures, String(error?.stack ?? error)] });
  }
}
await mkdir(path.dirname(output), { recursive: true });
await writeFile(output, `${JSON.stringify({ schemaVersion: 1,
  commit: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
  fixtureSha256: digest(fixtureBytes), targetTools: [{ name: 'lark', version: expectedVersion }], results }, null, 2)}\n`);
const failures = results.flatMap((entry) => entry.failures.map((failure) => `${entry.fixture}: ${failure}`));
for (const failure of failures) console.error(failure);
console.log(`independent Lark cycles: ${results.length} fixtures, ${failures.length} failures; ${output}`);
if (failures.length) process.exitCode = 1;
