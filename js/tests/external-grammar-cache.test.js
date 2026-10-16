import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { gunzipSync, gzipSync } from 'node:zlib';
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
  {
    const raw = Buffer.from('pinned external grammar source\n'.repeat(32));
    const recompressed = gzipSync(raw, { level: 9 });
    const otherHeader = Buffer.from(recompressed);
    otherHeader[4] = 1;
    for (const bytes of [otherHeader, gzipSync(raw, { level: 1 })]) {
      assert.notDeepEqual(bytes, recompressed, 'the exact archive is genuinely a different gzip representation');
      assert.deepEqual(gunzipSync(bytes), raw, 'the alternate representation decodes to the same independently pinned source');
      const asset = { ...receipt(bytes), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: digest(raw), downloads: [
        { url: 'https://upstream.example.test/source', transform: 'gzip' },
        { url: 'https://archive.example.test/exact', transform: 'identity' },
        { url: 'https://mirror.example.test/exact', transform: 'identity' },
      ] };
      const order = [...asset.downloads];
      const options = workspace(context, { 'source/grammar.gz': asset });
      const requests = [], delays = [];
      await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
        fetchImpl: async url => {
          requests.push(url);
          assert.equal(url, 'https://archive.example.test/exact');
          assert.equal(existsSync(options.cacheRoot), false);
          assert.equal(existsSync(join(options.root, 'source/grammar.gz')), false);
          return new Response(bytes);
        }, sleepImpl: async delay => { delays.push(delay); } });
      assert.deepEqual(requests, ['https://archive.example.test/exact']);
      assert.deepEqual(delays, []);
      assert.deepEqual(asset.downloads, order, 'manifest ordering and descriptors were not mutated');
      for (let index = 0; index < order.length; index++) assert.strictEqual(asset.downloads[index], order[index]);
      assert.deepEqual(readFileSync(join(options.cacheRoot, 'source/grammar.gz')), bytes);
      assert.deepEqual(readExternalGrammarAsset('source/grammar.gz', options), bytes);
      assert.equal(digest(gunzipSync(readExternalGrammarAsset('source/grammar.gz', options))), asset.decodedSha256);
    }
  }
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

test('transient HTTP downloads recover with capped attempts, backoffs and fresh timeout signals', async context => {
  const bytes = Buffer.from('pinned source');
  for (const status of [408, 429, 500, 503, 599]) {
    const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
    const delays = [], signals = [];
    let requests = 0;
    const results = await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async (url, { signal }) => {
        assert.equal(url, 'https://assets.example.test/pinned');
        assert.equal(existsSync(options.cacheRoot), false, 'no partial attempt was published');
        assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
        assert.ok(signal instanceof AbortSignal);
        signals.push(signal);
        requests++;
        return requests < 3 ? new Response('temporary failure', { status }) : new Response(bytes);
      }, sleepImpl: async delay => { delays.push(delay); } });
    assert.equal(requests, 3);
    assert.equal(new Set(signals).size, 3);
    assert.deepEqual(delays, [250, 500]);
    assert.deepEqual(results, [{ path: 'source/grammar.txt', reused: false }]);
    assert.deepEqual(readExternalGrammarAsset('source/grammar.txt', options), bytes);
    assert.deepEqual(readFileSync(join(options.cacheRoot, 'source/grammar.txt')), bytes);
  }
});

test('transport and timeout download failures retry without publishing incomplete assets', async context => {
  const bytes = Buffer.from('pinned source');
  for (const failure of [
    new TypeError('fetch failed', { cause: Object.assign(new Error('connection reset'), { code: 'ECONNRESET' }) }),
    Object.assign(new Error('connection timed out'), { code: 'ETIMEDOUT' }),
    Object.assign(new Error('temporary name lookup failure'), { code: 'EAI_AGAIN' }),
    Object.assign(new Error('request timeout'), { name: 'TimeoutError' }),
    Object.assign(new Error('request aborted'), { name: 'AbortError' }),
  ]) {
    const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
    const delays = [];
    let requests = 0;
    await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async () => {
        assert.equal(existsSync(options.cacheRoot), false);
        assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
        if (++requests === 1) throw failure;
        return new Response(bytes);
      }, sleepImpl: async delay => { delays.push(delay); } });
    assert.equal(requests, 2);
    assert.deepEqual(delays, [250]);
    assert.deepEqual(readExternalGrammarAsset('source/grammar.txt', options), bytes);
  }
});

