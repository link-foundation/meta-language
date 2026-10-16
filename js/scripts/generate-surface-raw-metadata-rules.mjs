// Translate the shared raw metadata decisions, retaining nullable field
// presence in the host adapter rather than manufacturing source semantics.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { simplifySourceTextRustParentheses } from './source-text-byte-library-style.mjs';

const policy = {
  schemaVersion: 1,
  model: 'raw surface nullable Boolean metadata 1',
  fields: [
    { context: 'expression', field: 'parenthesized' },
    { context: 'program', field: 'referenceSemantics' },
    { context: 'index', field: 'optional' },
    { context: 'index', field: 'provenInBounds' },
  ],
  presentForms: ['boolean', 'null'],
  sharedReferenceKinds: ['array', 'record', 'map', 'set'],
  absence: 'An absent raw field remains absent; false, null and true are distinct present values.',
  checkedProgram: 'The checker selects referenceSemantics true when the raw program flag is true; false and null retain the existing unselected checked representation.',
  scope: 'Raw wire metadata and existing JavaScript representation selection only. This does not port closure, class, heap, structural identity analysis or arbitrary frontend behavior.',
};
const policyFile = new URL('../src/translation/data/surface-raw-metadata.lino', import.meta.url);
const policyCopy = new URL('../../rust/src/data/surface-raw-metadata.lino', import.meta.url);
const check = process.argv.includes('--check');
if (check) {
  checkProjectData(policyFile, policy);
  assert.ok(readFileSync(policyCopy).equals(readFileSync(policyFile)), 'both runtimes share exact raw metadata policy bytes');
} else {
  writeProjectData(policyFile, policy);
  writeFileSync(policyCopy, readFileSync(policyFile));
}

const input = new URL('../src/translation/surface-raw-metadata-rules.js', import.meta.url);
const source = readFileSync(input, 'utf8');
const { acceptSurfaceRawMetadataField, acceptSurfaceRawMetadataForm, acceptSharedSurfaceReferenceKind } = await import(input);
for (const { context, field } of policy.fields) assert.ok(acceptSurfaceRawMetadataField(context, field));
for (const form of policy.presentForms) assert.ok(acceptSurfaceRawMetadataForm(form));
for (const kind of policy.sharedReferenceKinds) assert.ok(acceptSharedSurfaceReferenceKind(kind));

if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const definitions = emitRust(checkProgram(parseJavaScript(source)), { arrayConsumption: true }).definitions;
  assert.equal(definitions.length, 4, 'every complete checked metadata declaration is translated');
  const raw = definitions.join('\n\n');
  const actions = [];
  for (const name of ['context', 'field', 'form', 'kind']) {
    if (raw.includes(name + ': String')) actions.push({ op: 'replace', field: 'source', from: name + ': String', to: name + ': &str' });
    if (raw.includes(name + '.clone()')) actions.push({ op: 'replace', field: 'source', from: name + '.clone()', to: name });
  }
  for (const literal of new Set([...raw.matchAll(/String::from\("([^"\\]*)"\)/gu)].map(match => match[0]))) {
    actions.push({ op: 'replace', field: 'source', from: literal, to: literal.slice('String::from('.length, -1) });
  }
  for (const name of ['accept_surface_raw_metadata_field', 'accept_surface_raw_metadata_form', 'accept_shared_surface_reference_kind', 'select_surface_reference_semantics']) {
    assert.ok(raw.includes('pub fn ' + name + '('), 'each metadata decision retains its checked name');
    actions.push({ op: 'replace', field: 'source', from: 'pub fn ' + name + '(', to: '/// Read a shared raw surface metadata decision.\n#[must_use]\npub fn ' + name + '(' });
  }
  const styleFile = new URL('../../parity/self-translation/surface-raw-metadata-rules-decorators.lino', import.meta.url);
  const initial = new DecoratorSet([{ id: 'surface-raw-metadata-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  const adapted = initial.decorate('emitter', { format: 'Rust', source: raw }).source;
  const grouped = simplifySourceTextRustParentheses(adapted);
  if (grouped !== adapted) actions.push({ op: 'replace', field: 'source', from: adapted, to: grouped });
  const styles = new DecoratorSet([{ id: 'surface-raw-metadata-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'raw metadata library styles are fresh');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const native = '// Generated from the checked JavaScript raw surface metadata decisions. Do not edit.\n' + formatted.stdout;
  const nativeFile = new URL('../../rust/src/translation/surface_raw_metadata_rules.rs', import.meta.url);
  if (check) assert.equal(readFileSync(nativeFile, 'utf8'), native, 'native raw metadata decisions are fresh');
  else writeFileSync(nativeFile, native);
}
console.log('Raw surface metadata: shared field, value and reference-kind decisions');
