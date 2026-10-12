// Replay recorded data migrations against independently pinned original Git blobs.
// Bulk replay belongs in CI; local tests use only tiny synthetic Git repositories.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { isAbsolute, join, posix, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { convertStructuredData } from './convert-structured-data.mjs';
import { renderFixtureSourceBytes } from './fixture-source-bytes.mjs';
import { readProjectData, resolveProjectDataPath } from './project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

const repositoryRoot = fileURLToPath(new URL('../../', import.meta.url));
const baselinePath = 'parity/fixtures/structured-data-migration-baselines.lino';
const layoutPath = 'parity/fixtures/issue-195-json-source-layouts.lino';
const sha256 = value => createHash('sha256').update(value).digest('hex');
const fullRevision = /^(?:[0-9a-f]{40}|[0-9a-f]{64})$/u;
const digest = /^[0-9a-f]{64}$/u;
const inCI = () => ['true', '1'].includes(process.env.CI);

function filePath(file, repository) {
  return file instanceof URL ? fileURLToPath(file) : resolve(repository, file);
}

function git(repository, arguments_) {
  return execFileSync('git', arguments_, {
    cwd: repository, maxBuffer: 128 * 1024 * 1024, timeout: 30_000, stdio: ['ignore', 'pipe', 'pipe'],
  });
}

function revisionOf(entry, originalGitRevision) {
  const revision = entry.sourceRevision ?? originalGitRevision;
  if (typeof revision !== 'string' || !fullRevision.test(revision)) {
    throw new Error(`${entry.logicalPath ?? 'migration baseline'}: missing immutable original Git provenance; supply a full originalGitRevision or recorded sourceRevision`);
  }
  return revision;
}

/** Check values, ordered occurrences and collection identities in both directions. */
export function compareStructuredDataMigrationValues(original, migrated, label = 'structured data') {
  if (!isDeepStrictEqual(original, migrated)) throw new Error(`${label}: migrated values or semantic identity fields differ from the original`);
  const forward = new WeakMap();
  const backward = new WeakMap();
  let collectionCount = 0;
  let scalarCount = 0;
  function compare(first, second, path) {
    if (first === null || typeof first !== 'object') {
      scalarCount += 1;
      return;
    }
    if (forward.has(first)) {
      if (forward.get(first) !== second) throw new Error(`${label}${path}: shared collection identity was split`);
      return;
    }
    if (backward.has(second)) throw new Error(`${label}${path}: distinct collection identities were merged`);
    forward.set(first, second); backward.set(second, first);
    collectionCount += 1;
    const names = Object.keys(first);
    if (!isDeepStrictEqual(names, Object.keys(second))) throw new Error(`${label}${path}: collection order differs from the original`);
    for (const name of names) compare(first[name], second[name], `${path}[${JSON.stringify(name)}]`);
  }
  compare(original, migrated, '');
  return { collectionCount, scalarCount };
}

/**
 * Authenticate the recorded baseline with original Git JSON, then replay the
 * actual converter and compare the authoritative LiNo without rewriting it.
 * Missing provenance fails; current decoded data never substitutes for Git.
 */
