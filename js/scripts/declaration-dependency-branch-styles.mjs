// Additional reviewed compiler feedback after the original family styles.
// Keep the original producer, diagnostic fixture and every predecessor exact.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {tokenize} from '../src/translation/lexer.js';
import {readProjectData} from './project-data.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';
import {acceptRustWholeConditionNegation, simplifyRustBranches} from './reviewed-rust-branch-styles.mjs';
import {flattenFamilyLibraryElseIf} from './portable-conditional-source-style.mjs';

const digest = value => createHash('sha256').update(value).digest('hex');

export function readDeclarationDependencyBranchStyles(source, family) {
  if (family !== 'declaration_dependency_rules') return {source, actions: []};
  const fixture = readProjectData(new URL('../../parity/fixtures/ci-39971598320-declaration-branches.lino', import.meta.url));
  assert.equal(fixture.model, 'reviewed declaration dependency branch styles 1');
  assert.equal(fixture.revision, '1d25066a857b16e58ed954b0cfd3a4b63c3ef902');
  assert.equal(Buffer.byteLength(fixture.diagnostic.source), 194737);
  assert.equal(digest(fixture.diagnostic.source), '16324ac2e41aa561276ed4fe18c1a21848d7eebd1d48490ea85a2e716d3a53f8');
  assert.equal(fixture.files.length, 2);
  assert.equal(fixture.corrections.length, 2);
  const actions = [];
  for (const correction of fixture.corrections) {
    const path = 'rust/src/translation/declaration_dependency_rules/part_' + correction.part + '.rs';
    const original = fixture.files.find(file => file.path === path);
    assert.ok(original);
    const bytes = Buffer.from(original.source);
    assert.equal(bytes.length, original.bytes);
    assert.equal(digest(bytes), original.sha256);
    verifyGitArtifactCertificate(original.certificate, bytes, {revision: fixture.revision, path});
    const from = readSourceTextRustBlock(source, 'fn', correction.function);
    const certified = readSourceTextRustBlock(original.source, 'fn', correction.function);
    assert.equal(formatProductRuleSource(from), formatProductRuleSource(certified), 'the complete owning function retains its certified predecessor');
    let to;
    if (correction.function === 'accept_recursive_declaration_owned_publication') {
      assert.equal(correction.lint, 'clippy::if_not_else');
      const {tokens} = tokenize(from, 'Rust');
      const branches = tokens.filter(token => token.kind === 'identifier' && token.value === 'if');
      assert.equal(branches.length, 1, 'only the complete outer Boolean branch is inverted');
      const at = tokens.indexOf(branches[0]);
      const open = tokens.findIndex((token, index) => index > at && token.kind === 'punct' && token.value === '{');
      assert.ok(open > at);
      const condition = from.slice(tokens[at + 1].start, tokens[open].start).trim();
      assert.ok(acceptRustWholeConditionNegation(condition), 'a whole negation preserves every NaN and ordered comparison');
      to = simplifyRustBranches(from);
    } else {
      assert.equal(correction.function, 'ml_read_declaration_deferred_binding_loop61');
      assert.equal(correction.lint, 'clippy::collapsible_else_if');
      to = flattenFamilyLibraryElseIf(from);
    }
    assert.notEqual(to, from, 'each recorded diagnostic has a concrete mechanical correction');
    assert.equal(source.split(from).length, 2, 'a complete owning declaration occurs exactly once');
    const action = {op: 'replace', field: 'source', from, to};
    actions.push(action);
    source = source.replace(from, to);
  }
  return {source, actions};
}
