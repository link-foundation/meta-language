// Author the finite source receipt from independently pinned complete Git objects.
// This captures source/provenance only; it does not execute native code.
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { checkProjectData, writeProjectData, readProjectData } from './project-data.mjs';
import { ASCII_HOST_REVISION, ASCII_HOST_ADDITIONS, ASCII_HOST_REGISTRATIONS,
  ASCII_HOST_EDITS, ASCII_ASSERTION_EDITS, asciiHostPin, replayAsciiHostEdits, verifyAsciiTextPredicateHosts } from './ascii-text-predicate-host-history.mjs';

const root = new URL('../../', import.meta.url);

export function generateAsciiTextPredicateHostHistory({ check = false, assertionDraft = null } = {}) {
  const output = new URL('parity/self-translation/ascii-text-predicate-hosts.lino', root);
  const draftSource = assertionDraft === null
    ? (assert.ok(existsSync(output), 'first capture requires the retained exact unexecuted draft via --assertion-draft'),
      readProjectData(output).assertionCorrection.draft.source)
    : readFileSync(assertionDraft, 'utf8');
  const assertionPath = 'js/tests/translation-ascii-text-predicates.test.js';
  const currentAssertionSource = readFileSync(new URL(assertionPath, root), 'utf8');
  const assertionReplay = replayAsciiHostEdits(draftSource, ASCII_ASSERTION_EDITS);
  assert.equal(currentAssertionSource, assertionReplay.source, 'only the finite documented incomplete-source correction and added String spread control');
  const assertionCorrection = { path: assertionPath,
    draft: { kind: 'unexecuted private draft', commit: null, source: draftSource, pin: asciiHostPin(draftSource) },
    current: { source: currentAssertionSource, pin: asciiHostPin(currentAssertionSource) }, edits: assertionReplay.edits,
    nativeTest: 'rust/tests/unit/translation_ascii_text_predicates.rs',
    nativeAssertion: 'both_actual_whole_helpers_preserve_sources_cst_native_execution_and_bidirectional_semantic_archives',
    reason: 'The old new-test draft treated emitted function definitions without their actual generated runtime/prelude/source context as a complete default Rust source interpretation. The actual input retains its exact unsupported move-closure diagnostic and span; the other helper still reparses. All original scalar inputs and independent truth observations survive, now including execution of the complete reverse JavaScript program and full emitted Rust CST through both codecs. Native CI executes complete original/emitted/reverse Rust and JavaScript programs on every input and both semantic archives. Complete runtime semantic reimport remains an unmet translation obligation; this correction does not claim it solved.' };
  const hosts = ASCII_HOST_EDITS.map(specification => {
    const { blob, certificate } = captureGitArtifactCertificate(ASCII_HOST_REVISION, specification.path);
    const source = blob.toString('utf8');
    assert.deepEqual(Buffer.from(source), blob);
    const replay = replayAsciiHostEdits(source, specification.changes);
    assert.equal(readFileSync(new URL(specification.path, root), 'utf8'), replay.source,
      'capture the exact finite reviewed successor, without unrelated source drift');
    return { path: specification.path, source, certificate, current: asciiHostPin(replay.source), edits: replay.edits };
  });
  const additions = ASCII_HOST_ADDITIONS.map(path => {
    const source = readFileSync(new URL(path, root), 'utf8');
    return { path, source, pin: asciiHostPin(source) };
  });
  const artifactPath = 'parity/self-translation/ascii-text-predicate-rules-native.lino';
  const artifactBytes = readFileSync(new URL(artifactPath, root));
  const artifact = readProjectData(new URL(artifactPath, root));
  const archive = { schemaVersion: 1, model: 'Rust ASCII character query host replay 1',
    revision: ASCII_HOST_REVISION, coordinateUnit: 'UTF-16 code units', hosts, additions,
    registrations: ASCII_HOST_REGISTRATIONS, assertionCorrection,
    sourceArtifact: { path: artifactPath, pin: asciiHostPin(artifactBytes), libraryInterface: artifact.libraryInterface },
    boundary: 'Only declared frontend transport edits and complete new adapter/test sources. All predicate/query/callback decisions come from the separately checked complete JavaScript closure and its actual generated Text ABI. Complete predecessor inverses and native registration are checked independently; source capture is not compilation or runtime evidence.' };
  verifyAsciiTextPredicateHosts(archive);
  if (check) checkProjectData(output, archive);
  else writeProjectData(output, archive);
  return archive;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  let check = false, assertionDraft = null;
  for (let index = 0; index < args.length; index += 1) {
    if (args[index] === '--check') { assert.equal(check, false); check = true; }
    else if (args[index] === '--assertion-draft') {
      assert.equal(assertionDraft, null); assertionDraft = args[++index]; assert.equal(typeof assertionDraft, 'string');
    } else assert.fail('unknown argument ' + args[index]);
  }
  const archive = generateAsciiTextPredicateHostHistory({ check, assertionDraft });
  console.log(JSON.stringify({ predecessor: archive.revision, hosts: archive.hosts.map(row => row.path),
    additions: archive.additions.map(row => row.path), nativeCompilation: false, nativeExecution: false }));
}
