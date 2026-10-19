// Generate both classifier adapters from the one linked-program schema; the
// Rust decisions are translated by the same checked JavaScript pipeline.
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { readProjectData, readProjectFileBytes } from './project-data.mjs';
import { applyCompleteExpressionParenthesisStyles, replaceRecordedLibraryBaseStyle } from './complete-expression-parenthesis-styles.mjs';
import { flattenFamilyLibraryElseIf } from './portable-conditional-source-style.mjs';
import { readPortableSourceParameterStyles, readPortableSourceLintStyles } from './portable-source-library-style.mjs';
import { readSourceTextRustBlock } from './source-text-byte-library-style.mjs';

if (process.argv.includes('--check') && process.argv.some(argument => argument.startsWith('--update-'))) throw new Error('freshness checks cannot update recorded styles');

const schemaSource = new URL('../src/data/typed-program-links-schema.json', import.meta.url);
const schemaText = readProjectFileBytes(schemaSource).toString('utf8');
const schema = readProjectData(schemaSource);
const referenceRoles = Object.fromEntries(Object.entries(schema.references).map(([role, label]) => [label, role]));
for (const [label, role] of Object.entries(schema.referenceAliases ?? {})) {
  if (Object.hasOwn(referenceRoles, label) || !Object.hasOwn(schema.references, role)) throw new Error('a shared reference alias must name one existing role and a distinct label');
  referenceRoles[label] = role;
}
// These clauses contain only pure string selector comparisons. Factoring a
// repeated comparison within one equal-result group preserves its exact union.
const combineSelectorConditions = clauses => {
  if (clauses.some(clause => clause.length === 0)) return 'true';
  const counts = new Map();
  for (const clause of clauses) for (const selector of new Set(clause)) {
    counts.set(selector, (counts.get(selector) ?? 0) + 1);
  }
  const shared = [...counts].sort((left, right) => right[1] - left[1])[0];
  if (!shared || shared[1] === 1) {
    return clauses.map(clause => `(${clause.join(' && ')})`).join(' || ');
  }
  const [selector] = shared;
  const selected = clauses.filter(clause => clause.includes(selector));
  const remaining = clauses.filter(clause => !clause.includes(selector));
  const suffix = combineSelectorConditions(selected.map(clause => clause.filter(part => part !== selector)));
  const factored = suffix === 'true' ? selector : `(${selector}) && (${suffix})`;
  return remaining.length ? `(${factored}) || (${combineSelectorConditions(remaining)})` : factored;
};
// Only adjacent equal results are combined. Priority between distinct results
// stays in the original order, including overlapping fallback selectors.
const decisionRows = rows => {
  const grouped = [];
  for (const [condition, result] of rows) {
    if (grouped.at(-1)?.result === result) grouped.at(-1).conditions.push(condition);
    else grouped.push({ conditions: [condition], result });
  }
  return grouped.map(({ conditions, result }) => {
    const condition = conditions.every(Array.isArray) ? combineSelectorConditions(conditions)
      : conditions.map(condition => `(${condition})`).join(' || ');
    return `  if (${condition}) return ${result};`;
  }).join('\n');
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
  + decisionRows([
    ...Object.entries(schema.neutralFieldDefaults).flatMap(([context, fields]) => Object.entries(fields).map(([field, value]) =>
      [[`context === ${JSON.stringify(context)}`, `field === ${JSON.stringify(field)}`], JSON.stringify(value)])),
    ...[...new Set(Object.keys(schema.expressionProjections).map(key => key.split(':')[0]))].map(kind =>
      [[`field === "span"`, `context === ${JSON.stringify(`expression:${kind}`)}`], '"null"']),
  ])
  + '\n  return "";\n}\n';
const neutralTypeFields = '/** @param {string} kind @param {string} field @returns {string} */\nexport function readProgramTypeNeutralFieldDefault(kind, field) {\n'
  + decisionRows(Object.entries(schema.neutralTypeFieldDefaults).flatMap(([kind, fields]) => Object.entries(fields).map(([field, value]) =>
    [[`kind === ${JSON.stringify(kind)}`, `field === ${JSON.stringify(field)}`], JSON.stringify(value)])))
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
  + neutralTypeFields + '\n'
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
const propositionSource = '\n' + decide('readProgramPropositionOperandFields', Object.fromEntries(Object.entries(schema.propositionOperands).map(([kind, fields]) => [kind, fields.join(' ')])))
  + '\n' + decide('readProgramPropositionBindingForm', schema.propositionBindingForms);
const entryOperandSource = '\n/** @param {string} kind @param {string} field @returns {string} */\nexport function readProgramExpressionEntryOperandField(kind, field) {\n'
  + decisionRows(Object.entries(schema.expressionOperandEntries ?? {}).flatMap(([kind, fields]) => Object.entries(fields).map(([field, child]) =>
    [[`kind === ${JSON.stringify(kind)}`, `field === ${JSON.stringify(field)}`], JSON.stringify(child)])))
  + '\n  return "";\n}\n';
const declarationTypeSource = '\n/** @param {string} kind @param {string} role @returns {string} */\nexport function readProgramDeclarationTypeProjectionField(kind, role) {\n'
  + decisionRows(Object.entries(schema.declarationTypeProjections ?? {}).flatMap(([kind, fields]) => Object.entries(fields).map(([role, field]) =>
    [[`kind === ${JSON.stringify(kind)}`, `role === ${JSON.stringify(role)}`], JSON.stringify(field)])))
  + '\n  return "";\n}\n';
const javascript = source + propositionSource + entryOperandSource + declarationTypeSource;
const javascriptFile = new URL('../src/typed-program-link-rules.js', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(javascriptFile, 'utf8') !== javascript) throw new Error('typed program JavaScript rules are stale');
} else writeFileSync(javascriptFile, javascript);
if (process.argv.includes('--javascript-only')) process.exit(0);
// The current classifier must exist before frontend/checker modules load it.
// JS-only schema generation never imports or executes native emission.
const [{parseJavaScript}, {checkProgram}, {emitRust}] = await Promise.all([
  import('../src/translation/javascript.js'),
  import('../src/translation/check.js'),
  import('../src/translation/emit-rust.js'),
]);
const emitted = emitRust(checkProgram(parseJavaScript(source)), { arrayConsumption: true });
const propositionDefinitions = emitRust(checkProgram(parseJavaScript(propositionSource + entryOperandSource + declarationTypeSource)), { arrayConsumption: true }).definitions;
const raw = emitted.definitions.join('\n\n') + '\n\n' + propositionDefinitions.join('\n\n');
const styleFile = new URL('../../parity/self-translation/typed-program-link-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--update-styles')) {
  const actions = [];
  let styled = raw;
  const replace = (from, to) => { if (styled.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to); } };
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence', 'kind', 'operation', 'rounding', 'construct_kind', 'type_kind', 'context', 'field', 'model', 'role']) replace(`${name}: String`, `${name}: &str`);
  replace('String::from("")', 'String::new()');
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence', 'kind', 'operation', 'rounding', 'construct_kind', 'type_kind', 'context', 'field', 'model', 'role']) replace(`${name} == ""`, `${name}.is_empty()`);
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
  for (const name of ['read_program_container_kind', 'accept_program_container_fields', 'accept_program_expression_field', 'accept_program_tuple_product_model', 'read_program_scalar_kind', 'read_program_reference_role', 'read_program_expression_projection', 'read_program_pattern_binding_form', 'read_program_expression_operand_fields', 'read_program_expression_entry_operand_field', 'read_program_declaration_type_projection_field', 'read_program_proposition_operand_fields', 'read_program_proposition_binding_form', 'read_program_reference_source_field', 'read_program_reference_target_field', 'read_program_neutral_field_default', 'read_program_type_neutral_field_default', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence', 'accept_program_expression_operation', 'accept_program_division_rounding']) {
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
  writeFileSync(styleFile, replaceRecordedLibraryBaseStyle(existsSync(styleFile) ? readFileSync(styleFile, 'utf8') : '',
    { id: 'typed-program-link-library-style', level: 'emitter', order: 0,
      when: [['format', 'Rust']], actions }));
}
let styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
const format = (input) => {
  const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
    input, encoding: 'utf8', maxBuffer: 1024 * 1024,
  });
  if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
  return result.stdout;
};
// A changed classifier no longer matches an older whole-module replacement.
// Flatten its actual complete else-if blocks before rustfmt sees nested tails;
// retain the exact reviewed input/output as an additional Links style record.
let base = styles.decorate('emitter', { format: 'Rust', source: raw }).source;
// Only the new generated selector receives this additional source-derived ABI/documentation row.
// Every older library record and all unrelated complete definitions remain byte-exact.
{
  const name = 'read_program_type_neutral_field_default';
  const scope = 'typed-program-type-neutral-field-library';
  const from = readSourceTextRustBlock(base, 'fn', name);
  if (base.split(from).length !== 2) throw new Error('the new type-neutral selector needs one complete generated definition');
  const loans = readPortableSourceParameterStyles(from, { publicNames: [name] });
  const to = readPortableSourceLintStyles(loans.source).source;
  const replay = document => new DecoratorSet(DecoratorSet.fromLino(document).decorators.filter(entry =>
    entry.when.some(([field, value]) => field === 'scope' && value === scope)))
    .decorate('emitter', { format: 'Rust', scope, source: from }).source;
  let document = readFileSync(styleFile, 'utf8');
  if (replay(document) !== to) {
    if (!process.argv.includes('--update-type-neutral-styles')) throw new Error('new type-neutral source styles are stale; run --update-type-neutral-styles');
    const id = 'typed-program-type-neutral-field-' + createHash('sha256').update(from + '\0' + to).digest('hex').slice(0, 16);
    if (styles.decorators.some(entry => entry.id === id)) throw new Error('the recorded type-neutral source style must remain applicable');
    const addition = new DecoratorSet([{ id, level: 'emitter', order: 2000,
      when: [['format', 'Rust'], ['scope', scope]], actions: [{ op: 'replace', field: 'source', from, to }] }]).toLino();
    document += (document.endsWith('\n') ? '' : '\n') + addition;
    writeFileSync(styleFile, document); styles = DecoratorSet.fromLino(document);
  }
  const selected = replay(document);
  if (selected !== to) throw new Error('the additional source-derived selector style must replay exactly');
  base = base.replace(from, selected);
}

