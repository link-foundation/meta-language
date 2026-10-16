// Translate the entire array decision library; only shared library style is
// applied after checked JavaScript emission. No native algorithm is supplied.
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { simplifySourceTextRustParentheses } from './source-text-byte-library-style.mjs';
import { simplifyRustBranches } from './reviewed-rust-branch-styles.mjs';

const input = new URL('../src/translation/array-value-rules.js', import.meta.url);
const source = readFileSync(input, 'utf8');
const policy = {
  schemaVersion: 1,
  source: 'js/src/translation/array-value-rules.js',
  sourceSha256: createHash('sha256').update(source).digest('hex'),
  decisions: ['selectOptionalArrayRead', 'selectOptionalArrayIndexNumber', 'readArrayIterableForm', 'readReferenceMutationDiagnostic', 'renderOptionalArrayRead', 'renderStringPointIteration', 'renderCollectionIteration'],
  observableAbsence: 'Unproved JavaScript array reads produce an undefined optional value; actual compiler-generated iterable reads retain their bounds proof.',
  targets: ['JavaScript', 'Rust', 'Lean', 'Rocq'],
};
const policyFile = new URL('../src/translation/data/array-values.lino', import.meta.url);
const policyCopy = new URL('../../rust/src/data/array-values.lino', import.meta.url);
const check = process.argv.includes('--check');
if (check) {
  checkProjectData(policyFile, policy);
  assert.ok(readFileSync(policyCopy).equals(readFileSync(policyFile)), 'both packages share the array decision policy');
} else {
  writeProjectData(policyFile, policy);
  writeFileSync(policyCopy, readFileSync(policyFile));
}
if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const translated = emitRust(checkProgram(parseJavaScript(source)));
  assert.equal(translated.definitions.length, policy.decisions.length, 'all complete array definitions are emitted');
  const raw = translated.definitions.join('\n\n');
  const actions = [];
  for (const name of ['language', 'kind', 'element', 'target', 'values', 'position', 'field', 'value', 'operation', 'key_type', 'value_type']) {
    if (raw.includes(name + ': String')) actions.push({ op: 'replace', field: 'source', from: name + ': String', to: name + ': &str' });
    if (raw.includes(name + '.clone()')) actions.push({ op: 'replace', field: 'source', from: name + '.clone()', to: name + '.to_owned()' });
  }
  if (raw.includes('String::from("")')) actions.push({ op: 'replace', field: 'source', from: 'String::from("")', to: 'String::new()' });
  for (const match of raw.matchAll(/^pub fn (\w+)/gmu)) {
    actions.push({ op: 'replace', field: 'source', from: 'pub fn ' + match[1] + '(', to: '/// Read the shared array value decision.\n#[must_use]\npub fn ' + match[1] + '(' });
  }
  const initial = new DecoratorSet([{ id: 'array-value-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  const adapted = initial.decorate('emitter', { format: 'Rust', source: raw }).source;
  const simplified = simplifyRustBranches(simplifySourceTextRustParentheses(adapted));
  if (simplified !== adapted) actions.push({ op: 'replace', field: 'source', from: adapted, to: simplified });
  const styles = new DecoratorSet([{ id: 'array-value-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
  const styleFile = new URL('../../parity/self-translation/array-value-rules-decorators.lino', import.meta.url);
  if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'array library style remains derived from actual emitted definitions');
  else writeFileSync(styleFile, styles.toLino());
  const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const text = '// Generated from the checked JavaScript array value decisions. Do not edit.\n' + formatted.stdout;
  const output = new URL('../../rust/src/translation/array_value_rules.rs', import.meta.url);
  if (check) assert.equal(readFileSync(output, 'utf8'), text, 'native array value rules are fresh');
  else writeFileSync(output, text);
}
console.log('Array values: shared optional read admission and complete target rendering');
