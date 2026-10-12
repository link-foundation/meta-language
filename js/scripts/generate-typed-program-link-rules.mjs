// Generate both classifier adapters from the one linked-program schema; the
// Rust decisions are translated by the same checked JavaScript pipeline.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectData, readProjectFileBytes } from './project-data.mjs';

const schemaSource = new URL('../src/data/typed-program-links-schema.json', import.meta.url);
const schemaText = readProjectFileBytes(schemaSource).toString('utf8');
const schema = readProjectData(schemaSource);
const referenceRoles = Object.fromEntries(Object.entries(schema.references).map(([role, label]) => [label, role]));
for (const [label, role] of Object.entries(schema.referenceAliases ?? {})) {
  if (Object.hasOwn(referenceRoles, label) || !Object.hasOwn(schema.references, role)) throw new Error('a shared reference alias must name one existing role and a distinct label');
  referenceRoles[label] = role;
}
const classify = (name, table) => `/** @param {string} term @returns {string} */\nexport function ${name}(term) {\n`
  + Object.entries(table).map(([kind, term]) => `  if (term === ${JSON.stringify(term)}) return ${JSON.stringify(kind)};`).join('\n')
  + '\n  return "";\n}\n';
const decide = (name, table, parameter = 'kind') => `/** @param {string} ${parameter} @returns {string} */\nexport function ${name}(${parameter}) {\n`
  + Object.entries(table).map(([kind, value]) => `  if (${parameter} === ${JSON.stringify(kind)}) return ${JSON.stringify(value)};`).join('\n')
  + '\n  return "";\n}\n';
const referenceSource = '/** @param {string} label @param {string} constructKind @param {string} typeKind @returns {string} */\nexport function readProgramReferenceSourceField(label, constructKind, typeKind) {\n'
  + Object.entries(referenceRoles).flatMap(([label, role]) => schema.referenceSourceFields[role].map(([construct, type, field]) =>
    `  if (label === ${JSON.stringify(label)} && constructKind === ${JSON.stringify(construct)} && typeKind === ${JSON.stringify(type)}) return ${JSON.stringify(field)};`)).join('\n')
  + '\n  return "";\n}\n';
const neutralFields = '/** @param {string} context @param {string} field @returns {string} */\nexport function readProgramNeutralFieldDefault(context, field) {\n'
  + Object.entries(schema.neutralFieldDefaults).flatMap(([context, fields]) => Object.entries(fields).map(([field, value]) =>
    `  if (context === ${JSON.stringify(context)} && field === ${JSON.stringify(field)}) return ${JSON.stringify(value)};`)).join('\n')
  + `\n  if (field === "span" && (${[...new Set(Object.keys(schema.expressionProjections).map(key => key.split(':')[0]))].map(kind => `context === ${JSON.stringify(`expression:${kind}`)}`).join(' || ')})) return "null";`
  + '\n  return "";\n}\n';
const source = '// Generated from data/typed-program-links-schema.json. Do not edit by hand.\n\n'
  + classify('readProgramContainerKind', schema.containers) + '\n'
  + '/** @param {string} kind @param {boolean} hasFields @returns {boolean} */\nexport function acceptProgramContainerFields(kind, hasFields) {\n'
  + Object.entries(schema.containerFieldPolicies).map(([kind, policy]) => {
    if (!['named', 'positional'].includes(policy)) throw new Error('a container field policy must be named or positional');
    return `  if (kind === ${JSON.stringify(kind)}) return ${policy === 'named' ? 'true' : '!hasFields'};`;
  }).join('\n') + '\n  return false;\n}\n\n'
  + classify('readProgramScalarKind', schema.scalars) + '\n'
  + decide('readProgramReferenceRole', referenceRoles, 'term') + '\n'
  + decide('readProgramPatternBindingForm', schema.patternBindingForms) + '\n'
  + decide('readProgramExpressionOperandFields', Object.fromEntries(Object.entries(schema.expressionOperands).map(([kind, fields]) => [kind, fields.join(' ')]))) + '\n'
  + referenceSource + '\n'
  + decide('readProgramReferenceTargetField', Object.fromEntries(Object.entries(referenceRoles).map(([label, role]) => [label, schema.referenceTargetFields[role]])), 'label') + '\n'
  + neutralFields + '\n'
  + classify('readProgramExpressionProjection', Object.fromEntries(Object.keys(schema.expressionProjections).map((kind) => [kind, kind]))) + '\n'
  + '/** @param {string} kind @param {string} operation @returns {boolean} */\nexport function acceptProgramExpressionOperation(kind, operation) {\n'
  + Object.entries(schema.expressionOperations).map(([kind, operations]) => `  if (kind === ${JSON.stringify(kind)}) return ${operations.map((operation) => `operation === ${JSON.stringify(operation)}`).join(' || ')};`).join('\n')
  + '\n  return false;\n}\n\n'
  + '/** @param {string} label @returns {boolean} */\nexport function acceptProgramReferenceField(label) {\n'
  + `  return ${Object.keys(referenceRoles).map(label => `label === ${JSON.stringify(label)}`).join(' || ')};\n}\n\n`
  + '/** @param {string} name @returns {boolean} */\nexport function isUnsafeProgramField(name) {\n'
  + '  return name === "__proto__" || name === "constructor" || name === "prototype";\n}\n\n'
  + '/** @param {string} term @param {string} revision @param {string} arity @returns {boolean} */\nexport function acceptProgramSchemaRoot(term, revision, arity) {\n'
  + `  return term === ${JSON.stringify(schema.root)} && revision === ${JSON.stringify(String(schema.schemaVersion))} && arity === "1";\n}\n\n`
  + '/** @param {string} width @returns {boolean} */\nexport function acceptProgramMachineIntegerWidth(width) {\n'
  + `  return ${schema.machineIntegerWidths.map((width) => `width === ${JSON.stringify(String(width))}`).join(' || ')};\n}\n\n`
  + '/** @param {string} absence @returns {boolean} */\nexport function acceptProgramOptionalAbsence(absence) {\n'
  + `  return ${schema.optionalAbsences.map((absence) => `absence === ${JSON.stringify(absence)}`).join(' || ')};\n}\n\n`
  + '/** @param {string} rounding @returns {boolean} */\nexport function acceptProgramDivisionRounding(rounding) {\n'
  + `  return ${schema.divisionRoundingPolicies.map((rounding) => `rounding === ${JSON.stringify(rounding)}`).join(' || ')};\n}\n`;
