// Future CI execution only. Never run during the whole requirements draft.
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve, join } from 'node:path';
import { readProjectData } from './project-data.mjs';
import { translateProgram } from '../src/program-translation.js';
const target = { lean: 'Lean', rocq: 'Rocq' }[process.argv[process.argv.indexOf('--target') + 1]];
assert.ok(target, 'use --target lean|rocq');
const fixture = readProjectData(new URL('../../parity/fixtures/big-integer-bitwise.lino', import.meta.url));
const directory = resolve('issue-195-artifacts/big-integer-bitwise', target.toLowerCase());
const work = join(directory, 'work');
mkdirSync(work, { recursive: true });
const run = (name, command, arguments_, options = {}) => {
  const result = spawnSync(command, arguments_, { encoding: 'utf8', timeout: 120000, maxBuffer: 8 * 1024 * 1024, ...options });
  writeFileSync(join(directory, name + '.stdout.log'), result.stdout ?? '');
  writeFileSync(join(directory, name + '.stderr.log'), result.stderr ?? '');
  assert.equal(result.error, undefined, name + ': ' + result.error);
  assert.equal(result.signal, null, name + ': interrupted');
  assert.equal(result.status, 0, name + ': ' + result.stderr);
  return result.stdout;
};
const [nodeMajor, nodeMinor] = process.versions.node.split('.').map(Number);
assert.ok(nodeMajor === 22 && nodeMinor >= 15 || nodeMajor === 24, 'uncovered Node ' + process.versions.node);
const version = run('version', target.toLowerCase(), ['--version']);
const parts = version.match(/version\s+(\d+)\.(\d+)\.(\d+)/iu)?.slice(1).map(Number);
assert.ok(parts && (target === 'Lean' ? parts[0] === 4 && parts[1] === 34 && parts[2] >= 1 : parts[0] === 9 && parts[1] === 3),
  'uncovered ' + target + ': ' + version + '; ' + fixture.versions[target]);
if (target === 'Rocq') {
  const standard = run('standard-library-version', 'opam', ['var', 'rocq-stdlib:version']).trim();
  assert.match(standard, /^9\.2\.\d+$/u, 'uncovered Rocq standard library ' + standard);
}
writeFileSync(join(directory, 'original.mjs'), fixture.kernelSource);
const expected = run('original', process.execPath, ['--input-type=module', '-e', fixture.kernelSource]);
const translated = translateProgram(fixture.kernelSource, 'JavaScript', target);
assert.equal(translated.diagnostic, null);
writeFileSync(join(directory, target === 'Lean' ? 'Main.lean' : 'Main.v'), translated.code);
if (target === 'Lean') {
  const file = join(work, 'Main.lean');
  writeFileSync(file, translated.code);
  assert.equal(run('native', 'lean', ['--run', file]), expected);
} else {
  const lines = expected.replace(/\n$/u, '').split('\n').map(line => '"' + line.replaceAll('"', '""') + '"%string');
  const statement = '\nExample original_big_integer_output : main = (' + lines.join(' :: ') + ' :: nil).\nProof. vm_compute. reflexivity. Qed.\n';
  const file = join(work, 'Main.v');
  writeFileSync(file, translated.code + statement);
  run('native', 'rocq', ['compile', '-q', file]);
}