const scope = 'typed-program-pre-format';
const flattened = flattenFamilyLibraryElseIf(base);
const preFormatStyles = new DecoratorSet(styles.decorators.filter(({ when }) =>
  when.some(([field, value]) => field === 'scope' && value === scope)));
let preFormatted = preFormatStyles.decorate('emitter', { format: 'Rust', scope, source: base }).source;
if (preFormatted !== flattened) {
  if (!process.argv.includes('--update-complete-expression-styles') && !process.argv.includes('--update-styles')) {
    throw new Error('typed program pre-format styles are stale; run --update-complete-expression-styles');
  }
  const id = 'typed-program-pre-format-' + createHash('sha256').update(base).digest('hex').slice(0, 16);
  if (styles.decorators.some(entry => entry.id === id)) throw new Error('the recorded pre-format style must remain applicable');
  const document = readFileSync(styleFile, 'utf8');
  const addition = new DecoratorSet([{ id, level: 'emitter', order: 3000,
    when: [['format', 'Rust'], ['scope', scope]],
    actions: [{ op: 'replace', field: 'source', from: base, to: flattened }] }]).toLino();
  writeFileSync(styleFile, document + (document.endsWith('\n') ? '' : '\n') + addition);
  styles = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
  preFormatted = new DecoratorSet(styles.decorators.filter(({ when }) =>
    when.some(([field, value]) => field === 'scope' && value === scope)))
    .decorate('emitter', { format: 'Rust', scope, source: base }).source;
}
if (preFormatted !== flattened) throw new Error('the recorded pre-format style must replay the complete derived module');
let formatted = format(preFormatted);
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
const document = readFileSync(styleFile, 'utf8');
const correction = applyCompleteExpressionParenthesisStyles(document, formatted, {
  update: process.argv.includes('--update-complete-expression-styles') || process.argv.includes('--update-styles'), format,
});
if (correction.document !== document) writeFileSync(styleFile, correction.document);
formatted = correction.source;
const rust = '// Generated from the shared typed-program-links schema through\n'
  + '// js/scripts/generate-typed-program-link-rules.mjs. Do not edit by hand.\n' + formatted;
const artifacts = [
  [javascriptFile, javascript],
  [new URL('../../rust/src/translation/typed_program_link_rules.rs', import.meta.url), rust],
  [new URL('../../rust/src/data/typed-program-links-schema.lino', import.meta.url), schemaText],
];
for (const [file, expected] of artifacts) {
  if (process.argv.includes('--check')) {
    if (readFileSync(file, 'utf8') !== expected) throw new Error(`typed program link rules are stale: ${file.pathname}`);
  } else writeFileSync(file, expected);
}
