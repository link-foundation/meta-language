// Translate the actual lexical/equality algorithms, then split complete emitted
// declarations into bounded library modules. No native decision is handwritten.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { tokenize } from '../src/translation/lexer.js';
import { readProjectFileBytes } from './project-data.mjs';

const sourceFile = new URL('../src/resource-description-ntriples-rules.js', import.meta.url);
// Dependency assembly selects real authored function declarations, without
// rewriting any lexical decisions or synthesizing native implementations.
const base = readFileSync(new URL('../src/resource-description-rules.js', import.meta.url), 'utf8')
  .split('/** @param {number} code @param {boolean} query')[0];
const characters = readFileSync(new URL('../src/resource-description-ntriples-characters.js', import.meta.url), 'utf8');
const source = base + '\n' + characters + '\n' + readFileSync(sourceFile, 'utf8').replace(/^import .*;\n/gmu, '');
const stylesFile = new URL('../../parity/self-translation/resource-description-ntriples-rules-decorators.lino', import.meta.url);
const directory = new URL('../../rust/src/resource_description_ntriples_rules/', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(source, { referenceSemantics: false })));
const header = translated.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new Error('the checked translator omitted its generated module header');
const items = [...translated.preludes, ...translated.definitions];
const raw = items.join('\n\n');
const names = [...raw.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
const format = value => {
  const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: value, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
  return result.stdout;
};
if (process.argv.includes('--update-styles')) {
  const actions = [];
  const replace = (from, to) => { if (raw.includes(from)) actions.push({ op: 'replace', field: 'source', from, to }); };
  // Flatten the actual emitted else-only-if blocks without changing any
  // branch, condition, operation or fallback.
  let flattened = raw;
  for (;;) {
    const match = /\} else \{\n\s+if /u.exec(flattened);
    if (!match) break;
    const opening = match.index + '} else '.length;
    let depth = 1;
    let cursor = opening + 1;
    let quoted = false;
    for (; cursor < flattened.length && depth; cursor += 1) {
      const character = flattened[cursor];
      if (quoted && character === '\\') { cursor += 1; continue; }
      if (character === '"') quoted = !quoted;
      if (!quoted && character === '{') depth += 1;
      if (!quoted && character === '}') depth -= 1;
    }
    if (depth) throw new Error('a translated N-Triples decision has an unclosed branch');
    const from = flattened.slice(match.index, cursor);
    const to = '} else ' + flattened.slice(opening + 1, cursor - 1).trim();
    actions.push({ op: 'replace', field: 'source', from, to });
    flattened = flattened.split(from).join(to);
  }
  // A single-use owned continuation is moved into its match/result. The
  // source never accesses it again and its payload has no user-defined Drop.
  for (const match of raw.matchAll(/\blet (ml_s[0-9]+|ml_result[0-9]+) = /gu)) {
    const name = match[1];
    const occurrences = [...raw.matchAll(new RegExp('\\b' + name + '\\b', 'gu'))].length;
    if (occurrences === 2) replace(name + '.clone()', name);
  }
  // The authored accessor has checked Option<f64> output and a pure literal
  // fallback. Keep the exact value while avoiding a redundant Copy clone.
  replace('(crate::ml_array::optional_at(&units, crate::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))',
    '(crate::ml_array::optional_at(&units, crate::ml_array::number_index(index))).unwrap_or(-1f64)');
  for (const match of raw.matchAll(/^([ \t]+)pub fn ([A-Za-z_0-9]+)([<(])/gmu)) {
    if (/#\[must_use\]\s*$/u.test(raw.slice(0, match.index))) continue;
    const declaration = match[1] + 'pub fn ' + match[2] + match[3];
    replace(declaration, match[1] + '#[must_use]\n' + declaration);
  }
  for (const name of names) replace('crate::' + name, 'super::' + name);
  // Clippy's pedantic literal style retains every numeric value and suffix.
  // Discover the actual emitted numeric tokens, avoiding edits to source rules.
  for (const token of tokenize(raw, 'Rust').tokens) {
    if (token.kind !== 'number') continue;
    const digits = /^[0-9]{6,}(?=\.|[a-z]|$)/u.exec(token.raw)?.[0];
    if (digits) replace(token.raw, digits.replace(/\B(?=(?:[0-9]{3})+$)/gu, '_') + token.raw.slice(digits.length));
  }

  // The input units and equality fields are never assigned. Borrowing them
  // preserves their checked read-only semantics and avoids quadratic cloning.
  for (const name of ['units']) {
    const type = 'f64';
    if (new RegExp('\\b' + name + ' =').test(raw)) throw new Error('a proposed borrowed input is assigned: ' + name);
    replace('mut ' + name + ': Vec<' + type + '>', name + ': &[' + type + ']');
    replace(name + ': Vec<' + type + '>', name + ': &[' + type + ']');
    replace(name + '.clone()', name);
    replace('&' + name + ',', name + ',');
  }
  // Locally constructed lexical-position arrays use the same read-only unit
  // accessor. Borrow those actual call operands as well as the external input.
  const localUnitArrays = new Set([...raw.matchAll(/resource_description_unit\(([A-Za-z_][A-Za-z_0-9]*)\.clone\(\),/gu)].map(match => match[1]));
  for (const name of localUnitArrays) {
    replace('resource_description_unit(' + name + '.clone(),', 'resource_description_unit(&' + name + ',');
  }
  for (const item of items) {
    const match = /^pub fn ([A-Za-z_0-9]+)\(([^)]*)\)/mu.exec(item);
    if (!match) continue;
    const attributes = [];
    if (match[2].split(',').length > 7) attributes.push('clippy::too_many_arguments');
    if (format(item).split('\n').length > 100) attributes.push('clippy::too_many_lines');
    replace('pub fn ' + match[1], '/// Execute the checked shared resource-description algorithm.\n#[must_use]\n'
      + (attributes.length ? '// The checked continuation explicitly carries its lexical state.\n#[allow(' + attributes.join(', ') + ')]\n' : '') + 'pub fn ' + match[1]);
  }
  writeFileSync(stylesFile, new DecoratorSet([{ id: 'resource-description-ntriples-library-adapter-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
const styles = DecoratorSet.fromLino(readFileSync(stylesFile, 'utf8'));
const styledItems = items.map(item => format(styles.decorate('emitter', { format: 'Rust', source: item }).source));
const groups = [];
let group = [];
let size = 0;
for (const item of styledItems) {
  const lines = item.split('\n').length;
  if (lines > 780) throw new Error('a generated declaration needs source-level decomposition');
  if (size + lines > 780 && group.length) { groups.push(group); group = []; size = 0; }
  group.push(item); size += lines;
}
if (group.length) groups.push(group);
const outputs = [];
for (const [index, group] of groups.entries()) {
  const content = group.join('\n\n');
  const own = new Set([...content.matchAll(/^pub (?:fn|enum|struct|mod) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const imports = names.filter(name => !own.has(name) && new RegExp('(?<!::)\\b' + name + '\\b').test(content));
  const generatedHeader = header.replace(')]', ', clippy::float_cmp, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]');
  const file = 'part_' + index + '.rs';
  const text = '// Generated from resource-description-ntriples-rules.js and its dependencies by the checked translator.\n// Do not edit by hand.\n'
    + format(generatedHeader + '\n' + (imports.length ? 'use super::{' + imports.join(', ') + '};\n' : '') + content);
  if (text.split('\n').length > 900) throw new Error('a generated module exceeds the bounded source-file target');
  outputs.push([file, text]);
}
const modules = groups.map((_, index) => 'mod part_' + index + ';\npub use part_' + index + '::*;').join('\n');
outputs.push(['mod.rs', '// Generated module partition; each declaration retains the same checked algorithm.\n' + modules + '\n']);
mkdirSync(directory, { recursive: true });
for (const [file, expected] of outputs) {
  const path = new URL(file, directory);
  if (process.argv.includes('--check')) {
    if (readFileSync(path, 'utf8') !== expected) throw new Error('resource description N-Triples rules are stale: ' + file);
  } else writeFileSync(path, expected);
}
const expectedFiles = new Set(outputs.map(([file]) => file));
for (const file of readdirSync(directory)) if (file.endsWith('.rs') && !expectedFiles.has(file)) {
  if (process.argv.includes('--check')) throw new Error('a stale generated partition remains');
  unlinkSync(new URL(file, directory));
}
const schemaFile = new URL('../../rust/src/data/resource-description-ntriples-profile.lino', import.meta.url);
const schema = readProjectFileBytes(new URL('../src/data/resource-description-ntriples-profile.lino', import.meta.url));
if (process.argv.includes('--check')) {
  if (!readFileSync(schemaFile).equals(schema)) throw new Error('the native resource description N-Triples profile is stale');
} else writeFileSync(schemaFile, schema);
console.log('resource description N-Triples algorithms: ' + groups.length + ' generated bounded modules');
