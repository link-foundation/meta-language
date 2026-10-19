#!/usr/bin/env node
// Temporary, authenticated upstream assets. The manifest stores metadata only;
// downloads reproduce the original bytes without compiling external grammars.
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, lstatSync, mkdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { basename, dirname, isAbsolute, join, relative, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync, gzipSync } from 'node:zlib';
import { setTimeout as sleep } from 'node:timers/promises';
import { readStructuredDataLino } from '../src/structured-data-links.js';

const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const MANIFEST = 'parity/grammars/external-assets.lino';
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const manifests = new Map();

function readManifest(root) {
  const path = join(root, MANIFEST);
  const { mtimeMs, size } = statSync(path);
  const existing = manifests.get(path);
  if (existing?.mtimeMs === mtimeMs && existing.size === size) return existing.value;
  const value = readStructuredDataLino(readFileSync(path, 'utf8'));
  manifests.set(path, { mtimeMs, size, value });
  return value;
}

function logicalPath(value) {
  if (typeof value !== 'string' || !value || isAbsolute(value) || value.includes('\\') || value.includes('\0')
    || value.split('/').some(part => part === '' || part === '.' || part === '..')) throw new Error(`unsafe external grammar asset path: ${value}`);
  return value;
}

function settings(options = {}) {
  const root = resolve(options.root ?? ROOT);
  const cacheRoot = resolve(options.cacheRoot ?? process.env.META_LANGUAGE_EXTERNAL_GRAMMAR_CACHE ?? join(root, '.external-grammars'));
  const manifest = options.manifest ?? readManifest(root);
  if (manifest.schemaVersion !== 1 || !manifest.assets || Array.isArray(manifest.assets)) throw new Error('invalid external grammar asset manifest');
  return { root, cacheRoot, manifest };
}

function record(path, context) {
  logicalPath(path);
  const asset = Object.hasOwn(context.manifest.assets, path) ? context.manifest.assets[path] : null;
  if (!asset || !/^[a-f0-9]{64}$/.test(asset.sha256) || !Number.isSafeInteger(asset.bytes) || asset.bytes < 0) throw new Error(`unregistered external grammar asset: ${path}`);
  return asset;
}

function verify(bytes, asset, path) {
  if (bytes.length !== asset.bytes || digest(bytes) !== asset.sha256) throw new Error(`external grammar asset checksum mismatch: ${path}`);
  if (asset.encoding === 'gzip') {
    if (!Number.isSafeInteger(asset.decodedBytes) || asset.decodedBytes < 0 || !/^[a-f0-9]{64}$/.test(asset.decodedSha256)) throw new Error(`invalid decoded grammar asset pin: ${path}`);
    const decoded = gunzipSync(bytes, { maxOutputLength: Math.max(1, asset.decodedBytes + 1) });
    if (decoded.length !== asset.decodedBytes || digest(decoded) !== asset.decodedSha256) throw new Error(`decoded external grammar asset checksum mismatch: ${path}`);
  }
  return bytes;
}

function safeFile(base, path, create = false) {
  const relative = logicalPath(path);
  const target = resolve(base, relative);
  if (!target.startsWith(`${resolve(base)}${sep}`)) throw new Error(`external grammar cache path escapes its root: ${path}`);
  if (create) mkdirSync(base, { recursive: true });
  let at = resolve(base);
  if (existsSync(at) && lstatSync(at).isSymbolicLink()) throw new Error(`symbolic external grammar cache root: ${at}`);
  for (const part of relative.split('/').slice(0, -1)) {
    at = join(at, part);
    if (existsSync(at) && (!lstatSync(at).isDirectory() || lstatSync(at).isSymbolicLink())) throw new Error(`unsafe external grammar asset parent: ${at}`);
    if (create) mkdirSync(at, { recursive: true });
  }
  if (existsSync(target) && (!lstatSync(target).isFile() || lstatSync(target).isSymbolicLink())) throw new Error(`unsafe external grammar asset file: ${target}`);
  if (existsSync(base) && existsSync(dirname(target)) && !realpathSync(dirname(target)).startsWith(`${realpathSync(base)}${sep}`)
    && realpathSync(dirname(target)) !== realpathSync(base)) throw new Error(`external grammar asset follows an escaping parent: ${path}`);
  return target;
}

