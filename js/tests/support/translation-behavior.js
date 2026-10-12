import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { format } from 'node:util';
import { runInNewContext } from 'node:vm';

import { translateProgram } from '../../src/program-translation.js';
import { checkProgram } from '../../src/translation/check.js';
import { emitJavaScript } from '../../src/translation/emit-javascript.js';
import { parseJavaScript } from '../../src/translation/javascript.js';
import { nativeRustChecksEnabled } from './native-rust-checks.js';

function observations(source) {
  const lines = [];
  runInNewContext(source, { console: { log: (...values) => lines.push(format(...values)) } }, { timeout: 1000 });
  return `${lines.join('\n')}\n`;
}

/** Compare executable behavior to the source; the native stage also runs Rust. */
export function preservesTranslationBehavior(source, expected, { environment = process.env, execute = execFileSync } = {}) {
  const original = observations(source);
  assert.equal(original, expected, 'the unchanged source supplies the behavioral oracle');
  const checked = checkProgram(parseJavaScript(source));
  assert.equal(observations(emitJavaScript(checked).text), original);
  const translated = translateProgram(source, 'JavaScript', 'Rust');
  assert.equal(translated.diagnostic, null);
  if (!nativeRustChecksEnabled(environment)) return;
  const directory = mkdtempSync(join(tmpdir(), 'translation-behavior-'));
  try {
    const file = join(directory, 'main.rs');
    const executable = join(directory, process.platform === 'win32' ? 'main.exe' : 'main');
    writeFileSync(file, translated.code);
    execute('rustc', ['--edition', '2024', '--crate-type', 'bin', '-o', executable, file], { stdio: 'pipe', timeout: 60_000 });
    assert.equal(execute(executable, [], { encoding: 'utf8', timeout: 10_000 }), original);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}
