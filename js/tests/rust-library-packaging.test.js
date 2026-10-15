import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, mkdtempSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { spawnSync } from 'node:child_process';
import { tokenize } from '../src/translation/lexer.js';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData, checkProjectData } from '../scripts/project-data.mjs';
import { readRustLibraryItems, readRustLibraryStyles, packageRustLibrary } from '../scripts/rust-library-packaging.mjs';
import { createTextCertificateLibrary, materializeTextCertificateLibrary } from '../../experiments/rust-text-certificate-library/generate.mjs';

const root = new URL('../../experiments/rust-text-certificate-library/', import.meta.url);
const modulePath = 'text_certificate_library';
const tokens = source => tokenize(source, 'Rust').tokens.map(token => [token.kind, token.raw ?? token.value]);
const styled = source => readRustLibraryStyles(source, { modulePath }).decorated.map(row => row.source).join('\n');

test('complete generated items retain their exact source order and every definition', () => {
  const original = readProjectData(new URL('../rust-text-certificate/data/translation.lino', root));
  const inventory = readRustLibraryItems(original.completeRust);
  assert.equal(inventory.items.map(row => row.source).join('') + inventory.trailing, original.completeRust);
  assert.equal(inventory.items.filter(row => row.kind === 'inner attribute').length, 1);
  assert.equal(inventory.items.filter(row => row.kind !== 'inner attribute').length, 96);
  assert.equal(original.definitions.length, 92);
  for (const source of original.definitions) assert.equal(inventory.items.filter(row => row.source.trim() === source.trim()).length, 1);
});

test('recorded Links decorators reproduce all candidate items without new lint allowances', () => {
  const { packet, decorators } = createTextCertificateLibrary();
  checkProjectData(new URL('./data/library.lino', root), packet);
  assert.equal(readFileSync(new URL('./data/decorators.lino', root), 'utf8'), decorators);
  assert.equal(DecoratorSet.fromLino(decorators).toLino(), decorators);
  assert.equal(packet.definitions.length, 92); assert.equal(packet.items.length, 96);
  assert.equal(packet.items.reduce((count, row) => count + row.flattenedBranches, 0), 143);
  assert.equal(packet.modules.slice(1).flatMap(row => row.itemIds).join(','), packet.items.map(row => row.id).join(','));
  assert.ok(packet.originalHeaders[0].source.includes('#![allow('));
  assert.ok(packet.modules.every(row => !row.source.includes('#![allow(')));
  assert.ok(packet.modules.every(row => row.rawLines < 900));
  assert.ok(packet.validationBoundary.includes('Native compilation and strict lint validation have not been executed for this library packet.'));
});

test('branch decorators remove only redundant tail blocks and retain literal/comment bytes', () => {
  const source = 'pub fn value(flag: bool) -> f64 {\n if flag { 1f64 } else { /* else { if */ if !flag { 2f64 } else { 3f64 } }\n}\n';
  const result = styled(source);
  assert.match(result, /else\s+\/\* else \{ if \*\/ if/u);
  assert.equal(tokenize(result, 'Rust').comments[0].text, '/* else { if */');
  assert.equal(tokens(source).length - tokens(result).length, 2);
  const literal = 'pub fn text() -> String { "else { if crate::value\n    keep".to_string() }';
  assert.deepEqual(tokens(styled(literal)), tokens(literal));
  assert.ok(styled(literal).includes('    keep'));
  // One tiny syntax-formatting probe, never a complete-kernel formatter loop.
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: result, encoding: 'utf8', timeout: 5000 });
  assert.ifError(formatted.error); assert.equal(formatted.status, 0, formatted.stderr);
  assert.deepEqual(tokens(formatted.stdout), tokens(result));
});

test('scope-bearing extras prevent redundant-block removal', () => {
  for (const body of [
    'if flag { 1f64 } else { 2f64 };',
    'if flag { 1f64 } else { 2f64 }\n3f64',
    'let local = 3f64; if flag { local } else { 2f64 }',
    "'scope: { if flag { break 'scope 1f64 } else { 2f64 } }",
    '#[allow(unused)] if flag { 1f64 } else { 2f64 }',
  ]) {
    const source = `pub fn value(flag: bool) -> f64 { if flag { 0f64 } else { ${body} } }`;
    assert.deepEqual(tokens(styled(source)), tokens(source));
  }
});

test('nested tail alternatives keep complete conditions and branch order', () => {
  const source = 'pub fn value(first: bool, second: bool) -> f64 { if first { 1f64 } else { /* outer */ if second { 2f64 } else { /* inner */ if first { 3f64 } else { 4f64 } } } }';
  const result = readRustLibraryStyles(source, { modulePath });
  assert.equal(result.observations[0].flattenedBranches, 2);
  const output = result.decorated[0].source;
  assert.deepEqual(tokens(output).filter(([, value]) => !['{', '}'].includes(value)), tokens(source).filter(([, value]) => !['{', '}'].includes(value)));
  assert.deepEqual(tokenize(output, 'Rust').comments.map(row => row.text), ['/* outer */', '/* inner */']);
  assert.equal(tokens(source).length - tokens(output).length, 4);
});