function save(base, path, bytes) {
  const target = safeFile(base, path, true);
  const temporary = `${target}.${randomUUID()}.part`;
  try {
    writeFileSync(temporary, bytes, { flag: 'wx' });
    renameSync(temporary, target);
  } finally { rmSync(temporary, { force: true }); }
  return target;
}

/** A verified materialized asset, or its verified temporary cache copy. */
export function externalGrammarPath(path, options = {}) {
  const context = settings(options);
  const asset = record(path, context);
  for (const base of [context.root, context.cacheRoot]) {
    const target = safeFile(base, path);
    if (!existsSync(target)) continue;
    verify(readFileSync(target), asset, path);
    return target;
  }
  throw new Error(`external grammar asset is not downloaded: ${path}; run node js/scripts/external-grammar-cache.mjs --asset ${path} --materialize`);
}

export function readExternalGrammarAsset(path, options = {}) {
  return readFileSync(externalGrammarPath(path, options));
}

/** A directory only qualifies when every registered asset below it verifies. */
export function externalGrammarDirectory(directory, options = {}) {
  const context = settings(options);
  logicalPath(directory);
  const paths = Object.keys(context.manifest.assets).filter(path => path.startsWith(`${directory}/`));
  if (!paths.length) throw new Error(`unregistered external grammar asset directory: ${directory}`);
  for (const base of [context.root, context.cacheRoot]) {
    const present = paths.every(path => existsSync(safeFile(base, path)));
    if (!present) continue;
    for (const path of paths) verify(readFileSync(safeFile(base, path)), record(path, context), path);
    return join(base, directory);
  }
  throw new Error(`external grammar asset directory is not downloaded: ${directory}`);
}

function selectedPaths(context, options) {
  const assets = options.assets ?? [];
  const groups = options.groups ?? [];
  const known = new Set(Object.values(context.manifest.assets).map(asset => asset.group));
  for (const group of groups) if (!known.has(group)) throw new Error(`unknown external grammar asset group: ${group}`);
  for (const path of assets) record(path, context);
  const selected = Object.keys(context.manifest.assets).filter(path => options.all || assets.includes(path) || groups.includes(context.manifest.assets[path].group));
  if (!selected.length) throw new Error('select external grammar assets with --asset, --group or --all');
  return selected;
}

// A custom cache inside this worktree is a source-storage location until Git
// explicitly ignores it. Outside-worktree scratch and Gitless workspaces keep
// their existing behavior; the guard never adds or changes an ignore rule.
function verifyCacheDirectory(context, paths, materialize = false) {
  let worktree;
  try {
    worktree = execFileSync('git', ['rev-parse', '--show-toplevel'], {
      cwd: context.root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'],
    }).trim();
  } catch (error) {
    if (error.status === 128 && /not a git repository/u.test(String(error.stderr))) return;
    throw error;
  }
  worktree = realpathSync(worktree);
  // Git canonicalizes macOS /var and other symbolic ancestor spellings. Resolve
  // the existing prefix even when the new cache directory does not exist yet.
  const suffix = [];
  let existing = context.cacheRoot;
  while (!existsSync(existing)) { suffix.unshift(basename(existing)); existing = dirname(existing); }
  const cache = join(realpathSync(existing), ...suffix);
  const inside = cache === worktree || cache.startsWith(`${worktree}${sep}`);
  if (!inside && !materialize) return;
  const location = relative(worktree, cache).split(sep).join('/');
  const probe = location ? `${location}/__meta_language_external_asset_probe__` : '__meta_language_external_asset_probe__';
  const cachePaths = inside ? [probe, ...paths.map(path => location ? `${location}/${path}` : path)] : [];
  const project = relative(worktree, realpathSync(context.root)).split(sep).join('/');
  const materializedPaths = materialize ? paths.map(path => project ? `${project}/${path}` : path) : [];
  let ignored;
  try {
    ignored = new Set(execFileSync('git', ['check-ignore', '--no-index', '-z', '--stdin'], {
      cwd: worktree, encoding: 'utf8', input: [...cachePaths, ...materializedPaths].join('\0') + '\0', stdio: ['pipe', 'pipe', 'pipe'],
    }).split('\0'));
  } catch (error) {
    if (error.status === 1) ignored = new Set();
    else throw error;
  }
  for (const file of cachePaths) if (!ignored.has(file)) throw new Error(`external grammar cache inside the worktree must be explicitly ignored by Git: ${file}`);
  for (const file of materializedPaths) if (!ignored.has(file)) throw new Error(`external grammar materialized input must be explicitly ignored by Git: ${file}`);
  const tracked = new Set(execFileSync('git', ['ls-files', '-z'], {
    cwd: worktree, encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'],
  }).split('\0'));
  for (const path of inside ? paths : []) {
    const file = location ? `${location}/${path}` : path;
    if (tracked.has(file)) throw new Error(`external grammar cache input must be untracked: ${file}`);
  }
  for (const file of materializedPaths) if (tracked.has(file)) throw new Error(`external grammar materialized input must be untracked: ${file}`);
}

