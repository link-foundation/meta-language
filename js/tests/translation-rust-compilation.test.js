import assert from 'node:assert/strict';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { test } from 'node:test';
import { fileURLToPath } from 'node:url';

import { compileRustTranslation } from './support/self-translation-rust.js';

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
  if (!process.env.SELF_TRANSLATION_CHECK_RUST && !process.env.ISSUE_195_OBSERVATION_FILE) return;
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
