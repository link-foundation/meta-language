// Replay recorded data migrations against independently pinned original Git blobs.
// Bulk replay belongs in CI; local tests use only tiny synthetic Git repositories.
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync } from 'node:fs';
import { isAbsolute, join, posix, relative, resolve, sep } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { isDeepStrictEqual } from 'node:util';
import { convertStructuredData } from './convert-structured-data.mjs';
import { renderFixtureSourceBytes } from './fixture-source-bytes.mjs';
import { checkProjectData, readProjectData, resolveProjectDataPath } from './project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

const repositoryRoot = fileURLToPath(new URL('../../', import.meta.url));
const baselinePath = 'parity/fixtures/structured-data-migration-baselines.lino';
const layoutPath = 'parity/fixtures/issue-195-json-source-layouts.lino';
const revisionPath = 'parity/fixtures/structured-data-migration-revisions.lino';
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

function revisionOf(entry, recorded, fallback, field) {
  const revision = recorded?.[field] ?? (field === 'originalGitRevision' ? entry.sourceRevision : entry.migrationRevision) ?? fallback;
  if (typeof revision !== 'string' || !fullRevision.test(revision)) {
    throw new Error(`${entry.logicalPath ?? 'migration baseline'}: missing immutable ${field === 'originalGitRevision' ? 'original Git provenance' : 'migration Git provenance'}; supply a full ${field} or its recorded per-file revision`);
  }
  return revision;
}

