import {readFileSync, writeFileSync} from 'node:fs';
import {spawnSync} from 'node:child_process';
import {readProjectData} from './project-data.mjs';
import {DecoratorSet} from '../src/decorators.js';

const javascript = new URL('../src/translation/javascript-scoped-binding-emission-rules.js', import.meta.url);
const data = new URL('../src/translation/data/javascript-scoped-binding-emission.lino', import.meta.url);
const policy = readProjectData(data);
const native = new URL('../../rust/src/translation/scoped_binding_emission_rules.rs', import.meta.url);
const copy = new URL('../../rust/src/data/javascript-scoped-binding-emission.lino', import.meta.url);
const style = new URL('../../parity/self-translation/javascript-scoped-binding-emission-rules-decorators.lino', import.meta.url);
const check = process.argv.includes('--check');
const strings = names => names.map(name => `@param {string} ${name}`).join(' ');
const definition = (name, parameters, result, body) => `/** ${parameters} @returns {${result}} */\nexport function ${name}(${[...parameters.matchAll(/@param \{[^}]+\} (\w+)/gu)].map(match => match[1]).join(', ')}) { ${body} }\n`;
const member = (key, values) => values.map(value => `${key} === ${JSON.stringify(value)}`).join(' || ');
let source = '// Generated from linked owned JavaScript scoped binding emission policy.\n';
source += definition('acceptJavaScriptScopedBindingEmissionModel', strings(['model']), 'boolean', `return model === ${JSON.stringify(policy.model)};`);
source += definition('acceptJavaScriptScopedBindingEmissionContext', `${strings(['language'])} @param {boolean} forbidden`, 'boolean', `return (${member('language', policy.languages)}) && !forbidden;`);
source += definition('acceptJavaScriptScopedBindingEmissionReferenceContext', `@param {boolean} present ${strings(['form'])} @param {boolean} ownedSnapshot`, 'boolean', 'return !present || form === "false" || (form === "true" && ownedSnapshot);');
source += definition('acceptJavaScriptScopedBindingEmissionDeclaration', `${strings(['kind'])} @param {boolean} nonFlat @param {boolean} generated`, 'boolean', 'return kind === "fn" && !nonFlat && !generated;');
source += definition('acceptJavaScriptScopedBindingEmissionValueType', `${strings(['kind'])} @param {boolean} singleField`, 'boolean', `return (${member('kind', policy.valueTypes)}) && singleField;`);
source += definition('acceptJavaScriptScopedBindingEmissionAbsentValueType', `${strings(['kind', 'element', 'absence'])} @param {boolean} exact`, 'boolean', `return kind === ${JSON.stringify(policy.absentValueType.kind)} && element === ${JSON.stringify(policy.absentValueType.element)} && absence === ${JSON.stringify(policy.absentValueType.absence)} && exact;`);
source += definition('acceptJavaScriptScopedBindingEmissionAbsentExpression', strings(['kind', 'absence']), 'boolean', `return kind === ${JSON.stringify(policy.absentValueType.kind)} && absence === ${JSON.stringify(policy.absentValueType.absence)};`);
source += definition('acceptJavaScriptScopedBindingEmissionExpression', strings(['kind']), 'boolean', `return ${member('kind', policy.expressions)};`);
source += definition('acceptJavaScriptScopedBindingEmissionEffect', strings(['kind']), 'boolean', `return ${member('kind', policy.effects)};`);
source += definition('acceptJavaScriptScopedBindingEmissionRole', strings(['role', 'owner']), 'boolean', `return ${policy.roles.map(value => `(role === ${JSON.stringify(value.role)} && owner === ${JSON.stringify(value.owner)})`).join(' || ')};`);
const ownedFields = Object.entries(policy.ownedFields).flatMap(([role, kinds]) => Array.isArray(kinds)
  ? [{role,kind:'',fields:kinds}] : Object.entries(kinds).map(([kind,fields])=>({role,kind,fields})));
