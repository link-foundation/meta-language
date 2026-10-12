import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { gzipSync } from 'node:zlib';
import test from 'node:test';
import { cleanMaterializedExternalGrammarAssets, downloadExternalGrammarAssets, externalGrammarDirectory, externalGrammarPath, readExternalGrammarAsset } from '../scripts/external-grammar-cache.mjs';

const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const receipt = (bytes, group = 'sources') => ({ group, bytes: bytes.length, sha256: digest(bytes), encoding: 'identity', downloads: [{ url: 'https://assets.example.test/pinned', transform: 'identity' }] });
function workspace(context, assets) {
  const root = mkdtempSync(join(tmpdir(), 'meta-language-assets-'));
  context.after(() => rmSync(root, { recursive: true, force: true }));
  return { root, cacheRoot: join(root, '.external-grammars'), manifest: { schemaVersion: 1, assets } };
}

test('selected downloads verify exact bytes and reuse the authenticated cache offline', async context => {
  const bytes = Buffer.from('a tiny external grammar\n');
  const options = workspace(context, { 'upstream/grammar.txt': receipt(bytes), 'oracle/parser.wasm': receipt(Buffer.from([0, 97, 115, 109]), 'wasm') });
  let requests = 0;
  const results = await downloadExternalGrammarAssets({ ...options, groups: ['sources'], fetchImpl: async () => { requests++; return new Response(bytes); } });
  assert.deepEqual(results, [{ path: 'upstream/grammar.txt', reused: false }]);
  assert.equal(requests, 1);
  assert.equal(existsSync(join(options.cacheRoot, 'oracle/parser.wasm')), false);
  assert.deepEqual(readExternalGrammarAsset('upstream/grammar.txt', options), bytes);
  await downloadExternalGrammarAssets({ ...options, assets: ['upstream/grammar.txt'], materialize: true, fetchImpl: () => { throw new Error('unexpected network'); } });
  assert.deepEqual(readFileSync(join(options.root, 'upstream/grammar.txt')), bytes);
  assert.equal(externalGrammarPath('upstream/grammar.txt', options), join(options.root, 'upstream/grammar.txt'));
});

test('failed receipts never write a cache or materialized file', async context => {
  const options = workspace(context, { 'source/grammar.txt': receipt(Buffer.from('correct')) });
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true, fetchImpl: async () => new Response('wrong') }), /checksum mismatch/);
  assert.equal(existsSync(join(options.cacheRoot, 'source/grammar.txt')), false);
  assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
});

test('a valid cache hit refuses to overwrite a modified materialized download', async context => {
  const bytes = Buffer.from('pinned source');
  const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
  await downloadExternalGrammarAssets({ ...options, all: true, materialize: true, fetchImpl: async () => new Response(bytes) });
  const target = join(options.root, 'source/grammar.txt');
  writeFileSync(target, 'user changed source');
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true, fetchImpl: () => { throw new Error('unexpected network'); } }), /checksum mismatch/);
  assert.equal(readFileSync(target, 'utf8'), 'user changed source');
  assert.deepEqual(readFileSync(join(options.cacheRoot, 'source/grammar.txt')), bytes);
});

test('redirects to HTTP cannot populate the authenticated cache', async context => {
  const bytes = Buffer.from('source');
  const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
  const response = new Response(bytes);
  Object.defineProperty(response, 'url', { value: 'http://assets.example.test/redirected' });
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, fetchImpl: async () => response }), /redirect requires HTTPS/);
  assert.equal(existsSync(join(options.cacheRoot, 'source/grammar.txt')), false);
});

test('gzip downloads verify compressed and decoded pins and reject counterfeit content', async context => {
  const raw = Buffer.from('grammar rule\n');
  const bytes = gzipSync(raw, { level: 9 });
  const asset = { ...receipt(bytes), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: digest(raw), downloads: [{ url: 'https://assets.example.test/upstream', transform: 'gzip' }] };
  const options = workspace(context, { 'source/grammar.gz': asset });
  await downloadExternalGrammarAssets({ ...options, all: true, fetchImpl: async () => new Response(raw) });
  assert.deepEqual(readExternalGrammarAsset('source/grammar.gz', options), bytes);
  const counterfeit = workspace(context, { 'source/grammar.gz': { ...asset, decodedSha256: '0'.repeat(64) } });
  await assert.rejects(downloadExternalGrammarAssets({ ...counterfeit, all: true, fetchImpl: async () => new Response(raw) }), /decoded external grammar asset checksum mismatch/);
  assert.equal(existsSync(join(counterfeit.cacheRoot, 'source/grammar.gz')), false);
});

test('asset traversal, unregistered paths and symbolic parent escapes are refused', async context => {
  const bytes = Buffer.from('x');
  const options = workspace(context, { '../escape': receipt(bytes), 'source/grammar.txt': receipt(bytes) });
  assert.throws(() => externalGrammarPath('../escape', options), /unsafe.*path/);
  assert.throws(() => externalGrammarPath('unknown', options), /unregistered/);
  const outside = mkdtempSync(join(tmpdir(), 'meta-language-outside-'));
  context.after(() => rmSync(outside, { recursive: true, force: true }));
  symlinkSync(outside, join(options.root, 'source'), 'junction');
  await assert.rejects(downloadExternalGrammarAssets({ ...options, assets: ['source/grammar.txt'], materialize: true, fetchImpl: async () => new Response(bytes) }), /unsafe.*parent/);
  assert.equal(existsSync(join(outside, 'grammar.txt')), false);
});

