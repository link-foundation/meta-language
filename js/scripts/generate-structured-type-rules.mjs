#!/usr/bin/env node
// Translate the portable type decisions in one batch; shared data decorators
// qualify generated module references without changing target program text.
import { spawnSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';

import { DecoratorSet } from '../src/decorators.js';
import { decorateEmitted } from '../src/grammar-emitters/common.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';

const input = new URL('../src/translation/structured-type-rules.js', import.meta.url);
const output = new URL('../../rust/src/translation/structured_type_rules.rs', import.meta.url);
const styleFile = new URL('../../parity/self-translation/structured-type-rules-decorators.lino', import.meta.url);
let decorators = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8'));
const program = checkProgram(parseJavaScript(readFileSync(input, 'utf8')));
const translated = emitRust(program);
const raw = decorateEmitted('Rust', { source: translated.text }, decorators).source;
if (process.argv.includes('--update-styles')) {
  // Reviewed repairs for the actual native CI diagnostics. Plain diagnostic
  // text is not compiler applicability metadata; these are explicit styles.
  const actions = [];
  let reviewed = raw;
  const replace = (from, to) => {
    if (!reviewed.includes(from)) throw new Error('the reviewed structural type style no longer matches: ' + from);
    actions.push({ op: 'replace', field: 'source', from, to });
    reviewed = reviewed.split(from).join(to);
  };
  const harness = reviewed.indexOf('\nfn ml_main() {\n\n}\n');
  if (harness < 0 || program.main.effects.length || program.main.result !== undefined
    || translated.definitions.some(definition => !translated.text.slice(0, translated.text.indexOf('\nfn ml_main()')).includes(definition))) {
    throw new Error('the structural type library has no checked empty harness boundary');
  }
  replace(reviewed.slice(harness), '\n');
  // The complete checked runtime remains available at the same public alias.
  // Its shared provider retains every operation and the same signatures.
  const arrayPrelude = translated.preludes.find(prelude => prelude.startsWith('/// Reads of JavaScript arrays'));
  if (!arrayPrelude) throw new Error('the checked structural type array prelude is missing');
  replace(decorateEmitted('Rust', { source: arrayPrelude }, decorators).source,
    'pub use crate::translation::frontend_rules::ml_array;');
  for (const name of ['inner', 'outer', 'left', 'right', 'expected', 'actual', 'kind', 'detail', 'identity', 'type_', 'separator']) {
    if (reviewed.includes('mut ' + name + ': String')) replace('mut ' + name + ': String', name + ': &str');
    if (reviewed.includes(name + ': String')) replace(name + ': String', name + ': &str');
  }
  if (reviewed.includes('mut children: Vec<String>')) replace('mut children: Vec<String>', 'children: &[String]');
  replace('children: Vec<String>', 'children: &[String]');
  replace('children.clone()', 'children');
  replace('&children,', 'children,');
  replace('separator.clone()', 'separator');
  replace('children, String::from(","))', 'children, ",")');
  replace('children, String::from(", "))', 'children, ", ")');
  // Borrowed result spellings still return owned strings. Owned continuation
  // values move at their actual final use, without altering any evaluation.
  for (const name of ['inner', 'detail', 'kind', 'identity', 'type_']) {
    if (reviewed.includes(name + '.clone()')) replace(name + '.clone()', name + '.to_owned()');
  }
  for (const name of new Set([...reviewed.matchAll(/\b(text(?:_[0-9]+)?|ml_present)\.clone\(\)/gu)].map(match => match[1]))) {
    replace(name + '.clone()', name);
  }
  replace('String::from("")', 'String::new()');
  replace('((children.len() as f64)) == (1f64)', 'children.len() == 1');
  // Decimal conversion retains the previous binary64 rounded length. usize
  // text is always a finite nonnegative decimal integer; no sign or truncated
  // index conversion, fused operation or NaN policy is introduced here.
  replace('children.len() as f64', 'children.len().to_string().parse::<f64>().expect("a usize length is a finite binary64 number")');
  replace('format!("{}{}", text, separator)', 'format!("{text}{separator}")');
  for (;;) {
    const { tokens } = tokenize(reviewed, 'Rust');
    const stack = [];
    const closing = new Map();
    for (const [index, token] of tokens.entries()) {
      if (token.kind !== 'punct') continue;
      if (['(', '[', '{'].includes(token.value)) stack.push(index);
      else if ([')', ']', '}'].includes(token.value)) closing.set(stack.pop(), index);
    }
    if (stack.length || closing.has(undefined)) throw new Error('unbalanced checked structural type delimiters');
    let changed = false;
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index].value !== 'match') continue;
      let opening = index + 1;
      while (opening < tokens.length && tokens[opening].value !== '{') {
        if (closing.has(opening)) opening = closing.get(opening);
        opening += 1;
      }
      const end = closing.get(opening);
      if (end === undefined) throw new Error('the checked optional read has no complete match');
      const body = reviewed.slice(tokens[opening].start, tokens[end].end);
      if (!/^\{\s*Some\(ml_present\)\s*=>\s*ml_present,\s*None\s*=>\s*String::from\("undefined"\),?\s*\}$/u.test(body)) continue;
      const operand = reviewed.slice(tokens[index + 1].start, tokens[opening].start).trim();
      replace(reviewed.slice(tokens[index].start, tokens[end].end),
        operand + '.unwrap_or_else(|| String::from("undefined"))');
      changed = true; break;
    }
    if (changed) continue;
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index].kind !== 'punct' || tokens[index].value !== '{'
        || tokens[index + 1]?.value !== 'let' || tokens[index + 2]?.kind !== 'identifier'
        || tokens[index + 3]?.value !== '=') continue;
      let semicolon = index + 4;
      while (semicolon < tokens.length && tokens[semicolon].value !== ';') {
        if (closing.has(semicolon)) semicolon = closing.get(semicolon);
        semicolon += 1;
      }
      if (tokens[semicolon + 1]?.value !== tokens[index + 2].value
        || closing.get(index) !== semicolon + 2) continue;
      replace(reviewed.slice(tokens[index + 1].start, tokens[semicolon + 1].end),
        reviewed.slice(tokens[index + 4].start, tokens[semicolon].start));
      changed = true; break;
    }
    if (changed) continue;
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index].value !== 'else' || tokens[index + 1]?.value !== '{' || tokens[index + 2]?.value !== 'if') continue;
      const end = closing.get(index + 1);
      if (end === undefined || tokens[end - 1]?.value !== '}') throw new Error('the checked else-if boundary changed');
      replace(reviewed.slice(tokens[index + 1].start, tokens[end].end),
        reviewed.slice(tokens[index + 2].start, tokens[end].start).trim());
      changed = true; break;
    }
    if (!changed) break;
  }
  for (const definition of translated.definitions) {
    const name = /^pub fn ([A-Za-z_][A-Za-z_0-9]*)/u.exec(definition)?.[1];
    if (!name) throw new Error('a checked structural type declaration has no public function name');
    replace('pub fn ' + name, '/// Execute the checked shared structural type decision.\n#[must_use]\npub fn ' + name);
  }
  const id = 'reviewed-native-structured-type-library-style';
  decorators = new DecoratorSet([...decorators.decorators.filter(decorator => decorator.id !== id),
    { id, level: 'emitter', order: 1100, when: [['format', 'Rust'], ['scope', 'raw-module']], actions }]);
  writeFileSync(styleFile, decorators.toLino());
}
const adapted = decorators.decorate('emitter', { format: 'Rust', scope: 'raw-module', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: adapted, encoding: 'utf8', maxBuffer: 4 * 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const text = '// Generated from js/src/translation/structured-type-rules.js by\n// js/scripts/generate-structured-type-rules.mjs. Do not edit by hand.\n' + formatted.stdout;
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== text) throw new Error('structured type rules are stale; run node js/scripts/generate-structured-type-rules.mjs');
  console.log('generated Rust structural type rules match JavaScript');
} else writeFileSync(output, text);
