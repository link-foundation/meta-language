import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { compileRustTranslation } from './support/self-translation-rust.js';
import { nativeRustChecksEnabled } from './support/native-rust-checks.js';
import { preservesTranslationBehavior } from './support/translation-behavior.js';

test('native Rust compilation needs explicit opt-in and zero remains disabled', () => {
  for (const flag of ['TRANSLATION_CHECK_RUST', 'SELF_TRANSLATION_CHECK_RUST']) {
    for (const value of [undefined, '', '0', 'false', 'true', 'yes']) assert.equal(nativeRustChecksEnabled({ [flag]: value }, flag), false, `${flag}=${value}`);
    assert.equal(nativeRustChecksEnabled({ [flag]: '1' }, flag), true);
    assert.equal(nativeRustChecksEnabled({ ISSUE_195_OBSERVATION_FILE: 'native-stage.jsonl' }, flag), true);
  }
  const source = 'console.log(3);';
  for (const environment of [{}, { TRANSLATION_CHECK_RUST: '0' }]) {
    preservesTranslationBehavior(source, '3\n', { environment, execute: () => assert.fail('a JavaScript-only check invoked a native tool') });
  }
  const commands = [];
  preservesTranslationBehavior(source, '3\n', { environment: { TRANSLATION_CHECK_RUST: '1' }, execute: (command, args) => {
    commands.push(command);
    if (command === 'rustc') {
      assert.match(readFileSync(args.at(-1), 'utf8'), /js_console\(3f64\)/u);
      return Buffer.alloc(0);
    }
    return '3\n';
  } });
  assert.equal(commands.length, 2);
  assert.equal(commands[0], 'rustc');
});

test('original Rust uses rustc and every generated Rust case denies Clippy warnings', () => {
  for (const language of ['Rust', 'JavaScript', 'TypeScript']) {
    const commands = [];
    compileRustTranslation(language, 'geometry.rs', 'geometry.rmeta', (...args) => commands.push(args));
    assert.equal(commands.length, 1);
    const [compiler, args] = commands[0];
    assert.equal(compiler, language === 'Rust' ? 'rustc' : 'clippy-driver');
    assert.deepEqual(args.slice(0, 6), ['--edition', '2024', '--crate-type', 'lib', '--emit', 'metadata']);
    assert.deepEqual(args.slice(-3), ['-o', 'geometry.rmeta', 'geometry.rs']);
    if (language === 'Rust') assert.ok(!args.includes('-D'));
    else assert.deepEqual(args.slice(6, 8), ['-D', 'warnings']);
  }
});

test('the unchanged geometry fixture compiles while the generated-code check rejects its warnings', () => {
  if (!nativeRustChecksEnabled(process.env, 'SELF_TRANSLATION_CHECK_RUST')) return;
  const source = fileURLToPath(new URL('../../parity/self-translation/sources/geometry.rs', import.meta.url));
  const original = readFileSync(source);
  const directory = mkdtempSync(path.join(tmpdir(), 'self-translation-compiler-'));
  try {
    const metadata = path.join(directory, 'geometry.rmeta');
    compileRustTranslation('Rust', source, metadata);
    assert.ok(readFileSync(metadata).length > 0);
    assert.throws(() => compileRustTranslation('JavaScript', source, path.join(directory, 'generated.rmeta')),
      (error) => /dead_code|manual_is_multiple_of/u.test(String(error.stderr)));
    assert.deepEqual(readFileSync(source), original);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
