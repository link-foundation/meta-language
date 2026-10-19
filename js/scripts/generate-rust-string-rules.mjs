// Generate the Rust string lexer decisions from the checked JavaScript source.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { DecoratorSet } from '../src/decorators.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { parseJavaScript } from '../src/translation/javascript.js';
import { tokenize } from '../src/translation/lexer.js';
import { applyRecordedCompilerFeedback } from './compiler-style-feedback.mjs';

const sourcePath = new URL('../src/translation/rust-string-rules.js', import.meta.url);
const stylesPath = new URL('../../parity/self-translation/rust-string-rules-decorators.lino', import.meta.url);
const outputPath = new URL('../../rust/src/translation/frontend_rules/rust_strings.rs', import.meta.url);
const feedbackIndex = process.argv.indexOf('--feedback');
if (feedbackIndex >= 0) {
  const feedbackFile = process.argv[feedbackIndex + 1];
  if (!feedbackFile || feedbackFile.startsWith('--')) throw new Error('--feedback requires the recorded compiler feedback file');
  applyRecordedCompilerFeedback({ root: fileURLToPath(new URL('../../', import.meta.url)), feedbackFile,
    decoratorFile: 'parity/self-translation/rust-string-rules-decorators.lino',
    allowedPaths: ['rust/src/translation/frontend_rules/rust_strings.rs'] });
}
const raw = emitRust(checkProgram(parseJavaScript(readFileSync(sourcePath, 'utf8')))).definitions.join('\n\n');
const format = input => {
  const result = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input, encoding: 'utf8', maxBuffer: 1024 * 1024 });
  if (result.error || result.status !== 0) throw result.error ?? new Error(result.stderr);
  return result.stdout;
};

