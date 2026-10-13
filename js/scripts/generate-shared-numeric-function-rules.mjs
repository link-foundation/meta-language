// Translate the shared decisions automatically; recorded emitter decorators
// adapt the generated functions to the native borrowed-string library boundary.
import { readFileSync, writeFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import { parseJavaScript } from '../src/translation/javascript.js';
import { checkProgram } from '../src/translation/check.js';
import { emitRust } from '../src/translation/emit-rust.js';
import { DecoratorSet } from '../src/decorators.js';

// Formatting only: parse generated Boolean expressions with Rust precedence.
// This removes emitter parentheses without changing grouped model premises.
function formatDecisionExpression(source) {
  const tokens = source.match(/"(?:[^"\\]|\\.)*"|[a-zA-Z_][a-zA-Z_0-9:]*|[0-9]+(?:_?f64)?|&&|\|\||==|!=|[!(),]/gu) ?? [];
  if (tokens.join('') !== source.replaceAll(/\s/gu, '')) throw new Error('unsupported decision style expression');
  const precedence = { '||': 1, '&&': 2, '==': 3, '!=': 3 };
  let at = 0;
  const expression = (minimum = 0) => {
    let result;
    if (tokens[at] === '!') { at += 1; result = { text: '!' + render(expression(4), 4), precedence: 4 }; }
    else if (tokens[at] === '(') { at += 1; result = expression(); if (tokens[at++] !== ')') throw new Error('unclosed decision style expression'); }
    else {
      const token = tokens[at++]; if (!token) throw new Error('missing decision style operand');
      let text = token;
      if (tokens[at] === '(') {
        at += 1; const argumentsText = [];
        if (tokens[at] !== ')') do { argumentsText.push(render(expression(), 0)); if (tokens[at] !== ',') break; at += 1; } while (true);
        if (tokens[at++] !== ')') throw new Error('unclosed decision style call');
        text += '(' + argumentsText.join(', ') + ')';
      }
      result = { text, precedence: 5 };
    }
    while ((precedence[tokens[at]] ?? -1) >= minimum) {
      const operator = tokens[at++]; const level = precedence[operator]; const right = expression(level + 1);
      result = { text: render(result, level) + ' ' + operator + ' ' + render(right, level), precedence: level };
    }
    return result;
  };
  const render = (value, minimum) => value.precedence < minimum ? '(' + value.text + ')' : value.text;
  const result = render(expression(), 0);
  if (at !== tokens.length) throw new Error('unconsumed decision style expression');
  return result;
}

const source = readFileSync(new URL('../src/shared-numeric-function-rules.js', import.meta.url), 'utf8');
const raw = emitRust(checkProgram(parseJavaScript(source))).definitions.join('\n\n');
const styleFile = new URL('../../parity/self-translation/shared-numeric-function-rules-decorators.lino', import.meta.url);
if (process.argv.includes('--update-styles')) {
  let styled = raw;
  const actions = [];
  const replace = (from, to) => {
    if (styled.includes(from)) { actions.push({ op: 'replace', field: 'source', from, to }); styled = styled.split(from).join(to); }
  };
  for (const parameter of ['kind', 'operation', 'domain', 'semantics', 'language']) replace(`${parameter}: String`, `${parameter}: &str`);
  for (const match of [...styled.matchAll(/\([a-z_]+ (?:==|!=) "[^"\n]*"\)/gu)]) replace(match[0], match[0].slice(1, -1));
  replace('crate::accept_shared_numeric_type(kind.clone())', 'accept_shared_numeric_type(kind)');
  replace('crate::accept_shared_numeric_expression(kind.clone())', 'accept_shared_numeric_expression(kind)');
  replace('(1f64)', '1_f64');
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
    if (depth) throw new Error('a translated formal function decision has an unclosed branch');
    replace(styled.slice(match.index, cursor), `} else ${styled.slice(opening + 1, cursor - 1).trim()}`);
  }
  for (const name of ['accept_shared_numeric_type', 'accept_shared_numeric_expression', 'accept_shared_numeric_unary', 'accept_shared_numeric_binary', 'accept_shared_numeric_type_shape', 'accept_shared_numeric_context', 'accept_shared_numeric_declaration', 'accept_shared_numeric_parameter', 'accept_shared_numeric_entry', 'accept_shared_numeric_expression_shape', 'accept_shared_numeric_literal']) {
    replace(`pub fn ${name}(`, `/// Read a shared formal function graph decision.\n#[must_use]\npub fn ${name}(`);
  }
  for (const line of styled.split('\n')) {
    const condition = /^(\s*(?:\} else )?if )(.+)( \{)$/u.exec(line);
    const tail = /^\s+([^{}]+)$/u.exec(line);
    if (condition) replace(line, condition[1] + formatDecisionExpression(condition[2]) + condition[3]);
    else if (tail) replace(line, line.slice(0, line.length - tail[1].length) + formatDecisionExpression(tail[1]));
  }
  replace('semantics == ""', 'semantics.is_empty()');
  replace('pub fn accept_shared_numeric_parameter(', 'pub const fn accept_shared_numeric_parameter(');
  writeFileSync(styleFile, new DecoratorSet([{ id: 'shared-numeric-function-library-style', level: 'emitter', order: 0,
    when: [['format', 'Rust']], actions }]).toLino());
}
const decorated = DecoratorSet.fromLino(readFileSync(styleFile, 'utf8')).decorate('emitter', { format: 'Rust', source: raw }).source;
const formatted = spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], { input: decorated, encoding: 'utf8', maxBuffer: 1024 * 1024 });
if (formatted.error || formatted.status !== 0) throw formatted.error ?? new Error(formatted.stderr);
const generated = '// Generated automatically from js/src/shared-numeric-function-rules.js.\n'
  + '// Emitter styles are recorded in parity/self-translation/shared-numeric-function-rules-decorators.lino.\n'
  + '// Do not edit by hand.\n' + formatted.stdout;
const output = new URL('../../rust/src/shared_numeric_function_rules.rs', import.meta.url);
if (process.argv.includes('--check')) {
  if (readFileSync(output, 'utf8') !== generated) throw new Error('formal function link decisions are stale');
} else writeFileSync(output, generated);