test('uninterpreted macro token trees and semantic inner attributes cannot be silently rewritten', () => {
  const source = 'pub fn value(flag: bool) -> f64 { custom!(if flag { 1f64 } else { if flag { 2f64 } else { 3f64 } }) }';
  assert.deepEqual(tokens(styled(source)), tokens(source));
  assert.throws(() => styled('pub fn value() -> f64 { custom!(crate::value()) }'), /unrepresented macro/u);
  assert.throws(() => styled('#![no_std]\npub fn value() -> f64 { 1f64 }'), /inner attribute/u);
  assert.throws(() => styled('#![cfg(example)]\npub fn value() -> f64 { 1f64 }'), /inner attribute/u);
  const vector = 'pub fn value() -> Vec<f64> { vec![crate::item()] }\npub fn item() -> f64 { 1f64 }';
  assert.throws(() => styled(vector), /unrepresented macro/u);
  const selected = readRustLibraryStyles(vector, { modulePath, namespaceExpressionMacros: ['vec'] });
  assert.ok(selected.decorated[0].source.includes('vec![crate::text_certificate_library::item()]'));
  assert.throws(() => readRustLibraryStyles('pub fn value() -> f64 { vec![custom!(crate::value())][0] }',
    { modulePath, namespaceExpressionMacros: ['vec'] }), /unrepresented macro/u);
});

test('namespace qualification changes actual known crate paths and preserves external paths and strings', () => {
  const source = 'pub fn item() -> f64 { 1f64 }\npub fn caller() -> f64 { crate::item() + crate::external::item() }\npub fn text() -> String { "crate::item()".to_string() }\n// crate::item()\n';
  const result = styled(source);
  assert.ok(result.includes('crate::text_certificate_library::item()'));
  assert.ok(result.includes('crate::external::item()'));
  assert.ok(result.includes('"crate::item()"'));
  // Trailing trivia is retained separately rather than silently attributed to an item.
  assert.equal(readRustLibraryItems(source).trailing, '\n// crate::item()\n');
  assert.ok(!result.includes('crate::text_certificate_library::external'));
});

test('module paths reject every excluded registered Rust name in every segment', () => {
  const names = readProjectData(new URL('../src/translation/data/rust-pure-unit-calls.lino', import.meta.url));
  const source = 'pub fn item() -> f64 { 1f64 }\npub fn caller() -> f64 { crate::item() }';
  for (const segment of names.forbiddenNames) {
    for (const path of [segment, `outer::${segment}`, `${segment}::inner`]) {
      assert.throws(() => readRustLibraryStyles(source, { modulePath: path }), /explicit module path/u, path);
    }
  }
  // The existing policy also reserves generated helper prefixes and main.
  assert.throws(() => readRustLibraryStyles(source, { modulePath: 'outer::ml_reserved' }), /explicit module path/u);
  const accepted = readRustLibraryStyles(source, { modulePath: 'translation::text_certificate_library' });
  assert.ok(accepted.decorated[1].source.includes('crate::translation::text_certificate_library::item()'));
});

test('whole-item partitioning rejects oversized items instead of trimming or splitting algorithms', () => {
  const source = Array.from({ length: 9 }, (_, index) => `pub fn item_${index}() -> f64 {\n ${index}f64\n}\n`).join('');
  const result = packageRustLibrary(source, { modulePath, maximumLines: 20 });
  assert.ok(result.modules.length > 2);
  assert.equal(result.modules.slice(1).flatMap(row => row.itemIds).length, 9);
  assert.ok(result.modules.every(row => row.source.split('\n').length < 20));
  const oversized = `pub fn value() -> f64 {${'\n'.repeat(30)}1f64}`;
  assert.throws(() => packageRustLibrary(oversized, { modulePath, maximumLines: 20 }), /whole-item line budget/u);
  assert.throws(() => packageRustLibrary(source, { modulePath: '../escape' }), /explicit module path/u);
  assert.throws(() => readRustLibraryItems('pub fn value()'), /complete boundary/u);
  assert.throws(() => readRustLibraryItems('pub fn value() {'), /unbalanced/u);
});

test('materialization emits the exact complete module bytes and validates every destination first', () => {
  const { packet } = createTextCertificateLibrary(), directory = mkdtempSync(join(tmpdir(), 'text-certificate-library-'));
  try {
    materializeTextCertificateLibrary(packet, directory);
    for (const row of packet.modules) assert.equal(readFileSync(join(directory, row.file), 'utf8'), row.source);
    const counterfeit = structuredClone(packet); counterfeit.modules.at(-1).file = '../escape.rs';
    assert.throws(() => materializeTextCertificateLibrary(counterfeit, directory), /invalid name or source digest/u);
    counterfeit.modules.at(-1).file = packet.modules.at(-1).file; counterfeit.modules.at(-1).source += ' ';
    assert.throws(() => materializeTextCertificateLibrary(counterfeit, directory), /invalid name or source digest/u);
  } finally { rmSync(directory, { recursive: true, force: true }); }
});
