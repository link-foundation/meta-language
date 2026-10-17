// Generate the complete portable reference analysis and its borrowed library
// ABI. Native semantic bodies always come from the checked JavaScript emitter.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { DecoratorSet } from '../src/decorators.js';
import { checkProjectData, writeProjectData } from './project-data.mjs';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { readSurfaceReferenceMutatingMethods } from '../src/translation/reference-analysis.js';
import { simplifySourceTextRustParentheses } from './source-text-byte-library-style.mjs';
import {readPortableSourceParameterStyles, readPortableSourceLintStyles, partitionPortableRustLibrary} from './portable-source-library-style.mjs';
import { readSurfaceReferenceSourceModule } from './surface-reference-source-module.mjs';

const check = process.argv.includes('--check');
const input = new URL('../src/translation/surface-reference-analysis-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/surface_reference_analysis_rules.rs', import.meta.url);
const policyFile = new URL('../src/translation/data/surface-reference-analysis.lino', import.meta.url);
const policyCopy = new URL('../../rust/src/data/surface-reference-analysis.lino', import.meta.url);
const styleFile = new URL('../../parity/self-translation/surface-reference-analysis-rules-decorators.lino', import.meta.url);
const policy = {
  schemaVersion: 1,
  model: 'surface source reference selection and lexical structural identity 1',
  factFields: ['forms', 'texts', 'starts', 'counts', 'labels', 'targets'],
  packedColumns: ['forms', 'texts', 'labels'],
  packedRows: ['starts', 'counts', 'targets'],
  packedNodeCount: 'forms.length',
  sourceModules: [
    ['js/src/translation/fact-column-rules.js', new URL('../src/translation/fact-column-rules.js', import.meta.url)],
    ['js/src/translation/surface-reference-analysis-rules.js', input],
  ].map(([file, source]) => ({file, sha256: createHash('sha256').update(readFileSync(source)).digest('hex')})),
  mutatingMethods: readSurfaceReferenceMutatingMethods(),
  sourceOperations: 'js/src/data/reference-operations.lino',
  originalAnalysisSha256: '14490e98696240c8441f3dfc447e7654deedbc25427fad4d69a7224465692ac4',
  typeResults: { absent: -1, synthesizedNonstructural: -2, synthesizedStructural: -3 },
  selection: 'Original token windows and recursive lexical type flow select the existing representation; explicit none equality and scalar equality retain their existing exclusions.',
  scope: 'Source selection, recursive type metadata and existing allocate/read IR behavior. Other heap operations, callable and nullable support, target state models and whole-language equivalence retain their independent obligations.',
};
if (check) {
  checkProjectData(policyFile, policy);
  assert.ok(readFileSync(policyCopy).equals(readFileSync(policyFile)), 'both runtimes share the exact source reference policy');
} else {
  writeProjectData(policyFile, policy);
  writeFileSync(policyCopy, readFileSync(policyFile));
}
if (process.argv.includes('--javascript-only')) process.exit(0);

const source = readFileSync(input, 'utf8');
const closedSource = readSurfaceReferenceSourceModule(source);
const sourceNames = [...closedSource.matchAll(/^(?:export )?function (\w+)\(/gmu)].map(match => match[1]);
const snake = name => name.replace(/([a-z])([A-Z])/gu, '$1_$2').toLowerCase();
const emitted = emitRust(checkProgram(parseJavaScript(closedSource, { referenceSemantics: false })), { arrayConsumption: true });
const moduleHeader = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
assert.ok(moduleHeader, 'the complete checked module retains its generated source-model header');
const raw = [moduleHeader, ...emitted.preludes, ...emitted.definitions].join('\n\n');
for (const name of sourceNames) assert.ok(raw.includes('pub fn ' + snake(name) + '('), 'every original reference algorithm declaration is translated: ' + name);

const actions = [];
let adapted = raw;
const replace = (from, to) => {
  if (from === to || !adapted.includes(from)) return;
  actions.push({ op: 'replace', field: 'source', from, to });
  adapted = adapted.split(from).join(to);
};
// Use the existing complete immutable-array provider, including append/index
// behavior. No runtime operation or source arithmetic is replaced.
for (const prelude of emitted.preludes) {
  assert.ok(prelude.startsWith('/// Reads of JavaScript arrays'), 'the closed reference kernel has only its existing immutable-array runtime dependency');
  replace(prelude, 'pub use crate::translation::frontend_rules::ml_array;');
}
replace('crate::ml_array::', 'crate::translation::frontend_rules::ml_array::');
for (const name of [...raw.matchAll(/\b(?:pub )?(?:fn|enum|struct) (\w+)/gu)].map(match => match[1]).sort((left, right) => right.length - left.length)) {
  replace('crate::' + name, 'crate::translation::surface_reference_analysis_rules::' + name);
}

// The common mechanical ABI retains owned parameters when assigned by the
// emitted flow or required as the selected owned result.
const loans = readPortableSourceParameterStyles(adapted, {
  publicNames: sourceNames.map(snake),
  ownedParameters: {render_surface_reference_operation: ['value']},
});
actions.push(...loans.actions);
adapted = loans.source;
const grouped = simplifySourceTextRustParentheses(adapted);
replace(adapted, grouped);
const reviewed = readPortableSourceLintStyles(adapted, {modulePath: 'crate::translation::surface_reference_analysis_rules'});
actions.push(...reviewed.actions);
adapted = reviewed.source;
const styles = new DecoratorSet([{ id: 'surface-reference-analysis-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]);
if (check) assert.equal(readFileSync(styleFile, 'utf8'), styles.toLino(), 'surface reference library styles are fresh');
else writeFileSync(styleFile, styles.toLino());
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const notice = '// Generated from the complete checked JavaScript surface reference analysis. Do not edit.\n';
const outputs = partitionPortableRustLibrary(formatted.stdout, {
  notice, directory: 'surface_reference_analysis_rules',
}).map(part => [part.path === null ? output : new URL('../../rust/src/translation/' + part.path, import.meta.url), part.source]);
if (!check) mkdirSync(new URL('../../rust/src/translation/surface_reference_analysis_rules/', import.meta.url), {recursive: true});
for (const [file, native] of outputs) {
  if (check) assert.equal(readFileSync(file, 'utf8'), native, 'native reference analysis is fresh');
  else writeFileSync(file, native);
}
console.log('Surface reference analysis: all ' + sourceNames.length + ' original declarations, linked method aliases and exact flat facts in ' + outputs.length + ' native modules');
