#!/usr/bin/env node
// Immutable source provenance for the existing source/manifest audit. This
// checker establishes captured bytes and revision identity, not adoption,
// theoretical consistency, or execution of upstream providers.
import { createHash } from 'node:crypto';
import { readFile, mkdir, writeFile } from 'node:fs/promises';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData } from './project-data.mjs';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const auditFile = 'parity/associative-stack-audit.json';
const digest = (algorithm, bytes) => createHash(algorithm).update(bytes).digest('hex');
const blobHash = bytes => digest('sha1', Buffer.concat([Buffer.from(`blob ${bytes.length}\0`), bytes]));
const sourcePath = (repository, revision, path) => `parity/sources/associative-stack/${repository}/${revision}/${path}`;

/** Validate exact pinned bytes and Git object identities without network access. */
export function checkAssociativeStackSources(audit, snapshots) {
  const problems = [];
  if (audit.evidenceKind !== 'source-and-manifest-inspection' || audit.executionEvidence !== false) {
    problems.push('source provenance must retain the source/manifest audit boundary and executionEvidence:false');
  }
  if (!Array.isArray(audit.guides) || audit.guides.length !== 2
    || new Set(audit.guides.map(guide => guide.repository)).size !== 2) {
    problems.push('the audit must preserve both distinct registered foundation guides');
    return problems;
  }
  for (const guide of audit.guides) {
    const label = guide.repository;
    if (typeof label !== 'string' || !/^[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+$/u.test(label)
      || typeof guide.revision !== 'string' || !/^[a-f0-9]{40}$/u.test(guide.revision)) {
      problems.push('a guide needs a repository and complete immutable Git revision');
      continue;
    }
    const paths = guide.paths ?? (guide.path ? [guide.path] : []);
    if (!paths.length || paths.some(path => typeof path !== 'string' || !path || path.startsWith('/') || path.split('/').includes('..'))
      || new Set(paths).size !== paths.length) {
      problems.push(`${label}: source paths must be unique repository-relative paths`);
      continue;
    }
    const verification = guide.commitVerification;
    if (verification?.requestedRevision !== guide.revision || verification?.resolvedRevision !== guide.revision
      || !/^[a-f0-9]{40}$/u.test(verification?.treeSha1 ?? '')
      || verification?.url !== `https://api.github.com/repos/${label}/git/commits/${guide.revision}`
      || verification?.metadataFile !== sourcePath(label, guide.revision, 'github-commit-response.json')) {
      problems.push(`${label}: commit verification must resolve the exact recorded revision and tree`);
    } else {
      const captured = snapshots.get(verification.metadataFile);
      try {
        const bytes = Buffer.from(captured);
        const commit = JSON.parse(bytes.toString('utf8'));
        if (verification.bytes !== bytes.length || verification.sha256 !== digest('sha256', bytes)
          || commit.sha !== guide.revision || commit.url !== verification.url || commit.tree?.sha !== verification.treeSha1) {
          problems.push(`${label}: recorded commit metadata differs from its captured GitHub response`);
        }
      } catch { problems.push(`${label}: exact captured GitHub commit response is missing or invalid`); }
    }
    if (!Array.isArray(guide.files) || guide.files.length !== paths.length
      || new Set(guide.files.map(file => file.path)).size !== paths.length
      || guide.files.some(file => !paths.includes(file.path))) {
      problems.push(`${label}: every declared guide path needs exactly one captured file`);
      continue;
    }
    for (const file of guide.files) {
      const expectedFile = sourcePath(label, guide.revision, file.path);
      if (file.sourceFile !== expectedFile
        || file.url !== `https://github.com/${label}/blob/${guide.revision}/${file.path}`) {
        problems.push(`${label}/${file.path}: captured source identity or immutable URL differs`);
        continue;
      }
      const snapshot = snapshots.get(expectedFile);
      if (snapshot === undefined) { problems.push(`${label}/${file.path}: captured source is missing`); continue; }
      const bytes = Buffer.from(snapshot);
      if (!Number.isSafeInteger(file.bytes) || file.bytes <= 0 || file.bytes !== bytes.length
        || file.sha256 !== digest('sha256', bytes) || file.blobSha1 !== blobHash(bytes)) {
        problems.push(`${label}/${file.path}: byte length, content hash or Git blob identity differs`);
      }
    }
  }
  return problems;
}

async function githubJson(path) {
  const response = await fetch(`https://api.github.com/repos/${path}`, {
    headers: { Accept: 'application/vnd.github+json', 'User-Agent': 'meta-language-source-provenance' },
  });
  if (!response.ok) throw new Error(`GitHub source request failed (${response.status}): ${path}`);
  const bytes = Buffer.from(await response.arrayBuffer());
  return { value: JSON.parse(bytes.toString('utf8')), bytes };
}

async function main(args) {
  const audit = readProjectData(join(root, auditFile));
  const snapshots = new Map();
  if (args.includes('--refresh')) {
    const results = await Promise.allSettled(audit.guides.map(async guide => {
      const { value: commit, bytes: commitBytes } = await githubJson(`${guide.repository}/git/commits/${guide.revision}`);
      if (commit.sha !== guide.revision) throw new Error(`GitHub resolved a different source revision: ${guide.repository}`);
      const paths = guide.paths ?? [guide.path];
      const contents = await Promise.allSettled(paths.map(async path => {
        const { value: content } = await githubJson(`${guide.repository}/contents/${path}?ref=${guide.revision}`);
        if (content.type !== 'file' || content.encoding !== 'base64') throw new Error(`GitHub returned no exact file bytes: ${guide.repository}/${path}`);
        const bytes = Buffer.from(content.content, 'base64');
        if (content.sha !== blobHash(bytes) || content.size !== bytes.length) throw new Error(`GitHub file object differs from its decoded bytes: ${guide.repository}/${path}`);
        const sourceFile = sourcePath(guide.repository, guide.revision, path);
        snapshots.set(sourceFile, bytes);
        return { path, sourceFile, url: `https://github.com/${guide.repository}/blob/${guide.revision}/${path}`,
          bytes: bytes.length, sha256: digest('sha256', bytes), blobSha1: content.sha };
      }));
      const rejected = contents.filter(result => result.status === 'rejected');
      if (rejected.length) throw new AggregateError(rejected.map(result => result.reason), 'foundation source capture failed');
      const metadataFile = sourcePath(guide.repository, guide.revision, 'github-commit-response.json');
      snapshots.set(metadataFile, commitBytes);
      return { ...guide, commitVerification: { requestedRevision: guide.revision, resolvedRevision: commit.sha,
        treeSha1: commit.tree.sha, url: `https://api.github.com/repos/${guide.repository}/git/commits/${guide.revision}`,
        metadataFile, bytes: commitBytes.length, sha256: digest('sha256', commitBytes) },
      files: contents.map(result => result.value) };
    }));
    const rejected = results.filter(result => result.status === 'rejected');
    if (rejected.length) throw new AggregateError(rejected.map(result => result.reason), 'foundation provenance refresh failed');
    audit.guides = results.map(result => result.value);
  } else {
    for (const guide of audit.guides ?? []) {
      const metadataFile = guide.commitVerification?.metadataFile;
      const files = (guide.files ?? []).filter(file => file.sourceFile === sourcePath(guide.repository, guide.revision, file.path)).map(file => file.sourceFile);
      if (metadataFile === sourcePath(guide.repository, guide.revision, 'github-commit-response.json')) files.push(metadataFile);
      for (const file of files) {
        try { snapshots.set(file, await readFile(join(root, file))); } catch (error) {
          if (error.code !== 'ENOENT') throw error;
        }
      }
    }
  }
  const problems = checkAssociativeStackSources(audit, snapshots);
  if (problems.length) throw new Error(`Associative foundation provenance failed:\n- ${problems.join('\n- ')}`);
  if (args.includes('--refresh')) {
    for (const [file, bytes] of snapshots) {
      await mkdir(dirname(join(root, file)), { recursive: true });
      await writeFile(join(root, file), bytes);
    }
    writeProjectData(join(root, auditFile), audit);
  }
  console.log(`Associative foundation source provenance: ${audit.guides.length} immutable revisions, ${snapshots.size} exact snapshots; execution evidence remains separate`);
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) await main(process.argv.slice(2));
