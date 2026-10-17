// Isolated exact-source prototype. No published public selector or native host.
import assert from 'node:assert/strict';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { readProjectData, writeProjectData, checkProjectData } from '../../js/scripts/project-data.mjs';
import { parseJavaScript } from '../../js/src/translation/javascript.js';
import { checkProgram } from '../../js/src/translation/check.js';
import { emitRust } from '../../js/src/translation/emit-rust.js';
import { DecoratorSet } from '../../js/src/decorators.js';
import { formatProductRuleSource } from '../../js/scripts/product-rule-style-corrections.mjs';
import { stripRustSourceHeaderStyleParentheses } from '../../js/scripts/rust-source-header-library-style.mjs';

const raw = readProjectData(new URL('../rust-raw-binding-identifiers/data/policy.lino', import.meta.url));
export const rawBindingCertificatePolicy = {
  schemaVersion: 1,
  model: 'program owned Rust raw binding certificate adapter 1',
  sourceModel: raw.model,
  contextModel: 'program owned Rust raw binding source context 1',
  sourceOption: 'rustRawBindingIdentifierModel',
  facetField: 'sourceRawBindingIdentifiers',
  contextField: 'sourceRawBindingIdentifierContext',
  contextFields: ['model', 'sourceExtent'],
  sourceOptions: ['rustArrayResultModel', 'rustCompletionModel', 'rustSourceSignatureModel', 'rustScalarLoopModel', 'javaScriptScalarLoopModel', 'rustPrimitiveOwnerModel', 'rustEmptyOptionSourceModel', 'rustTypeNamespaceModel', 'rustPureUnitCallModel', 'rustOwnedLocalStateModel', 'rustConstFunctionModel', 'rustRuntimeShellModel'],
  targetOptions: ['sourceHeaderModel', 'scopedBindingModel'],
  inputAssumption: { id: 'owned-raw-binding-primitive-invocation', statement: raw.invocation.meaning },
  encoding: { id: 'program-owned-rust-raw-binding-certificate', statement: 'The actual complete source tokens and checked declaration, parameter, expression and invocation owners remain represented. The raw prefix is a spelling of the same semantic identifier.' },
  dependencies: [
    { policy: 'experiments/rust-raw-binding-identifiers/data/policy.lino', model: raw.model },
    { operation: 'createRustRawBindingIdentifiers', source: 'experiments/rust-raw-binding-identifiers/model.mjs' },
    { operation: 'checkRustRawBindingIdentifiers', source: 'experiments/rust-raw-binding-identifiers/model.mjs' },
    { operation: 'checkRustRawBindingIdentifierInvocation', source: 'experiments/rust-raw-binding-identifiers/model.mjs' },
    { operation: 'checkOwnedProgramContainers', source: 'js/src/translation/owned-program-containers.js' },
  ],
  boundaries: [
    'Only the isolated decorated source overlay recognizes the candidate option. Published source files, public declarations and default raw-identifier refusals remain unchanged.',
    'The program-owned facet owns its actual main object. The main source context owns the exact complete-source extent object. Either surviving fact makes missing or misplaced certificate facts refuse.',
    'Actual Program and main owners are plain data records. Functions and inherited/custom-prototype roots cannot suppress selection by hiding surviving source facts from own-descriptor traversal.',
    'Core checking uses an ephemeral view without adapter facets, with the exact original declaration, parameter and expression objects retained. The main view is explicitly grounded in the actual main identity before projection.',
    'No source grammar, identifier normalization, alias resolution or invocation domain is reimplemented; the unchanged complete raw-binding interpretation validates those meanings.',
    'Source and target profile compositions require independent proofs and are refused. JavaScript emission retains explicit owned invocation premises rather than injecting a runtime wrapper.',
  ],
  nativeBoundary: {
    programSource: 'rust/src/translation/ir.rs',
    codecSource: 'rust/src/typed_program_links/codec.rs',
    requiredRelations: ['program facet to actual main', 'main context to exact source extent', 'alias inverse to actual declaration or parameter', 'occurrence to actual checked expression and alias', 'invocation premise to exact ordered core parameter array'],
    gap: 'The native Program and Main records have no corresponding facets or original-LinkId certificate. Lowered serde_json::Value equality cannot prove these identities. Generated predicates are not a native host implementation.',
  },
};
export function rawBindingCertificateRules() {
  return '// Generated isolated raw-binding certificate decisions. Do not edit.\n'
    + '/** @param {string} language @param {boolean} independent @returns {boolean} */\n'
    + 'export function acceptRustRawBindingCertificateSource(language, independent) { return language === "Rust" && independent; }\n'
    + '/** @param {string} model @returns {boolean} */\n'
    + `export function acceptRustRawBindingCertificateContext(model) { return model === ${JSON.stringify(rawBindingCertificatePolicy.contextModel)}; }\n`
    + '/** @param {boolean} facet @param {boolean} context @returns {boolean} */\n'
    + 'export function acceptRustRawBindingCertificateRoots(facet, context) { return facet && context; }\n'
    + '/** @param {boolean} worker @param {boolean} extent @returns {boolean} */\n'
    + 'export function acceptRustRawBindingCertificateOwners(worker, extent) { return worker && extent; }\n';
}
export function checkRawBindingCertificateRules(check) {
  mkdirSync(new URL('./data/', import.meta.url), { recursive: true });
  const policyFile = new URL('./data/policy.lino', import.meta.url), sourceFile = new URL('./rules.mjs', import.meta.url);
  if (check) { checkProjectData(policyFile, rawBindingCertificatePolicy); assert.equal(readFileSync(sourceFile, 'utf8'), rawBindingCertificateRules()); }
  else { writeProjectData(policyFile, rawBindingCertificatePolicy); writeFileSync(sourceFile, rawBindingCertificateRules()); }
  const translated = emitRust(checkProgram(parseJavaScript(rawBindingCertificateRules()))).definitions.join('\n\n');
  let current = translated;
  const actions = [];
  const replace = (from, to) => { assert.ok(current.includes(from)); actions.push({ op: 'replace', field: 'source', from, to }); current = current.split(from).join(to); };
  for (const name of ['language', 'model']) replace(name + ': String', name + ': &str');
  for (const match of [...current.matchAll(/pub fn accept_rust_raw_binding_certificate_[\s\S]*?\n\}/gu)]) {
    const definition = match[0], open = definition.indexOf('{');
    replace(definition, '/// Isolated raw binding certificate decision.\n#[must_use]\n' + definition.slice(0, open + 1) + '\n    ' + stripRustSourceHeaderStyleParentheses(definition.slice(open + 1, -1)) + '\n}');
  }
  for (const name of ['roots', 'owners']) replace('pub fn accept_rust_raw_binding_certificate_' + name, 'pub const fn accept_rust_raw_binding_certificate_' + name);
  const styles = new DecoratorSet([{ id: 'raw-binding-certificate-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino();
  const output = '// Experimental automatic translation; no native certificate host.\n' + formatProductRuleSource(DecoratorSet.fromLino(styles).decorate('emitter', { format: 'Rust', source: translated }).source);
  for (const [name, value] of [['library-styles.lino', styles], ['rules.rs', output]]) {
    const file = new URL('./data/' + name, import.meta.url);
    if (check) assert.equal(readFileSync(file, 'utf8'), value); else writeFileSync(file, value);
  }
}
if (process.argv[1] === fileURLToPath(import.meta.url)) {
  checkRawBindingCertificateRules(process.argv.includes('--check'));
  console.log('Raw binding certificate: linked policy and actual automatically translated decisions');
}
