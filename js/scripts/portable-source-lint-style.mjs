// Recorded source styles for the checked library's existing Number model.
// No native decision body or operation is supplied by this adapter.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock, simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';
import {simplifyPortableProductLibraryStyles, simplifyRustOptionIdentityMatches} from './reviewed-rust-branch-styles.mjs';
import {flattenRustSourceHeaderStyleFormat, stripRustSourceHeaderStyleParentheses} from './rust-source-header-library-style.mjs';
import {flattenFamilyLibraryElseIf, mergeFamilyLibraryIdenticalBranches, qualifyFamilyLibraryBooleanFunctions,
  invertFamilyLibraryInequalityBranches, bindFamilyLibraryInequalityConditions,
  simplifyFamilyLibraryNegatedConjunctions, liftFamilyLibraryTruthinessConditions} from './portable-conditional-source-style.mjs';
import {readPortableRuntimeSourceStyles} from './portable-runtime-source-style.mjs';
import {readPortableNumberSourceStyles} from './portable-number-source-style.mjs';
import {readPortableOwnedSourceStyles} from './portable-owned-source-style.mjs';

function readPortableLintTokens(source) {
  const {tokens, comments} = tokenize(source, 'Rust'), matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[index].value)) stack.push(index);
    else if ([')', ']', '}'].includes(tokens[index].value)) {
      const start = stack.pop();
      assert.notEqual(start, undefined, 'a checked library delimiter has its opening');
      assert.equal(tokens[start].value, ({')': '(', ']': '[', '}': '{'})[tokens[index].value]);
      matching.set(start, index); matching.set(index, start);
    }
  }
  assert.equal(stack.length, 0);
  const opaque = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    const attribute = tokens[index].kind === 'punct' && tokens[index].value === '#';
    const macro = tokens[index].kind === 'macro' && !['format', 'vec'].includes(tokens[index].value);
    const namedMacro = macro && tokens[index].value === 'macro_rules' && tokens[index + 1]?.kind === 'identifier';
    const open = index + (namedMacro || attribute && tokens[index + 1]?.value === '!' ? 2 : 1);
    if (!(attribute || macro) || !matching.has(open)) continue;
    for (let at = index; at <= matching.get(open); at += 1) opaque.add(at);
  }
  return {tokens, comments, matching, opaque};
}

function removePortableTemporaryCopies(source) {
  const {tokens, matching, opaque} = readPortableLintTokens(source), edits = [];
  const counts = new Map();
  for (const token of tokens) if (token.kind === 'identifier') counts.set(token.value, (counts.get(token.value) ?? 0) + 1);
  const bindings = new Set();
  const numericOptions = new Set();
  const declarationCounts = new Map();
  const header = /^pub fn \w+\(([^)]*)\)/u.exec(source)?.[1] ?? '';
  const numberColumns = new Set([...header.matchAll(/(?:^|,)\s*(?:mut\s+)?(\w+):\s*(?:&\[f64\]|Vec<f64>)(?=\s*(?:,|$))/gu)].map(match => match[1]));
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].value !== 'let') continue;
    const name = tokens[index + 1]?.value === 'mut' ? index + 2 : index + 1;
    if (tokens[name]?.kind !== 'identifier') continue;
    declarationCounts.set(tokens[name].value, (declarationCounts.get(tokens[name].value) ?? 0) + 1);
    if (tokens[name + 1]?.value !== '=') continue;
    let end = name + 2;
    while (end < tokens.length && ![';', ')', ']', '}'].includes(tokens[end].value)) end = matching.get(end) > end ? matching.get(end) + 1 : end + 1;
    if (tokens[end]?.value !== ';') continue;
    const expression = source.slice(tokens[name + 2].start, tokens[end]?.start).trim();
    // optional_at on an actual Number column has Option<f64> result. Its
    // copies are Copy values even if read repeatedly; Option<Text> stays owned.
    const column = /^(?:self::|crate::)?ml_array::optional_at\(\s*([A-Za-z_][A-Za-z_0-9]*)\s*,/u.exec(expression);
    if (column && numberColumns.has(column[1])) numericOptions.add(tokens[name].value);
  }
  for (const name of numericOptions) if (declarationCounts.get(name) !== 1) numericOptions.delete(name);
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index)) continue;
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
    removable ||= receiver?.kind === 'identifier' && numericOptions.has(receiver.value)
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

