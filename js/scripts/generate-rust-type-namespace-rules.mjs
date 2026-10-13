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
    for (const name of ['model', 'kind']) replace(`${name}: String`, `${name}: &str`);
    replace('model == ""', 'model.is_empty()');
    for (const match of raw.matchAll(/^pub fn (\w+)\(/gmu)) replace(`\n\npub fn ${match[1]}(`, `\n\n/// Apply the shared resolved type namespace policy.\n#[must_use]\npub fn ${match[1]}(`);
    replace('pub fn accept_rust_type_namespace_fields(', 'pub const fn accept_rust_type_namespace_fields(');
    replace('pub fn accept_rust_type_namespace_anchor(', 'pub const fn accept_rust_type_namespace_anchor(');
    for (const definition of current.matchAll(/pub fn accept_rust_type_namespace_[\s\S]*?\n\}/gu)) {
      const block = definition[0]; const open = block.indexOf('{');
      const expression = stripRustSourceHeaderStyleParentheses(block.slice(open + 1, -1)).replace(/\((\w+\s*==\s*"[^"]*")\)/gu, '$1');
      replace(block, `${block.slice(0, open + 1)}\n    ${expression}\n}`);
    }
    replace('((kind == "primitive" || kind == "declaration") && !invalid_wrappers) && !borrowed_result', '(kind == "primitive" || kind == "declaration") && !invalid_wrappers && !borrowed_result');
    replace('(model.is_empty())', 'model.is_empty()');
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
