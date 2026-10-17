#!/usr/bin/env node
// Migrate repository-owned data without changing its decoded values. Historical
// protocol snapshots retain their original byte and Git object identities.
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, readdirSync, unlinkSync, writeFileSync } from 'node:fs';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData } from './project-data.mjs';
import { readStructuredDataLino, structuredDataToLino } from '../src/structured-data-links.js';

const root = resolve(dirname(fileURLToPath(import.meta.url)), '../..');
const recordFile = 'docs/case-studies/snapshot-provenance.lino';
const ownData = ['experiments/lark-parity/cases.json', 'rust/benches/corpus-manifest.json'];
const hash = (bytes) => createHash('sha256').update(bytes).digest('hex');
const sourceFiles = () => execFileSync('git', ['ls-files', '-z', 'docs/case-studies/**/*.json'],
  { cwd: root, encoding: 'utf8' }).split('\0').filter((file) => file && existsSync(join(root, file)));

function markdownFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const file = join(directory, entry.name);
    return entry.isDirectory() ? markdownFiles(file) : entry.name.endsWith('.md') ? [file] : [];
  });
}

function updateSnapshotLinks(records) {
  for (const file of markdownFiles(join(root, 'docs/case-studies'))) {
    const original = readFileSync(file, 'utf8');
    let text = original;
    const local = relative(root, file).replaceAll('\\', '/');
    for (const record of records) {
      if (!record.source.startsWith('docs/case-studies/')) continue;
      text = text.replaceAll(record.source, record.file);
      const owner = record.source.split('/').slice(0, 3).join('/') + '/';
      if (local.startsWith(owner)) {
        const name = record.source.split('/').at(-1);
        text = text.replaceAll(name, name.replace(/\.json$/u, '.lino'));
      }
    }
    if (text !== original) writeFileSync(file, text);
  }
}

export function migrateStandaloneProjectData() {
  const previous = existsSync(join(root, recordFile)) ? readProjectData(join(root, recordFile)) : { records: [] };
  const records = new Map(previous.records.map((record) => [record.source, record]));
  const revision = execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim();
  for (const source of [...ownData, ...sourceFiles()]) {
    if (!existsSync(join(root, source))) continue;
    const original = readFileSync(join(root, source));
    const value = JSON.parse(original.toString('utf8'));
    const file = source.replace(/\.json$/u, '.lino');
    if (existsSync(join(root, file))) throw new Error(`standalone data has competing storage: ${source}`);
    const links = structuredDataToLino(value);
    assert.deepEqual(readStructuredDataLino(links), value, `${source}: Links storage changes decoded data`);
    const sourceGitBlob = execFileSync('git', ['rev-parse', `${revision}:${source}`], { cwd: root, encoding: 'utf8' }).trim();
    const captured = execFileSync('git', ['cat-file', 'blob', sourceGitBlob], { cwd: root });
    assert.deepEqual(captured, original, `${source}: captured bytes differ from their Git provenance`);
    const record = { source, file, sourceRevision: revision, sourceGitBlob, sourceSha256: hash(original),
      sourceBytes: original.length, semanticSha256: hash(JSON.stringify(value)), linksSha256: hash(links) };
    writeFileSync(join(root, file), links);
    assert.deepEqual(readStructuredDataLino(readFileSync(join(root, file), 'utf8')), value);
    unlinkSync(join(root, source));
    records.set(source, record);
  }
  const document = { schemaVersion: 1, generatedBy: 'js/scripts/migrate-standalone-project-data.mjs',
    records: [...records.values()].sort((left, right) => left.source.localeCompare(right.source)) };
  writeFileSync(join(root, recordFile), structuredDataToLino(document));
  updateSnapshotLinks(document.records);
  return document;
}

export function checkStandaloneProjectData() {
  const document = readProjectData(join(root, recordFile));
  assert.equal(document.schemaVersion, 1);
  assert.ok(document.records.length >= 58, 'both owned records and all 56 historical snapshots remain');
  const sources = new Set();
  for (const record of document.records) {
    assert.equal(sources.has(record.source), false, `duplicate provenance ${record.source}`);
    sources.add(record.source);
    assert.equal(record.file, record.source.replace(/\.json$/u, '.lino'));
    assert.match(record.sourceGitBlob, /^[a-f0-9]{40}$/u);
    assert.match(record.sourceSha256, /^[a-f0-9]{64}$/u);
    assert.ok(Number.isSafeInteger(record.sourceBytes) && record.sourceBytes > 0);
    assert.equal(existsSync(join(root, record.source)), false, `${record.source} must not duplicate its linked storage`);
    const links = readFileSync(join(root, record.file), 'utf8');
    assert.equal(hash(links), record.linksSha256, `${record.file}: captured archive changed`);
    const value = readProjectData(join(root, record.source));
    assert.equal(hash(JSON.stringify(value)), record.semanticSha256, `${record.file}: decoded archive changed`);
  }
  return document.records.length;
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (!process.argv.includes('--check')) migrateStandaloneProjectData();
  console.log(`standalone project data: ${checkStandaloneProjectData()} linked archives retain their original values and provenance`);
}