/** Remove an untyped binding immediately returned by its own enclosing block.
 * The initializer is evaluated once in the same scope and no comment is lost. */
function simplifyPortableReturnedBindings(source) {
  for (;;) {
    const {tokens, matching, opaque} = readPortableLintTokens(source);
    let edit;
    for (let index = 0; index < tokens.length; index += 1) {
      if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'let'
        || tokens[index + 1]?.kind !== 'identifier' || tokens[index + 2]?.value !== '=') continue;
      let end = index + 3;
      while (end < tokens.length && ![';', ')', ']', '}'].includes(tokens[end].value)) end = matching.get(end) > end ? matching.get(end) + 1 : end + 1;
      if (tokens[end]?.value !== ';') continue;
      if (tokens[end + 1]?.kind !== 'identifier' || tokens[end + 1].value !== tokens[index + 1].value
        || tokens[end + 2]?.value !== '}') continue;
      if (!/^let\s+[A-Za-z_][A-Za-z_0-9]*\s*=\s*$/u.test(source.slice(tokens[index].start, tokens[index + 3].start))
        || !/^;\s*[A-Za-z_][A-Za-z_0-9]*\s*$/u.test(source.slice(tokens[end].start, tokens[end + 2].start))) continue;
      edit = [tokens[index].start, tokens[end + 1].end, source.slice(tokens[index + 3].start, tokens[end].start)];
      break;
    }
    if (!edit) return source;
    source = source.slice(0, edit[0]) + edit[2] + source.slice(edit[1]);
  }
}

/** Keep a Boolean Option's exact present-value function and false fallback. */
function simplifyPortableBooleanOptionMatches(source, functionNames) {
  const {tokens, comments, matching, opaque} = readPortableLintTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'match') continue;
    let open = index + 1;
    while (open < tokens.length && !['{', ';', ')', ']', '}'].includes(tokens[open].value)) open = matching.get(open) > open ? matching.get(open) + 1 : open + 1;
    if (tokens[open]?.value !== '{') continue;
    const close = matching.get(open), begin = open + 1;
    if (close === undefined || tokens[begin]?.value !== 'Some' || tokens[begin + 1]?.value !== '('
      || tokens[begin + 2]?.kind !== 'identifier' || tokens[begin + 3]?.value !== ')'
      || tokens[begin + 4]?.value !== '=>') continue;
    let target = begin + 5;
    if (!['self', 'crate'].includes(tokens[target]?.value) || tokens[target + 1]?.value !== '::') continue;
    while (tokens[target + 1]?.value === '::' && tokens[target + 2]?.kind === 'identifier') target += 2;
    if (!functionNames.has(tokens[target].value) || tokens[target + 1]?.value !== '('
      || tokens[target + 2]?.value !== tokens[begin + 2].value || tokens[target + 3]?.value !== ')') continue;
    const tail = tokens.slice(target + 4, close).map(token => token.value);
    if (tail.at(-1) === ',') tail.pop();
    if (tail.join(' ') !== ', None => false') continue;
    if (comments.some(comment => comment.start >= tokens[index].start && comment.end <= tokens[close].end)) continue;
    const selected = source.slice(tokens[index + 1].start, tokens[open].start).trim();
    const path = source.slice(tokens[begin + 5].start, tokens[target].end);
    edits.push([tokens[index].start, tokens[close].end, '(' + selected + ').is_some_and(' + path + ')']);
    index = close;
  }
  for (const [start, end, text] of edits.reverse()) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

/** Known String and &str receivers admit is_empty without evaluating their
 * expression twice. Type evidence comes from actual parameters/initializers. */
