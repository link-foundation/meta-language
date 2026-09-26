// Translates a corpus project into every target, runs each artifact with its
// native toolchain and compares the printed lines with the expected output.
// Usage: node experiments/translation-native-check.mjs <project> <expected.txt> [workdir]
import { execFileSync } from 'node:child_process';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { parseLean } from '../src/translation/lean.js';
import { parseRocq } from '../src/translation/rocq.js';
import { parseRust } from '../src/translation/rust.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRocq } from '../src/translation/emit-rocq.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { emitLean } from '../src/translation/emit-lean.js';
import { emitRust } from '../src/translation/emit-rust.js';

const [source, expectedFile, workdir = '/tmp/native-check'] = process.argv.slice(2);
const frontends = { lean: parseLean, v: parseRocq, rs: parseRust };
const program = checkProgram(frontends[source.split('.').pop()](readFileSync(source, 'utf8')));
const expected = readFileSync(expectedFile, 'utf8');
mkdirSync(workdir, { recursive: true });

const run = (command, args, options = {}) => execFileSync(command, args, { cwd: workdir, encoding: 'utf8', maxBuffer: 1 << 26, ...options });
const rocqLines = (log) => {
  const start = log.indexOf('= ');
  const strings = [...log.slice(start).matchAll(/"((?:[^"]|"")*)"/gu)].map((match) => match[1].replaceAll('""', '"'));
  return strings.map((line) => `${line}\n`).join('');
};
const targets = {
  rocq: { emit: emitRocq, file: 'Out.v', run: () => rocqLines(run('rocq', ['compile', '-q', 'Out.v'])) },
  lean: { emit: emitLean, file: 'Out.lean', run: () => run('lean', ['--run', 'Out.lean']) },
  js: { emit: emitJavaScript, file: 'out.mjs', run: () => run('node', ['out.mjs']) },
  rust: {
    emit: emitRust,
    file: 'out.rs',
    run: () => {
      run('rustc', ['--edition', '2021', '-O', '-o', 'out-rs', 'out.rs']);
      return run(join(workdir, 'out-rs'), []);
    },
  },
};
let failed = false;
for (const [name, target] of Object.entries(targets)) {
  const result = target.emit(program);
  writeFileSync(join(workdir, target.file), result.text);
  try {
    const output = target.run();
    const same = output === expected;
    failed ||= !same;
    console.log(`${name}: ${same ? 'match' : 'MISMATCH'}`);
    if (!same) writeFileSync(join(workdir, `${name}.actual.txt`), output);
  } catch (error) {
    failed = true;
    console.log(`${name}: FAILED ${String(error.stderr ?? error.message).slice(0, 2000)}`);
  }
}
process.exitCode = failed ? 1 : 0;