function applyRecordedCorrections(original, corrections, logicalPath, migrationRevision, repository) {
  if (!Array.isArray(corrections)) throw new Error('migration corrections must be recorded data');
  if (!corrections.length) return { value: original, observations: [] };
  const value = structuredClone(original);
  const observations = [];
  const targets = new Set();
  for (const rule of corrections) {
    if (rule.kind !== 'grammar parser fingerprint receipt correction' || rule.logicalPath !== logicalPath
      || typeof rule.language !== 'string' || typeof rule.provider !== 'string'
      || !digest.test(rule.previousSha256 ?? '') || !digest.test(rule.originalDecodedSha256 ?? '')
      || !digest.test(rule.grammarSha256 ?? '') || !fullRevision.test(rule.grammarRevision ?? '')
      || rule.temporalRelation !== 'later committed canonical grammar snapshot') {
      throw new Error('unsupported or unauthenticated migration metadata correction');
    }
    for (const path of [rule.grammarFile, rule.catalogFile]) {
      if (typeof path !== 'string' || !path.endsWith('.lino') || isAbsolute(path) || path.includes('\\')
        || path.includes('\0') || posix.normalize(path) !== path || path.startsWith('../')) {
        throw new Error('migration correction has an invalid immutable artifact path');
      }
    }
    if (sha256(JSON.stringify(original)) !== rule.originalDecodedSha256) throw new Error('migration correction does not authenticate the original committed JSON value');
    const target = `${rule.language}\0${rule.provider}`;
    if (targets.has(target)) throw new Error('migration corrections repeat a provider receipt');
    targets.add(target);
    const receipt = value.languages?.[rule.language]?.grammars?.[rule.provider];
    if (!receipt || receipt.parserSha256 !== rule.previousSha256) throw new Error('migration correction does not match the original provider receipt');
    if (rule.grammarRevision === migrationRevision) throw new Error('the correction artifact must be explicitly later than the historical conversion');
    if (git(repository, ['rev-parse', '--verify', `${rule.grammarRevision}^{commit}`]).toString('utf8').trim() !== rule.grammarRevision) {
      throw new Error('the correction artifact does not name the exact immutable commit');
    }
    try { git(repository, ['merge-base', '--is-ancestor', migrationRevision, rule.grammarRevision]); }
    catch { throw new Error('the correction artifact is not the recorded later grammar snapshot'); }
    const grammarSha256 = sha256(git(repository, ['show', `${rule.grammarRevision}:${rule.grammarFile}`]));
    if (grammarSha256 !== rule.grammarSha256) throw new Error('the correction fingerprint differs from its immutable grammar bytes');
    const catalog = readStructuredDataLino(git(repository, ['show', `${rule.grammarRevision}:${rule.catalogFile}`]).toString('utf8'));
    const provider = catalog.languages?.find(language => language.name === rule.language)?.grammars?.find(grammar => grammar.id === rule.provider);
    const file = catalog.nativeGrammars?.[rule.provider]?.file;
    if (!provider || provider.version !== receipt.version || provider.parserSha256 !== grammarSha256
      || typeof file !== 'string' || !file.startsWith('native-grammars/') || posix.normalize(file) !== file) {
      throw new Error('the correction fingerprint does not match the immutable language and provider identity');
    }
    for (const runtime of ['js', 'rust']) {
      if (sha256(git(repository, ['show', `${rule.grammarRevision}:${runtime}/src/data/${file}`])) !== grammarSha256) {
        throw new Error('the correction grammar differs between its immutable runtime copies');
      }
    }
    // The only authorized edit is an existing provider fingerprint scalar.
    // The complete transformed value/order is checked against historical LiNo.
    receipt.parserSha256 = grammarSha256;
    observations.push({ language: rule.language, provider: rule.provider, previousSha256: rule.previousSha256,
      grammarRevision: rule.grammarRevision, grammarSha256, temporalRelation: rule.temporalRelation });
  }
  return { value, observations };
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
 * actual converter against the committed historical LiNo. Changed current
 * outputs separately require an executed registered generator freshness check.
 */
export function validateStructuredDataMigration({
  repository = repositoryRoot,
  baselineFile = baselinePath,
  layoutsFile = layoutPath,
  originalGitRevision,
  migrationRevision,
  migrationRevisionFile = revisionPath,
  currentFreshnessChecks = [],
  freshnessCheckFile,
} = {}) {
  repository = filePath(repository, repositoryRoot);
  const baselines = readProjectData(filePath(baselineFile, repository));
  if (!Array.isArray(baselines) || baselines.length === 0) throw new Error('migration baselines must be a nonempty recorded array');
  // A function import may validate bounded synthetic fixtures locally. The
  // complete repository register cannot be replayed outside CI.
  if (baselines.length > 8 && !inCI()) throw new Error('bulk structured data migration replay runs only in CI');
  const recordedFile = filePath(migrationRevisionFile, repository);
  const recorded = existsSync(recordedFile) ? readProjectData(recordedFile) : [];
  if (!Array.isArray(recorded) || (recorded.length && !isDeepStrictEqual(recorded.map(entry => entry.logicalPath), baselines.map(entry => entry.logicalPath)))) {
    throw new Error('historical migration revisions must preserve every baseline identity and its order');
  }
  const byPath = new Map(recorded.map(entry => [entry.logicalPath, entry]));
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
    revisions.add(revisionOf(entry, byPath.get(entry.logicalPath), originalGitRevision, 'originalGitRevision'));
    revisions.add(revisionOf(entry, byPath.get(entry.logicalPath), migrationRevision, 'migrationRevision'));
  }
  for (const revision of revisions) {
    let resolved;
    try { resolved = git(repository, ['rev-parse', '--verify', `${revision}^{commit}`]).toString('utf8').trim(); }
    catch { throw new Error(`original Git revision is unavailable: ${revision}; CI needs the recorded history`); }
    if (resolved !== revision) throw new Error(`original Git revision does not name the exact recorded commit: ${revision}`);
  }
  const layouts = baselines.some(entry => entry.sourceSha256 !== undefined)
    ? readProjectData(filePath(layoutsFile, repository)) : null;
  const historicalLayouts = new Map();
  const conversionChanges = new Map();
  if (freshnessCheckFile) currentFreshnessChecks = readProjectData(filePath(freshnessCheckFile, repository)).currentFreshnessChecks;
  if (!Array.isArray(currentFreshnessChecks)) throw new Error('current freshness checks must be recorded command data');
  const checksByPath = new Map();
  for (const check of currentFreshnessChecks) {
    if (!Array.isArray(check.logicalPaths) || !check.logicalPaths.length || !Array.isArray(check.command)
      || check.command.length < 3 || check.command.some(part => typeof part !== 'string' || !part)
      || check.command[0] !== 'node' || !/\.(?:mjs|js)$/u.test(check.command[1]) || !check.command.includes('--check')) {
      throw new Error('current freshness needs a registered JavaScript generator --check command and its output identities');
    }
    for (const path of check.logicalPaths) {
      if (!names.has(path) || checksByPath.has(path)) throw new Error(`current freshness has an unknown or duplicate output identity: ${path}`);
      checksByPath.set(path, check);
    }
  }
  const freshnessResults = new Map();
  function verifyFreshness(check) {
    if (freshnessResults.has(check)) return freshnessResults.get(check);
    const before = check.logicalPaths.map(path => sha256(readFileSync(resolveProjectDataPath(join(repository, path)))));
    let observation;
    try {
      const output = execFileSync(process.execPath, check.command.slice(1), {
        cwd: repository, timeout: 600_000, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'],
      });
      const after = check.logicalPaths.map(path => sha256(readFileSync(resolveProjectDataPath(join(repository, path)))));
      if (!isDeepStrictEqual(before, after)) throw new Error('the freshness command rewrote a current output instead of checking it');
      observation = { command: check.command, exitCode: 0, outputSha256: sha256(output), logicalPaths: check.logicalPaths, currentSha256: after };
    } catch (error) {
      observation = new Error(`current generator freshness failed: ${error.message}`);
    }
    freshnessResults.set(check, observation);
    return observation;
  }
  const results = [];
  const failures = [];
  for (const entry of baselines) {
    const provenance = byPath.get(entry.logicalPath);
    const revision = revisionOf(entry, provenance, originalGitRevision, 'originalGitRevision');
    const conversionRevision = revisionOf(entry, provenance, migrationRevision, 'migrationRevision');
    try {
      let originalBytes;
      try { originalBytes = git(repository, ['show', `${revision}:${entry.logicalPath}`]); }
      catch { throw new Error('the recorded original JSON blob is unavailable; migrated data is not a baseline'); }
      const originalText = originalBytes.toString('utf8');
      if (!Buffer.from(originalText, 'utf8').equals(originalBytes)) throw new Error('the original JSON blob is not lossless UTF-8');
      const original = JSON.parse(originalText);
      const originalDecodedSha256 = sha256(JSON.stringify(original));
      const corrections = provenance?.corrections ?? [];
      if (corrections.length && entry.sourceSha256 !== undefined) throw new Error('metadata correction cannot rewrite an unchanged raw source byte contract');
      const corrected = applyRecordedCorrections(original, corrections, entry.logicalPath, conversionRevision, repository);
      const expected = corrected.value;
      const decodedSha256 = sha256(JSON.stringify(expected));
      if (decodedSha256 !== entry.decodedSha256) throw new Error('recorded decoded baseline hash differs from the original Git value');
      const originalSourceSha256 = sha256(originalBytes);
      if (entry.sourceSha256 !== undefined && originalSourceSha256 !== entry.sourceSha256) {
        throw new Error('recorded source byte hash differs from the original Git blob');
      }
      const physicalPath = entry.logicalPath.replace(/\.json$/u, '.lino');
      if (!conversionChanges.has(conversionRevision)) {
        const parent = git(repository, ['rev-parse', `${conversionRevision}^`]).toString('utf8').trim();
        const fields = git(repository, ['diff', '--name-status', '-z', '--no-renames', parent, conversionRevision]).toString('utf8').split('\0');
        const changes = new Map();
        for (let index = 0; index + 1 < fields.length; index += 2) changes.set(fields[index + 1], fields[index]);
        conversionChanges.set(conversionRevision, { parent, changes });
      }
      const conversion = conversionChanges.get(conversionRevision);
      if (conversion.changes.get(entry.logicalPath) !== 'D' || conversion.changes.get(physicalPath) !== 'A') {
        throw new Error('the migration revision is not the committed deletion of original JSON and addition of LiNo');
      }
      const sourceBlob = git(repository, ['rev-parse', `${revision}:${entry.logicalPath}`]).toString('utf8').trim();
      const parentBlob = git(repository, ['rev-parse', `${conversion.parent}:${entry.logicalPath}`]).toString('utf8').trim();
      if (sourceBlob !== parentBlob) throw new Error('the recorded original differs from the actual pre-conversion JSON blob');
      const historicalBytes = git(repository, ['show', `${conversionRevision}:${physicalPath}`]);
      const historical = readStructuredDataLino(historicalBytes.toString('utf8'));
      const compared = compareStructuredDataMigrationValues(expected, historical, `${entry.logicalPath}: historical conversion`);
      if (sha256(JSON.stringify(historical)) !== entry.decodedSha256) throw new Error('the historical ordered value differs from its frozen baseline hash');
      const originalReplay = convertStructuredData(originalText);
      compareStructuredDataMigrationValues(original, readStructuredDataLino(originalReplay), `${entry.logicalPath}: automatic original conversion`);
      // Apply recorded scalar corrections in the decoded links value model.
      // Passing it back through JSON would erase a preserved negative zero.
      const replay = corrections.length ? structuredDataToLino(expected) : originalReplay;
      compareStructuredDataMigrationValues(expected, readStructuredDataLino(replay), `${entry.logicalPath}: automatic conversion`);
      if (historicalBytes.toString('utf8') !== replay) throw new Error('historical LiNo differs from automatic conversion of the actual original Git source');
      const logical = join(repository, entry.logicalPath);
      const actual = resolveProjectDataPath(logical);
      if (existsSync(logical) || !actual.endsWith('.lino')) throw new Error('the migration must have one authoritative LiNo representation');
      const migrated = readProjectData(logical);
      const migratedBytes = readFileSync(actual);
      let freshness;
      if (!migratedBytes.equals(historicalBytes)) {
        const check = checksByPath.get(entry.logicalPath);
        if (!check) throw new Error('changed current output has no independently executed registered generator freshness check');
        freshness = verifyFreshness(check);
        if (freshness instanceof Error) throw freshness;
        checkProjectData(logical, migrated);
      } else compareStructuredDataMigrationValues(expected, migrated, entry.logicalPath);
      if (entry.sourceSha256 !== undefined) {
        if (!historicalLayouts.has(conversionRevision)) {
          const path = relative(repository, filePath(layoutsFile, repository)).split(sep).join('/');
          historicalLayouts.set(conversionRevision, readStructuredDataLino(git(repository, ['show', `${conversionRevision}:${path}`]).toString('utf8')));
        }
        const historicalLayout = historicalLayouts.get(conversionRevision);
        if (!Object.hasOwn(historicalLayout, entry.logicalPath)
          || !renderFixtureSourceBytes(historical, historicalLayout[entry.logicalPath]).equals(originalBytes)) {
          throw new Error('historical source layout does not reconstruct the exact original Git bytes');
        }
        if (!layouts || !Object.hasOwn(layouts, entry.logicalPath)) throw new Error('the original source byte contract has no recorded layout');
        const reconstructed = renderFixtureSourceBytes(migrated, layouts[entry.logicalPath]);
        if (!reconstructed.equals(originalBytes) || sha256(reconstructed) !== entry.sourceSha256) {
          throw new Error('source layout does not reconstruct the exact original Git bytes');
        }
      }
      results.push({
        logicalPath: entry.logicalPath, originalGitRevision: revision,
        migrationRevision: conversionRevision, decodedSha256, originalDecodedSha256, originalSourceSha256,
        historicalLinoSha256: sha256(historicalBytes), migratedLinoSha256: sha256(migratedBytes),
        ...(freshness ? { currentFreshness: freshness } : {}),
        ...(corrected.observations.length ? { metadataCorrections: corrected.observations } : {}),
        ...compared, ...(entry.sourceSha256 === undefined ? {} : { sourceSha256: entry.sourceSha256 }),
      });
    } catch (error) {
      failures.push({ logicalPath: entry.logicalPath, originalGitRevision: revision, migrationRevision: conversionRevision, reason: error.message });
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
    ['--baseline', 'baselineFile'], ['--layouts', 'layoutsFile'], ['--migration-revision', 'migrationRevision'],
    ['--migration-revisions', 'migrationRevisionFile'], ['--freshness-checks', 'freshnessCheckFile']]);
  const arguments_ = process.argv.slice(2);
  for (let index = 0; index < arguments_.length; index += 2) {
    const field = fields.get(arguments_[index]);
    const value = arguments_[index + 1];
    if (!field || !value || value.startsWith('--') || Object.hasOwn(options, field)) {
      throw new Error('usage: validate-structured-data-migration.mjs [--migration-revisions FILE] [--freshness-checks CONTRACT_FILE] [--original-revision FULL_GIT_SHA] [--migration-revision FULL_GIT_SHA] [--repository DIR] [--baseline FILE] [--layouts FILE]');
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
