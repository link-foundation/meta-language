// Capture the complete original compiler feedback; no suggestion applicability
// is inferred from rendered diagnostics.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {writeProjectData} from './project-data.mjs';

const revision = '1d25066a857b16e58ed954b0cfd3a4b63c3ef902';
const digest = value => createHash('sha256').update(value).digest('hex');

export function generateDeclarationDependencyBranchFixture(logFile) {
  const diagnostic = readFileSync(logFile);
  assert.equal(diagnostic.length, 194737);
  assert.equal(digest(diagnostic), '16324ac2e41aa561276ed4fe18c1a21848d7eebd1d48490ea85a2e716d3a53f8');
  const files = [2, 5].map(part => {
    const path = 'rust/src/translation/declaration_dependency_rules/part_' + part + '.rs';
    const {blob, certificate} = captureGitArtifactCertificate(revision, path);
    return {path, source: blob.toString('utf8'), bytes: blob.length, sha256: digest(blob), certificate};
  });
  const fixture = {
    schemaVersion: 1, model: 'reviewed declaration dependency branch styles 1', revision,
    diagnostic: {
      run: 39971598320, job: 120499721035,
      url: 'https://github.com/link-foundation/meta-language/actions/runs/39971598320/job/120499721035',
      kind: 'reviewed rendered diagnostics, not machine-applicable suggestions',
      source: diagnostic.toString('utf8'), bytes: diagnostic.length, sha256: digest(diagnostic),
    },
    files,
    corrections: [
      {part: 2, function: 'accept_recursive_declaration_owned_publication', lint: 'clippy::if_not_else', operation: 'swap complete branches and remove one whole Boolean negation'},
      {part: 5, function: 'ml_read_declaration_deferred_binding_loop61', lint: 'clippy::collapsible_else_if', operation: 'flatten a sole nested if without moving any binding'},
    ],
    boundary: 'Exact original Git-certified sources and complete rendered CI feedback. Corrections preserve the complete ordered floating-point condition and both branch bodies. This is a review input, not successful execution evidence.',
  };
  writeProjectData(new URL('../../parity/fixtures/ci-39971598320-declaration-branches.lino', import.meta.url), fixture);
  return fixture;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.equal(process.argv.length, 3, 'supply the complete original CI lint log');
  generateDeclarationDependencyBranchFixture(process.argv[2]);
}
