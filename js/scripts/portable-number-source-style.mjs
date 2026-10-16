// Recorded styles for positively identified binary64 source expressions.
// The checked source, its operation order and its fixed interfaces stay intact.
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock, simplifySourceTextRustParentheses} from './source-text-byte-library-style.mjs';

function formatterTokens(source) {
  return tokenize(simplifySourceTextRustParentheses(source), 'Rust').tokens
    .filter(token => token.kind !== 'eof').map(token => [token.kind, token.kind === 'number' ? token.raw.replaceAll('_', '') : token.value]);
}

function checkedNumberFormatter() {
  const emitter = readFileSync(new URL('../src/translation/emit-rust.js', import.meta.url), 'utf8');
  const prelude = /const NUMBER_PRELUDE = `([\s\S]*?)`;\n/u.exec(emitter);
  assert.ok(prelude, 'the shared Number runtime retains its literal checked provider');
  return readSourceTextRustBlock(prelude[1], 'fn', 'js_number');
}

function syntax(source) {
  const tokens = tokenize(source, 'Rust').tokens, pairs = new Map(), stack = [], macros = new Set();
  for (let at = 0; at < tokens.length; at += 1) {
    const token = tokens[at];
    if (token.kind !== 'punct') continue;
    if (['(', '[', '{'].includes(token.value)) stack.push(at);
    else if ([')', ']', '}'].includes(token.value)) {
      const start = stack.pop();
      assert.equal(tokens[start]?.value, {')':'(', ']':'[', '}':'{'}[token.value], 'numeric styles require balanced source delimiters');
      pairs.set(start, at); pairs.set(at, start);
    }
  }
  assert.equal(stack.length, 0, 'numeric styles require complete source');
  for (let at = 0; at < tokens.length; at += 1) {
    let open;
    if (tokens[at].kind === 'macro') open = at + (tokens[at].value === 'macro_rules' ? 2 : 1);
    else if (tokens[at].value === '#' && tokens[at].kind === 'punct') open = at + (tokens[at + 1]?.value === '!' ? 2 : 1);
    else if (tokens[at].kind === 'punct' && tokens[at].value === '!' && tokens[at - 1]?.kind === 'identifier'
      && !['if', 'while', 'return', 'match', 'let', 'else', 'break', 'continue', 'loop'].includes(tokens[at - 1].value)) {
      open = at + (tokens[at - 1].value === 'macro_rules' ? 2 : 1);
    } else continue;
    if (!['(', '[', '{'].includes(tokens[open]?.value) || !pairs.has(open)) continue;
    for (let index = open; index <= pairs.get(open); index += 1) macros.add(index);
  }
  const modules = [];
  for (let at = 0; at < tokens.length - 2; at += 1) {
    if (!macros.has(at) && tokens[at].kind === 'identifier' && tokens[at].value === 'mod' && tokens[at + 1].kind === 'identifier' && tokens[at + 2].value === '{') {
      modules.push({name:tokens[at + 1].value, start:at + 2, end:pairs.get(at + 2)});
    }
  }
  const functions = [];
  for (let at = 0; at < tokens.length - 2; at += 1) {
    if (macros.has(at) || tokens[at].kind !== 'identifier' || tokens[at].value !== 'fn' || tokens[at + 1].kind !== 'identifier' || tokens[at + 2].value !== '(') continue;
    const close = pairs.get(at + 2);
    let body = close + 1;
    while (body < tokens.length && !['{', ';'].includes(tokens[body].value)) body += 1;
    if (tokens[body]?.value !== '{') continue;
    const params = [];
    for (let start = at + 3, end = start; end <= close; end += 1) {
      if (['(', '[', '{'].includes(tokens[end]?.value)) { end = pairs.get(end); continue; }
      if (end !== close && tokens[end].value !== ',') continue;
      const part = tokens.slice(start, end), offset = part[0]?.value === 'mut' ? 1 : 0;
      if (part[offset]?.kind === 'identifier' && part[offset + 1]?.value === ':') {
        params.push({name:part[offset].value, type:part.slice(offset + 2).map(token => token.value).join('')});
      }
      start = end + 1;
    }
    const owner = modules.filter(module => module.start < at && at < module.end).map(module => module.name).join('::');
    functions.push({at, name:tokens[at + 1].value, owner, params, body, end:pairs.get(body),
      float:tokens[close + 1]?.value === '->' && tokens[close + 2]?.value === 'f64' && body === close + 3});
  }
  return {tokens, pairs, macros, functions};
}

