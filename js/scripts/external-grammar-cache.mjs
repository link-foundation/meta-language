#!/usr/bin/env node
// Temporary, authenticated upstream assets. The manifest stores metadata only;
// downloads reproduce the original bytes without compiling external grammars.
import { createHash, randomUUID } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { existsSync, lstatSync, mkdirSync, readFileSync, realpathSync, renameSync, rmSync, statSync, writeFileSync } from 'node:fs';
import { dirname, isAbsolute, join, resolve, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { gunzipSync, gzipSync } from 'node:zlib';
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

async function responseBytes(response, limit) {
  if (!response.ok) throw new Error(`HTTP ${response.status}`);
  if (response.url && new URL(response.url).protocol !== 'https:') throw new Error('external grammar redirect requires HTTPS');
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
  for (const download of asset.downloads ?? []) {
    try {
      const url = new URL(download.url);
      if (url.protocol !== 'https:') throw new Error('external grammar downloads require HTTPS');
      const limit = download.transform === 'gzip' ? asset.decodedBytes : asset.bytes;
      const response = await (options.fetchImpl ?? fetch)(url.href, { signal: AbortSignal.timeout(options.timeoutMs ?? 30_000) });
      const raw = await responseBytes(response, limit);
      const bytes = download.transform === 'gzip' ? gzipSync(raw, { level: 9 }) : raw;
      verify(bytes, asset, path);
      storeAcquired(path, bytes, context, options);
      return { path, reused: false };
    } catch (error) { errors.push(`${download.url}: ${error.message}`); }
  }
  throw new Error(`cannot download authenticated external grammar asset ${path}: ${errors.join('; ')}`);
}

/** Download only the explicitly selected pinned assets, with bounded concurrency. */
export async function downloadExternalGrammarAssets(options = {}) {
  const context = settings(options);
  const paths = selectedPaths(context, options);
  const jobs = options.jobs ?? 4;
  if (!Number.isInteger(jobs) || jobs < 1 || jobs > 16) throw new Error('external grammar download jobs must be between 1 and 16');
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
