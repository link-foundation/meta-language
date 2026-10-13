// Translate the complete byte/profile algorithm. Native adapters only supply
// UTF-8 strings and graph objects; no native framing algorithm is handwritten.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';

const source = new URL('../src/link-network-binary-rules.js', import.meta.url);
const styles = new URL('../../parity/self-translation/link-network-binary-rules-decorators.lino', import.meta.url);
const destination = new URL('../../rust/src/link_network_binary_rules/', import.meta.url);
const emitted = emitRust(checkProgram(parseJavaScript(readFileSync(source, 'utf8'), { referenceSemantics: false })));
const items = [...emitted.preludes, ...emitted.definitions];
const raw = items.join('\n\n');
const names = [...raw.matchAll(/^pub (?:fn|mod|struct|enum) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]);
const format = input => {
  const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
  if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
  return result.stdout;
};
if (process.argv.includes('--update-styles')) {
  const actions = [];
  const replace = (from, to) => { if (raw.includes(from)) actions.push({ op: 'replace', field: 'source', from, to }); };
  // Recorded style changes preserve every emitted branch and fallback.
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
    if (depth) throw new Error('a translated binary framing decision has an unclosed branch');
    const from = flattened.slice(match.index, cursor);
    const to = '} else ' + flattened.slice(opening + 1, cursor - 1).trim();
    actions.push({ op: 'replace', field: 'source', from, to });
    flattened = flattened.split(from).join(to);
  }
  // These generated owned continuation values occur only at their binding
  // and single match/result use; moving retains their checked semantics.
  for (const match of raw.matchAll(/\blet (ml_s[0-9]+|ml_result[0-9]+) = /gu)) {
    const name = match[1];
    const occurrences = [...raw.matchAll(new RegExp('\\b' + name + '\\b', 'gu'))].length;
    if (occurrences === 2) replace(name + '.clone()', name);
  }
  // optional_at over the checked numeric input returns Copy Option<f64>.
  replace('(crate::ml_array::optional_at(&input, crate::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64))',
    '(crate::ml_array::optional_at(&input, crate::ml_array::number_index(index))).unwrap_or(-1f64)');
  for (const name of names) replace(`crate::${name}`, `super::${name}`);
  // The source never mutates these arrays. Generated continuations can borrow
  // them instead of cloning the complete byte buffer at every field.
  for (const name of ['input', 'units', 'header']) {
    replace(`mut ${name}: Vec<f64>`, `${name}: &[f64]`);
    replace(`${name}: Vec<f64>`, `${name}: &[f64]`);
    replace(`${name}.clone()`, name);
    replace(`&${name},`, `${name},`);
  }
  // Only the entry point constructs a local header; its continuations borrow it.
  for (const item of items) {
    const signature = /^pub fn ([A-Za-z_0-9]+)\(([^)]*)\)/mu.exec(item);
    if (!signature) continue;
    if (item.includes('let header = vec!')) {
      for (const call of item.matchAll(/crate::([A-Za-z_0-9]+)\(input\.clone\(\), header\.clone\(\),/gu)) {
        actions.push({ op: 'replace', field: 'source', from: `super::${call[1]}(input, header,`, to: `super::${call[1]}(input, &header,` });
      }
      actions.push({ op: 'replace', field: 'source', from: 'super::ml_transcode_link_network_binary_loop33(binary, output.clone(), header,', to: 'super::ml_transcode_link_network_binary_loop33(binary, output.clone(), &header,' });
    }
    const allowances = [];
    if (signature[2].split(',').length > 7) allowances.push('clippy::too_many_arguments');
    if (format(item).split('\n').length > 100) allowances.push('clippy::too_many_lines');
    replace(`pub fn ${signature[1]}(`, '/// Execute the checked shared binary-network framing algorithm.\n#[must_use]\n'
      + (allowances.length ? '// Generated continuations retain the checked lexical state.\n#[allow(' + allowances.join(', ') + ')]\n' : '') + `pub fn ${signature[1]}(`);
  }
  // Numeric grouping is a representation-only style adjustment.
  for (const token of raw.matchAll(/\b[0-9]{6,}(?:f64|\.0)?\b/gu)) replace(token[0], token[0].replace(/^([0-9]+)(.*)$/u, (_, digits, suffix) => digits.replace(/\B(?=(?:[0-9]{3})+$)/gu, '_') + suffix));
  writeFileSync(styles, new DecoratorSet([{ id: 'binary-network-library-adapter-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }]).toLino());
}
const decorators = DecoratorSet.fromLino(readFileSync(styles, 'utf8'));
const styled = items.map(item => format(decorators.decorate('emitter', { format: 'Rust', source: item }).source));
const groups = [];
let current = [], lines = 0;
for (const item of styled) {
  const length = item.split('\n').length;
  if (length > 780) throw new Error('a generated declaration needs source-level decomposition');
  if (lines + length > 780 && current.length) { groups.push(current); current = []; lines = 0; }
  current.push(item); lines += length;
}
if (current.length) groups.push(current);
const header = emitted.text.split('\n').find(line => line.startsWith('#![allow('));
if (!header) throw new Error('the checked translator omitted its generated header');
const outputs = groups.map((group, index) => {
  const body = group.join('\n\n');
  const own = new Set([...body.matchAll(/^pub (?:fn|mod|struct|enum) ([A-Za-z_][A-Za-z_0-9]*)\b/gmu)].map(match => match[1]));
  const imports = names.filter(name => !own.has(name) && new RegExp('(?<!::)\\b' + name + '\\b').test(body));
  // Equality and casts retain ECMAScript Number semantics under checked bounds.
  const generatedHeader = header.replace(')]', ', clippy::float_cmp, clippy::suboptimal_flops, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]');
  return [`part_${index}.rs`, '// Generated from link-network-binary-rules.js by the checked translator.\n// Do not edit by hand.\n'
    + format(generatedHeader + '\n' + (imports.length ? 'use super::{' + imports.join(', ') + '};\n' : '') + body)];
});
outputs.push(['mod.rs', '// Generated module partition; the entire checked algorithm is retained.\n'
  + groups.map((_, index) => `mod part_${index};\npub use part_${index}::*;`).join('\n') + '\n']);
mkdirSync(destination, { recursive: true });
for (const [name, expected] of outputs) {
  if (expected.split('\n').length > 900) throw new Error('a generated module exceeds the bounded file target');
  const file = new URL(name, destination);
  if (process.argv.includes('--check')) { if (readFileSync(file, 'utf8') !== expected) throw new Error('binary network rules are stale: ' + name); }
  else writeFileSync(file, expected);
}
const expectedNames = new Set(outputs.map(([name]) => name));
for (const name of readdirSync(destination)) if (name.endsWith('.rs') && !expectedNames.has(name)) {
  if (process.argv.includes('--check')) throw new Error('a stale generated partition remains');
  unlinkSync(new URL(name, destination));
}
const schema = readFileSync(new URL('../src/data/link-network-binary-schema.lino', import.meta.url));
const nativeSchema = new URL('../../rust/src/data/link-network-binary-schema.lino', import.meta.url);
if (process.argv.includes('--check')) { if (!readFileSync(nativeSchema).equals(schema)) throw new Error('native binary schema is stale'); }
else writeFileSync(nativeSchema, schema);
console.log(`Binary network framing: ${emitted.definitions.length} checked definitions in ${groups.length} bounded modules`);