test('interrupted response bodies restart from authenticated complete bytes', async context => {
  const bytes = Buffer.from('pinned complete source');
  const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
  const delays = [];
  let requests = 0, prefixes = 0;
  await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
    fetchImpl: async () => {
      assert.equal(existsSync(options.cacheRoot), false);
      assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
      if (++requests === 3) return new Response(bytes);
      return { ok: true, url: 'https://assets.example.test/pinned', body: (async function* () {
        prefixes++;
        yield bytes.subarray(0, 6);
        throw Object.assign(new Error('body connection reset'), { code: 'UND_ERR_SOCKET' });
      })() };
    }, sleepImpl: async delay => { delays.push(delay); } });
  assert.equal(requests, 3);
  assert.equal(prefixes, 2);
  assert.deepEqual(delays, [250, 500]);
  assert.deepEqual(readFileSync(join(options.cacheRoot, 'source/grammar.txt')), bytes);
  assert.deepEqual(readFileSync(join(options.root, 'source/grammar.txt')), bytes);
});

test('persistent transient failures exhaust each pinned mirror with bounded requests and no publication', async context => {
  const bytes = Buffer.from('pinned source');
  const asset = receipt(bytes);
  asset.downloads.push({ url: 'https://mirror.example.test/pinned', transform: 'identity' });
  const options = workspace(context, { 'source/grammar.txt': asset });
  const requests = [], delays = [];
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
    fetchImpl: async url => { requests.push(url); return new Response('still unavailable', { status: 503 }); },
    sleepImpl: async delay => { delays.push(delay); } }), /HTTP 503/u);
  assert.deepEqual(requests, [...Array(3).fill(asset.downloads[0].url), ...Array(3).fill(asset.downloads[1].url)]);
  assert.deepEqual(delays, [250, 500, 250, 500]);
  assert.equal(existsSync(options.cacheRoot), false);
  assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
  let transportRequests = 0;
  const transportDelays = [];
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true,
    fetchImpl: async () => { transportRequests++; throw Object.assign(new Error('network unreachable'), { code: 'ENETUNREACH' }); },
    sleepImpl: async delay => { transportDelays.push(delay); } }), /network unreachable/u);
  assert.equal(transportRequests, 6);
  assert.deepEqual(transportDelays, delays);
  assert.equal(existsSync(options.cacheRoot), false);
});

test('permanent HTTP failures are not retried and retain authenticated mirror fallback', async context => {
  const bytes = Buffer.from('pinned source');
  for (const status of [400, 401, 403, 404, 410]) {
    const options = workspace(context, { 'source/grammar.txt': receipt(bytes) });
    const delays = [];
    let requests = 0;
    await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true,
      fetchImpl: async () => { requests++; return new Response('permanent failure', { status }); },
      sleepImpl: async delay => { delays.push(delay); } }), new RegExp(`HTTP ${status}`, 'u'));
    assert.equal(requests, 1);
    assert.deepEqual(delays, []);
    assert.equal(existsSync(options.cacheRoot), false);
  }
  const asset = receipt(bytes);
  asset.downloads.push({ url: 'https://mirror.example.test/pinned', transform: 'identity' });
  const options = workspace(context, { 'source/grammar.txt': asset });
  const requests = [], delays = [];
  await downloadExternalGrammarAssets({ ...options, all: true,
    fetchImpl: async url => { requests.push(url); return requests.length === 1 ? new Response('missing', { status: 404 }) : new Response(bytes); },
    sleepImpl: async delay => { delays.push(delay); } });
  assert.deepEqual(requests, asset.downloads.map(({ url }) => url));
  assert.deepEqual(delays, []);
  assert.deepEqual(readExternalGrammarAsset('source/grammar.txt', options), bytes);
  {
    const raw = Buffer.from('pinned source fallback');
    const bytes = gzipSync(raw, { level: 9 });
    const asset = { ...receipt(bytes), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: digest(raw), downloads: [
      { url: 'https://upstream.example.test/first', transform: 'gzip' },
      { url: 'https://archive.example.test/first', transform: 'identity' },
      { url: 'https://upstream.example.test/second', transform: 'gzip' },
      { url: 'https://archive.example.test/second', transform: 'identity' },
    ] };
    const order = [...asset.downloads];
    const options = workspace(context, { 'source/grammar.gz': asset });
    const requests = [], delays = [];
    await downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async url => {
        requests.push(url);
        assert.equal(existsSync(options.cacheRoot), false);
        assert.equal(existsSync(join(options.root, 'source/grammar.gz')), false);
        if (url === 'https://archive.example.test/second') return new Response('temporary failure', { status: 503 });
        if (url !== 'https://upstream.example.test/second') return new Response('missing', { status: 404 });
        return new Response(raw);
      }, sleepImpl: async delay => { delays.push(delay); } });
    assert.deepEqual(requests, ['https://archive.example.test/first',
      ...Array(3).fill('https://archive.example.test/second'),
      'https://upstream.example.test/first', 'https://upstream.example.test/second']);
    assert.deepEqual(delays, [250, 500]);
    assert.deepEqual(asset.downloads, order);
    assert.deepEqual(readFileSync(join(options.cacheRoot, 'source/grammar.gz')), bytes);
    assert.deepEqual(readFileSync(join(options.root, 'source/grammar.gz')), bytes);
    assert.equal(digest(gunzipSync(bytes)), asset.decodedSha256);
  }
});

