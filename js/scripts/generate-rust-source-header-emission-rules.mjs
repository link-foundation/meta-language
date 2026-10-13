import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { readProjectData } from './project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';

const policyFile = new URL('../src/translation/data/rust-source-header-emission.lino', import.meta.url);
const policy = readProjectData(policyFile);
const javascript = new URL('../src/translation/rust-source-header-emission-rules.js', import.meta.url);
const native = new URL('../../rust/src/translation/source_header_emission_rules.rs', import.meta.url);
const copy = new URL('../../rust/src/data/rust-source-header-emission.lino', import.meta.url);
const style = new URL('../../parity/self-translation/rust-source-header-emission-rules-decorators.lino', import.meta.url);
const strings = (names) => names.map(name => `@param {string} ${name}`).join(' ');
const functionSource = (name, parameters, result, body) => `/** ${parameters} @returns {${result}} */\nexport function ${name}(${[...parameters.matchAll(/@param \{[^}]+\} (\w+)/gu)].map(match => match[1]).join(', ')}) { ${body} }\n`;
const rule = (name, names, body) => functionSource(name, strings(names), 'string', body);
let source = '// Generated from linked primitive borrowed Rust header emission policy.\n';
source += functionSource('acceptRustSourceHeaderEmissionModel', strings(['model']), 'boolean', `return model === ${JSON.stringify(policy.model)};`);
source += functionSource('acceptRustSourceHeaderEmissionContext', `${strings(['language'])} @param {boolean} forbidden`, 'boolean', `return language === ${JSON.stringify(policy.language)} && !forbidden;`);
source += functionSource('acceptRustSourceHeaderEmissionDeclaration', `${strings(['kind'])} @param {boolean} nonFlat @param {boolean} generated`, 'boolean', 'return kind === "fn" && !nonFlat && !generated;');
source += functionSource('acceptRustSourceHeaderEmissionPosition', `${strings(['kind', 'ownerKind', 'ownerPath'])} @param {boolean} invalidArguments @param {boolean} invalidWrappers @param {boolean} borrowedResult`, 'boolean', `return !invalidArguments && !invalidWrappers && !borrowedResult && ownerKind === "primitive" && (${Object.entries(policy.primitives).map(([kind, path]) => `(kind === ${JSON.stringify(kind)} && ownerPath === ${JSON.stringify(path)})`).join(' || ')});`);
source += functionSource('acceptRustSourceHeaderEmissionExpression', strings(['kind']), 'boolean', `return ${policy.expressions.map(kind => `kind === ${JSON.stringify(kind)}`).join(' || ')};`);
source += functionSource('acceptRustSourceHeaderLifetimeCharacter', '@param {number} character @param {boolean} first', 'boolean', `return character === ${policy.lifetimeCharacters.underscore} || ${policy.lifetimeCharacters.letterRanges.map(([from, to]) => `(character >= ${from} && character <= ${to})`).join(' || ')} || (!first && character >= ${policy.lifetimeCharacters.digitRange[0]} && character <= ${policy.lifetimeCharacters.digitRange[1]});`);
source += rule('readRustSourceHeaderPrimitive', ['kind'], Object.entries(policy.primitives).map(([kind, path]) => `if (kind === ${JSON.stringify(kind)}) return ${JSON.stringify(path)};`).join(' ') + ' return "";');
source += rule('renderRustSourceHeaderReference', ['lifetime', 'inner'], 'return lifetime === "" ? "&" + inner : "&\'" + lifetime + " " + inner;');
source += rule('renderRustSourceHeaderParameter', ['name', 'type'], 'return name + ": " + type;');
source += rule('renderRustSourceHeaderLifetime', ['name'], 'return "\'" + name;');
source += rule('renderRustSourceHeaderDereference', ['inner'], 'return "*" + inner;');
source += rule('renderRustSourceHeaderCopy', ['name', 'dereferenced'], 'return "let " + name + " = " + dereferenced + ";";');
source += rule('renderRustSourceHeaderFunction', ['name', 'lifetimes', 'parameters', 'result', 'body'], 'return "pub fn " + name + (lifetimes === "" ? "" : "<" + lifetimes + ">") + "(" + parameters + ") -> " + result + " {\\n" + body + "\\n}";');

if (process.argv.includes('--check')) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('source header emission JavaScript rules are stale');
} else writeFileSync(javascript, source);
if (process.argv.includes('--javascript-only')) {
  console.log('Rust source header emission: generated JavaScript admission and rendering rules');
} else {
  const { parseJavaScript } = await import('../src/translation/javascript.js');
  const { checkProgram } = await import('../src/translation/check.js');
  const { emitRust } = await import('../src/translation/emit-rust.js');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  if (process.argv.includes('--update-styles')) {
    const actions = [];
    let current = raw;
    const replace = (from, to) => {
      if (current.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); current = current.split(from).join(to); }
    };
    for (const name of ['model', 'language', 'kind', 'owner_kind', 'owner_path', 'lifetime', 'inner', 'name', 'r#type', 'dereferenced', 'lifetimes', 'parameters', 'result', 'body']) replace(`${name}: String`, `${name}: &str`);
    for (const match of raw.matchAll(/^pub fn (\w+)/gmu)) replace(`pub fn ${match[1]}`, `/// Apply the shared primitive borrowed header emission policy.\n#[must_use]\npub fn ${match[1]}`);
    replace('lifetime == ""', 'lifetime.is_empty()'); replace('lifetimes == ""', 'lifetimes.is_empty()');
    writeFileSync(style, new DecoratorSet([{ id: 'rust-source-header-emission-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
  }
  const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8' });
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated through the checked JavaScript translator. Do not edit.\n' + formatted.stdout;
  if (process.argv.includes('--check')) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(policyFile))) throw new Error('native source header emission rules or policy are stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(policyFile)); }
  console.log('Rust source header emission: shared classifiers and renderers translated automatically');
}
