// Generate both classifier adapters from the one linked-program schema; the
// Rust decisions are translated by the same checked JavaScript pipeline.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { readProjectData, readProjectFileBytes } from './project-data.mjs';

const schemaSource = new URL('../src/data/typed-program-links-schema.json', import.meta.url);
const schemaText = readProjectFileBytes(schemaSource).toString('utf8');
const schema = readProjectData(schemaSource);
const referenceRoles = Object.fromEntries(Object.entries(schema.references).map(([role, label]) => [label, role]));
for (const [label, role] of Object.entries(schema.referenceAliases ?? {})) {
  if (Object.hasOwn(referenceRoles, label) || !Object.hasOwn(schema.references, role)) throw new Error('a shared reference alias must name one existing role and a distinct label');
  referenceRoles[label] = role;
}
// Only adjacent equal results are combined. Their tests remain in their
// original order, so this preserves priority even for overlapping selectors.
const decisionRows = rows => {
  const grouped = [];
  for (const [condition, result] of rows) {
    if (grouped.at(-1)?.result === result) grouped.at(-1).conditions.push(condition);
    else grouped.push({ conditions: [condition], result });
  }
  return grouped.map(({ conditions, result }) => `  if (${conditions.map(condition => `(${condition})`).join(' || ')}) return ${result};`).join('\n');
};
const classify = (name, table) => `/** @param {string} term @returns {string} */\nexport function ${name}(term) {\n`
  + decisionRows(Object.entries(table).map(([kind, term]) => [`term === ${JSON.stringify(term)}`, JSON.stringify(kind)]))
  + '\n  return "";\n}\n';
const decide = (name, table, parameter = 'kind') => `/** @param {string} ${parameter} @returns {string} */\nexport function ${name}(${parameter}) {\n`
  + decisionRows(Object.entries(table).map(([kind, value]) => [`${parameter} === ${JSON.stringify(kind)}`, JSON.stringify(value)]))
  + '\n  return "";\n}\n';
const referenceSource = '/** @param {string} label @param {string} constructKind @param {string} typeKind @returns {string} */\nexport function readProgramReferenceSourceField(label, constructKind, typeKind) {\n'
  + decisionRows(Object.entries(referenceRoles).flatMap(([label, role]) => schema.referenceSourceFields[role].map(([construct, type, field]) =>
    [`label === ${JSON.stringify(label)} && constructKind === ${JSON.stringify(construct)} && typeKind === ${JSON.stringify(type)}`, JSON.stringify(field)])))
  + '\n  return "";\n}\n';
const neutralFields = '/** @param {string} context @param {string} field @returns {string} */\nexport function readProgramNeutralFieldDefault(context, field) {\n'
  + decisionRows(Object.entries(schema.neutralFieldDefaults).flatMap(([context, fields]) => Object.entries(fields).map(([field, value]) =>
    [`context === ${JSON.stringify(context)} && field === ${JSON.stringify(field)}`, JSON.stringify(value)])))
  + `\n  if (field === "span" && (${[...new Set(Object.keys(schema.expressionProjections).map(key => key.split(':')[0]))].map(kind => `context === ${JSON.stringify(`expression:${kind}`)}`).join(' || ')})) return "null";`
  + '\n  return "";\n}\n';