if (process.argv.includes('--update-styles')) {
  const actions = [];
  let adapted = raw;
  const replace = (from, to) => {
    if (!adapted.includes(from)) throw new Error(`the reviewed string rule adaptation no longer matches: ${from}`);
    actions.push({ op: 'replace', field: 'source', from, to });
    adapted = adapted.split(from).join(to);
  };
  // The immutable input remains borrowed across the automatically lowered loop.
  replace('mut units: Vec<f64>', 'units: &[f64]');
  replace('units: Vec<f64>', 'units: &[f64]');
  replace('units.clone()', 'units');
  replace('&units,', 'units,');
  replace('unit.clone()', 'unit');
  for (const unit of [9, 10, 13, 32]) {
    replace(`match (unit, ${unit}f64) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }`, `unit == Some(${unit}f64)`);
  }
  replace('crate::ml_array::', 'crate::translation::frontend_rules::ml_array::');
  for (const name of ['accept_rust_string_continuation_whitespace', 'ml_read_rust_string_continuation_end_loop1']) {
    replace(`crate::${name}`, `crate::translation::frontend_rules::${name}`);
  }
  for (const name of ['accept_rust_string_continuation_whitespace', 'read_rust_string_continuation_end', 'read_rust_source_line_break_end', 'ml_read_rust_string_continuation_end_loop1']) {
    replace(`pub fn ${name}`, `/// Read the corresponding shared Rust string lexical decision.\n#[must_use]\npub fn ${name}`);
  }
  let formatted = format(adapted);
  const decorators = [{ id: 'rust-string-lexical-library-adaptation', level: 'emitter', order: 0,
    when: [['format', 'Rust'], ['scope', 'raw-module']], actions }];
  for (let round = 0; round < 10; round += 1) {
  // Remove only parentheses enclosing a complete condition or block value.
  // Interior precedence and the original translated control flow stay intact.
  const { tokens } = tokenize(formatted, 'Rust');
  const stack = [];
  const closing = new Map();
  for (const [index, token] of tokens.entries()) {
    if (['(', '{', '['].includes(token.value)) stack.push(index);
    if ([')', '}', ']'].includes(token.value)) closing.set(stack.pop(), index);
  }
  if (stack.length || closing.has(undefined)) throw new Error('unbalanced translated string rule parentheses');
  const removed = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    if (!['if', '{', '=', '('].includes(tokens[index].value) || tokens[index + 1]?.value !== '(') continue;
    let begin = index + 1;
    let end = closing.get(begin);
    const next = tokens[index].value === 'if' ? '{' : tokens[index].value === '{' ? '}' : tokens[index].value === '=' ? ';' : ')';
    if (end === undefined || tokens[end + 1]?.value !== next) continue;
    do {
      removed.add(begin); removed.add(end);
      begin += 1; end -= 1;
    } while (tokens[begin]?.value === '(' && closing.get(begin) === end);
  }
  let simplified = formatted;
  for (const index of [...removed].sort((left, right) => right - left)) {
    const token = tokens[index];
    simplified = simplified.slice(0, token.start) + simplified.slice(token.end);
  }
  // Eliminate a sole immutable binding whose only use is the block's value.
  // This substitutes the original expression, without changing its evaluation.
  if (simplified === formatted) {
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index].value !== '{' || tokens[index + 1]?.value !== 'let'
          || tokens[index + 2]?.kind !== 'identifier' || tokens[index + 3]?.value !== '=') continue;
      let semicolon = index + 4;
      while (semicolon < tokens.length && tokens[semicolon].value !== ';') {
        if (closing.has(semicolon)) semicolon = closing.get(semicolon);
        semicolon += 1;
      }
      if (tokens[semicolon + 1]?.value !== tokens[index + 2].value || tokens[semicolon + 2]?.value !== '}') continue;
      const begin = tokens[index + 1].start;
      const end = tokens[semicolon + 1].end;
      const expression = formatted.slice(tokens[index + 4].start, tokens[semicolon].start);
      simplified = formatted.slice(0, begin) + expression + formatted.slice(end);
      break;
    }
  }
  // Collapse an else block containing only the original conditional.
  if (simplified === formatted) {
    for (let index = 0; index < tokens.length; index += 1) {
      if (tokens[index].value !== 'else' || tokens[index + 1]?.value !== '{' || tokens[index + 2]?.value !== 'if') continue;
      const end = closing.get(index + 1);
      if (end === undefined || tokens[end - 1]?.value !== '}') continue;
      simplified = formatted.slice(0, tokens[index + 1].start)
        + formatted.slice(tokens[index + 2].start, tokens[end].start).trimEnd()
        + formatted.slice(tokens[end].end);
      break;
    }
  }
  if (simplified === formatted) break;
  decorators.push({ id: `rust-string-reviewed-expression-style-${round}`, level: 'emitter', order: 100 + round,
    when: [['format', 'Rust'], ['scope', 'formatted-module']],
    actions: [{ op: 'replace', field: 'source', from: formatted, to: simplified }] });
  formatted = format(simplified);
  if (round === 9) throw new Error('reviewed string expression styles did not converge');
  }
  const compilerStyles = DecoratorSet.fromLino(readFileSync(stylesPath, 'utf8')).decorators
    .filter(decorator => decorator.id === 'compiler-feedback-style');
  writeFileSync(stylesPath, new DecoratorSet([...decorators, ...compilerStyles]).toLino());
}

const styles = DecoratorSet.fromLino(readFileSync(stylesPath, 'utf8'));
let formatted = format(styles.decorate('emitter', { format: 'Rust', scope: 'raw-module', source: raw }).source);
for (let round = 0; round < 10; round += 1) {
  const next = format(styles.decorate('emitter', { format: 'Rust', scope: 'formatted-module', source: formatted }).source);
  if (next === formatted) break;
  formatted = next;
  if (round === 9) throw new Error('string lexical style decorators do not converge');
}
const expected = '// Generated from js/src/translation/rust-string-rules.js by\n'
  + '// js/scripts/generate-rust-string-rules.mjs. Do not edit by hand.\n' + formatted;
if (process.argv.includes('--check')) {
  if (readFileSync(outputPath, 'utf8') !== expected) throw new Error('Rust string lexical rules are stale');
} else writeFileSync(outputPath, expected);