test('a metadata-only directory cannot replace a fully verified asset directory', async context => {
  const bytes = Buffer.from('source');
  const options = workspace(context, { 'vendor/src/parser.c': receipt(bytes), 'vendor/src/header.h': receipt(bytes) });
  mkdirSync(join(options.root, 'vendor/src'), { recursive: true });
  writeFileSync(join(options.root, 'vendor/NOTICE.md'), 'authored metadata');
  assert.throws(() => externalGrammarDirectory('vendor', options), /not downloaded/);
  await downloadExternalGrammarAssets({ ...options, all: true, fetchImpl: async () => new Response(bytes) });
  assert.equal(externalGrammarDirectory('vendor', options), join(options.cacheRoot, 'vendor'));
});

test('materialized cleanup preserves metadata and refuses changed downloads', async context => {
  const bytes = Buffer.from('source');
  const options = workspace(context, { 'vendor/source.txt': receipt(bytes) });
  execFileSync('git', ['init', '--quiet'], { cwd: options.root });
  writeFileSync(join(options.root, '.gitignore'), '/vendor/\n/.external-grammars/\n');
  await downloadExternalGrammarAssets({ ...options, all: true, materialize: true, fetchImpl: async () => new Response(bytes) });
  writeFileSync(join(options.root, 'vendor/NOTICE.md'), 'metadata');
  writeFileSync(join(options.root, 'vendor/source.txt'), 'edited');
  assert.throws(() => cleanMaterializedExternalGrammarAssets(options), /checksum mismatch/);
  writeFileSync(join(options.root, 'vendor/source.txt'), bytes);
  assert.deepEqual(cleanMaterializedExternalGrammarAssets(options), ['vendor/source.txt']);
  assert.equal(readFileSync(join(options.root, 'vendor/NOTICE.md'), 'utf8'), 'metadata');
  assert.deepEqual(readExternalGrammarAsset('vendor/source.txt', options), bytes);
});

test('custom caches inside a worktree require an actual ignore rule before acquisition', async context => {
  const bytes = Buffer.from('pinned source');
  const options = workspace(context, { 'vendor/source.txt': receipt(bytes) });
  execFileSync('git', ['init', '--quiet'], { cwd: options.root });
  let requests = 0;
  const custom = { ...options, cacheRoot: join(options.root, 'custom-cache'), all: true,
    fetchImpl: async () => { requests++; return new Response(bytes); } };
  await assert.rejects(downloadExternalGrammarAssets(custom), /cache inside the worktree must be explicitly ignored/u);
  assert.equal(requests, 0);
  assert.equal(existsSync(custom.cacheRoot), false);
  writeFileSync(join(options.root, '.gitignore'), '/custom-cache/\n');
  await downloadExternalGrammarAssets(custom);
  assert.equal(requests, 1);
  assert.deepEqual(readExternalGrammarAsset('vendor/source.txt', custom), bytes);
});

test('a custom cache rule must cover actual selected files as well as a directory probe', async context => {
  const bytes = Buffer.from('pinned source');
  const options = workspace(context, { 'vendor/source.txt': receipt(bytes) });
  execFileSync('git', ['init', '--quiet'], { cwd: options.root });
  writeFileSync(join(options.root, '.gitignore'), '/custom-cache/*\n!/custom-cache/vendor/\n');
  const custom = { ...options, cacheRoot: join(options.root, 'custom-cache'), all: true,
    fetchImpl: () => { throw new Error('unignored selected input must be refused before fetching'); } };
  await assert.rejects(downloadExternalGrammarAssets(custom), /explicitly ignored by Git: custom-cache\/vendor\/source.txt/u);
  assert.equal(existsSync(custom.cacheRoot), false);
});

test('an ignore rule cannot authorize overwriting a tracked cache input', async context => {
  const bytes = Buffer.from('pinned source');
  const options = workspace(context, { 'vendor/source.txt': receipt(bytes) });
  execFileSync('git', ['init', '--quiet'], { cwd: options.root });
  writeFileSync(join(options.root, '.gitignore'), '/.external-grammars/\n');
  mkdirSync(join(options.cacheRoot, 'vendor'), { recursive: true });
  const target = join(options.cacheRoot, 'vendor/source.txt');
  writeFileSync(target, 'tracked human source');
  execFileSync('git', ['add', '--force', '--', '.external-grammars/vendor/source.txt'], { cwd: options.root });
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true,
    fetchImpl: () => { throw new Error('no request before the tracked-source guard'); } }), /cache input must be untracked/u);
  assert.equal(readFileSync(target, 'utf8'), 'tracked human source');
});

test('outside-worktree temporary caches retain checked acquisition without editing repository ignore rules', async context => {
  const bytes = Buffer.from('pinned source');
  const options = workspace(context, { 'vendor/source.txt': receipt(bytes) });
  execFileSync('git', ['init', '--quiet'], { cwd: options.root });
  const outside = mkdtempSync(join(tmpdir(), 'meta-external-cache-outside-'));
  context.after(() => rmSync(outside, { recursive: true, force: true }));
  const configured = { ...options, cacheRoot: outside };
  await downloadExternalGrammarAssets({ ...configured, all: true, fetchImpl: async () => new Response(bytes) });
  assert.deepEqual(readExternalGrammarAsset('vendor/source.txt', configured), bytes);
  assert.equal(existsSync(join(options.root, '.gitignore')), false);
});
