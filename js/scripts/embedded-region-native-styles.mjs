// Bounded native spelling proofs for the actual embedded-region source family.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readRustArrayPrelude} from '../src/translation/emit-rust.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

function readEmbeddedRegionNativeSyntax(source) {
  const {tokens, comments} = tokenize(source, 'Rust'), matching = new Map(), stack = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[at].value)) stack.push(at);
    else if ([')', ']', '}'].includes(tokens[at].value)) {
      const open = stack.pop();
      assert.equal(tokens[open]?.value, {')': '(', ']': '[', '}': '{'}[tokens[at].value],
        'embedded-region styles retain complete balanced source');
      matching.set(open, at); matching.set(at, open);
    }
  }
  assert.equal(stack.length, 0, 'embedded-region styles retain every delimiter');
  const functions = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].value === 'pub' && tokens[at + 1]?.value === 'fn'
      && tokens[at + 2]?.kind === 'identifier' && tokens[at + 3]?.value === '(') {
      const close = matching.get(at + 3);
      let body = close + 1;
      while (body < tokens.length && !['{', ';'].includes(tokens[body].value)) body += 1;
      const end = matching.get(body);
      if (end === undefined) continue;
      functions.push({at, name: tokens[at + 2].value, open: at + 3, close, body, end});
      at = end;
    } else if (tokens[at].kind === 'punct' && ['{', '[', '('].includes(tokens[at].value)) at = matching.get(at);
  }
  return {tokens, comments, matching, functions};
}

function readEmbeddedRegionNativeParameters(source, syntax, fn) {
  const {tokens, matching} = syntax, parameters = new Map();
  let start = fn.open + 1;
  for (let at = start; at <= fn.close; at += 1) {
    if (at !== fn.close && matching.get(at) > at) { at = matching.get(at); continue; }
    if (at !== fn.close && tokens[at].value !== ',') continue;
    if (start !== at) {
      const name = tokens[start]?.value === 'mut' ? start + 1 : start;
      if (tokens[name]?.kind !== 'identifier' || tokens[name + 1]?.value !== ':'
        || parameters.has(tokens[name].value) || name + 2 >= at) return undefined;
      parameters.set(tokens[name].value, source.slice(tokens[name + 2].start, tokens[at - 1].end).replace(/\s+/gu, ''));
    }
    start = at + 1;
  }
  return parameters;
}

function acceptEmbeddedRegionPrimitiveBody(syntax, fn, parameters) {
  const {tokens, matching} = syntax;
  if (!parameters || [...parameters.values()].some(type => !['f64', 'bool'].includes(type))
    || tokens[fn.close + 1]?.value !== '->' || !['f64', 'bool'].includes(tokens[fn.close + 2]?.value)
    || fn.body !== fn.close + 3) return false;
  let at = fn.body + 1;
  const precedence = new Map([['||', 1], ['&&', 2], ['==', 3], ['!=', 3], ['<', 3], ['<=', 3], ['>', 3], ['>=', 3],
    ['+', 4], ['-', 4], ['*', 5], ['/', 5], ['%', 5]]);
  const fail = () => { throw new Error('unsupported embedded-region primitive expression'); };
  const primary = () => {
    const token = tokens[at++];
    if (token?.value === '(' || token?.value === '{') {
      const end = matching.get(at - 1), type = expression(0);
      if (at !== end) fail();
      at += 1; return type;
    }
    if (token?.value === 'if') {
      if (expression(0) !== 'bool' || tokens[at]?.value !== '{') fail();
      const first = primary();
      if (tokens[at++]?.value !== 'else' || !['{', 'if'].includes(tokens[at]?.value)) fail();
      const second = primary();
      if (first !== second) fail();
      return first;
    }
    if (token?.value === '!' || token?.value === '-') {
      const type = primary();
      if (type !== (token.value === '!' ? 'bool' : 'f64')) fail();
      return type;
    }
    if (token?.kind === 'number' && /f64$/u.test(token.raw)) return 'f64';
    if (token?.kind === 'identifier' && ['true', 'false'].includes(token.value)) return 'bool';
    if (token?.kind === 'identifier' && parameters.has(token.value)) return parameters.get(token.value);
    fail();
  };
  const expression = minimum => {
    let type = primary();
    while (precedence.has(tokens[at]?.value) && precedence.get(tokens[at].value) >= minimum) {
      const operation = tokens[at++].value, right = expression(precedence.get(operation) + 1);
      if (['&&', '||'].includes(operation)) {
        if (type !== 'bool' || right !== 'bool') fail();
      } else if (['==', '!=', '<', '<=', '>', '>='].includes(operation)) {
        if (type !== 'f64' || right !== 'f64') fail();
        type = 'bool';
      } else if (type !== 'f64' || right !== 'f64') fail();
    }
    return type;
  };
  try { return expression(0) === tokens[fn.close + 2].value && at === fn.end; }
  catch (error) { if (error.message === 'unsupported embedded-region primitive expression') return false; throw error; }
}

