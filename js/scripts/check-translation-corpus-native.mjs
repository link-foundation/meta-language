#!/usr/bin/env node
// Execute a focused translated corpus case with its native toolchain.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { translateProgram } from '../src/program-translation.js';

const option = (name) => process.argv[process.argv.indexOf(name) + 1];
const target = { lean: 'Lean', rocq: 'Rocq' }[option('--target')];
const fixture = option('--fixture');
if (!target || !/^[a-z][a-z0-9-]*$/u.test(fixture ?? '')) throw new Error('pass --target lean|rocq and --fixture NAME');
const corpus = new URL('../../parity/fixtures/translation-corpus/', import.meta.url);
const source = readFileSync(new URL(`${fixture}.mjs`, corpus), 'utf8');
const expected = readFileSync(new URL(`${fixture}.expected.txt`, corpus), 'utf8');
assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', source], { encoding: 'utf8' }), expected);
const translated = translateProgram(source, 'JavaScript', target);
assert.equal(translated.diagnostic, null);
const directory = mkdtempSync(join(tmpdir(), 'translation-native-'));
try {
  if (target === 'Lean') {
    const file = join(directory, 'Main.lean');
    writeFileSync(file, translated.code);
    assert.equal(execFileSync('lean', ['--run', file], { encoding: 'utf8', maxBuffer: 16 * 1024 * 1024 }), expected);
  } else {
    const file = join(directory, 'Main.v');
    const lines = expected.replace(/\n$/u, '').split('\n').map((line) => `"${line.replaceAll('"', '""')}"%string`);
    const assertion = `\nExample corpus_output : main = (${lines.join(' :: ')} :: nil).\nProof. vm_compute. reflexivity. Qed.\n`;
    writeFileSync(file, translated.code + assertion);
    execFileSync('rocq', ['compile', '-q', file], { stdio: 'pipe', maxBuffer: 16 * 1024 * 1024 });
  }
  console.log(`${fixture}: ${target} execution matches JavaScript`);
} finally { rmSync(directory, { recursive: true, force: true }); }
