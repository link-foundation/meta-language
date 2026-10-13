// Translate the complete byte/profile algorithm. Native adapters only supply
// UTF-8 strings and graph objects; no native framing algorithm is handwritten.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, unlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
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

// These corrections are reviewed policies for the authentic rendered CI log.
// A rendered diagnostic has no compiler suggestion-applicability metadata.
// Pin its published inputs and retain the actual diagnostic digest in LiNo.
function pairedEnd(text, start, opening = '{', closing = '}') {
  let depth = 0;
  for (let at = start; at < text.length; at += 1) {
    if (text.startsWith('//', at)) { at = text.indexOf('\n', at); if (at < 0) break; continue; }
    if (text[at] === '"') {
      for (at += 1; at < text.length; at += 1) {
        if (text[at] === '\\') at += 1;
        else if (text[at] === '"') break;
      }
      continue;
    }
    if (text[at] === opening) depth += 1;
    else if (text[at] === closing && --depth === 0) return at;
  }
  throw new Error('a reviewed generated source context is unbalanced');
}

function reverseCondition(text, position, removeElse = false) {
  const opening = text.indexOf('{', position);
  const firstEnd = pairedEnd(text, opening);
  const otherwise = /^\s*else\s*\{/u.exec(text.slice(firstEnd + 1));
  if (!otherwise) throw new Error('a reviewed branch lacks its complete alternative');
  const secondStart = firstEnd + 1 + otherwise[0].lastIndexOf('{');
  const secondEnd = pairedEnd(text, secondStart);
  const condition = text.slice(position + 2, opening).trim();
  const first = text.slice(opening + 1, firstEnd);
  const second = text.slice(secondStart + 1, secondEnd);
  if (removeElse) {
    // The diagnosed first branch always continues. Keep the inner lexical
    // block in the remaining branch and its original continuation assignment.
    return { start: position, end: secondEnd + 1,
      replacement: `if ${condition} {${first}}\n${second}` };
  }
  const positive = condition.startsWith('!') ? condition.slice(1)
    : condition.includes('!=') ? condition.replace('!=', '==') : null;
  if (!positive) throw new Error('a reviewed branch has no supported logical complement');
  // Swap complete alternatives; both IEEE ==/!= and Boolean negation are
  // exact complements, including NaN. Arithmetic evaluation is unchanged.
  return { start: position, end: secondEnd + 1,
    replacement: `if ${positive} {${second}} else {${first}}` };
}

function editSource(text, edits) {
  const ordered = [...edits].sort((left, right) => right.start - left.start);
  let boundary = text.length;
  for (const edit of ordered) {
    if (edit.end > boundary || edit.start < 0 || edit.end < edit.start) throw new Error('reviewed source corrections overlap');
    text = text.slice(0, edit.start) + edit.replacement + text.slice(edit.end);
    boundary = edit.start;
  }
  return text;
}

function correctPrelude(text) {
  // Explicitly checked conversions retain every value of the original types.
  // The shortest binary64 significand has at most seventeen decimal digits;
  // the checked conversion cannot fail for a Number formatter input.
  text = text.replace('let k = digits.len() as i32;',
    'let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");');
  for (const [from, to] of [
    ['ECMAScript Number::toString:', 'ECMAScript `Number::toString`:'],
    ['/// SameValue,', '/// `SameValue`,'],
    ['where f64::signum does not.', 'where `f64::signum` does not.'],
    ['pub fn same_value(', 'pub const fn same_value('],
    ['fn integer(', 'const fn integer('],
    ['pub fn from_units(', 'pub const fn from_units('],
    ['pub fn length(', 'pub const fn length('],
    ['if x < 0.0 && x >= -0.5 {', 'if (-0.5..0.0).contains(&x) {'],
    ['first as u32', 'u32::from(first)'], ['second as u32', 'u32::from(second)'],
    ['unit as u32', 'u32::from(unit)'], ['Some(first as f64)', 'Some(f64::from(first))'],
    ['.map_or(false, |second|', '.is_some_and(|second|'],
    ['.map_or(false, |c|', '.is_some_and(|c|'],
  ]) text = text.split(from).join(to);
  text = text.replace(/let separator = match separator \{\s*Some\(value\) => value,\s*None => return vec!\[self\.clone\(\)\],\s*\};/u,
    'let Some(separator) = separator else { return vec![self.clone()]; };');
  const start = text.indexOf('if (0xd800..=0xdbff).contains(&first) {');
  if (start >= 0) {
    const opening = text.indexOf('{', start), end = pairedEnd(text, opening);
    const inner = /return Some\(\s*\(\(u32::from\(first\) - 0xd800\) \* 1024 \+ u32::from\(second\) - 0xdc00 \+ 0x10000\)\s*as f64,\s*\);/u;
    if (!inner.test(text.slice(start, end + 1))) throw new Error('the reviewed UTF-16 scalar conversion differs');
    text = text.slice(0, start) + `if (0xd800..=0xdbff).contains(&first)
      && let Some(&second) = self.0.get(index + 1)
      && (0xdc00..=0xdfff).contains(&second) {
      return Some(f64::from((u32::from(first) - 0xd800) * 1024 + u32::from(second) - 0xdc00 + 0x10000));
    }` + text.slice(end + 1);
  }
  // integer() maps NaN to zero. repeat additionally rejects non-finite counts;
  // a Vec length is finite, so its product is finite or infinite, never NaN.
  // pad likewise has no NaN length after integer(). The assertions keep each
  // original failure predicate and message, without changing float rounding.
  text = text.replace(/if count < 0\.0 \|\| !count\.is_finite\(\) \{\s*panic!\("Invalid count value: \{\}", count\);\s*\}/u,
    'assert!(count >= 0.0 && count.is_finite(), "Invalid count value: {count}");');
  for (const name of ['size', 'length']) text = text.replace(new RegExp(`if ${name} > 536_870_888\\.0 \\{\\s*panic!\\("Invalid string length"\\);\\s*\\}`, 'u'),
    `assert!(${name} <= 536_870_888.0, "Invalid string length");`);
  return text;
}

function recordReviewedStyles(logPath) {
  const log = readFileSync(logPath, 'utf8');
  const diagnostics = new Map();
  for (const block of log.split(/(?=^error:)/mu)) {
    const match = /^error: ([^\n]+)\n\s*--> (src\/link_network_binary_rules\/part_[0-3]\.rs):(\d+):(\d+)/mu.exec(block);
    if (match) diagnostics.set(match.slice(1).join(':'), { message: match[1], path: match[2], line: Number(match[3]), column: Number(match[4]) });
  }
  if (diagnostics.size !== 104) throw new Error('the reviewed binary CI round requires its complete 104 distinct diagnostics');
  const revision = '285c8b13edf3b1720cd28b6e46f59ba3be08d550';
  const snapshots = new Map();
  const corrections = [];
  for (let part = 0; part < 4; part += 1) {
    const path = `src/link_network_binary_rules/part_${part}.rs`;
    const snapshot = spawnSync('git', ['show', `${revision}:rust/${path}`], { encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
    if (snapshot.error || snapshot.status !== 0) throw snapshot.error ?? new Error(snapshot.stderr);
    const source = snapshot.stdout;
    const local = readFileSync(new URL(`../../rust/${path}`, import.meta.url), 'utf8');
    if (local !== source) throw new Error('reviewed diagnostics need their exact published generated source: ' + path);
    snapshots.set(path, createHash('sha256').update(source).digest('hex'));
    const lines = source.split('\n'); const starts = [0];
    for (const line of lines.slice(0, -1)) starts.push(starts.at(-1) + line.length + 1);
    const located = [...diagnostics.values()].filter(row => row.path === path).map(row => ({ ...row, offset: starts[row.line - 1] + row.column - 1 }));
    const atomic = [];
    for (const row of located) {
      if (row.message === 'redundant clone') {
        if (source.slice(row.offset, row.offset + 8) !== '.clone()') throw new Error('a reviewed final-use clone source differs');
        atomic.push({ start: row.offset, end: row.offset + 8, replacement: '' });
      } else if (row.message === 'unnecessary parentheses') {
        if (source[row.offset] !== '(') throw new Error('a reviewed parenthesis source differs');
        const end = pairedEnd(source, row.offset, '(', ')');
        atomic.push({ start: row.offset, end: row.offset + 1, replacement: '' }, { start: end, end: end + 1, replacement: '' });
      }
    }
    let corrected = editSource(source, atomic);
    const shifted = offset => offset + atomic.reduce((delta, edit) => edit.end <= offset ? delta + edit.replacement.length - (edit.end - edit.start) : delta, 0);
    const branches = [];
    for (const row of located) {
      if (row.message === 'unnecessary boolean `not` operation' || row.message === 'unnecessary `!=` operation') {
        branches.push(reverseCondition(corrected, shifted(row.offset)));
      } else if (row.message === 'redundant else block') {
        const otherwise = shifted(row.offset);
        let start = corrected.lastIndexOf('if binary {', otherwise);
        if (start < 0 || pairedEnd(corrected, corrected.indexOf('{', start)) > otherwise) throw new Error('the reviewed continuation alternative differs');
        branches.push(reverseCondition(corrected, start, true));
      }
    }
    corrected = editSource(corrected, branches);
    corrected = corrected.replace(/\boutput_14\b/gu, 'completed_output').replace(/\boutput_17\b/gu, 'framed_output');
    corrected = corrected.replace('((unit >= (55296f64)) && (unit <= (56319f64)))', '(55296f64..=56319f64).contains(&unit)')
      .replace('((unit >= (56320f64)) && (unit <= (57343f64)))', '(56320f64..=57343f64).contains(&unit)');
    corrected = correctPrelude(corrected);
    // Exact ml_array provider reuse retains the existing checked array/index
    // semantics and every operation alias, instead of copying its runtime.
    const arrayStart = corrected.indexOf('pub mod ml_array {');
    if (arrayStart >= 0) {
      const end = pairedEnd(corrected, corrected.indexOf('{', arrayStart));
      corrected = corrected.slice(0, arrayStart) + 'pub use crate::translation::frontend_rules::ml_array;' + corrected.slice(end + 1);
    }
    for (const match of source.matchAll(/^pub (?:fn|mod) ([A-Za-z_0-9]+)\b/gmu)) {
      const opening = source.indexOf('{', match.index), end = pairedEnd(source, opening);
      const from = source.slice(match.index, end + 1);
      let to;
      if (match[1] === 'ml_array') to = 'pub use crate::translation::frontend_rules::ml_array;';
      else {
        const start = corrected.search(new RegExp(`^pub (?:const )?(?:fn|mod) ${match[1]}\\b`, 'mu'));
        if (start < 0) throw new Error('a reviewed generated declaration was lost');
        to = corrected.slice(start, pairedEnd(corrected, corrected.indexOf('{', start)) + 1);
      }
      if (from !== to) corrections.push({ op: 'replace', field: 'source', from, to });
    }
  }
  const current = DecoratorSet.fromLino(readFileSync(styles, 'utf8'));
  const reviewed = { id: 'binary-network-native-reviewed-style', level: 'emitter', order: 900,
    when: [['format', 'Rust'], ['scope', 'formatted-item']], actions: [
      { op: 'set', field: 'published-source-revision', value: revision },
      { op: 'set', field: 'ci-merge-revision', value: 'a4d544cd7634dff3d47194a30cda9f43744a0cf9' },
      { op: 'set', field: 'diagnostics-sha256', value: createHash('sha256').update(log).digest('hex') },
      { op: 'set', field: 'diagnostic-evidence-kind', value: 'reviewed rendered compiler log; applicability not recorded' },
      { op: 'set', field: 'published-input-digests', value: [...snapshots].map(([path, hash]) => `${path}=${hash}`).join('; ') },
      ...corrections,
    ] };
  writeFileSync(styles, new DecoratorSet([...current.decorators.filter(row => row.id !== reviewed.id), reviewed]).toLino());
  console.log(`Recorded ${corrections.length} reviewed declaration corrections for ${diagnostics.size} authentic binary CI diagnostics`);
}
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
  const previous = DecoratorSet.fromLino(readFileSync(styles, 'utf8')).decorators
    .filter(({ id }) => id !== 'binary-network-library-adapter-style');
  writeFileSync(styles, new DecoratorSet([{ id: 'binary-network-library-adapter-style', level: 'emitter', order: 0, when: [['format', 'Rust']], actions }, ...previous]).toLino());
}
const reviewedIndex = process.argv.indexOf('--reviewed-log');
if (reviewedIndex >= 0) {
  if (!process.argv[reviewedIndex + 1]) throw new Error('--reviewed-log needs the authentic CI diagnostic file');
  recordReviewedStyles(process.argv[reviewedIndex + 1]);
}
const decorators = DecoratorSet.fromLino(readFileSync(styles, 'utf8'));
const libraryStyles = new DecoratorSet(decorators.decorators.filter(({ when }) => !when.some(([field]) => field === 'scope')));
const itemStyles = new DecoratorSet(decorators.decorators.filter(({ when }) => when.some(([field, value]) => field === 'scope' && value === 'formatted-item')));
const formattedItems = items.map(item => format(libraryStyles.decorate('emitter', { format: 'Rust', source: item }).source));
for (const { actions } of itemStyles.decorators) for (const action of actions) {
  if (action.op === 'replace' && !formattedItems.some(item => item.includes(action.from))) throw new Error('a reviewed binary declaration context is stale');
}
const styled = formattedItems.map(item => format(itemStyles.decorate('emitter', { format: 'Rust', scope: 'formatted-item', source: item }).source));
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
