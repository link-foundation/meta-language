// Replay reviewed token/CST adapters. All pattern and owner decisions remain
// in the separately checked, automatically translated JavaScript rule module.
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData } from './project-data.mjs';
import { verifyGitArtifactCertificate } from './git-object-certificate.mjs';

const root = new URL('../../', import.meta.url);
const revision = '897bfc8b2dcc5ec561feffa455ddeb9f95d24335';
const predecessors = [
  'rust/src/translation/rust/expressions/control.rs',
  'rust/src/translation/rust/source_origins/type_owners.rs',
];
const additions = [
  'rust/src/translation/rust/expressions/control/literal_matches.rs',
  'rust/tests/unit/translation_rust_literal_matches.rs',
];
const registrations = [
  { path: 'rust/src/translation/mod.rs', declaration: 'pub(crate) mod rust_literal_match_rules;' },
  { path: 'rust/tests/unit/mod.rs', declaration: 'mod translation_rust_literal_matches;' },
];
const digest = source => createHash('sha256').update(source).digest('hex');

function checkSource(source, bytes, sha256) {
  assert.equal(typeof source, 'string');
  assert.equal(Buffer.from(source).toString('utf8'), source, 'lossless complete source encoding');
  assert.equal(Buffer.byteLength(source), bytes);
  assert.equal(digest(source), sha256);
}

/** Validate the full predecessor proofs and return every complete replayed file.
 * This verifier reads no repository or process state and never executes Rust. */
export function verifyRustLiteralMatchHosts(archive) {
  assert.equal(archive.schemaVersion, 1);
  assert.equal(archive.model, 'Rust borrowed string literal match host replay 1');
  assert.equal(archive.revision, revision);
  assert.equal(archive.coordinateUnit, 'UTF-16 code units');
  assert.deepEqual(archive.hosts.map(row => row.path), predecessors);
  assert.deepEqual(archive.additions.map(row => row.path), additions);
  assert.deepEqual(archive.registrations, registrations);
  const files = [];
  for (const row of archive.hosts) {
    assert.equal(typeof row.source, 'string');
    const original = Buffer.from(row.source);
    assert.equal(original.toString('utf8'), row.source);
    verifyGitArtifactCertificate(row.certificate, original, { revision, path: row.path });
    assert.ok(Array.isArray(row.edits) && row.edits.length > 0);
    let source = row.source;
    for (const edit of row.edits) {
      assert.ok(Number.isSafeInteger(edit.start) && edit.start >= 0 && edit.start <= source.length);
      assert.equal(typeof edit.removed, 'string');
      assert.equal(typeof edit.inserted, 'string');
      assert.equal(source.slice(edit.start, edit.start + edit.removed.length), edit.removed);
      source = source.slice(0, edit.start) + edit.inserted + source.slice(edit.start + edit.removed.length);
    }
    checkSource(source, row.currentBytes, row.currentSha256);
    let inverse = source;
    for (const edit of [...row.edits].reverse()) {
      assert.equal(inverse.slice(edit.start, edit.start + edit.inserted.length), edit.inserted);
      inverse = inverse.slice(0, edit.start) + edit.removed + inverse.slice(edit.start + edit.inserted.length);
    }
    assert.equal(inverse, row.source, 'every predecessor byte and old macro/catalog branch survives the inverse');
    files.push({ path: row.path, source, predecessor: row.source });
  }
  for (const row of archive.additions) {
    checkSource(row.source, row.bytes, row.sha256);
    files.push({ path: row.path, source: row.source });
  }
  return files;
}

/** Rebuild only the four recorded native adapter/test files, without Git or a compiler. */
export function generateRustLiteralMatchHosts({ check = false } = {}) {
  const archive = readProjectData(new URL('parity/self-translation/rust-literal-match-hosts.lino', root));
  const files = verifyRustLiteralMatchHosts(archive);
  // Root module ownership is separate: observe the exact registered paths,
  // rather than rewriting either shared module to install these adapters.
  for (const row of registrations) {
    const source = readFileSync(new URL(row.path, root), 'utf8');
    assert.equal(source.split(row.declaration).length, 2, row.path + ' retains the registered unit/module path');
  }
  // Validate every destination before the first write; refuse unrelated drift.
  for (const row of files) {
    const file = new URL(row.path, root);
    if (check) assert.equal(readFileSync(file, 'utf8'), row.source, row.path);
    else if (existsSync(file)) {
      const source = readFileSync(file, 'utf8');
      assert.ok(source === row.source || source === row.predecessor, 'unknown current source: ' + row.path);
    }
  }
  if (!check) for (const row of files) {
    const file = fileURLToPath(new URL(row.path, root));
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, row.source);
  }
  return { archive, files };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length <= 1 && args.every(argument => argument === '--check'));
  const result = generateRustLiteralMatchHosts({ check: args.includes('--check') });
  console.log(JSON.stringify({ predecessor: revision, hosts: result.files.map(row => row.path),
    registrations, nativeCompilation: false, nativeExecution: false }));
}