source += definition('acceptJavaScriptScopedBindingOwnField', strings(['role','kind','field']), 'boolean', `return ${ownedFields.map(value=>`(role === ${JSON.stringify(value.role)} && kind === ${JSON.stringify(value.kind)} && (${member('field',value.fields)}))`).join(' || ')};`);
source += definition('acceptJavaScriptScopedBindingSourceFacet', `${strings(['field'])} @param {boolean} record`, 'boolean', `return (${member('field',policy.validatedSourceFacets)}) && record;`);
source += definition('acceptJavaScriptScopedBindingBooleanField', strings(['role','kind','field']), 'boolean', `return ${policy.booleanFields.map(value=>`(role === ${JSON.stringify(value.role)} && kind === ${JSON.stringify(value.kind)} && (${member('field',value.fields)}))`).join(' || ')};`);
source += definition('acceptJavaScriptScopedBindingEmptySequence', `${strings(['field'])} @param {boolean} present @param {boolean} sequence @param {boolean} empty`, 'boolean', `return !(${member('field',policy.emptySequenceFields)}) || !present || (sequence && empty);`);
source += definition('acceptJavaScriptScopedBindingDefinedField', '@param {boolean} undefinedValue', 'boolean', policy.allowUndefinedFieldValues?'return true;':'return !undefinedValue;');
if (typeof policy.preservedScopeReserveDeclarations !== 'boolean') throw new Error('the owned scope reservation premise must be Boolean');
source += definition('reserveJavaScriptScopedBindingDeclarationNames', '@param {boolean} preserve', 'boolean', policy.preservedScopeReserveDeclarations ? 'return true;' : 'return !preserve;');
source += definition('renderJavaScriptScopedBindingIdentifier', `${strings(['name'])} @param {boolean} preserve`, 'string', `if (preserve) return name; return ${JSON.stringify(policy.prefix)} + name;`);
if (check) {
  if (readFileSync(javascript, 'utf8') !== source) throw new Error('JavaScript scoped binding rules are stale');
} else {
  writeFileSync(javascript, source);
}
if (!process.argv.includes('--javascript-only')) {
  const {parseJavaScript} = await import('../src/translation/javascript.js');
  const {checkProgram} = await import('../src/translation/check.js');
  const {emitRust} = await import('../src/translation/emit-rust.js');
  const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
  if (process.argv.includes('--update-styles')) {
    const actions = []; let current = raw;
    const replace = (from, to) => { if (current.includes(from)) { actions.push({op: 'replace', field: 'source', from, to}); current = current.split(from).join(to); } };
    for (const name of ['model', 'language', 'form', 'kind', 'element', 'absence', 'role', 'owner', 'name','field']) replace(`${name}: String`, `${name}: &str`);
    for (const match of [...current.matchAll(/^pub fn (\w+)/gmu)]) replace(`pub fn ${match[1]}`, `/// Apply the shared owned scoped binding emission policy.\n#[must_use]\npub fn ${match[1]}`);
    for (const match of [...current.matchAll(/pub fn accept_java_script_scoped_binding_[\s\S]*?\n\}/gu)]) {
      const block = match[0]; const open = block.indexOf('{'); let expression = block.slice(open + 1, -1).trim();
      expression = expression.replace(/\((\w+ == "[^"]*")\)/gu, '$1');
      while (expression.startsWith('(') && expression.endsWith(')')) {
        let depth = 0; let wraps = true;
        for (let index = 0; index < expression.length - 1; index += 1) { if (expression[index] === '(') depth += 1; else if (expression[index] === ')' && --depth === 0) { wraps = false; break; } }
        if (!wraps) break; expression = expression.slice(1, -1);
      }
      replace(block, `${block.slice(0, open + 1)}\n    ${expression}\n}`);
    }
    const renderer = current.match(/pub fn render_java_script_scoped_binding_identifier[\s\S]*?\n\}/u)?.[0];
    if (renderer) replace(renderer, `pub fn render_java_script_scoped_binding_identifier(name: &str, preserve: bool) -> String {\n    if preserve { name.to_owned() } else { format!("${policy.prefix}{name}") }\n}`);
    writeFileSync(style, new DecoratorSet([{id: 'javascript-scoped-binding-emission-library-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions}]).toLino());
  }
  const decorated = DecoratorSet.fromLino(readFileSync(style, 'utf8')).decorate('emitter', {format: 'Rust', source: raw}).source;
  const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {input: decorated, encoding: 'utf8'});
  if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
  const generated = '// Generated through the checked JavaScript translator. Do not edit.\n' + formatted.stdout;
  if (check) {
    if (readFileSync(native, 'utf8') !== generated || !readFileSync(copy).equals(readFileSync(data))) throw new Error('native scoped binding rules or policy are stale');
  } else { writeFileSync(native, generated); writeFileSync(copy, readFileSync(data)); }
}
console.log('JavaScript scoped binding emission: shared role, scope and alias decisions');