test('integrity and HTTPS failures are fatal without retries or concealment by another mirror', async context => {
  const bytes = Buffer.from('pinned source');
  const raw = Buffer.from('decoded source');
  const compressed = gzipSync(raw, { level: 9 });
  for (const entry of [
    { asset: receipt(bytes), response: () => new Response(Buffer.alloc(bytes.length, 0)), error: /checksum mismatch/u },
    { asset: receipt(bytes), response: () => new Response(Buffer.concat([bytes, Buffer.from('!')])), error: /exceeds its pinned size/u },
    { asset: receipt(bytes), response: () => {
      const response = new Response(bytes);
      Object.defineProperty(response, 'url', { value: 'http://assets.example.test/redirected' });
      return response;
    }, error: /redirect requires HTTPS/u },
    { asset: receipt(bytes), response: () => {
      const response = new Response('temporary failure', { status: 503 });
      Object.defineProperty(response, 'url', { value: 'http://assets.example.test/redirected' });
      return response;
    }, error: /redirect requires HTTPS/u },
    { asset: { ...receipt(compressed), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: '0'.repeat(64) },
      response: () => new Response(compressed), error: /decoded external grammar asset checksum mismatch/u },
  ]) {
    entry.asset.downloads.push({ url: 'https://mirror.example.test/pinned', transform: 'identity' });
    const options = workspace(context, { 'source/grammar.txt': entry.asset });
    const delays = [];
    let requests = 0;
    await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async () => { requests++; return entry.response(); },
      sleepImpl: async delay => { delays.push(delay); } }), entry.error);
    assert.equal(requests, 1);
    assert.deepEqual(delays, []);
    assert.equal(existsSync(options.cacheRoot), false);
    assert.equal(existsSync(join(options.root, 'source/grammar.txt')), false);
  }
  const unsafe = receipt(bytes);
  unsafe.downloads[0].url = 'http://assets.example.test/pinned';
  unsafe.downloads.push({ url: 'https://mirror.example.test/pinned', transform: 'identity' });
  const options = workspace(context, { 'source/grammar.txt': unsafe });
  await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true,
    fetchImpl: () => { throw new Error('unsafe URLs must be refused before fetching'); },
    sleepImpl: () => { throw new Error('unsafe URLs must never retry'); } }), /downloads require HTTPS/u);
  assert.equal(existsSync(options.cacheRoot), false);
  {
    const raw = Buffer.from('pinned source fallback');
    const changed = Buffer.from(raw);
    changed[0] ^= 1;
    const bytes = gzipSync(raw, { level: 9 });
    const asset = { ...receipt(bytes), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: digest(raw), downloads: [
      { url: 'https://upstream.example.test/changed', transform: 'gzip' },
      { url: 'https://archive.example.test/missing', transform: 'identity' },
      { url: 'https://upstream.example.test/correct', transform: 'gzip' },
    ] };
    const options = workspace(context, { 'source/grammar.gz': asset });
    const requests = [], delays = [];
    await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async url => {
        requests.push(url);
        if (url === 'https://archive.example.test/missing') return new Response('missing', { status: 404 });
        return new Response(url === 'https://upstream.example.test/changed' ? changed : raw);
      }, sleepImpl: async delay => { delays.push(delay); } }), /decoded external grammar asset checksum mismatch/u);
    assert.deepEqual(requests, ['https://archive.example.test/missing', 'https://upstream.example.test/changed']);
    assert.deepEqual(delays, []);
    assert.equal(existsSync(options.cacheRoot), false);
    assert.equal(existsSync(join(options.root, 'source/grammar.gz')), false);
  }
  {
    const raw = Buffer.from('pinned source fallback');
    const bytes = gzipSync(raw, { level: 9 });
    for (const pins of [{}, { decodedBytes: raw.length }, { decodedSha256: digest(raw) },
      { decodedBytes: -1, decodedSha256: digest(raw) }, { decodedBytes: 1.5, decodedSha256: digest(raw) },
      { decodedBytes: raw.length, decodedSha256: 'not a digest' }]) {
      const asset = { ...receipt(bytes), encoding: 'gzip', ...pins,
        downloads: [{ url: 'https://upstream.example.test/source', transform: 'gzip' }] };
      const options = workspace(context, { 'source/grammar.gz': asset });
      let requests = 0;
      const delays = [];
      await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
        fetchImpl: async () => { requests++; return new Response(raw); },
        sleepImpl: async delay => { delays.push(delay); } }), /invalid decoded grammar asset pin/u);
      assert.equal(requests, 0);
      assert.deepEqual(delays, []);
      assert.equal(existsSync(options.cacheRoot), false);
      assert.equal(existsSync(join(options.root, 'source/grammar.gz')), false);
    }
  }
  {
    const raw = Buffer.from('pinned source fallback');
    const bytes = gzipSync(raw, { level: 9 });
    const corrupt = Buffer.from(bytes);
    corrupt[0] ^= 1;
    const asset = { ...receipt(bytes), encoding: 'gzip', decodedBytes: raw.length, decodedSha256: digest(raw), downloads: [
      { url: 'https://upstream.example.test/correct', transform: 'gzip' },
      { url: 'https://archive.example.test/corrupt', transform: 'identity' },
    ] };
    const options = workspace(context, { 'source/grammar.gz': asset });
    const requests = [], delays = [];
    await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true, materialize: true,
      fetchImpl: async url => { requests.push(url); return new Response(url === 'https://archive.example.test/corrupt' ? corrupt : raw); },
      sleepImpl: async delay => { delays.push(delay); } }), /external grammar asset checksum mismatch/u);
    assert.deepEqual(requests, ['https://archive.example.test/corrupt']);
    assert.deepEqual(delays, []);
    assert.equal(existsSync(options.cacheRoot), false);
    assert.equal(existsSync(join(options.root, 'source/grammar.gz')), false);
  }
});