function simplifyPortableEmptyStringComparisons(source, stringFunctions) {
  const {tokens, comments, matching, opaque} = readPortableLintTokens(source), edits = [];
  const strings = new Set(), declarations = new Map(), counts = new Map();
  const header = /^pub fn \w+\(([^)]*)\)/u.exec(source)?.[1] ?? '';
  for (const [, name] of header.matchAll(/(?:^|,)\s*(?:mut\s+)?(\w+):\s*(?:String|&str)(?=\s*(?:,|$))/gu)) strings.add(name);
  const known = expression => {
    const bare = stripRustSourceHeaderStyleParentheses(expression);
    if (strings.has(bare) || /^String::(?:from|new)\(/u.test(bare)) return true;
    const call = /^(?:self::|crate::)?([A-Za-z_][A-Za-z_0-9]*)\(/u.exec(bare);
    if (call && stringFunctions.has(call[1])) return true;
    const copied = /^(\w+)\.(?:clone|to_owned)\(\)$/u.exec(bare);
    return copied !== null && strings.has(copied[1]);
  };
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].value !== 'let') continue;
    const name = tokens[index + 1]?.value === 'mut' ? index + 2 : index + 1;
    if (tokens[name]?.kind !== 'identifier') continue;
    counts.set(tokens[name].value, (counts.get(tokens[name].value) ?? 0) + 1);
    if (tokens[name + 1]?.value !== '=') continue;
    let end = name + 2;
    while (end < tokens.length && ![';', ')', ']', '}'].includes(tokens[end].value)) end = matching.get(end) > end ? matching.get(end) + 1 : end + 1;
    if (tokens[end]?.value !== ';') continue;
    declarations.set(tokens[name].value, source.slice(tokens[name + 2].start, tokens[end]?.start).trim());
  }
  for (const [name, count] of counts) if (count !== 1) strings.delete(name);
  for (let pass = 0; pass < declarations.size; pass += 1) {
    let changed = false;
    for (const [name, expression] of declarations) if (counts.get(name) === 1 && !strings.has(name) && known(expression)) {
      strings.add(name); changed = true;
    }
    if (!changed) break;
  }
  for (let index = 1; index + 1 < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].kind !== 'punct' || !['==', '!='].includes(tokens[index].value)
      || tokens[index + 1]?.kind !== 'string' || tokens[index + 1].value !== '') continue;
    let start = index - 1;
    if (tokens[start].value === ')') {
      start = matching.get(start);
      if (tokens[start - 1]?.kind === 'identifier') {
        start -= 1;
        while (tokens[start - 1]?.value === '::' && tokens[start - 2]?.kind === 'identifier') start -= 2;
      }
    }
    const expression = source.slice(tokens[start].start, tokens[index].start).trim();
    if (!known(expression)) continue;
    if (comments.some(comment => comment.start >= tokens[start].start && comment.end <= tokens[index + 1].end)) continue;
    edits.push([tokens[start].start, tokens[index + 1].end, (tokens[index].value === '!=' ? '!' : '') + '(' + expression + ').is_empty()']);
    index += 1;
  }
  for (const [start, end, text] of edits.reverse()) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

function simplifyPortableStringFormats(source, stringFunctions) {
  const {tokens, comments, matching, opaque} = readPortableLintTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index)) continue;
    if (tokens[index].kind === 'macro' && !['format', 'vec'].includes(tokens[index].value)
      && matching.has(index + 1)) { index = matching.get(index + 1); continue; }
    if (tokens[index].kind !== 'macro' || tokens[index].value !== 'format'
      || tokens[index + 1]?.kind !== 'punct' || tokens[index + 1].value !== '(') continue;
    const end = matching.get(index + 1), original = source.slice(tokens[index].start, tokens[end].end);
    if (comments.some(comment => comment.start >= tokens[index].start && comment.end <= tokens[end].end)) continue;
    let flattened;
    try {
      const format = JSON.parse(source.slice(tokens[index + 2].start, tokens[index + 2].end));
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

/** Edit only complete known Rust expressions, keeping other macro bodies and
 * all literal/comment bytes opaque. These forms neither duplicate operands
 * nor make a previously lazy fallback eager. */
function simplifyPortableMechanicalExpressions(source, functionNames) {
  const {tokens, comments, matching, opaque} = readPortableLintTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index)) continue;
    const token = tokens[index];
    if (token.kind === 'macro' && !['format', 'vec'].includes(token.value)
      && matching.has(index + 1)) { index = matching.get(index + 1); continue; }
    const part = tokens.slice(index, index + 6);
    if (part[0]?.kind === 'identifier' && part[0].value === 'String'
      && part[1]?.kind === 'punct' && part[1].value === '::'
      && part[2]?.kind === 'identifier' && part[2].value === 'from'
      && part[3]?.kind === 'punct' && part[3].value === '('
      && part[4]?.kind === 'string' && part[4].value === ''
      && part[5]?.kind === 'punct' && part[5].value === ')') {
      if (comments.some(comment => comment.start >= token.start && comment.end <= part[5].end)) continue;
      edits.push([token.start, part[5].end, 'String::new()']); index += 5; continue;
    }
    // A zero-argument closure that calls a declared function path captures no
    // state. Passing that same function item keeps invocation lazy and ordered.
    if (token.kind !== 'punct' || token.value !== '||') continue;
    let end = index + 1;
    if (tokens[end]?.kind !== 'identifier') continue;
    while (tokens[end + 1]?.kind === 'punct' && tokens[end + 1].value === '::'
      && tokens[end + 2]?.kind === 'identifier') end += 2;
    if (!functionNames.has(tokens[end].value) || tokens[end + 1]?.value !== '('
      || tokens[end + 2]?.value !== ')' || ![',', ')'].includes(tokens[end + 3]?.value)) continue;
    const path = source.slice(tokens[index + 1].start, tokens[end].end);
    if (!/^(?:self|crate)::/u.test(path)) continue;
    if (comments.some(comment => comment.start >= token.start && comment.end <= tokens[end + 2].end)) continue;
    edits.push([token.start, tokens[end + 2].end, path]); index = end + 2;
  }
  for (const [start, end, text] of edits.reverse()) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