function acceptEmbeddedRegionNativeContext(syntax) {
  const {tokens, matching} = syntax;
  for (let at = 0; at < tokens.length; at += 1) {
    const token = tokens[at];
    if (token.kind === 'identifier' && ['use', 'extern', 'unsafe'].includes(token.value)) return false;
    if (token.kind === 'identifier' && ['type', 'struct', 'enum', 'union', 'mod'].includes(token.value)
      && ['f64', 'bool', 'Option', 'std', 'core'].includes(tokens[at + 1]?.value)) return false;
    if (token.kind === 'macro' && (!['vec', 'format', 'panic', 'assert'].includes(token.value)
      || tokens[at - 1]?.value === '::' && !(tokens[at - 2]?.value === 'std' && tokens[at - 3]?.value !== '::'))) return false;
    if (token.value !== '#' || token.kind !== 'punct') continue;
    const open = tokens[at + 1]?.value === '!' ? at + 2 : at + 1;
    if (!['allow', 'expect', 'must_use', 'derive'].includes(tokens[open + 1]?.value)) return false;
    if (tokens[open + 1]?.value === 'derive' && tokens.slice(open + 2, matching.get(open))
      .some(item => item.kind === 'identifier' && !['Clone', 'Debug', 'PartialEq', 'Eq'].includes(item.value))) return false;
    at = matching.get(open);
  }
  return true;
}

function acceptEmbeddedRegionArrayProvider(source, syntax) {
  const {tokens} = syntax;
  if (tokens.filter((token, at) => token.value === 'mod' && tokens[at + 1]?.value === 'ml_array').length !== 1) return false;
  const key = source => JSON.stringify(tokenize(source, 'Rust').tokens.filter(token => token.kind !== 'eof')
    .map(token => [token.kind, token.raw]));
  const provider = readSourceTextRustBlock(source, 'mod', 'ml_array');
  if (tokenize(provider, 'Rust').tokens.filter((token, at, all) => token.value === 'fn' && all[at + 1]?.value === 'optional_at').length !== 1) return false;
  return key(readSourceTextRustBlock(provider, 'fn', 'optional_at'))
    === key(readSourceTextRustBlock(readRustArrayPrelude(true), 'fn', 'optional_at'));
}

