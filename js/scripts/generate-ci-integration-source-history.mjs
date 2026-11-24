import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';

import { readBootstrapStructuredDataLino } from '../../scripts/lib/structured-data-bootstrap.mjs';
import { captureGitArtifactCertificate } from './capture-git-artifact-certificate.mjs';
import { ciIntegrationHistoryPath, ciIntegrationNamingSources, ciIntegrationParents } from './ci-integration-source-history.mjs';
import { checkProjectData, writeProjectData } from './project-data.mjs';

const root = new URL('../../', import.meta.url);
const digest = bytes => createHash('sha256').update(bytes).digest('hex');
const histories = {
  dependencies: 'parity/self-translation/dependency-workflow-source-history.lino',
  translationPause: 'parity/self-translation/automated-translation-pause-source-history.lino',
};
const integratedSources = {
  dependencies: [
    'js/scripts/program-string-host-registration-history.mjs',
    'parity/self-translation/ascii-text-predicate-rules-native.lino',
    'rust/src/formal_concept_definition.rs',
    'rust/src/shared_numeric_function_model.rs',
    'rust/src/translation/rust/expressions.rs',
    ...Object.keys(ciIntegrationNamingSources),
  ],
  translationPause: [
    'experiments/grammar-context-source-metadata/generate.mjs',
    'js/scripts/array-relative-element-history.mjs',
    'js/scripts/grammar-context-metadata-history.mjs',
    'js/scripts/test-groups.mjs',
    'js/tests/ci-workflow-structure.test.js',
  ],
};
const commits = {};
const trees = {};
const sources = [];
for (const [parent, archive] of Object.entries(histories)) {
  const rows = readBootstrapStructuredDataLino(readFileSync(new URL(archive, root), 'utf8')).sources;
  const additionalFiles = new Set(integratedSources[parent]);
  const files = new Set([...rows.map(row => row.file), ...additionalFiles]);
  const revision = ciIntegrationParents[parent];
  for (const file of files) {
    const actual = readFileSync(new URL(file, root));
    const current = digest(actual);
    if (!additionalFiles.has(file) && rows.some(row => row.file === file && (row.current ?? row.currentDigest) === current)) continue;
    const { blob, certificate } = captureGitArtifactCertificate(revision, file);
    if (blob.equals(actual)) continue;
    const { commitText, trees: ancestry, ...proof } = certificate;
    commits[revision] ??= commitText;
    assert.equal(commits[revision], commitText);
    proof.trees = ancestry.map(tree => {
      trees[tree.objectId] ??= tree;
      return tree.objectId;
    });
    sources.push({ parent, file, current, bytes: actual.length, original: blob.toString('utf8'), certificate: proof });
  }
}

const data = { parents: ciIntegrationParents, commits, trees, sources };
const archive = new URL(ciIntegrationHistoryPath, root);
const source = new URL('ci-integration-source-history.mjs', import.meta.url);
const check = process.argv.includes('--check');
assert.ok(process.argv.slice(2).every(argument => argument === '--check'));
if (check) checkProjectData(archive, data);
else writeProjectData(archive, data);
const actual = readFileSync(source, 'utf8');
const updated = actual.replace(/const archiveDigest = '(?:PENDING|[\da-f]{64})';/u,
  `const archiveDigest = '${digest(readFileSync(archive))}';`);
if (check) assert.equal(actual, updated);
else if (actual !== updated) writeFileSync(source, updated);