/** A branch containing only panic! is a failed assertion. Reuse its actual
 * condition and panic arguments once, then retain the original success tail. */
function simplifyPortablePanicBranches(source) {
  for (;;) {
    const {tokens, comments, matching, opaque} = readPortableLintTokens(source);
    let edit;
    for (let index = 0; index < tokens.length; index += 1) {
      if (opaque.has(index)) continue;
      if (tokens[index].kind === 'macro' && matching.has(index + 1)) { index = matching.get(index + 1); continue; }
      if (tokens[index].kind !== 'identifier' || tokens[index].value !== 'if') continue;
      let open = index + 1;
      while (open < tokens.length && !(tokens[open].kind === 'punct' && tokens[open].value === '{')) {
        if ([';', ')', ']', '}'].includes(tokens[open].value)) break;
        open = matching.get(open) > open ? matching.get(open) + 1 : open + 1;
      }
      if (tokens[open]?.value !== '{') continue;
      const close = matching.get(open);
      if (tokens.slice(index + 1, open).some(token => token.kind === 'identifier' && token.value === 'let')) continue;
      if (close === undefined || tokens[open + 1]?.kind !== 'macro' || tokens[open + 1].value !== 'panic'
        || tokens[open + 2]?.value !== '(') continue;
      const panicEnd = matching.get(open + 2);
      if (panicEnd !== close - 1 && !(panicEnd === close - 2 && tokens[close - 1]?.value === ';')) continue;
      if (tokens[close + 1]?.value !== 'else' || tokens[close + 2]?.value !== '{') continue;
      const end = matching.get(close + 2);
      if (end === undefined || panicEnd === open + 3) continue;
      if (comments.some(comment => comment.start >= tokens[index].start && comment.end <= tokens[end].end)) continue;
      const condition = source.slice(tokens[index + 1].start, tokens[open].start).trim();
      const arguments_ = source.slice(tokens[open + 2].end, tokens[panicEnd].start);
      const success = source.slice(tokens[close + 2].end, tokens[end].start);
      edit = [tokens[index].start, tokens[end].end,
        '{\nassert!(!(' + condition + '), ' + arguments_ + ');\n' + success + '\n}'];
      break;
    }
    if (!edit) return source;
    source = source.slice(0, edit[0]) + edit[2] + source.slice(edit[1]);
  }
}

