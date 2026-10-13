import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { readProjectData } from './project-data.mjs';
import { DecoratorSet } from '../src/decorators.js';
import { stripRustSourceHeaderStyleParentheses, flattenRustSourceHeaderStyleFormat } from './rust-source-header-library-style.mjs';

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
source += functionSource('acceptRustSourceHeaderEmissionDeclarationFields', '@param {boolean} unrepresented', 'boolean', 'return !unrepresented;');
source += functionSource('acceptRustSourceHeaderEmissionPosition', `${strings(['kind', 'ownerKind', 'ownerPath'])} @param {boolean} invalidArguments @param {boolean} invalidWrappers @param {boolean} borrowedResult`, 'boolean', `return !invalidArguments && !invalidWrappers && !borrowedResult && ownerKind === "primitive" && (${Object.entries(policy.primitives).map(([kind, path]) => `(kind === ${JSON.stringify(kind)} && ownerPath === ${JSON.stringify(path)})`).join(' || ')});`);
source += functionSource('acceptRustSourceHeaderEmissionExpression', strings(['kind']), 'boolean', `return ${policy.expressions.map(kind => `kind === ${JSON.stringify(kind)}`).join(' || ')};`);
source += functionSource('acceptRustSourceHeaderEmissionExpressionField', strings(['kind', 'field']), 'boolean', Object.entries(policy.expressionFields).map(([kind, fields]) => `if (kind === ${JSON.stringify(kind)}) return ${fields.map(field => `field === ${JSON.stringify(field)}`).join(' || ')};`).join(' ') + ' return false;');
source += functionSource('acceptRustSourceHeaderEmissionSemantics', strings(['semantics']), 'boolean', `return ${policy.expressionSemantics.map(semantics => `semantics === ${JSON.stringify(semantics)}`).join(' || ')};`);
source += functionSource('acceptRustSourceHeaderEmissionParameter', '@param {boolean} guarded @param {boolean} defaulted @param {boolean} unrepresented', 'boolean', 'return !guarded && !defaulted && !unrepresented;');
source += functionSource('acceptRustSourceHeaderLifetimeCharacter', '@param {number} character @param {boolean} first', 'boolean', `return (character >= ${policy.lifetimeCharacters.underscore} && character <= ${policy.lifetimeCharacters.underscore}) || ${policy.lifetimeCharacters.letterRanges.map(([from, to]) => `(character >= ${from} && character <= ${to})`).join(' || ')} || (!first && (character >= ${policy.lifetimeCharacters.digitRange[0]} && character <= ${policy.lifetimeCharacters.digitRange[1]}));`);
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
    for (const name of ['model', 'language', 'kind', 'field', 'semantics', 'owner_kind', 'owner_path', 'lifetime', 'inner', 'name', 'type_', 'dereferenced', 'lifetimes', 'parameters', 'result', 'body']) replace(`${name}: String`, `${name}: &str`);
    for (const match of raw.matchAll(/^pub fn (\w+)/gmu)) replace(`pub fn ${match[1]}`, `/// Apply the shared primitive borrowed header emission policy.\n#[must_use]\npub fn ${match[1]}`);
    replace('lifetime == ""', 'lifetime.is_empty()'); replace('lifetimes == ""', 'lifetimes.is_empty()');
    for (const condition of ['kind == "float"', 'kind == "bool"', 'lifetime.is_empty()', 'lifetimes.is_empty()']) replace(`if (${condition})`, `if ${condition}`);
    replace('String::from("")', 'String::new()');
    while (/\} else \{\n\s+if /u.test(current)) {
      const start = current.match(/\} else \{\n\s+if /u).index; const open = start + 7;
      let depth = 0; let end = open;
      for (; end < current.length; end += 1) { if (current[end] === '{') depth += 1; else if (current[end] === '}' && --depth === 0) break; }
      const from = current.slice(start, end + 1); replace(from, from.replace(/\} else \{\n\s+if /u, '} else if ').slice(0, -1));
    }
    for (const definition of current.matchAll(/pub fn accept_rust_source_header_[\s\S]*?\n\}/gu)) {
      const block = definition[0]; const open = block.indexOf('{');
      let expression = stripRustSourceHeaderStyleParentheses(block.slice(open + 1, -1));
      expression = expression.replace(/\(([0-9]+f64)\)/gu, '$1').replace(/\((\w+\s*(?:==|>=|<=)\s*(?:"[^"]*"|[0-9]+f64))\)/gu, '$1');
      expression = expression.replace(/(\n[ \t]+)([^\n{}]+)(\n[ \t]*\})/gu, (_match, before, value, after) => before + stripRustSourceHeaderStyleParentheses(value) + after);
      for (const [from, to] of [[policy.lifetimeCharacters.underscore, policy.lifetimeCharacters.underscore], ...policy.lifetimeCharacters.letterRanges, policy.lifetimeCharacters.digitRange]) expression = expression.replaceAll(`character >= ${from}f64 && character <= ${to}f64`, `(${from}f64..=${to}f64).contains(&character)`);
      expression = stripRustSourceHeaderStyleParentheses(expression);
      replace(block, `${block.slice(0, open + 1)}\n    ${expression}\n}`);
    }
    // Flatten only the pure two-argument concatenation macros in these rules.
    // Work from inner macros out so an opaque conditional retains its branch.
    for (let pass = 0; pass < 30; pass += 1) {
      let changed = false;
      for (const match of [...current.matchAll(/format!\("\{\}\{\}", /gu)].reverse()) {
        const start = match.index; let depth = 1; let end = start + 8; let quoted = false; let escaped = false;
        for (; end < current.length; end += 1) {
          const character = current[end];
          if (quoted) { if (escaped) escaped = false; else if (character === '\\') escaped = true; else if (character === '"') quoted = false; continue; }
          if (character === '"') quoted = true;
          else if (character === '(') depth += 1;
          else if (character === ')' && --depth === 0) break;
        }
        const from = current.slice(start, end + 1); const to = flattenRustSourceHeaderStyleFormat(from);
        if (from !== to) { replace(from, to); changed = true; }
      }
      if (!changed) break;
    }
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
