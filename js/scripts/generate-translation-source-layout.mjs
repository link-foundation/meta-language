// Retain complete original sources and prove mechanical module/test moves.
// Formatting is textual; this author never invokes a Rust compiler.
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {readFileSync} from 'node:fs';
import {fileURLToPath} from 'node:url';
import {captureGitArtifactCertificate} from './capture-git-artifact-certificate.mjs';
import {verifyGitArtifactCertificate} from './git-object-certificate.mjs';
import {checkProjectData, readProjectData, writeProjectData} from './project-data.mjs';
import {formatProductRuleSource} from './product-rule-style-corrections.mjs';

const revision = '1d25066a857b16e58ed954b0cfd3a4b63c3ef902';
const root = new URL('../../', import.meta.url);
const artifact = new URL('parity/self-translation/translation-source-layout.lino', root);
const paths = [
  'rust/src/self_translation.rs',
  'rust/src/translation/rust/expressions.rs',
  'rust/tests/unit/self_translation_recursive_declaration_groups.rs',
  'rust/src/self_translation/binding/deferred_scope.rs',
  'rust/src/self_translation/binding/owned_recursive_emission.rs',
];
const pin = source => ({bytes: Buffer.byteLength(source), sha256: createHash('sha256').update(source).digest('hex')});
const current = path => readFileSync(new URL(path, root), 'utf8');

function readOriginalTestModule(source) {
  const marker = '#[cfg(test)]\nmod tests {\n';
  assert.equal(source.split(marker).length, 2);
  assert.ok(source.endsWith('}\n'));
  return source.slice(source.indexOf(marker) + marker.length, -2)
    .split('\n').map(line => line.startsWith('    ') ? line.slice(4) : line).join('\n');
}

export function generateTranslationSourceLayout({check = false} = {}) {
  const archives = check ? readProjectData(artifact).archives : paths.map(path => {
    const {blob, certificate} = captureGitArtifactCertificate(revision, path);
    return {path, source: blob.toString('utf8'), ...pin(blob), certificate};
  });
  assert.deepEqual(archives.map(row => row.path), paths);
  for (const row of archives) {
    assert.deepEqual(pin(row.source), {bytes: row.bytes, sha256: row.sha256});
    verifyGitArtifactCertificate(row.certificate, Buffer.from(row.source), {revision, path: row.path});
  }
  const original = path => archives.find(row => row.path === path).source;
  const moves = [];
  function move(source, marker, target, normalize = value => value) {
    const before = original(source), at = before.indexOf(marker);
    assert.ok(at >= 0);
    assert.equal(before.indexOf(marker, at + marker.length), -1);
    const text = before.slice(at), expected = normalize(text), actual = current(target);
    assert.equal(formatProductRuleSource(actual), formatProductRuleSource(expected), 'every complete moved body, input and assertion survives in ' + target);
    moves.push({source, start: Buffer.byteLength(before.slice(0, at)), end: Buffer.byteLength(before), original: text, target, current: actual, ...pin(actual)});
  }
  move(paths[0], 'fn record_translated_definitions(', 'rust/src/self_translation/publication.rs', source => {
    for (const name of ['record_translated_definitions', 'carry', 'source_line', 'comment_fits']) {
      source = source.replace('fn ' + name + '(', 'pub(super) fn ' + name + '(');
    }
    return '//! Lossless publication markers and source provenance for checked items.\nuse super::*;\n\n' + source;
  });
  move(paths[1], '    /// A name, a call of a function or constructor, or a conversion', 'rust/src/translation/rust/expressions/paths.rs', source => {
    assert.ok(source.endsWith('}\n'));
    return '//! Qualified name, constructor, primitive and conversion expressions.\n'
      + 'use super::array_rules::accept_rust_source_array_owner;\n'
      + 'use super::indirection_rules::accept_rust_source_box_owner;\n'
      + 'use super::{\n    FLOAT, Flavor, Options, Result, RustParser, SExpr, SNode, SourceOrigin,\n'
      + '    Span, Token, Type, rust_fixed_type, span, type_error, unsupported,\n};\n\nimpl RustParser {\n' + source;
  });
  move(paths[2], '// Independent finite transitive closure', 'rust/tests/unit/self_translation_recursive_declaration_groups/graph_and_scope.rs',
    source => '//! Complete original graph and deferred-scope controls.\nuse super::*;\n\n' + source);

  let bindings = readOriginalTestModule(original(paths[3]));
  bindings = bindings.replace('use super::read_inert_binding;\n', '')
    .replaceAll('crate::read_structured_data_lino', 'meta_language::read_structured_data_lino')
    .replaceAll('"../../data/', '"../../src/data/');
  let owned = readOriginalTestModule(original(paths[4]));
  owned = owned.slice(owned.indexOf('#[test]'))
    .replaceAll('read_capture(&captured, &names).is_some()', 'accept_declaration_owned_emission(&captured, &names)')
    .replaceAll('read_capture(&captured, &names).is_none()', '!accept_declaration_owned_emission(&captured, &names)')
    .replace('let context = Context {\n        module: ModuleContext {\n            externals: provider.signatures,\n            ..ModuleContext::default()\n        },\n        ..Context::default()\n    };',
      'let mut context = Context::default();\n    context.module = ModuleContext {\n        externals: provider.signatures,\n        ..ModuleContext::default()\n    };');
  const target = 'rust/tests/unit/self_translation_source_ownership.rs';
  const expected = '//! Original inert-binding inputs and every name/owner assertion.\n'
    + 'use meta_language::read_inert_module_binding as read_inert_binding;\n' + bindings
    + '\nuse meta_language::{\n    DeclarationTranslationContext as Context, accept_declaration_owned_emission,\n'
    + '    check_and_emit_declaration_with_ownership as check_and_emit_item_with_ownership,\n};\n'
    + 'use meta_language::translation::javascript::ModuleContext;\nuse std::collections::HashMap;\n\n' + owned;
  assert.equal(formatProductRuleSource(current(target)), formatProductRuleSource(expected), 'all three original private tests run externally with the same production validators, inputs, fields and mutation');
  for (const source of paths.slice(3)) {
    assert.equal(current(source).includes('#[cfg(test)]'), false, 'private test modules move out of production sources');
  }
  const value = {
    schemaVersion: 1, model: 'complete translation source module relocation 1', revision, archives, moves,
    testPlacement: {sources: paths.slice(3), target, source: current(target), ...pin(current(target))},
    publicInspection: {
      context: 'DeclarationTranslationContext', item: 'CheckedDeclarationEmission',
      producer: 'check_and_emit_declaration_with_ownership', validator: 'accept_declaration_owned_emission',
      binding: 'read_inert_module_binding',
    },
    boundary: 'Original Git membership and complete source preservation under mechanical module boundaries and public inspection adapters. Every original test input and assertion remains. Actual native compilation, registration, lint and execution remain CI obligations; source preservation is not semantic equivalence or executed evidence.',
  };
  if (check) checkProjectData(artifact, value); else writeProjectData(artifact, value);
  return value;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  assert.ok(process.argv.slice(2).every(value => value === '--check'));
  generateTranslationSourceLayout({check: process.argv.includes('--check')});
}