/** Attributes are one attached group even when rustfmt expands their tokens. */
function acceptPortableMustUseAttribute(prefix) {
  const {tokens} = tokenize(prefix, 'Rust');
  let index = tokens.length - 2;
  while (tokens[index]?.kind === 'punct' && tokens[index].value === ']') {
    let open = index - 1, depth = 1;
    for (; open >= 0; open -= 1) {
      if (tokens[open].kind !== 'punct') continue;
      if (tokens[open].value === ']') depth += 1;
      else if (tokens[open].value === '[' && --depth === 0) break;
    }
    if (depth !== 0) return false;
    if (tokens[open - 1]?.kind !== 'punct' || tokens[open - 1].value !== '#') break;
    if (tokens[open + 1]?.value === 'must_use' && open + 2 === index) return true;
    index = open - 2;
  }
  return false;
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
  // Lift the checked truthiness block while its exact IEEE test remains in
  // source form; the complete Number formatter has no such generated block.
  replace(adapted, liftFamilyLibraryTruthinessConditions(adapted));
  const numberStyles = readPortableNumberSourceStyles(adapted);
  for (const action of numberStyles.actions) replace(action.from, action.to);
  assert.equal(adapted, numberStyles.source, 'Number styles replay their complete checked source actions');
  const stringFunctions = new Set([...adapted.matchAll(/^pub fn (\w+)\([^)]*\)\s*->\s*String\s*\{/gmu)].map(match => match[1]));
  const allTokens = readPortableLintTokens(adapted);
  const functionNames = new Set(allTokens.tokens.flatMap((token, index) =>
    !allTokens.opaque.has(index) && token.kind === 'identifier' && token.value === 'fn'
      && allTokens.tokens[index + 1]?.kind === 'identifier' ? [allTokens.tokens[index + 1].value] : []));
  for (const [, name] of [...adapted.matchAll(/^pub fn (\w+)\(/gmu)]) {
    const original = readSourceTextRustBlock(adapted, 'fn', name);
    let styled = simplifySourceTextRustParentheses(original);
    const reviewed = simplifyPortableProductLibraryStyles(styled);
    // The frozen historical return producer selects token spans and can omit
    // an interstitial comment. Keep its export/body exact; decline only that
    // candidate when the portable derivative would lose any comment bytes.
    const comments = text => tokenize(text, 'Rust').comments.map(comment => comment.text).sort();
    if (JSON.stringify(comments(styled)) === JSON.stringify(comments(reviewed))) styled = reviewed;
    if (readPortableLintTokens(styled).opaque.size === 0) styled = simplifyRustOptionIdentityMatches(styled);
    styled = removePortableTemporaryCopies(styled);
    styled = simplifyPortableBooleanOptionMatches(styled, functionNames);
    styled = simplifyPortableStringFormats(styled, stringFunctions);
    styled = simplifyPortablePanicBranches(styled);
    styled = simplifyPortableMechanicalExpressions(styled, functionNames);
    styled = simplifyPortableReturnedBindings(styled);
    styled = simplifyPortableEmptyStringComparisons(styled, stringFunctions);
    styled = liftFamilyLibraryTruthinessConditions(styled);
    styled = flattenFamilyLibraryElseIf(styled);
    styled = mergeFamilyLibraryIdenticalBranches(styled);
    styled = invertFamilyLibraryInequalityBranches(styled, {context: adapted});
    styled = bindFamilyLibraryInequalityConditions(styled);
    styled = simplifyFamilyLibraryNegatedConjunctions(styled);
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
    if (!acceptPortableMustUseAttribute(adapted.slice(0, adapted.indexOf(original)))) attributes.push('/// Execute the complete checked source algorithm.\n#[must_use]');
    if (signature[1].split(',').filter(value => value.trim()).length > 7) attributes.push('// The checked primitive-column interface retains every ordered source argument.\n#[expect(clippy::too_many_arguments, reason = "checked source arguments and captured continuation state retain their original order")]');
    if (attributes.length) styled = attributes.join('\n') + '\n' + styled;
    replace(original, styled);
  }
  // Ownership needs complete function returns and enum payload declarations.
  // Inspect once before const qualification or runtime-provider spelling edits.
  const ownedStyles = readPortableOwnedSourceStyles(adapted);
  for (const action of ownedStyles.actions) replace(action.from, action.to);
  assert.equal(adapted, ownedStyles.source, 'ownership styles replay their complete declared source actions');
  // Runtime fragments retain their original generic bodies. Their public
  // results are useful, and decimal grouping changes only numeric spelling.
  const runtime = adapted.replace(/^( +)pub fn /gmu, (match, indent, offset) =>
    acceptPortableMustUseAttribute(adapted.slice(0, offset)) ? match : indent + '#[must_use]\n' + match);
  replace(adapted, runtime);
  replace(adapted, simplifyPortableMechanicalExpressions(adapted, functionNames));
  replace(adapted, qualifyFamilyLibraryBooleanFunctions(adapted));
  for (const read of [readPortableRuntimeSourceStyles]) {
    const styled = read(adapted);
    for (const action of styled.actions) replace(action.from, action.to);
    assert.equal(adapted, styled.source, 'each shared library style replays its complete recorded actions');
  }
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
