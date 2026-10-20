// Normal complete source replay: no Git, subprocess, compiler or formatter.
import assert from 'node:assert/strict';
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { readProjectData } from './project-data.mjs';
import { ASCII_HOST_REGISTRATIONS, asciiHostPin, verifyAsciiTextPredicateHosts } from './ascii-text-predicate-host-history.mjs';

const root = new URL('../../', import.meta.url);

export function generateAsciiTextPredicateHosts({ check = false } = {}) {
  const archive = readProjectData(new URL('parity/self-translation/ascii-text-predicate-hosts.lino', root));
  const files = verifyAsciiTextPredicateHosts(archive);
  const artifact = readFileSync(new URL(archive.sourceArtifact.path, root));
  assert.deepEqual(asciiHostPin(artifact), archive.sourceArtifact.pin, 'the exact separately generated source artifact supplies this ABI');
  assert.deepEqual(readProjectData(new URL(archive.sourceArtifact.path, root)).libraryInterface, archive.sourceArtifact.libraryInterface);
  assert.equal(readFileSync(new URL(archive.assertionCorrection.path, root), 'utf8'), archive.assertionCorrection.current.source,
    'the exact complete current test retains the documented finite correction');
  for (const row of ASCII_HOST_REGISTRATIONS) {
    assert.equal(readFileSync(new URL(row.path, root), 'utf8').split(row.declaration).length, 2,
      row.path + ' actually registers the generated module and native unit source');
  }
  for (const row of files) {
    const file = new URL(row.path, root);
    if (check) assert.equal(readFileSync(file, 'utf8'), row.source, row.path);
    else if (existsSync(file)) {
      const actual = readFileSync(file, 'utf8');
      assert.ok(actual === row.source || actual === row.predecessor, 'unknown current source must not be overwritten: ' + row.path);
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
  const result = generateAsciiTextPredicateHosts({ check: args.includes('--check') });
  console.log(JSON.stringify({ hosts: result.files.map(row => row.path),
    registrations: ASCII_HOST_REGISTRATIONS, nativeCompilation: false, nativeExecution: false }));
}
