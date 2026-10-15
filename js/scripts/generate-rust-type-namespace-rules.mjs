import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { readProjectData } from './project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { stripRustSourceHeaderStyleParentheses } from './rust-source-header-library-style.mjs';
const policyFile = new URL('../src/translation/data/rust-type-namespace.lino', import.meta.url);
const policy = readProjectData(policyFile);
const javascript = new URL('../src/translation/rust-type-namespace-rules.js', import.meta.url);
const native = new URL('../../rust/src/translation/type_namespace_rules.rs', import.meta.url);
const copy = new URL('../../rust/src/data/rust-type-namespace.lino', import.meta.url);
const style = new URL('../../parity/self-translation/rust-type-namespace-rules-decorators.lino', import.meta.url);
const fn = (name, parameters, body) => `/** ${parameters} @returns {boolean} */\nexport function ${name}(${[...parameters.matchAll(/@param \{[^}]+\} (\w+)/gu)].map(match => match[1]).join(', ')}) { ${body} }\n`;
let source = '// Generated from the linked resolved Rust type namespace policy.\n';
source += fn('acceptRustTypeNamespaceModel', '@param {string} model', `return model === "" || model === ${JSON.stringify(policy.model)};`);
source += fn('acceptRustTypeNamespaceDeclaration', '@param {string} kind', 'return kind === "data";');
source += fn('acceptRustTypeNamespaceFields', '@param {boolean} invalidFields', 'return !invalidFields;');
source += fn('acceptRustTypeNamespaceAnchor', '@param {boolean} aboveCrate', 'return !aboveCrate;');
source += fn('acceptRustTypeNamespaceBodyConstructor', '@param {boolean} selected @param {string} constructorName', `return !selected || (${policy.unprovedBodyConstructors.map(name=>`constructorName !== ${JSON.stringify(name)}`).join(' && ')});`);
source += fn('acceptRustTypeNamespaceBodyMethod', '@param {boolean} selected @param {string} methodName @param {boolean} ownedUnitClone', `return !selected || (methodName === "clone" && ownedUnitClone) || (${policy.unprovedBodyMethods.map(name=>`methodName !== ${JSON.stringify(name)}`).join(' && ')});`);
source += fn('acceptRustTypeNamespaceMethodDiscovery', '@param {boolean} discovery @param {string} methodName', `return discovery && (${policy.methodDiscoveryNames.map(name=>`methodName === ${JSON.stringify(name)}`).join(' || ')});`);
source += fn('acceptRustTypeNamespaceUnitClone', '@param {boolean} ownedDerivation @param {boolean} fieldFree @param {boolean} directReceiver', 'return ownedDerivation && fieldFree && directReceiver;');
source += fn('acceptRustTypeNamespaceContainer', '@param {string} kind @param {boolean} dense @param {boolean} unrepresented', `return (kind === ${JSON.stringify(policy.containerKinds[0])} && dense && !unrepresented) || ((kind === ${JSON.stringify(policy.containerKinds[1])} || kind === ${JSON.stringify(policy.containerKinds[2])}) && !unrepresented);`);
source += fn('acceptRustTypeNamespaceRecordValue', '@param {boolean} booleanRequired @param {boolean} booleanValue @param {boolean} definedValue', 'return definedValue && (!booleanRequired || booleanValue);');
source += fn('acceptRustTypeNamespaceDiscoveryType', '@param {boolean} discovery @param {string} typeName', `return discovery && (${policy.discoveryTypes.map(name=>`typeName === ${JSON.stringify(name)}`).join(' || ')});`);
source += fn('acceptRustTypeNamespacePosition', '@param {string} kind @param {boolean} invalidWrappers @param {boolean} borrowedResult', `return (${policy.ownerKinds.map(kind => `kind === ${JSON.stringify(kind)}`).join(' || ')}) && !invalidWrappers && !borrowedResult;`);
if (process.argv.includes('--check')) { if (readFileSync(javascript, 'utf8') !== source) throw new Error('type namespace JavaScript rules are stale'); }
else writeFileSync(javascript, source);
if (!process.argv.includes('--javascript-only')) {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const raw = '\n\n' + emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  if (process.argv.includes('--update-styles')) {
    let current = raw; const actions = [];
    const replace = (from, to) => { if (current.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); current = current.split(from).join(to); } };
    for (const name of ['model', 'kind', 'constructor_name', 'method_name', 'type_name']) replace(`${name}: String`, `${name}: &str`);
    replace('model == ""', 'model.is_empty()');
    for (const match of raw.matchAll(/^pub fn (\w+)\(/gmu)) replace(`\n\npub fn ${match[1]}(`, `\n\n/// Apply the shared resolved type namespace policy.\n#[must_use]\npub fn ${match[1]}(`);
    replace('pub fn accept_rust_type_namespace_fields(', 'pub const fn accept_rust_type_namespace_fields(');
    replace('pub fn accept_rust_type_namespace_anchor(', 'pub const fn accept_rust_type_namespace_anchor(');
    replace('pub fn accept_rust_type_namespace_unit_clone(', 'pub const fn accept_rust_type_namespace_unit_clone(');
    replace('pub fn accept_rust_type_namespace_record_value(', 'pub const fn accept_rust_type_namespace_record_value(');
    for (const definition of current.matchAll(/pub(?: const)? fn accept_rust_type_namespace_[\s\S]*?\n\}/gu)) {
      const block = definition[0]; const open = block.indexOf('{');
      let expression = stripRustSourceHeaderStyleParentheses(block.slice(open + 1, -1)).replace(/\((\w+\s*(?:==|!=)\s*"[^"]*")\)/gu, '$1');
      if (block.startsWith('pub fn accept_rust_type_namespace_body_constructor(') || block.startsWith('pub fn accept_rust_type_namespace_body_method(')) {
        // This generated predicate has one disjunction and a pure conjunction
        // of string inequalities; quoted constructor spellings stay untouched.
        expression = expression.replace(/("(?:\\.|[^"\\])*")|[()]/gu, (_match, literal) => literal ?? '');
      }
      replace(block, `${block.slice(0, open + 1)}\n    ${expression}\n}`);
    }
    replace('((kind == "primitive" || kind == "declaration") && !invalid_wrappers) && !borrowed_result', '(kind == "primitive" || kind == "declaration") && !invalid_wrappers && !borrowed_result');
    replace('(model.is_empty())', 'model.is_empty()');
    replace('(owned_derivation && field_free) && direct_receiver', 'owned_derivation && field_free && direct_receiver');
    replace('((kind == "array" && dense) && !unrepresented)', '(kind == "array" && dense && !unrepresented)');
    writeFileSync(style, new DecoratorSet([{ id: 'rust-type-namespace-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
  }
  const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated through the checked JavaScript translator. Do not edit.\n' + formatted.stdout;
  if (process.argv.includes('--check')) { if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('native type namespace rules or policy are stale'); }
  else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
}
console.log('Rust type namespace: shared admission rules generated');
