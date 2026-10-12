import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { gzipSync } from 'node:zlib';
import test from 'node:test';
import { readCompressedGrammarAsset } from '../src/grammar-asset-path.js';

const logical = 'js/src/vendor/grammars/own-fixture.wasm.gz';
const bytes = Buffer.from('an owned portable loader fixture');
const digest = createHash('sha256').update(bytes).digest('hex');

function directory(run) {
  const root = mkdtempSync(join(tmpdir(), 'grammar-asset-loader-'));
  const previous = process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE;
  process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE = join(root, 'cache');
  try { run(root); }
  finally {
    if (previous === undefined) delete process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE;
    else process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE = previous;
    rmSync(root, { recursive: true, force: true });
  }
}

const save = (path, contents) => {
  mkdirSync(dirname(path), { recursive: true });
  writeFileSync(path, gzipSync(contents));
};

test('offline runtime packages prefer their authenticated staged artifact', () => directory(root => {
  const staged = join(root, 'package', 'own-fixture.wasm.gz');
  save(staged, bytes);
  assert.deepEqual(readCompressedGrammarAsset(logical, pathToFileURL(staged), digest), bytes);
}));

test('a source checkout uses the pinned temporary cache when no artifact is staged', () => directory(root => {
  const cached = join(root, 'cache', logical);
  const staged = join(root, 'absent.wasm.gz');
  save(cached, bytes);
  assert.deepEqual(readCompressedGrammarAsset(logical, pathToFileURL(staged), digest), bytes);
  save(cached, Buffer.from('different bytes'));
  assert.throws(() => readCompressedGrammarAsset(logical, pathToFileURL(staged), digest), /digest differs/u);
}));

test('a changed staged artifact fails authentication without using another cache copy', () => directory(root => {
  const staged = join(root, 'package', 'own-fixture.wasm.gz');
  save(join(root, 'cache', logical), bytes);
  save(staged, Buffer.from('changed staged bytes'));
  assert.throws(() => readCompressedGrammarAsset(logical, pathToFileURL(staged), digest), /digest differs/u);
}));

test('missing temporary assets report the exact provisioning command', () => directory(root => {
  assert.throws(() => readCompressedGrammarAsset(logical, pathToFileURL(join(root, 'absent.wasm.gz')), digest),
    /external-grammar-cache\.mjs --asset js\/src\/vendor\/grammars\/own-fixture\.wasm\.gz/u);
  process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE = 'relative';
  assert.throws(() => readCompressedGrammarAsset(logical, pathToFileURL(join(root, 'absent.wasm.gz')), digest), /must be absolute/u);
}));
