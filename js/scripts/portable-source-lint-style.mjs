// Recorded source styles for the checked library's existing Number model.
// No native decision body or operation is supplied by this adapter.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock, simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';
import {simplifyPortableProductLibraryStyles} from './product-rule-style-corrections.mjs';
import {flattenRustSourceHeaderStyleFormat, stripRustSourceHeaderStyleParentheses} from './rust-source-header-library-style.mjs';

function readPortableLintTokens(source) {
  const tokens = tokenize(source, 'Rust').tokens, matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[index].value)) stack.push(index);
    else if ([')', ']', '}'].includes(tokens[index].value)) {
      const start = stack.pop();
      assert.notEqual(start, undefined, 'a checked library delimiter has its opening');
      matching.set(start, index); matching.set(index, start);
    }
  }
  assert.equal(stack.length, 0);
  return {tokens, matching};
}

function removePortableTemporaryCopies(source) {
  const {tokens, matching} = readPortableLintTokens(source), edits = [];
  const counts = new Map();
  for (const token of tokens) if (token.kind === 'identifier') counts.set(token.value, (counts.get(token.value) ?? 0) + 1);
  const bindings = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].value === 'let') {
      const name = tokens[index + 1]?.value === 'mut' ? tokens[index + 2] : tokens[index + 1];
      if (name?.kind === 'identifier') bindings.add(name.value);
    }
    if (tokens[index].value !== '.' || tokens[index + 1]?.value !== 'clone'
      || tokens[index + 2]?.value !== '(' || tokens[index + 3]?.value !== ')') continue;
    const receiver = tokens[index - 1];
    let removable = receiver?.kind === 'identifier' && counts.get(receiver.value) === 2
      && (bindings.has(receiver.value) || /^ml_result[0-9]+(?:_[0-9]+)?$/u.test(receiver.value))
      && !['.', '::'].includes(tokens[index - 2]?.value);
    if (receiver?.value === ')') {
      let start = matching.get(index - 1);
      if (tokens[start - 1]?.kind === 'identifier') {
        start -= 1;
        while (tokens[start - 1]?.value === '::' && tokens[start - 2]?.kind === 'identifier') start -= 2;
      }
      const expression = stripRustSourceHeaderStyleParentheses(source.slice(tokens[start].start, receiver.end));
      // optional_at returns an owned Option<T>. Cloning its fresh result
      // adds no alias or effect, including when T is a String or vector.
      removable ||= /^(?:[A-Za-z_][A-Za-z_0-9]*::)*ml_array::optional_at\(/u.test(expression);
    }
    if (removable) edits.push([tokens[index].start, tokens[index + 3].end, '']);
  }
  for (const [start, end, text] of edits.sort((left, right) => right[0] - left[0])) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