const source = '// Generated from data/typed-program-links-schema.json. Do not edit by hand.\n\n'
  + classify('readProgramContainerKind', schema.containers) + '\n'
  + '/** @param {string} kind @param {boolean} hasFields @returns {boolean} */\nexport function acceptProgramContainerFields(kind, hasFields) {\n'
  + decisionRows(Object.entries(schema.containerFieldPolicies).map(([kind, policy]) => {
    if (!['named', 'positional'].includes(policy)) throw new Error('a container field policy must be named or positional');
    return [`kind === ${JSON.stringify(kind)}`, policy === 'named' ? 'true' : '!hasFields'];
  })) + '\n  return false;\n}\n\n'
  + classify('readProgramScalarKind', schema.scalars) + '\n'
  + decide('readProgramReferenceRole', referenceRoles, 'term') + '\n'
  + decide('readProgramPatternBindingForm', schema.patternBindingForms) + '\n'
  + decide('readProgramExpressionOperandFields', Object.fromEntries(Object.entries(schema.expressionOperands).map(([kind, fields]) => [kind, fields.join(' ')]))) + '\n'
  + referenceSource + '\n'
  + decide('readProgramReferenceTargetField', Object.fromEntries(Object.entries(referenceRoles).map(([label, role]) => [label, schema.referenceTargetFields[role]])), 'label') + '\n'
  + neutralFields + '\n'
  + '/** @param {string} kind @param {string} field @returns {boolean} */\nexport function acceptProgramExpressionField(kind, field) {\n'
  + decisionRows(Object.entries(schema.expressionFields).map(([kind, fields]) => [`kind === ${JSON.stringify(kind)}`, fields.map(field => `field === ${JSON.stringify(field)}`).join(' || ')]))
  + '\n  return false;\n}\n\n'
  + '/** @param {string} model @param {boolean} frozen @returns {boolean} */\nexport function acceptProgramTupleProductModel(model, frozen) {\n'
  + `  return (${schema.tupleProductModels.map(model => `model === ${JSON.stringify(model)}`).join(' || ')}) && frozen;\n}\n\n`
  + classify('readProgramExpressionProjection', Object.fromEntries(Object.keys(schema.expressionProjections).map((kind) => [kind, kind]))) + '\n'
  + '/** @param {string} kind @param {string} operation @returns {boolean} */\nexport function acceptProgramExpressionOperation(kind, operation) {\n'
  + decisionRows(Object.entries(schema.expressionOperations).map(([kind, operations]) => [`kind === ${JSON.stringify(kind)}`, operations.map((operation) => `operation === ${JSON.stringify(operation)}`).join(' || ')]))
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
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence', 'kind', 'operation', 'rounding', 'construct_kind', 'type_kind', 'context', 'field', 'model']) replace(`${name}: String`, `${name}: &str`);
  replace('String::from("")', 'String::new()');
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence', 'kind', 'operation', 'rounding', 'construct_kind', 'type_kind', 'context', 'field', 'model']) replace(`${name} == ""`, `${name}.is_empty()`);
  for (const match of [...styled.matchAll(/\([a-z_]+ == "[^"\n]*"\)/gu)]) replace(match[0], match[0].slice(1, -1));
  for (const name of ['accept_program_container_fields', 'accept_program_expression_field', 'accept_program_tuple_product_model', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence', 'accept_program_expression_operation', 'accept_program_division_rounding']) {
    const begin = styled.indexOf(`pub fn ${name}`);
    const end = styled.indexOf('\n}', begin);
    const body = styled.slice(begin, end);
    const line = body.split('\n').at(-1).trim();
    let expression = line;
    for (;;) {
      const tokens = tokenize(expression, 'Rust').tokens.filter(token => token.kind !== 'eof');
      if (tokens[0]?.value !== '(') break;
      let depth = 0, closing = -1;
      for (const [index, token] of tokens.entries()) {
        if (token.value === '(') depth += 1;
        else if (token.value === ')') depth -= 1;
        if (depth === 0) { closing = index; break; }
      }
      if (closing !== tokens.length - 1) break;
      expression = expression.slice(tokens[0].end, tokens[closing].start);
    }
    replace(line, expression);
  }
  for (const name of ['read_program_container_kind', 'accept_program_container_fields', 'accept_program_expression_field', 'accept_program_tuple_product_model', 'read_program_scalar_kind', 'read_program_reference_role', 'read_program_expression_projection', 'read_program_pattern_binding_form', 'read_program_expression_operand_fields', 'read_program_reference_source_field', 'read_program_reference_target_field', 'read_program_neutral_field_default', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence', 'accept_program_expression_operation', 'accept_program_division_rounding']) {
    replace(`pub fn ${name}`, `/// Read the corresponding decision of the shared typed program links schema.\n#[must_use]\npub fn ${name}`);
  }
  // Keep deeply nested generated tails out of rustfmt, and store one
  // linear-size receipt instead of every intermediate nested suffix.
  const nested = styled;
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(styled);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1, cursor = opening + 1, quoted = false;
    for (; cursor < styled.length && depth; cursor += 1) {
      const character = styled[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('the translated classifier has an unclosed else block');
    styled = styled.slice(0, match.index) + `} else ${styled.slice(opening + 1, cursor - 1).trim()}` + styled.slice(cursor);
  }
  if (styled !== nested) actions.push({ op: 'replace', field: 'source', from: nested, to: styled });
  writeFileSync(styleFile, new DecoratorSet([{ id: 'typed-program-link-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
let styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
const format = (input) => {
  const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
    input, encoding: 'utf8', maxBuffer: 1024 * 1024,
  });
  if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
  return result.stdout;
};
let formatted = format(styles.decorate('emitter', { format: 'Rust', source: raw }).source);
if (process.argv.includes('--update-parenthesis-styles')) {
  // Reviewed response to CI 38346650821, rather than invented machine-applicable
  // diagnostics. Remove only groups enclosing a complete condition or the sole
  // expression of a block; all interior precedence and opaque literals remain.
  // Record the linear-size final module once, instead of every successively
  // shorter nested tail. Conditions and branch order are unchanged.
  let corrected = formatted;
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(corrected);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1, cursor = opening + 1, quoted = false;
    for (; cursor < corrected.length && depth; cursor += 1) {
      const character = corrected[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('the translated classifier has an unclosed else block');
    corrected = corrected.slice(0, match.index) + `} else ${corrected.slice(opening + 1, cursor - 1).trim()}` + corrected.slice(cursor);
  }
  const { tokens } = tokenize(corrected, 'Rust');
  const openings = [];
  const closing = new Map();
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].value === '(') openings.push(index);
    else if (tokens[index].value === ')') closing.set(openings.pop(), index);
  }
  if (openings.length || closing.has(undefined)) throw new Error('translated rules have unbalanced parentheses');
  const removed = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    if (!['if', '{'].includes(tokens[index].value) || tokens[index + 1]?.value !== '(') continue;
    let start = index + 1;
    let end = closing.get(start);
    if (end === undefined || tokens[end + 1]?.value !== (tokens[index].value === 'if' ? '{' : '}')) continue;
    do {
      removed.add(start); removed.add(end);
      start += 1; end -= 1;
    } while (tokens[start]?.value === '(' && closing.get(start) === end);
  }
  if (!removed.size && corrected === formatted) throw new Error('translated rules have no reviewed complete-expression changes');
  for (const index of [...removed].sort((left, right) => right - left)) {
    const token = tokens[index];
    corrected = corrected.slice(0, token.start) + corrected.slice(token.end);
  }
  const id = 'reviewed-ci-38346650821-complete-expression-parentheses';
  const existing = readFileSync(styleFile, 'utf8').split('\n').filter(line => !line.startsWith(`(decorator ${id} `)).join('\n').trimEnd();
  writeFileSync(styleFile, `${existing}\n\n${new DecoratorSet([{ id, level: 'emitter', order: 1000,
    when: [['format', 'Rust'], ['scope', 'formatted-module']],
    actions: [{ op: 'replace', field: 'source', from: formatted, to: corrected }] }]).toLino()}`);
  styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
}
// Reviewed multiline corrections share the frontend generator's convergent
// formatted-module pipeline, so ordinary regeneration replays the Links data.
// Legacy unscoped signature/documentation actions belong to raw emission only.
const moduleStyles = new DecoratorSet(styles.decorators.filter(({ when }) =>
  when.some(([field, value]) => field === 'scope' && value === 'formatted-module')));
for (let round = 0; round < 10; round += 1) {
  const next = format(moduleStyles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: formatted }).source);
  if (next === formatted) break;
  formatted = next;
  if (round === 9) throw new Error('typed program link style decorators do not converge');
}
const rust = '// Generated from the shared typed-program-links schema through\n'
  + '// js/scripts/generate-typed-program-link-rules.mjs. Do not edit by hand.\n' + formatted;
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