test('TLS, unsafe redirect and unclassified programming errors do not enter transport retries', async context => {
  const bytes = Buffer.from('pinned source');
  for (const failure of [
    new TypeError('fetch failed', { cause: Object.assign(new Error('expired certificate'), { code: 'CERT_HAS_EXPIRED' }) }),
    new TypeError('fetch failed', { cause: Object.assign(new Error('certificate name mismatch'), { code: 'ERR_TLS_CERT_ALTNAME_INVALID' }) }),
    new TypeError('fetch failed', { cause: Object.assign(new Error('invalid redirect URL'), { code: 'ERR_INVALID_URL' }) }),
    new TypeError('unexpected host programming error'),
  ]) {
    const asset = receipt(bytes);
    asset.downloads.push({ url: 'https://mirror.example.test/pinned', transform: 'identity' });
    const options = workspace(context, { 'source/grammar.txt': asset });
    const delays = [];
    let requests = 0;
    await assert.rejects(downloadExternalGrammarAssets({ ...options, all: true,
      fetchImpl: async () => { requests++; throw failure; }, sleepImpl: async delay => { delays.push(delay); } }), /fetch failed|programming error/u);
    assert.equal(requests, 1);
    assert.deepEqual(delays, []);
    assert.equal(existsSync(options.cacheRoot), false);
  }
});

test('download retry attempts remain inside the unchanged finite worker concurrency', async context => {
  const bytes = Buffer.from('pinned source');
  const assets = Object.fromEntries(Array.from({ length: 4 }, (_, index) => {
    const asset = receipt(bytes);
    asset.downloads[0].url = `https://assets.example.test/${index}`;
    return [`source/${index}.txt`, asset];
  }));
  const options = workspace(context, assets);
  const requests = new Map(), delays = [];
  let active = 0, maximum = 0;
  const result = await downloadExternalGrammarAssets({ ...options, all: true, jobs: 2,
    fetchImpl: async url => {
      active++;
      maximum = Math.max(maximum, active);
      assert.ok(active <= 2);
      await Promise.resolve();
      active--;
      const count = (requests.get(url) ?? 0) + 1;
      requests.set(url, count);
      return count === 1 ? new Response('unavailable', { status: 503 }) : new Response(bytes);
    }, sleepImpl: async delay => { delays.push(delay); } });
  assert.equal(result.length, 4);
  assert.equal(maximum, 2);
  assert.deepEqual([...requests.values()], [2, 2, 2, 2]);
  assert.deepEqual(delays, [250, 250, 250, 250]);
  for (const path of Object.keys(assets)) assert.deepEqual(readExternalGrammarAsset(path, options), bytes);
});