function readEmbeddedRegionNumericFallbackEdits(syntax, fn, parameters) {
  const {tokens, matching} = syntax, edits = [];
  if (!parameters || [...parameters.values()].some(type => !['f64', '&[f64]'].includes(type))) return edits;
  for (let at = fn.body + 1; at < fn.end; at += 1) {
    if (tokens[at].kind === 'macro' || tokens[at].kind === 'identifier'
      && ['fn', 'for', 'match', 'move', 'ref', 'unsafe', 'use'].includes(tokens[at].value)) return [];
    if (tokens[at].value !== 'let' || tokens[at].kind !== 'identifier') continue;
    let end = at + 1;
    while (end < fn.end && !['=', ';', '{'].includes(tokens[end].value)) end += 1;
    if (tokens.slice(at + 1, end).some(token => token.kind === 'identifier' && parameters.has(token.value))) return [];
  }
  const strip = (from, to) => {
    while (tokens[from]?.value === '(' && matching.get(from) === to - 1) { from += 1; to -= 1; }
    return [from, to];
  };
  for (let at = fn.body + 1; at + 7 < fn.end; at += 1) {
    if (tokens[at].value !== '.' || tokens[at + 1]?.value !== 'unwrap_or_else' || tokens[at + 2]?.value !== '(') continue;
    const end = matching.get(at + 2);
    if (end !== at + 7 || tokens.slice(at + 3, end).map(token => token.value).join(' ') !== '|| f64 :: NAN'
      || tokens[at - 1]?.value !== ')') continue;
    let from = matching.get(at - 1), to = at;
    if (tokens[from - 1]?.value === 'optional_at') from -= 5;
    [from, to] = strip(from, to);
    if (tokens.slice(from, from + 6).map(token => token.value).join(' ') !== 'self :: ml_array :: optional_at ('
      || matching.get(from + 5) !== to - 1) continue;
    let column = from + 6;
    const comma = tokens.findIndex((token, index) => index >= column && index < to && token.value === ',');
    if (comma < 0) continue;
    const [columnStart, columnEnd] = strip(column, comma);
    if (columnEnd !== columnStart + 1 || parameters.get(tokens[columnStart]?.value) !== '&[f64]') continue;
    const index = comma + 1;
    if (tokens.slice(index, index + 6).map(token => token.value).join(' ') !== 'self :: ml_array :: number_index ('
      || matching.get(index + 5) !== to - 2 || to - 2 !== index + 7
      || parameters.get(tokens[index + 6]?.value) !== 'f64') continue;
    edits.push([tokens[at + 1].start, tokens[at + 1].end, 'unwrap_or'],
      [tokens[at + 3].start, tokens[at + 3].end, '']);
  }
  return edits;
}

/** Qualify two proved primitive bodies and spell only actual numeric NaN defaults eagerly. */
export function readEmbeddedRegionNativeStyles(source) {
  const syntax = readEmbeddedRegionNativeSyntax(source), actions = [];
  if (!acceptEmbeddedRegionNativeContext(syntax)) return {source, actions};
  const qualified = new Set(['read_embedded_region_charge', 'accept_embedded_region_source_coverage']);
  const arrayProvider = acceptEmbeddedRegionArrayProvider(source, syntax);
  let adapted = source;
  for (const fn of syntax.functions) {
    if (syntax.functions.filter(other => other.name === fn.name).length !== 1) continue;
    const start = syntax.tokens[fn.at].start, end = syntax.tokens[fn.end].end;
    const from = source.slice(start, end);
    if (source.split(from).length !== 2) continue;
    const parameters = readEmbeddedRegionNativeParameters(source, syntax, fn), edits = [];
    if (qualified.has(fn.name) && acceptEmbeddedRegionPrimitiveBody(syntax, fn, parameters)) {
      edits.push([syntax.tokens[fn.at + 1].start, syntax.tokens[fn.at + 1].start, 'const ']);
    }
    if (arrayProvider && /^ml_read_embedded_region_ancestor_loop[0-9]+$/u.test(fn.name)) {
      edits.push(...readEmbeddedRegionNumericFallbackEdits(syntax, fn, parameters));
    }
    let to = from;
    for (const [begin, finish, text] of edits.sort((left, right) => right[0] - left[0])) {
      to = to.slice(0, begin - start) + text + to.slice(finish - start);
    }
    if (to !== from) {
      actions.push({op: 'replace', field: 'source', from, to});
      adapted = adapted.split(from).join(to);
    }
  }
  return {source: adapted, actions};
}