class GrammarDownloadResponseError extends Error {
  constructor(status) {
    super(`HTTP ${status}`);
    this.status = status;
  }
}

function isTransientDownloadError(error) {
  if (error instanceof GrammarDownloadResponseError) return error.status === 408 || error.status === 429 || error.status >= 500 && error.status <= 599;
  if (error?.name === 'TimeoutError' || error?.name === 'AbortError') return true;
  const codes = ['ECONNRESET', 'ECONNREFUSED', 'ECONNABORTED', 'ETIMEDOUT', 'EAI_AGAIN', 'ENOTFOUND', 'ENETUNREACH', 'EHOSTUNREACH', 'EPIPE',
    'UND_ERR_CONNECT_TIMEOUT', 'UND_ERR_HEADERS_TIMEOUT', 'UND_ERR_BODY_TIMEOUT', 'UND_ERR_SOCKET'];
  return codes.includes(error?.code) || codes.includes(error?.cause?.code);
}

async function responseBytes(response, limit) {
  if (response.url && new URL(response.url).protocol !== 'https:') throw new Error('external grammar redirect requires HTTPS');
  if (!response.ok) {
    const error = new GrammarDownloadResponseError(response.status);
    try { await response.body?.cancel(); }
    catch (cause) { error.cause = cause; }
    throw error;
  }
  if (!response.body) {
    const bytes = Buffer.from(await response.arrayBuffer());
    if (bytes.length > limit) throw new Error('external grammar download exceeds its pinned size');
    return bytes;
  }
  const chunks = [];
  let size = 0;
  for await (const chunk of response.body) {
    size += chunk.length;
    if (size > limit) throw new Error('external grammar download exceeds its pinned size');
    chunks.push(Buffer.from(chunk));
  }
  return Buffer.concat(chunks);
}

// Fixed attempt/backoff caps do not change request timeouts or worker limits.
// Only transport/status failures repeat; bytes are authenticated afterwards.
async function downloadBytesWithRetries(url, limit, options) {
  for (let attempt = 0; ; attempt++) {
    try {
      const response = await (options.fetchImpl ?? fetch)(url, { signal: AbortSignal.timeout(options.timeoutMs ?? 30_000) });
      return await responseBytes(response, limit);
    } catch (error) {
      if (!isTransientDownloadError(error) || attempt === 2) throw error;
      await (options.sleepImpl ?? sleep)(250 * 2 ** attempt);
    }
  }
}

function storeAcquired(path, bytes, context, options) {
  const target = options.materialize ? safeFile(context.root, path) : null;
  const existing = target !== null && existsSync(target);
  if (existing) verify(readFileSync(target), record(path, context), path);
  save(context.cacheRoot, path, bytes);
  // Existing authenticated materialized files need no replacement, and a
  // modified file must never be overwritten by a valid cache hit.
  if (options.materialize && !existing) save(context.root, path, bytes);
}

