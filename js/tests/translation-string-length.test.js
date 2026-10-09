import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { test } from 'node:test';
import { checkProgram } from '../src/translation/check.js';
import { emitJavaScript } from '../src/translation/emit-javascript.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { translateProgram } from '../src/program-translation.js';

const source = readFileSync(new URL('../../parity/fixtures/translation-corpus/string-length.mjs', import.meta.url), 'utf8');
const expected = readFileSync(new URL('../../parity/fixtures/translation-corpus/string-length.expected.txt', import.meta.url), 'utf8');

test('string lengths retain UTF-16 units and evaluate the receiver once', () => {
  for (const script of [source, emitJavaScript(checkProgram(parseJavaScript(source))).text]) {
    assert.equal(execFileSync(process.execPath, ['--input-type=module', '-e', script], { encoding: 'utf8' }), expected);
  }
  const directory = mkdtempSync(join(tmpdir(), 'string-length-'));
  try {
    const translated = translateProgram(source, 'JavaScript', 'Rust');
    assert.equal(translated.diagnostic, null);
    const file = join(directory, 'main.rs');
    const binary = join(directory, 'main');
    writeFileSync(file, translated.code);
    execFileSync('rustc', ['--edition', '2024', file, '-o', binary], { stdio: 'pipe' });
    assert.equal(execFileSync(binary, [], { encoding: 'utf8' }), expected);
    for (const target of ['Lean', 'Rocq']) {
      assert.equal(translateProgram(source, 'JavaScript', target).diagnostic, null, target);
    }
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test('inferred string returns and length arithmetic retain Number semantics', () => {
  const translated = translateProgram("function text() { return '😀'; } console.log(text().length / 2);", 'JavaScript', 'Rust');
  assert.equal(translated.diagnostic, null);
  assert.match(translated.code, /encode_utf16\(\)\.count\(\) as f64/u);
});