function topLevel(tokens, pairs, start, end, operation) {
  const found = [];
  for (let at = start; at < end; at += 1) {
    if (['(', '[', '{'].includes(tokens[at].value)) { at = pairs.get(at); continue; }
    if (tokens[at].value === operation) found.push(at);
  }
  return found;
}

/** Only source-proved f64 comparisons are styled; unknown or overloaded types stay untouched. */
export function readPortableNumberSourceStyles(source) {
  const {tokens, pairs, macros, functions} = syntax(source), changes = [];
  let primitive = true;
  for (let at = 0; at < tokens.length - 1; at += 1) {
    if (macros.has(at) || tokens[at].kind !== 'identifier') continue;
    if (['type', 'struct', 'enum', 'trait', 'mod'].includes(tokens[at].value) && tokens[at + 1].value === 'f64') primitive = false;
    if (tokens[at].value === 'use') {
      for (let end = at + 1; end < tokens.length && tokens[end].value !== ';'; end += 1) {
        if (tokens[end].kind === 'identifier' && tokens[end].value === 'f64') primitive = false;
      }
    }
  }
  const slice = (start, end) => source.slice(tokens[start].start, tokens[end - 1].end);
  const strip = (start, end) => {
    while (tokens[start]?.value === '(' && pairs.get(start) === end - 1) { start += 1; end -= 1; }
    return [start, end];
  };
  const groupTokens = (start, end) => {
    const result = [];
    for (let at = start; at < end; at += 1) {
      const token = tokens[at];
      if (token.kind === 'punct' && token.value === '(') {
        const close = pairs.get(at);
        let inner = at, last = close;
        // Only a complete nested group can lose a redundant outer pair. The
        // groups separating addition, division and the method receiver remain.
        while (tokens[inner + 1]?.value === '(' && pairs.get(inner + 1) === last - 1) { inner += 1; last -= 1; }
        result.push('(', ...groupTokens(inner + 1, last), ')');
        at = close;
      } else result.push(token.kind === 'number' ? token.raw.replaceAll('_', '') : token.value);
    }
    return result;
  };
  for (const fn of functions) {
    // Nested functions have their own lexical types and are processed separately.
    const nested = functions.filter(other => fn.body < other.at && other.end < fn.end);
    const excluded = at => macros.has(at) || nested.some(other => other.at <= at && at <= other.end);
    const locals = [], counts = new Map(fn.params.map(param => [param.name, 1]));
    let simpleBindings = primitive && nested.length === 0;
    for (let at = fn.body + 1; at < fn.end; at += 1) {
      if (excluded(at)) continue;
      if (tokens[at].kind === 'identifier' && ['match', 'for'].includes(tokens[at].value) || tokens[at].kind === 'punct' && tokens[at].value === '|') simpleBindings = false;
      if (tokens[at].kind !== 'identifier' || tokens[at].value !== 'let') continue;
      let name = at + 1;
      if (tokens[name]?.value === 'mut') name += 1;
      if (tokens[name]?.value === '(') {
        const close = pairs.get(name), names = tokens.slice(name + 1, close);
        if (tokens[close + 1]?.value === '=' && names.every(token => token.kind === 'identifier' || token.value === ',')) {
          for (const token of names) if (token.kind === 'identifier' && token.value !== 'mut') counts.set(token.value, (counts.get(token.value) ?? 0) + 1);
        } else simpleBindings = false;
        continue;
      }
      if (tokens[name]?.kind !== 'identifier' || !['=', ':'].includes(tokens[name + 1]?.value)) { simpleBindings = false; continue; }
      let equal = name + 1;
      while (equal < fn.end && !['=', ';', '{'].includes(tokens[equal].value)) equal += 1;
      counts.set(tokens[name].value, (counts.get(tokens[name].value) ?? 0) + 1);
      if (tokens[equal]?.value !== '=') continue;
      let end = equal + 1;
      while (end < fn.end && tokens[end].value !== ';') {
        if (['(', '[', '{'].includes(tokens[end].value)) end = pairs.get(end);
        end += 1;
      }
      locals.push({name:tokens[name].value, at, equal, end,
        explicit:tokens[name + 1]?.value === ':' && tokens[name + 2]?.value === 'f64' && equal === name + 3});
    }
    const numeric = (start, end, before, seen = new Set()) => {
      [start, end] = strip(start, end);
      if (start >= end || tokens.slice(start, end).some((_, index) => excluded(start + index))) return false;
      if (end === start + 1 && tokens[start].kind === 'number') return /f64$/u.test(slice(start, end));
      if (end === start + 2 && tokens[start].value === '-') return numeric(start + 1, end, before, seen);
      if (end === start + 1 && tokens[start].kind === 'identifier') {
        const name = tokens[start].value;
        if (counts.get(name) !== 1 || seen.has(name)) return false;
        if (fn.params.some(param => param.name === name && param.type === 'f64')) return true;
        const local = locals.find(local => local.name === name && local.end < before);
        return Boolean(local && (local.explicit || numeric(local.equal + 1, local.end, local.at, new Set([...seen, name]))));
      }
      if (tokens[end - 2]?.value === 'as' && tokens[end - 1]?.value === 'f64') return true;
      if (tokens[end - 1]?.value === ')' && pairs.has(end - 1)) {
        const open = pairs.get(end - 1), path = tokens.slice(start, open).map(token => token.value).join('');
        const name = path.startsWith('self::') ? path.slice(6) : path;
        if (/^[A-Za-z_][A-Za-z_0-9]*$/u.test(name) && !counts.has(name)) {
          return functions.filter(other => other.owner === fn.owner && other.name === name && other.float).length === 1;
        }
      }
      return false;
    };
    const edits = [];
    const comparison = (start, end, negate) => {
      [start, end] = strip(start, end);
      const operations = negate ? ['<', '>'] : ['=='];
      for (const operation of operations) {
        const positions = topLevel(tokens, pairs, start, end, operation);
        if (positions.length !== 1) continue;
        const at = positions[0];
        if (!numeric(start, at, start) || !numeric(at + 1, end, start)) continue;
        const ordering = operation === '<' ? 'Less' : operation === '>' ? 'Greater' : 'Equal';
        return `(${slice(start, at)}).partial_cmp(&(${slice(at + 1, end)})) ${negate ? '!=' : '=='} Some(std::cmp::Ordering::${ordering})`;
      }
      return null;
    };
    for (let at = fn.body + 1; at < fn.end; at += 1) {
      if (excluded(at) || !simpleBindings) continue;
      if (tokens[at].value === '!' && tokens[at + 1]?.value === '(') {
        const close = pairs.get(at + 1), replacement = comparison(at + 2, close, true);
        if (replacement) { edits.push([tokens[at].start, tokens[close].end, `(${replacement})`]); at = close; }
      } else if (tokens[at].value === '==' && !macros.has(at)) {
        // Equality operands are intentionally restricted to direct f64 names:
        // this cannot cross a call, an overloaded operator or a control boundary.
        const left = at - 1, right = at + 2;
        if (tokens[left]?.kind === 'identifier' && tokens[at + 1]?.kind === 'identifier'
          && ['(', '{', 'return', 'if', '&&', '||', '=>', '=', ';'].includes(tokens[left - 1]?.value)
          && [')', '}', '&&', '||', '?', ',', ';'].includes(tokens[right]?.value)) {
          const replacement = comparison(left, right, false);
          if (replacement) edits.push([tokens[left].start, tokens[at + 1].end, `(${replacement})`]);
        }
      }
    }
    if (fn.owner === 'ml_number' && fn.name === 'js_number') {
      assert.ok(primitive, 'the shared Number formatter retains the primitive binary64 type owner');
      assert.deepEqual(fn.params, [{name:'x', type:'f64'}], 'the checked Number formatter retains its source signature');
      let begin = fn.at;
      while (begin > 0 && ['pub', 'const'].includes(tokens[begin - 1].value)) begin -= 1;
      const actual = source.slice(tokens[begin].start, tokens[fn.end].end), provider = checkedNumberFormatter();
      const corrected = provider.replace('let k = digits.len() as i32;', 'let k = i32::try_from(digits.len()).expect("binary64 significand length fits i32");');
      const actualTokens = formatterTokens(actual);
      assert.ok(JSON.stringify(actualTokens) === JSON.stringify(formatterTokens(provider))
        || JSON.stringify(actualTokens) === JSON.stringify(formatterTokens(corrected)), 'the formatter cast retains the complete shared binary64 algorithm');
      for (const local of locals.filter(local => local.name === 'k')) {
        const value = slice(local.equal + 1, local.end);
        if (/^i32\s*::\s*try_from\s*\(/u.test(value)) continue;
        assert.equal(tokens.slice(local.equal + 1, local.end).map(token => token.value).join(''), 'digits.len()asi32', 'the formatter cast retains its checked significand context');
        assert.ok(locals.some(local => local.name === 'digits') || tokens.slice(fn.body, local.at).some(token => token.value === 'decimal'), 'the formatter owns its decimal significand');
        edits.push([tokens[local.equal + 1].start, tokens[local.end - 1].end, 'i32::try_from(digits.len()).expect("binary64 significand length fits i32")']);
      }
    }
    if (/^ml_read_record_field_insertion_order_loop[0-9]+$/u.test(fn.name) && fn.owner === '') {
      for (const local of locals.filter(local => local.name === 'middle')) {
        const statement = source.slice(tokens[local.at].start, tokens[local.end].end);
        const before = source.slice(tokens[fn.body].end, tokens[local.at].start);
        if (before.trimEnd().endsWith(')]') && /#\[expect\(clippy::manual_midpoint,[^\]]*\)\]\s*$/u.test(before)) continue;
        assert.ok(fn.params.some(param => param.name === 'lower' && param.type === 'f64') && fn.params.some(param => param.name === 'upper' && param.type === 'f64'), 'the midpoint owns its Number bounds');
        assert.equal(groupTokens(local.equal + 1, local.end).join(''), '((lower+upper)/2f64).floor()', 'the source midpoint retains add, divide and floor in their original order');
        edits.push([tokens[local.at].start, tokens[local.at].start, '#[expect(clippy::manual_midpoint, reason = "checked JavaScript Number addition, division and floor retain their separate rounding and evaluation order")]\n']);
        assert.ok(statement.endsWith(';'));
      }
    }
    let start = fn.at;
    while (start > 0 && ['pub', 'const'].includes(tokens[start - 1].value)) start -= 1;
    let original = source.slice(tokens[start].start, tokens[fn.end].end), replacement = original;
    if (fn.name === 'require_optional_result_binding' && fn.owner === '') {
      assert.deepEqual(fn.params.map(param => [param.name, param.type]), [['declared','bool'], ['additional','bool'], ['actual','&str'], ['result','&str'], ['same','bool'], ['recursive','bool']], 'the reviewed optional result interface retains every independent ordered source input');
      const prior = source.slice(0, tokens[start].start);
      if (!/#\[expect\(clippy::fn_params_excessive_bools,[^\]]*\)\]\s*$/u.test(prior)) {
        edits.push([tokens[start].start, tokens[start].start, '#[expect(clippy::fn_params_excessive_bools, reason = "the checked source interface retains independent Boolean and text inputs in their original evaluation order")]\n']);
      }
    }
    for (const [from, to, text] of edits.sort((left, right) => right[0] - left[0])) {
      replacement = replacement.slice(0, from - tokens[start].start) + text + replacement.slice(to - tokens[start].start);
    }
    if (original !== replacement) changes.push({start:tokens[start].start, end:tokens[fn.end].end, from:original, to:replacement});
  }
  let current = source;
  const actions = [];
  let boundary = source.length;
  for (const change of changes.sort((left, right) => right.start - left.start)) {
    assert.ok(change.end <= boundary, 'numeric styles refuse overlapping function owners');
    assert.equal(current.split(change.from).length, 2, 'a recorded numeric source context has exactly one owned occurrence');
    const action = {op:'replace', field:'source', from:change.from, to:change.to};
    actions.push(action);
    current = current.slice(0, change.start) + change.to + current.slice(change.end);
    boundary = change.start;
  }
  return {source:current, actions};
}
