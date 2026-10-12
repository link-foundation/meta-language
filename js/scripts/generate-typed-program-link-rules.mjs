// Generate both classifier adapters from the one linked-program schema; the
// Rust decisions are translated by the same checked JavaScript pipeline.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';

const schemaSource = new URL('../src/data/typed-program-links-schema.json', import.meta.url);
const schemaText = readFileSync(schemaSource, 'utf8');
const schema = JSON.parse(schemaText);
const classify = (name, table) => `/** @param {string} term @returns {string} */\nexport function ${name}(term) {\n`
  + Object.entries(table).map(([kind, term]) => `  if (term === ${JSON.stringify(term)}) return ${JSON.stringify(kind)};`).join('\n')
  + '\n  return "";\n}\n';
const source = '// Generated from data/typed-program-links-schema.json. Do not edit by hand.\n\n'
  + classify('readProgramContainerKind', schema.containers) + '\n'
  + classify('readProgramScalarKind', schema.scalars) + '\n'
  + classify('readProgramExpressionProjection', Object.fromEntries(Object.keys(schema.expressionProjections).map((kind) => [kind, kind]))) + '\n'
  + '/** @param {string} label @returns {boolean} */\nexport function acceptProgramReferenceField(label) {\n'
  + `  return label === ${JSON.stringify(schema.references.binding)} || label === ${JSON.stringify(schema.references.declaration)};\n}\n\n`
  + '/** @param {string} name @returns {boolean} */\nexport function isUnsafeProgramField(name) {\n'
  + '  return name === "__proto__" || name === "constructor" || name === "prototype";\n}\n\n'
  + '/** @param {string} term @param {string} revision @param {string} arity @returns {boolean} */\nexport function acceptProgramSchemaRoot(term, revision, arity) {\n'
  + `  return term === ${JSON.stringify(schema.root)} && revision === ${JSON.stringify(String(schema.schemaVersion))} && arity === "1";\n}\n\n`
  + '/** @param {string} width @returns {boolean} */\nexport function acceptProgramMachineIntegerWidth(width) {\n'
  + `  return ${schema.machineIntegerWidths.map((width) => `width === ${JSON.stringify(String(width))}`).join(' || ')};\n}\n\n`
  + '/** @param {string} absence @returns {boolean} */\nexport function acceptProgramOptionalAbsence(absence) {\n'
  + `  return ${schema.optionalAbsences.map((absence) => `absence === ${JSON.stringify(absence)}`).join(' || ')};\n}\n`;
const emitted = emitRust(checkProgram(parseJavaScript(source)));
const raw = emitted.definitions.join('\n\n');
const styleFile = new URL('../../parity/self-translation/typed-program-link-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--update-styles')) {
  const actions = [];
  let styled = raw;
  const replace = (from, to) => { if (styled.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to); } };
  for (const name of ['term', 'label', 'name', 'revision', 'arity', 'width', 'absence']) replace(`${name}: String`, `${name}: &str`);
  for (const match of [...styled.matchAll(/\([a-z]+ == "[^"\n]*"\)/gu)]) replace(match[0], match[0].slice(1, -1));
  for (const name of ['accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence']) {
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
  for (const name of ['read_program_container_kind', 'read_program_scalar_kind', 'read_program_expression_projection', 'accept_program_reference_field', 'is_unsafe_program_field', 'accept_program_schema_root', 'accept_program_machine_integer_width', 'accept_program_optional_absence']) {
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
  [new URL('../../rust/src/data/typed-program-links-schema.json', import.meta.url), schemaText],
];
for (const [file, expected] of artifacts) {
  if (process.argv.includes('--check')) {
    if (readFileSync(file, 'utf8') !== expected) throw new Error(`typed program link rules are stale: ${file.pathname}`);
  } else writeFileSync(file, expected);
}