async function acquire(path, context, options) {
  const asset = record(path, context);
  // Reuse authenticated local bytes; a corrupt existing copy is a refusal.
  for (const base of [context.cacheRoot, context.root]) {
    const target = safeFile(base, path);
    if (!existsSync(target)) continue;
    const bytes = verify(readFileSync(target), asset, path);
    storeAcquired(path, bytes, context, options);
    return { path, reused: true };
  }
  const errors = [];
  const downloads = asset.downloads ?? [];
  // Exact archived bytes do not depend on a local compressor's representation.
  // Keep every descriptor and its original order within each acquisition group.
  for (const download of [...downloads.filter(download => download.transform === 'identity'),
    ...downloads.filter(download => download.transform !== 'identity')]) {
    try {
      const url = new URL(download.url);
      if (url.protocol !== 'https:') throw new Error('external grammar downloads require HTTPS');
      if (download.transform === 'gzip' && (!Number.isSafeInteger(asset.decodedBytes) || asset.decodedBytes < 0
        || !/^[a-f0-9]{64}$/.test(asset.decodedSha256))) throw new Error(`invalid decoded grammar asset pin: ${path}`);
      const limit = download.transform === 'gzip' ? asset.decodedBytes : asset.bytes;
      const raw = await downloadBytesWithRetries(url.href, limit, options);
      if (download.transform === 'gzip' && (raw.length !== asset.decodedBytes || digest(raw) !== asset.decodedSha256)) {
        throw new Error(`decoded external grammar asset checksum mismatch: ${path}`);
      }
      const bytes = download.transform === 'gzip' ? gzipSync(raw, { level: 9 }) : raw;
      verify(bytes, asset, path);
      storeAcquired(path, bytes, context, options);
      return { path, reused: false };
    } catch (error) {
      errors.push(`${download.url}: ${error.message}`);
      if (!(error instanceof GrammarDownloadResponseError) && !isTransientDownloadError(error)) {
        throw new Error(`cannot download authenticated external grammar asset ${path}: ${errors.join('; ')}`, { cause: error });
      }
    }
  }
  throw new Error(`cannot download authenticated external grammar asset ${path}: ${errors.join('; ')}`);
}

/** Download only the explicitly selected pinned assets, with bounded concurrency. */
export async function downloadExternalGrammarAssets(options = {}) {
  const context = settings(options);
  const paths = selectedPaths(context, options);
  const jobs = options.jobs ?? 4;
  if (!Number.isInteger(jobs) || jobs < 1 || jobs > 16) throw new Error('external grammar download jobs must be between 1 and 16');
  verifyCacheDirectory(context, paths, Boolean(options.materialize));
  let cursor = 0;
  const results = [];
  const failures = [];
  await Promise.all(Array.from({ length: Math.min(jobs, paths.length) }, async () => {
    for (;;) {
      const index = cursor++;
      if (index >= paths.length) return;
      try { results[index] = await acquire(paths[index], context, options); }
      catch (error) { failures.push(error); }
    }
  }));
  if (failures.length) throw new Error(failures.map(error => error.message).join('\n'));
  return results;
}

/** Remove only authenticated, ignored, untracked materialized downloads. */
export function cleanMaterializedExternalGrammarAssets(options = {}) {
  const context = settings(options);
  const tracked = new Set(execFileSync('git', ['ls-files', '-z'], { cwd: context.root, encoding: 'utf8' }).split('\0'));
  const removed = [];
  for (const path of Object.keys(context.manifest.assets)) {
    const target = safeFile(context.root, path);
    if (!existsSync(target)) continue;
    if (tracked.has(path)) throw new Error(`refusing to clean tracked external grammar asset: ${path}`);
    execFileSync('git', ['check-ignore', '--quiet', '--', path], { cwd: context.root });
    verify(readFileSync(target), record(path, context), path);
    rmSync(target);
    removed.push(path);
  }
  return removed;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const options = { assets: [], groups: [] };
  let check = false;
  let clean = false;
  for (let at = 2; at < process.argv.length; at++) {
    const argument = process.argv[at];
    if (argument === '--asset') options.assets.push(process.argv[++at]);
    else if (argument === '--group') options.groups.push(process.argv[++at]);
    else if (argument === '--all') options.all = true;
    else if (argument === '--materialize') options.materialize = true;
    else if (argument === '--cache-root') options.cacheRoot = process.argv[++at];
    else if (argument === '--check') check = true;
    else if (argument === '--clean-materialized') clean = true;
    else throw new Error(`unknown external grammar download option: ${argument}`);
  }
  if (clean) console.log(`removed ${cleanMaterializedExternalGrammarAssets(options).length} temporary external grammar assets`);
  else if (check) {
    const context = settings(options);
    const paths = selectedPaths(context, options);
    for (const path of paths) externalGrammarPath(path, options);
    console.log(`verified ${paths.length} temporary external grammar assets`);
  } else console.log(`prepared ${(await downloadExternalGrammarAssets(options)).length} temporary external grammar assets`);
}
