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
import { reviewResourceDescriptionStyles, reuseResourceArrayPrelude } from './resource-description-reviewed-styles.mjs';

const sourceFile = new URL('../src/resource-description-rules.js', import.meta.url);
const stylesFile = new URL('../../parity/self-translation/resource-description-rules-decorators.lino', import.meta.url);
const directory = new URL('../../rust/src/resource_description_rules/', import.meta.url);
const translated = emitRust(checkProgram(parseJavaScript(readFileSync(sourceFile, 'utf8'), { referenceSemantics: false })));
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
const formatItems = values => {
  // Format the same complete declarations together to avoid a formatter process
  // for every item. Checked boundaries retain every original per-item gate.
  const marker = '// resource description formatted item ';
  if (values.some(value => value.includes(marker))) throw new Error('a formatter boundary occurs in its input');
  const formatted = format(values.map((value, index) => marker + index + '\n' + value).join('\n\n'));
  const pieces = formatted.split(/^\/\/ resource description formatted item ([0-9]+)\n/gmu);
  if (pieces.length !== values.length * 2 + 1 || pieces[0].trim()) throw new Error('the formatter changed its item boundaries');
  return values.map((_, index) => {
    if (pieces[index * 2 + 1] !== String(index)) throw new Error('the formatter changed item order');
    return pieces[index * 2 + 2].trimEnd() + '\n';
  });
};
if (process.argv.includes('--update-styles')) {
  const actions = [];
  const replace = (from, to) => { if (raw.includes(from)) actions.push({ op: 'replace', field: 'source', from, to }); };
  const arrayPrelude = translated.preludes.find(item => item.includes('pub mod ml_array {'));
  if (arrayPrelude) replace(arrayPrelude, reuseResourceArrayPrelude(arrayPrelude, readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8')));
  replace('for limb in remainder.iter_mut()', 'for limb in &mut remainder');
  replace("match text.strip_prefix('-') {\n                Some(rest) => (true, rest),\n                None => (false, text),\n            }", "text.strip_prefix('-').map_or((false, text), |rest| (true, rest))");
  replace('pub fn is_zero(', 'pub const fn is_zero(');
  replace('pub fn is_negative(', 'pub const fn is_negative(');
  replace('use std::fmt;', 'use std::fmt;\n    use std::fmt::Write as _;');
  replace('text.push_str(&format!("{chunk:09}"));', 'write!(text, "{chunk:09}").expect("formatting a String succeeds");');
  replace('if value.is_negative() {\n            panic!("{message}");\n        }', 'assert!(!value.is_negative(), "{message}");');
  replace('f64::signum does not.', '`f64::signum` does not.');
  replace('pub fn same_resource_description_term(left_kind: String', 'pub fn same_resource_description_term(left_kind: &str');
  replace('left_kind: String', 'left_kind: &str');
  replace('right_kind: String', 'right_kind: &str');
  // SSA labels are purely lexical; this phi position is distinct from part_3.
  replace('part_13', 'next_part_position');
  for (const match of raw.matchAll(/\blet (ml_s[0-9]+|ml_result[0-9]+) = |\b[A-Za-z_0-9]+Return\((ml_result[0-9]+)\) => \{/gu)) {
    const name = match[1] ?? match[2];
    if ([...raw.matchAll(new RegExp('\\b' + name + '\\b', 'gu'))].length === 2) replace(name + '.clone()', name);
  }
  replace('(crate::ml_array::optional_at(&units, crate::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))',
    '(crate::ml_array::optional_at(&units, crate::ml_array::number_index(index))).unwrap_or(-1f64)');
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
  for (const name of ['units', 'left_fields', 'right_fields']) {
    const type = name === 'units' ? 'f64' : 'String';
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
    replace('pub fn ' + match[1] + '(', '/// Execute the checked shared resource-description algorithm.\n#[must_use]\n'
      + (attributes.length ? '// The checked continuation explicitly carries its lexical state.\n#[allow(' + attributes.join(', ') + ')]\n' : '') + 'pub fn ' + match[1] + '(');
  }
  writeFileSync(stylesFile, new DecoratorSet([{ id: 'resource-description-library-adapter-style', level: 'emitter', order: 0,
    when: [['format', 'Rust'], ['scope', 'raw-item']], actions }]).toLino());
}
let styles = DecoratorSet.fromLino(readFileSync(stylesFile, 'utf8'));
if (process.argv.includes('--update-styles')) {
  const actions = [];
  for (const item of items) {
    const from = format(styles.decorate('emitter', { format: 'Rust', scope: 'raw-item', source: item }).source);
    const to = reviewResourceDescriptionStyles(from, format).source;
    if (from !== to) actions.push({ op: 'replace', field: 'source', from, to });
  }
  styles = new DecoratorSet([...styles.decorators, { id: 'reviewed-ci14-resource-description-functions', level: 'emitter', order: 1000,
    when: [['format', 'Rust'], ['scope', 'formatted-item']], actions }]);
  writeFileSync(stylesFile, styles.toLino());
}
const initialItems = formatItems(items.map(item => styles.decorate('emitter', { format: 'Rust', scope: 'raw-item', source: item }).source));
const styledItems = formatItems(initialItems.map(initial => styles.decorate('emitter', { format: 'Rust', scope: 'formatted-item', source: initial }).source));
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
  const text = '// Generated from resource-description-rules.js by the checked translator.\n// Do not edit by hand.\n'
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
    if (readFileSync(path, 'utf8') !== expected) throw new Error('resource description rules are stale: ' + file);
  } else writeFileSync(path, expected);
}
const expectedFiles = new Set(outputs.map(([file]) => file));
for (const file of readdirSync(directory)) if (file.endsWith('.rs') && !expectedFiles.has(file)) {
  if (process.argv.includes('--check')) throw new Error('a stale generated partition remains');
  unlinkSync(new URL(file, directory));
}
const schemaFile = new URL('../../rust/src/data/resource-description-schema.lino', import.meta.url);
const schema = readProjectFileBytes(new URL('../src/data/resource-description-schema.lino', import.meta.url));
if (process.argv.includes('--check')) {
  if (!readFileSync(schemaFile).equals(schema)) throw new Error('the native resource description schema is stale');
} else writeFileSync(schemaFile, schema);
console.log('resource description algorithms: ' + groups.length + ' generated bounded modules');