function simplifyPortableStringFormats(source, stringFunctions) {
  const {tokens, matching} = readPortableLintTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'format'
      || tokens[index + 1]?.value !== '!' || tokens[index + 2]?.value !== '(') continue;
    const end = matching.get(index + 2), original = source.slice(tokens[index].start, tokens[end].end);
    let flattened;
    try {
      const format = JSON.parse(source.slice(tokens[index + 3].start, tokens[index + 3].end));
      if (typeof format !== 'string' || /\{[^{}]*[:!][^{}]*\}/u.test(format)) continue;
      flattened = flattenRustSourceHeaderStyleFormat(original);
    } catch { continue; }
    const literal = /^format!\(("(?:[^"\\]|\\.)*")\)$/u.exec(flattened);
    if (literal) {
      const text = JSON.parse(literal[1]);
      if (!/[{}]/u.test(text.replaceAll('{{', '').replaceAll('}}', ''))) flattened = 'String::from(' + JSON.stringify(text.replaceAll('{{', '{').replaceAll('}}', '}')) + ')';
    }
    const single = /^format!\("\{\}", ([\s\S]+)\)$/u.exec(flattened);
    if (single) {
      const expression = stripRustSourceHeaderStyleParentheses(single[1]);
      const call = /^(?:[A-Za-z_][A-Za-z_0-9]*::)*([A-Za-z_][A-Za-z_0-9]*)\(/u.exec(expression);
      if (/^String::(?:from|new)\(/u.test(expression) || call && stringFunctions.has(call[1])) flattened = expression;
    }
    if (flattened !== original) edits.push([tokens[index].start, tokens[end].end, flattened]);
    index = end;
  }
  for (const [start, end, text] of edits.sort((left, right) => right[0] - left[0])) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

/** Preserve IEEE equality, separate arithmetic rounding and existing guarded
 * index conversions; correct only mechanical styles of actual checked output. */
export function readPortableSourceLintStyles(source) {
  let adapted = source;
  const actions = [];
  const replace = (from, to) => {
    if (from === to) return;
    assert.ok(adapted.includes(from), 'a library lint style retains its exact source context');
    actions.push({op: 'replace', field: 'source', from, to});
    adapted = adapted.split(from).join(to);
  };
  const header = /^#!\[allow\([\s\S]*?\)\]/mu.exec(adapted)?.[0];
  if (header) {
    const frontend = readFileSync(new URL('../../rust/src/translation/frontend_rules.rs', import.meta.url), 'utf8');
    const model = ['clippy::float_cmp', 'clippy::suboptimal_flops', 'clippy::cast_precision_loss', 'clippy::cast_possible_truncation', 'clippy::cast_sign_loss'];
    for (const lint of model) assert.ok(frontend.includes(lint), 'the existing frontend records the same Number model: ' + lint);
    const missing = model.filter(lint => !header.includes(lint));
    if (missing.length) replace(header, '// JavaScript Number equality and separate arithmetic rounding are preserved.\n// Array index casts retain the existing checked runtime guards.\n' + header.slice(0, -2).trimEnd().replace(/,$/u, '') + ', ' + missing.join(', ') + ')]');
  }
  const stringFunctions = new Set([...adapted.matchAll(/^pub fn (\w+)\([^)]*\)\s*->\s*String\s*\{/gmu)].map(match => match[1]));
  for (const [, name] of [...adapted.matchAll(/^pub fn (\w+)\(/gmu)]) {
    const original = readSourceTextRustBlock(adapted, 'fn', name);
    let styled = simplifySourceTextRustParentheses(original);
    styled = simplifyPortableProductLibraryStyles(styled);
    styled = removePortableTemporaryCopies(styled);
    styled = simplifyPortableStringFormats(styled, stringFunctions);
    const codeStarts = new Set(readPortableLintTokens(styled).tokens.map(token => token.start));
    styled = styled.replace(/\(([A-Za-z_][A-Za-z_0-9]*)\s*>=\s*(-?[0-9]+(?:\.[0-9]+)?f64)\)\s*&&\s*\(\1\s*(<=|<)\s*(-?[0-9]+(?:\.[0-9]+)?f64)\)/gu,
      (match, value, lower, upperOperation, upper, offset) => codeStarts.has(offset)
        ? `(${lower}..${upperOperation === '<=' ? '=' : ''}${upper}).contains(&${value})` : match);
    // Do not change constant values or target-source strings: digit grouping
    // is applied only to actual numeric tokens of this generated function.
    const digits = readPortableLintTokens(styled).tokens.filter(token => token.kind === 'number');
    for (const token of digits.reverse()) {
      const text = styled.slice(token.start, token.end);
      const grouped = text.replace(/^([0-9]{5,})(.*)$/u, (_, integer, suffix) => integer.replace(/\B(?=(?:[0-9]{3})+$)/gu, '_') + suffix);
      if (grouped !== text) styled = styled.slice(0, token.start) + grouped + styled.slice(token.end);
    }
    const signature = /^pub fn \w+\(([^)]*)\)/u.exec(styled);
    const attributes = [];
    if (!/#\[must_use\]\s*$/u.test(adapted.slice(0, adapted.indexOf(original)))) attributes.push('/// Execute the complete checked source algorithm.\n#[must_use]');
    if (signature[1].split(',').filter(value => value.trim()).length > 7) attributes.push('// The checked primitive-column interface retains every ordered source argument.\n#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]');
    if (attributes.length) styled = attributes.join('\n') + '\n' + styled;
    replace(original, styled);
  }
  // Runtime fragments retain their original generic bodies. Their public
  // results are useful, and decimal grouping changes only numeric spelling.
  const runtime = adapted.replace(/^( +)pub fn /gmu, (match, indent, offset) =>
    /#\[must_use\]\s*$/u.test(adapted.slice(0, offset)) ? match : indent + '#[must_use]\n' + match);
  replace(adapted, runtime);
  const numberTokens = readPortableLintTokens(adapted).tokens.filter(token => token.kind === 'number');
  let numeric = adapted;
  for (const token of numberTokens.reverse()) {
    const text = adapted.slice(token.start, token.end);
    const grouped = text.replace(/^([0-9]{5,})(.*)$/u, (_, integer, suffix) => integer.replace(/\B(?=(?:[0-9]{3})+$)/gu, '_') + suffix);
    if (grouped !== text) numeric = numeric.slice(0, token.start) + grouped + numeric.slice(token.end);
  }
  replace(adapted, numeric);
  return {source: adapted, actions};
}
