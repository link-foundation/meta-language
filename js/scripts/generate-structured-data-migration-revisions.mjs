// Derive immutable conversion provenance from Git tree changes, never new values.
import { execFileSync } from 'node:child_process';
import { resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';
import { existsSync, readFileSync } from 'node:fs';
import { structuredDataToLino } from '../src/structured-data-links.js';

const repositoryRoot = fileURLToPath(new URL('../../', import.meta.url));
const baselinePath = 'parity/fixtures/structured-data-migration-baselines.lino';
const revisionPath = 'parity/fixtures/structured-data-migration-revisions.lino';
const correctionPath = 'parity/fixtures/structured-data-migration-corrections.lino';
const git = (repository, arguments_) => execFileSync('git', arguments_, {
  cwd: repository, encoding: 'utf8', timeout: 30_000, maxBuffer: 16 * 1024 * 1024, stdio: ['ignore', 'pipe', 'pipe'],
});

/** Locate committed JSON deletion and LiNo addition using tree metadata only. */
export function generateStructuredDataMigrationRevisions({ repository = repositoryRoot, baselineFile = baselinePath,
  correctionFile = correctionPath } = {}) {
  repository = resolve(repository);
  const baselines = readProjectData(resolve(repository, baselineFile));
  if (!Array.isArray(baselines) || !baselines.length) throw new Error('migration revision generation needs recorded baselines');
  const logicalPaths = baselines.map(row => row.logicalPath);
  if (logicalPaths.some(path => typeof path !== 'string' || !path.endsWith('.json'))
    || new Set(logicalPaths).size !== logicalPaths.length) throw new Error('migration baseline identities are invalid');
  const physicalPaths = logicalPaths.map(path => path.replace(/\.json$/u, '.lino'));
  const corrections = existsSync(resolve(repository, correctionFile)) ? readProjectData(resolve(repository, correctionFile)) : [];
  if (!Array.isArray(corrections) || corrections.some(row => !logicalPaths.includes(row.logicalPath))) {
    throw new Error('migration corrections must identify actual recorded baseline files');
  }
  const commits = git(repository, ['log', '--no-renames', '--diff-filter=A', '--format=%H', '--', ...physicalPaths]).trim().split('\n').filter(Boolean);
  const entries = new Map();
  // Git lists newest first. Older genuine conversions replace later ones.
  for (const migrationRevision of commits) {
    const originalGitRevision = git(repository, ['rev-parse', `${migrationRevision}^`]).trim();
    const fields = git(repository, ['diff', '--name-status', '-z', '--no-renames', originalGitRevision, migrationRevision,
      '--', ...logicalPaths, ...physicalPaths]).split('\0');
    const changes = new Map();
    for (let index = 0; index + 1 < fields.length; index += 2) changes.set(fields[index + 1], fields[index]);
    for (const [index, logicalPath] of logicalPaths.entries()) {
      if (changes.get(logicalPath) === 'D' && changes.get(physicalPaths[index]) === 'A') {
        entries.set(logicalPath, { logicalPath, originalGitRevision, migrationRevision });
      }
    }
  }
  return logicalPaths.map(path => {
    if (!entries.has(path)) throw new Error(`no independently committed JSON-to-LiNo conversion was found: ${path}`);
    const entry = entries.get(path);
    const selected = corrections.filter(correction => correction.logicalPath === path);
    return selected.length ? { ...entry, corrections: selected } : entry;
  });
}

/** List current changed outputs without replaying or decoding their contents. */
export function readStructuredDataMigrationChanges(entries, { repository = repositoryRoot } = {}) {
  const groups = new Map();
  for (const entry of entries) {
    const paths = groups.get(entry.migrationRevision) ?? [];
    paths.push(entry.logicalPath.replace(/\.json$/u, '.lino'));
    groups.set(entry.migrationRevision, paths);
  }
  const changed = new Set();
  for (const [revision, paths] of groups) {
    const committed = git(repository, ['diff', '--name-only', '-z', '--no-renames', revision, 'HEAD', '--', ...paths]);
    const worktree = git(repository, ['diff', '--name-only', '-z', '--no-renames', 'HEAD', '--', ...paths]);
    for (const path of [...committed.split('\0'), ...worktree.split('\0')]) if (path) changed.add(path);
  }
  return entries.filter(entry => changed.has(entry.logicalPath.replace(/\.json$/u, '.lino')));
}

if (process.argv[1] && import.meta.url === pathToFileURL(resolve(process.argv[1])).href) {
  if (process.argv.slice(2).some(argument => argument !== '--check')) throw new Error('usage: generate-structured-data-migration-revisions.mjs [--check]');
  const entries = generateStructuredDataMigrationRevisions();
  const output = new URL(`../../${revisionPath}`, import.meta.url);
  if (process.argv.includes('--check')) {
    if (readFileSync(output, 'utf8') !== structuredDataToLino(entries)) throw new Error('structured data migration revision provenance is stale');
  } else writeProjectData(output, entries);
}
