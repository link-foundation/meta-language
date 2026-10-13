// Translate the actual lexical/equality algorithms, then split complete emitted
// declarations into bounded library modules. No native decision is handwritten.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';
import { readProjectFileBytes } from './project-data.mjs';

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
if (process.argv.includes('--update-styles')) {
  const actions = [];
  const replace = (from, to) => { if (raw.includes(from)) actions.push({ op: 'replace', field: 'source', from, to }); };
  for (const name of names) replace('crate::' + name, 'super::' + name);
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
    replace('pub fn ' + match[1], '/// Execute the checked shared resource-description algorithm.\n#[must_use]\n'
      + (attributes.length ? '// The checked continuation explicitly carries its lexical state.\n#[allow(' + attributes.join(', ') + ')]\n' : '') + 'pub fn ' + match[1]);
  }
  writeFileSync(stylesFile, new DecoratorSet([{ id: 'resource-description-library-adapter-style', level: 'emitter', order: 0,
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
