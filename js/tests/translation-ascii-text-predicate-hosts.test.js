import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { readProjectData } from '../scripts/project-data.mjs';
import { asciiHostPin, verifyAsciiTextPredicateHosts } from '../scripts/ascii-text-predicate-host-history.mjs';
import { generateAsciiTextPredicateHosts } from '../scripts/generate-ascii-text-predicate-hosts.mjs';
import { restoreNativeCompilerSource } from '../scripts/native-compiler-source-history.mjs';

test('complete ASCII native source hosts preserve certified predecessors actual ABI and registered test ownership', () => {
  const { archive, files } = generateAsciiTextPredicateHosts({ check: true });
  assert.equal(archive.hosts.length, 2);
  assert.equal(archive.additions.length, 2);
  assert.equal(files.length, 4);
  assert.equal(archive.registrations.find(row => row.path === 'rust/tests/unit/mod.rs').declaration,
    'mod translation_ascii_text_predicates;');
  const { path, pin } = archive.sourceArtifact;
  const current = readFileSync(new URL('../../' + path, import.meta.url), 'utf8');
  const original = restoreNativeCompilerSource(current, path);
  assert.notEqual(current, original);
  assert.deepEqual(asciiHostPin(original), pin);
  assert.equal(restoreNativeCompilerSource(original, path), original);
  assert.equal(restoreNativeCompilerSource(current + '\n', path), current + '\n');
});

test('unknown host edits source corruption and forged predecessor certificates cannot produce a reviewed source view', () => {
  const source = readProjectData(new URL('../../parity/self-translation/ascii-text-predicate-hosts.lino', import.meta.url));
  for (const mutate of [
    value => { value.hosts[0].source += '\n'; },
    value => { value.hosts[0].edits[0].inserted += '// unrelated successor\n'; },
    value => { value.hosts[0].certificate.commitText += '\n'; },
    value => { value.additions[0].source += '\n'; },
    value => { value.registrations[1].declaration = 'mod unrelated;'; },
    value => { value.sourceArtifact.libraryInterface = []; },
    value => { value.assertionCorrection.draft.commit = value.revision; },
    value => { value.assertionCorrection.draft.source += '\n'; },
    value => { value.assertionCorrection.current.source += '\n'; },
  ]) {
    const changed = structuredClone(source);
    mutate(changed);
    assert.throws(() => verifyAsciiTextPredicateHosts(changed));
  }
});
