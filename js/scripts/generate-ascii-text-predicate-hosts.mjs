// Normal complete source replay: no Git, subprocess, compiler or formatter.
import assert from 'node:assert/strict';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData, serializeProjectData } from './project-data.mjs';
import { ASCII_HOST_REGISTRATIONS, asciiHostPin, verifyAsciiTextPredicateHosts } from './ascii-text-predicate-host-history.mjs';
import {restoreRequiredFieldNativeHostSource} from './required-field-native-host-succession.mjs';

const root = new URL('../../', import.meta.url);

/** Read the current artifact through the exact source view used by the host archive. */
export function readAsciiTextPredicateHostArtifact(path, expected) {
  const location = new URL(path, root);
  const artifact = readProjectData(location);
  const current = asciiHostPin(readFileSync(location));
  if (expected === undefined || current.sha256 === expected.sha256) {
    if (expected !== undefined) assert.deepEqual(current, expected);
    return { artifact, pin: current };
  }
  const receipt = new URL('parity/self-translation/ascii-native-artifact-source-history.lino', root);
  assert.equal(asciiHostPin(readFileSync(receipt)).sha256, '9a295c5d799925c806cbc7022273263c50564f5de7e510e9db5208f1b177ebb8');
  const history = readProjectData(receipt);
  assert.equal(history.schemaVersion, 1);
  assert.equal(history.commit, 'c29d2b626cd05c209c814cc78cf654668ed8d29c');
  assert.equal(history.path, path);
  assert.deepEqual(history.previous, expected);
  assert.deepEqual(current, history.current, 'the exact generated ASCII source artifact supplies the current ABI');
  const part = artifact.files.find(row => row.path === history.nativePath);
  assert.deepEqual(asciiHostPin(part.source), history.currentPart);
  let source = part.source;
  let end = source.length;
  for (const edit of history.edits.toReversed()) {
    assert.ok(Number.isSafeInteger(edit.start) && edit.start >= 0 && edit.start + edit.from.length <= end);
    assert.equal(source.slice(edit.start, edit.start + edit.from.length), edit.from);
    source = source.slice(0, edit.start) + edit.to + source.slice(edit.start + edit.from.length);
    end = edit.start;
  }
  assert.deepEqual(asciiHostPin(source), history.previousPart);
  Object.assign(part, { source, bytes: history.previousPart.bytes, sourceSha256: history.previousPart.sha256 });
  const pin = asciiHostPin(serializeProjectData(artifact, path));
  assert.deepEqual(pin, expected, 'every archived source-artifact byte remains recoverable');
  return { artifact, pin };
}

export function generateAsciiTextPredicateHosts({ check = false } = {}) {
  const archive = readProjectData(new URL('parity/self-translation/ascii-text-predicate-hosts.lino', root));
  const files = verifyAsciiTextPredicateHosts(archive);
  const { artifact } = readAsciiTextPredicateHostArtifact(archive.sourceArtifact.path, archive.sourceArtifact.pin);
  assert.deepEqual(artifact.libraryInterface, archive.sourceArtifact.libraryInterface);
  assert.equal(readFileSync(new URL(archive.assertionCorrection.path, root), 'utf8'), archive.assertionCorrection.current.source,
    'the exact complete current test retains the documented finite correction');
  for (const row of ASCII_HOST_REGISTRATIONS) {
    assert.equal(readFileSync(new URL(row.path, root), 'utf8').split(row.declaration).length, 2,
      row.path + ' actually registers the generated module and native unit source');
  }
  const protectedSources = new Set();
  for (const row of files) {
    const file = new URL(row.path, root);
    if (check) assert.equal(restoreRequiredFieldNativeHostSource(readFileSync(file, 'utf8'), row.path, 'ASCII character host'), row.source, row.path);
    else if (existsSync(file)) {
      const physical = readFileSync(file, 'utf8');
      const actual = restoreRequiredFieldNativeHostSource(physical, row.path, 'ASCII character host');
      assert.ok(actual === row.source || actual === row.predecessor, 'unknown current source must not be overwritten: ' + row.path);
      if (actual !== physical) protectedSources.add(row.path);
    }
  }
  if (!check) for (const row of files) {
    if (protectedSources.has(row.path)) continue;
    const file = fileURLToPath(new URL(row.path, root));
    mkdirSync(dirname(file), { recursive: true });
    writeFileSync(file, row.source);
  }
  return { archive, files };
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  assert.ok(args.length <= 1 && args.every(argument => argument === '--check'));
  const result = generateAsciiTextPredicateHosts({ check: args.includes('--check') });
  console.log(JSON.stringify({ hosts: result.files.map(row => row.path),
    registrations: ASCII_HOST_REGISTRATIONS, nativeCompilation: false, nativeExecution: false }));
}