const emitted = emitRust(checkProgram(parseJavaScript(source)));
const raw = emitted.definitions.join('\n\n');
const styleFile = new URL('../../parity/self-translation/typed-program-link-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--update-styles')) {
  const actions = [];
  let styled = raw;
  const replace = (from, to) => { if (styled.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to); } };
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence', 'kind', 'operation', 'rounding', 'construct_kind', 'type_kind', 'context', 'field']) replace(`${name}: String`, `${name}: &str`);
  for (const match of [...styled.matchAll(/\([a-z]+ == "[^"\n]*"\)/gu)]) replace(match[0], match[0].slice(1, -1));
  for (const name of ['accept_program_container_fields', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence', 'accept_program_expression_operation', 'accept_program_division_rounding']) {
    const begin = styled.indexOf(`pub fn ${name}`);
    const end = styled.indexOf('\n}', begin);
    const body = styled.slice(begin, end);
    const line = body.split('\n').at(-1).trim();
    replace(line, line.replace(/[()]/gu, ''));
  }
  // Collapse a sole `if` inside an `else` block. The exact before/after
  // source is recorded as emitter data; the translated decision is retained.
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(styled);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1; let cursor = opening + 1; let quoted = false;
    for (; cursor < styled.length && depth; cursor += 1) {
      const character = styled[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('the translated classifier has an unclosed else block');
    const from = styled.slice(match.index, cursor);
    const to = `} else ${styled.slice(opening + 1, cursor - 1).trim()}`;
    replace(from, to);
  }
  for (const name of ['read_program_container_kind', 'accept_program_container_fields', 'read_program_scalar_kind', 'read_program_reference_role', 'read_program_expression_projection', 'read_program_pattern_binding_form', 'read_program_expression_operand_fields', 'read_program_reference_source_field', 'read_program_reference_target_field', 'read_program_neutral_field_default', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence', 'accept_program_expression_operation', 'accept_program_division_rounding']) {
    replace(`pub fn ${name}`, `/// Read the corresponding decision of the shared typed program links schema.\n#[must_use]\npub fn ${name}`);
  }
  writeFileSync(styleFile, new DecoratorSet([{ id: 'typed-program-link-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
const styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
const decorated = styles.decorate('emitter', { format: 'Rust', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
  input: decorated, encoding: 'utf8', maxBuffer: 1024 * 1024,
});
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const rust = '// Generated from the shared typed-program-links schema through\n'
  + '// js/scripts/generate-typed-program-link-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
const artifacts = [
  [new URL('../src/typed-program-link-rules.js', import.meta.url), source],
  [new URL('../../rust/src/translation/typed_program_link_rules.rs', import.meta.url), rust],
  [new URL('../../rust/src/data/typed-program-links-schema.lino', import.meta.url), schemaText],
];
for (const [file, expected] of artifacts) {
  if (process.argv.includes('--check')) {
    if (readFileSync(file, 'utf8') !== expected) throw new Error(`typed program link rules are stale: ${file.pathname}`);
  } else writeFileSync(file, expected);
}