export function validateStructuredDataMigration({
  repository = repositoryRoot,
  baselineFile = baselinePath,
  layoutsFile = layoutPath,
  originalGitRevision,
} = {}) {
  repository = filePath(repository, repositoryRoot);
  const baselines = readProjectData(filePath(baselineFile, repository));
  if (!Array.isArray(baselines) || baselines.length === 0) throw new Error('migration baselines must be a nonempty recorded array');
  // A function import may validate bounded synthetic fixtures locally. The
  // complete repository register cannot be replayed outside CI.
  if (baselines.length > 8 && !inCI()) throw new Error('bulk structured data migration replay runs only in CI');
  const names = new Set();
  const revisions = new Set();
  for (const entry of baselines) {
    if (!entry || typeof entry !== 'object' || typeof entry.logicalPath !== 'string'
      || !entry.logicalPath.endsWith('.json') || isAbsolute(entry.logicalPath)
      || entry.logicalPath.includes('\\') || entry.logicalPath.includes('\0')
      || posix.normalize(entry.logicalPath) !== entry.logicalPath || entry.logicalPath.startsWith('../')) {
      throw new Error('migration baseline has an invalid original logical JSON path');
    }
    if (names.has(entry.logicalPath)) throw new Error(`duplicate migration baseline identity: ${entry.logicalPath}`);
    names.add(entry.logicalPath);
    if (!digest.test(entry.decodedSha256 ?? '') || (entry.sourceSha256 !== undefined && !digest.test(entry.sourceSha256))) {
      throw new Error(`${entry.logicalPath}: migration baseline has an invalid recorded hash`);
    }
    revisions.add(revisionOf(entry, originalGitRevision));
  }
  for (const revision of revisions) {
    let resolved;
    try { resolved = git(repository, ['rev-parse', '--verify', `${revision}^{commit}`]).toString('utf8').trim(); }
    catch { throw new Error(`original Git revision is unavailable: ${revision}; CI needs the recorded history`); }
    if (resolved !== revision) throw new Error(`original Git revision does not name the exact recorded commit: ${revision}`);
  }
  const layouts = baselines.some(entry => entry.sourceSha256 !== undefined)
    ? readProjectData(filePath(layoutsFile, repository)) : null;
  const results = [];
  const failures = [];
  for (const entry of baselines) {
    const revision = revisionOf(entry, originalGitRevision);
    try {
      let originalBytes;
      try { originalBytes = git(repository, ['show', `${revision}:${entry.logicalPath}`]); }
      catch { throw new Error('the recorded original JSON blob is unavailable; migrated data is not a baseline'); }
      const originalText = originalBytes.toString('utf8');
      if (!Buffer.from(originalText, 'utf8').equals(originalBytes)) throw new Error('the original JSON blob is not lossless UTF-8');
      const original = JSON.parse(originalText);
      const decodedSha256 = sha256(JSON.stringify(original));
      if (decodedSha256 !== entry.decodedSha256) throw new Error('recorded decoded baseline hash differs from the original Git value');
      const originalSourceSha256 = sha256(originalBytes);
      if (entry.sourceSha256 !== undefined && originalSourceSha256 !== entry.sourceSha256) {
        throw new Error('recorded source byte hash differs from the original Git blob');
      }
      const logical = join(repository, entry.logicalPath);
      const actual = resolveProjectDataPath(logical);
      if (existsSync(logical) || !actual.endsWith('.lino')) throw new Error('the migration must have one authoritative LiNo representation');
      const migrated = readProjectData(logical);
      const compared = compareStructuredDataMigrationValues(original, migrated, entry.logicalPath);
      if (sha256(JSON.stringify(migrated)) !== entry.decodedSha256) throw new Error('the migrated ordered value differs from its recorded baseline hash');
      const replay = convertStructuredData(originalText);
      compareStructuredDataMigrationValues(original, readStructuredDataLino(replay), `${entry.logicalPath}: automatic conversion`);
      const migratedBytes = readFileSync(actual);
      if (migratedBytes.toString('utf8') !== replay) throw new Error('the stored LiNo differs from automatic conversion of the original Git source');
      if (entry.sourceSha256 !== undefined) {
        if (!layouts || !Object.hasOwn(layouts, entry.logicalPath)) throw new Error('the original source byte contract has no recorded layout');
        const reconstructed = renderFixtureSourceBytes(migrated, layouts[entry.logicalPath]);
        if (!reconstructed.equals(originalBytes) || sha256(reconstructed) !== entry.sourceSha256) {
          throw new Error('source layout does not reconstruct the exact original Git bytes');
        }
      }
      results.push({
        logicalPath: entry.logicalPath, originalGitRevision: revision,
        decodedSha256, originalSourceSha256, migratedLinoSha256: sha256(migratedBytes),
        ...compared, ...(entry.sourceSha256 === undefined ? {} : { sourceSha256: entry.sourceSha256 }),
      });
    } catch (error) {
      failures.push({ logicalPath: entry.logicalPath, originalGitRevision: revision, reason: error.message });
    }
  }
  const report = { verifiedCount: results.length, baselineCount: baselines.length, results, failures };
  if (failures.length) {
    const error = new AggregateError(failures.map(failure => new Error(`${failure.logicalPath}: ${failure.reason}`)),
      `structured data migration failed for ${failures.length} of ${baselines.length} recorded originals`);
    error.report = report;
    throw error;
  }
  return report;
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  if (!inCI()) throw new Error('structured data migration replay command runs only in CI');
  const options = {};
  const fields = new Map([['--original-revision', 'originalGitRevision'], ['--repository', 'repository'],
    ['--baseline', 'baselineFile'], ['--layouts', 'layoutsFile']]);
  const arguments_ = process.argv.slice(2);
  for (let index = 0; index < arguments_.length; index += 2) {
    const field = fields.get(arguments_[index]);
    const value = arguments_[index + 1];
    if (!field || !value || value.startsWith('--') || Object.hasOwn(options, field)) {
      throw new Error('usage: validate-structured-data-migration.mjs --original-revision FULL_GIT_SHA [--repository DIR] [--baseline FILE] [--layouts FILE]');
    }
    options[field] = value;
  }
  try { process.stdout.write(structuredDataToLino(validateStructuredDataMigration(options))); }
  catch (error) {
    if (error.report) process.stdout.write(structuredDataToLino(error.report));
    console.error(error.message);
    for (const failure of error.report?.failures ?? []) console.error(`${failure.logicalPath}: ${failure.reason}`);
    process.exitCode = 1;
  }
}
