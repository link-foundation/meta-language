// Token-aware conditional syntax for actual checked Rust library output.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {acceptRustWholeConditionNegation} from './reviewed-rust-branch-styles.mjs';

function readFamilyLibraryTokens(source) {
  const tokens = tokenize(source, 'Rust').tokens, matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[index].value)) stack.push(index);
    else if ([')', ']', '}'].includes(tokens[index].value)) {
      const open = stack.pop();
      assert.notEqual(open, undefined, 'a library style retains its opening delimiter');
      assert.equal(tokens[open].value, {')': '(', ']': '[', '}': '{'}[tokens[index].value]);
      matching.set(open, index); matching.set(index, open);
    }
  }
  assert.equal(stack.length, 0, 'a library style retains its closing delimiters');
  const opaque = new Set();
  for (let index = 0; index < tokens.length; index += 1) {
    const macro = tokens[index].kind === 'macro';
    const attribute = tokens[index].value === '#';
    const namedMacro = macro && tokens[index].value === 'macro_rules' && tokens[index + 1]?.kind === 'identifier';
    const open = index + (namedMacro || attribute && tokens[index + 1]?.value === '!' ? 2 : 1);
    if (!(macro || attribute) || !matching.has(open)) continue;
    for (let inner = open; inner <= matching.get(open); inner += 1) opaque.add(inner);
  }
  return {tokens, matching, opaque};
}

function readFamilyLibraryIf(tokens, matching, start) {
  if (tokens[start]?.kind !== 'identifier' || tokens[start].value !== 'if') return undefined;
  let open = start + 1;
  if (tokens[open]?.value === '{' && matching.has(open)) open = matching.get(open) + 1;
  while (open < tokens.length && tokens[open].value !== '{') {
    if (matching.has(open)) open = matching.get(open);
    open += 1;
  }
  if (!matching.has(open)) return undefined;
  const close = matching.get(open);
  let end = close;
  if (tokens[close + 1]?.value === 'else') {
    const alternate = close + 2;
    if (tokens[alternate]?.value === 'if') {
      const nested = readFamilyLibraryIf(tokens, matching, alternate);
      if (!nested) return undefined;
      end = nested.end;
    } else if (tokens[alternate]?.value === '{' && matching.has(alternate)) end = matching.get(alternate);
    else return undefined;
  }
  return {start, open, close, end};
}

function readFamilyLibraryEqualityProof(source, tokens, matching, context) {
  const complete = readFamilyLibraryTokens(context), functions = [], textMethods = new Set(), functionCounts = new Map();
  const contextTokens = complete.tokens, unavailable = new Set();
  for (let at = 0; at < contextTokens.length; at += 1) {
    if (complete.opaque.has(at)) continue;
    if (contextTokens[at].kind === 'identifier' && ['type', 'struct', 'enum', 'trait', 'mod'].includes(contextTokens[at].value)) unavailable.add(contextTokens[at + 1]?.value);
    if (contextTokens[at].kind === 'identifier' && contextTokens[at].value === 'use') {
      for (let end = at + 1; end < contextTokens.length && contextTokens[end].value !== ';'; end += 1) {
        unavailable.add(contextTokens[end].value);
      }
    }
  }
  const spelling = (items, start, end) => items.slice(start, end).map(token => token.value).join('');
  // The generated Text owns a standard u16 vector and derives PartialEq;
  // no hand-written eq/ne override is admitted as a proof for this style.
  const textDeclarations = [];
  for (let at = 0; at < contextTokens.length; at += 1) {
    if (complete.opaque.has(at) || contextTokens[at].value !== 'struct' || contextTokens[at + 1]?.value !== 'Text') continue;
    let previous = at - 1;
    if (contextTokens[previous]?.value === 'pub') previous -= 1;
    const attribute = complete.matching.get(previous);
    if (contextTokens[previous]?.value !== ']' || attribute === undefined || contextTokens[attribute - 1]?.value !== '#') continue;
    const derives = spelling(contextTokens, attribute + 1, previous);
    const close = complete.matching.get(at + 2);
    const body = close === undefined ? '' : spelling(contextTokens, at + 2, close + 2);
    let module;
    for (let index = 0; index < at; index += 1) {
      if (contextTokens[index].kind === 'identifier' && contextTokens[index].value === 'mod' && contextTokens[index + 1]?.value === 'ml_string'
        && contextTokens[index + 2]?.value === '{' && at < complete.matching.get(index + 2)) module = {start: index + 2, end: complete.matching.get(index + 2)};
    }
    if (module && /^derive\([^)]*\bPartialEq\b[^)]*\)$/u.test(derives) && body === '(pubVec<u16>);') textDeclarations.push(module);
  }
  const customTextEquality = contextTokens.some((token, at) => token.value === 'impl'
    && contextTokens.slice(at + 1, at + 20).some(item => item.value === 'PartialEq'));
  const text = textDeclarations.length === 1 && !customTextEquality
    && !['*', 'Vec', 'u16'].some(value => unavailable.has(value));
  const type = value => {
    value = value.replace(/^&(?:mut)?/u, '');
    if (['bool', 'f64', 'str', 'String'].includes(value)) return unavailable.has(value) || unavailable.has('*') ? null : value === 'str' ? 'String' : value;
    if (text && /^(?:self::|crate::)?ml_string::Text$/u.test(value)) return 'Text';
    if (value.startsWith('Vec<') && value.endsWith('>') && !unavailable.has('Vec') && !unavailable.has('*')) {
      const element = type(value.slice(4, -1));
      return element ? 'Vec<' + element + '>' : null;
    }
    if (value.startsWith('[') && value.endsWith(']')) {
      const element = type(value.slice(1, -1));
      return element ? 'Vec<' + element + '>' : null;
    }
    return null;
  };
  for (let at = 0; at < contextTokens.length; at += 1) {
    if (!complete.opaque.has(at) && contextTokens[at].kind === 'identifier' && contextTokens[at].value === 'fn'
      && contextTokens[at + 1]?.kind === 'identifier') {
      const name = contextTokens[at + 1].value;
      functionCounts.set(name, (functionCounts.get(name) ?? 0) + 1);
    }
    if (complete.opaque.has(at) || contextTokens[at].value !== 'fn' || contextTokens[at + 2]?.value !== '(') continue;
    const close = complete.matching.get(at + 2);
    let body = close + 1;
    while (body < contextTokens.length && !['{', ';'].includes(contextTokens[body].value)) body += 1;
    if (contextTokens[body]?.value !== '{') continue;
    if (text && contextTokens[close + 1]?.value === '->' && contextTokens[close + 2]?.value === 'Self' && body === close + 3
      && contextTokens.some((token, index) => token.kind === 'identifier' && token.value === 'impl' && contextTokens[index + 1]?.value === 'Text'
        && contextTokens[index + 2]?.value === '{' && textDeclarations[0].start < index && index < textDeclarations[0].end
        && index + 2 < at && at < complete.matching.get(index + 2))) {
      textMethods.add(contextTokens[at + 1].value);
    }
    if (contextTokens.some((token, index) => token.value === 'mod' && contextTokens[index + 2]?.value === '{'
      && index + 2 < at && at < complete.matching.get(index + 2))) continue;
    let method = false;
    for (let index = 0; index < at; index += 1) {
      if (contextTokens[index].kind !== 'identifier' || contextTokens[index].value !== 'impl') continue;
      let open = index + 1;
      while (open < at && contextTokens[open].value !== '{') open += 1;
      if (open < at && at < complete.matching.get(open)) method = true;
    }
    if (method || functions.some(value => value.body < at && at < value.end)) continue;
    const result = contextTokens[close + 1]?.value === '->' ? type(spelling(contextTokens, close + 2, body)) : null;
    functions.push({name: contextTokens[at + 1].value, result, body, end: complete.matching.get(body)});
  }
  return (left, operator, right, owner) => {
    if (tokens[owner + 2]?.value !== '(') return false;
    const ownerName = tokens[owner + 1].value;
    const rootOwner = functionCounts.get(ownerName) === 1 && functions.some(value => value.name === ownerName);
    const close = matching.get(owner + 2), params = new Map(), locals = [], counts = new Map();
    let body = close + 1;
    while (body < tokens.length && tokens[body].value !== '{') body += 1;
    if (tokens.slice(body + 1, matching.get(body)).some(token => token.kind === 'identifier' && ['fn', 'match', 'for'].includes(token.value)
      || token.kind === 'punct' && token.value === '|')) return false;
    for (let start = owner + 3, end = start; end <= close; end += 1) {
      if (['(', '[', '{'].includes(tokens[end]?.value)) { end = matching.get(end); continue; }
      if (end !== close && tokens[end].value !== ',') continue;
      const name = tokens[start]?.value === 'mut' ? start + 1 : start;
      if (tokens[name]?.kind === 'identifier' && tokens[name + 1]?.value === ':') {
        params.set(tokens[name].value, type(spelling(tokens, name + 2, end)));
        counts.set(tokens[name].value, (counts.get(tokens[name].value) ?? 0) + 1);
      }
      start = end + 1;
    }
    for (let at = body + 1; at < matching.get(body); at += 1) {
      if (tokens[at].value !== 'let') continue;
      const name = tokens[at + 1]?.value === 'mut' ? at + 2 : at + 1;
      if (tokens[name]?.value === '(') {
        for (const token of tokens.slice(name + 1, matching.get(name))) {
          if (token.kind === 'identifier') counts.set(token.value, (counts.get(token.value) ?? 0) + 1);
        }
        continue;
      }
      if (tokens[name]?.kind !== 'identifier' || !['=', ':'].includes(tokens[name + 1]?.value)) return false;
      counts.set(tokens[name].value, (counts.get(tokens[name].value) ?? 0) + 1);
      let equal = name + 1;
      while (equal < tokens.length && !['=', ';', '{'].includes(tokens[equal].value)) equal += 1;
      if (tokens[equal]?.value !== '=') continue;
      let end = equal + 1;
      while (end < tokens.length && tokens[end].value !== ';') {
        if (['(', '[', '{'].includes(tokens[end].value)) end = matching.get(end);
        end += 1;
      }
      locals.push({name: tokens[name].value, at, equal, end,
        explicit: tokens[name + 1]?.value === ':' ? type(spelling(tokens, name + 2, equal)) : null});
    }
    const infer = (start, end, before, seen = new Set()) => {
      while (tokens[start]?.value === '(' && matching.get(start) === end - 1) { start += 1; end -= 1; }
      if (start >= end) return null;
      if (end === start + 1) {
        const token = tokens[start];
        if (token.kind === 'string') return 'String';
        if (token.kind === 'number') return /f64$/u.test(source.slice(token.start, token.end)) ? type('f64') : null;
        if (['true', 'false'].includes(token.value)) return type('bool');
        if (token.kind !== 'identifier' || counts.get(token.value) !== 1 || seen.has(token.value)) return null;
        if (params.has(token.value)) return params.get(token.value);
        const local = locals.find(value => value.name === token.value && value.end < before);
        return local && (local.explicit || infer(local.equal + 1, local.end, local.at, new Set([...seen, token.value])));
      }
      if (tokens[start].value === '-' && infer(start + 1, end, before, seen) === 'f64') return 'f64';
      if (tokens[end - 2]?.value === 'as' && tokens[end - 1]?.value === 'f64') return type('f64');
      if (tokens[end - 1]?.value === ')' && matching.has(end - 1)) {
        const open = matching.get(end - 1), path = spelling(tokens, start, open);
        if (path === 'String::from' && type('String')) return 'String';
        if (text && textMethods.has('from_units') && /^(?:self::|crate::)?ml_string::Text::from_units$/u.test(path)) return 'Text';
        const name = path.replace(/^self::/u, '');
        if (rootOwner && /^[A-Za-z_][A-Za-z_0-9]*$/u.test(name) && !counts.has(name) && !unavailable.has(name)) {
          const found = functions.filter(value => value.name === name);
          return found.length === 1 ? found[0].result : null;
        }
        const method = open - 1;
        if (tokens[method - 1]?.value === '.') {
          const receiver = infer(start, method - 1, before, seen);
          if (['clone', 'to_owned'].includes(tokens[method].value) && receiver !== 'Text') return receiver;
          if (tokens[method].value === 'slice' && receiver === 'Text' && textMethods.has('slice')) return 'Text';
          if (tokens[method].value === 'as_str' && receiver === 'String') return 'String';
        }
      }
      const operations = [];
      for (let at = start; at < end; at += 1) {
        if (matching.has(at) && matching.get(at) > at) { at = matching.get(at); continue; }
        if (['==', '!='].includes(tokens[at].value)) operations.push(at);
      }
      if (operations.length === 1) {
        const first = infer(start, operations[0], before, seen), second = infer(operations[0] + 1, end, before, seen);
        if (first && first === second) return type('bool');
      }
      return null;
    };
    const first = infer(left, operator, left), second = infer(operator + 1, right, left);
    return Boolean(first && first === second && (rootOwner || !first.includes('Text')));
  };
}

function readFamilyLibraryInequalityCondition(tokens, matching, opaque, index) {
  if (opaque.has(index) || tokens[index]?.kind !== 'identifier' || tokens[index].value !== 'if') return null;
  const node = readFamilyLibraryIf(tokens, matching, index);
  if (!node || tokens[node.close + 1]?.value !== 'else') return null;
  let start = index + 1, end = node.open - 1;
  while (tokens[start]?.value === '(' && matching.get(start) === end) { start += 1; end -= 1; }
  if (tokens.slice(start, end + 1).some(token => token.kind === 'macro' && token.value !== 'vec'
    || token.kind !== 'string' && ['let', 'unsafe', '{', '}', '?', "'"].includes(token.value))) return null;
  const operations = [];
  for (let at = start; at <= end; at += 1) {
    if (matching.has(at) && matching.get(at) > at) { at = matching.get(at); continue; }
    if (['!=', '==', '&&', '||', '=', '<', '<=', '>', '>='].includes(tokens[at].value)) operations.push(at);
  }
  if (operations.length !== 1 || tokens[operations[0]].value !== '!=' || operations[0] === start || operations[0] === end) return null;
  return {node, start, end, operation: operations[0]};
}

/** Swap only a source-proved primitive !=, retaining both original arms. */
export function invertFamilyLibraryInequalityBranches(source, {context = source} = {}) {
  for (let pass = 0; pass < 100; pass += 1) {
    const {tokens, matching, opaque} = readFamilyLibraryTokens(source);
    const equivalent = readFamilyLibraryEqualityProof(source, tokens, matching, context);
    let replacement;
    for (let index = 0; index < tokens.length; index += 1) {
      const candidate = readFamilyLibraryInequalityCondition(tokens, matching, opaque, index);
      if (!candidate) continue;
      const {node, start, end} = candidate;
      // Generic PartialEq implementations may override ne independently of eq.
      // The checked primitive functions have no type-parameter declaration.
      let owner = index;
      while (owner >= 0 && !(tokens[owner].kind === 'identifier' && tokens[owner].value === 'fn')) owner -= 1;
      if (owner < 0 || !equivalent(start, candidate.operation, end + 1, owner)) continue;
      const operation = tokens[candidate.operation];
      const condition = source.slice(tokens[index].end, operation.start) + '==' + source.slice(operation.end, tokens[node.open].start);
      const originalThen = source.slice(tokens[node.open].start, tokens[node.close].end);
      const separator = source.slice(tokens[node.close].end, tokens[node.close + 1].end);
      const alternate = source.slice(tokens[node.close + 1].end, tokens[node.end].end);
      const alternateBlock = tokens[node.close + 2]?.value === 'if' ? '{\n' + alternate + '\n}' : alternate;
      replacement = {start: tokens[index].start, end: tokens[node.end].end,
        text: 'if' + condition + alternateBlock + separator + ' ' + originalThen};
      break;
    }
    if (!replacement) return source;
    source = source.slice(0, replacement.start) + replacement.text + source.slice(replacement.end);
  }
  throw new Error('the finite library inequality syntax must converge');
}

/** Bind an unproved comparison once; its actual ne operation never changes. */
export function bindFamilyLibraryInequalityConditions(source) {
  for (let pass = 0; pass < 100; pass += 1) {
    const {tokens, matching, opaque} = readFamilyLibraryTokens(source);
    const used = new Set(tokens.filter(token => token.kind === 'identifier').map(token => token.value));
    let replacement;
    for (let index = tokens.length - 1; index >= 0; index -= 1) {
      const candidate = readFamilyLibraryInequalityCondition(tokens, matching, opaque, index);
      if (!candidate) continue;
      const {node} = candidate;
      let count = 1;
      while (used.has('ml_library_inequality_' + count)) count += 1;
      const name = 'ml_library_inequality_' + count;
      const condition = source.slice(tokens[index].end, tokens[node.open].start);
      const branches = source.slice(tokens[node.open].start, tokens[node.end].end);
      replacement = {start: tokens[index].start, end: tokens[node.end].end,
        text: '{\nlet ' + name + ' =' + condition + ';\nif ' + name + ' ' + branches + '\n}'};
      break;
    }
    if (!replacement) return source;
    source = source.slice(0, replacement.start) + replacement.text + source.slice(replacement.end);
  }
  throw new Error('the finite library comparison-binding syntax must converge');
}

/** Apply De Morgan only to two whole unary Boolean operands of an if. */
export function simplifyFamilyLibraryNegatedConjunctions(source) {
  const {tokens, matching, opaque} = readFamilyLibraryTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'if') continue;
    const node = readFamilyLibraryIf(tokens, matching, index);
    if (!node) continue;
    let start = index + 1, end = node.open - 1;
    while (tokens[start]?.value === '(' && matching.get(start) === end) { start += 1; end -= 1; }
    const operators = [];
    for (let at = start; at <= end; at += 1) {
      if (matching.has(at) && matching.get(at) > at) { at = matching.get(at); continue; }
      if (['&&', '||'].includes(tokens[at].value)) operators.push(at);
    }
    if (operators.length !== 1 || tokens[operators[0]].value !== '&&') continue;
    const middle = operators[0];
    if (tokens[start]?.value !== '!' || tokens[middle + 1]?.value !== '!') continue;
    if (tokens.slice(start, end + 1).some(token => token.kind === 'macro' && token.value !== 'vec'
      || token.kind !== 'string' && ['let', 'unsafe', '{', '}'].includes(token.value))) continue;
    const first = source.slice(tokens[start].start, tokens[middle].start);
    const second = source.slice(tokens[middle + 1].start, tokens[end].end);
    if (!acceptRustWholeConditionNegation(first.trim()) || !acceptRustWholeConditionNegation(second.trim())) continue;
    edits.push([tokens[start].start, tokens[end].end,
      '!((\n' + first.slice(1) + '\n) || (\n' + second.slice(1) + '\n))']);
  }
  for (const [start, end, text] of edits.sort((left, right) => right[0] - left[0])) source = source.slice(0, start) + text + source.slice(end);
  return source;
}

/** Bind an actual numeric truthiness block once in its existing branch scope. */
export function liftFamilyLibraryTruthinessConditions(source) {
  for (let pass = 0; pass < 100; pass += 1) {
    const {tokens, matching, opaque} = readFamilyLibraryTokens(source);
    const used = new Set(tokens.filter(token => token.kind === 'identifier').map(token => token.value));
    let replacement;
    for (let index = tokens.length - 1; index >= 0; index -= 1) {
      if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'if') continue;
      const condition = index + 1, close = matching.get(condition);
      if (tokens[condition]?.value !== '{' || close === undefined || tokens[condition + 1]?.value !== 'let'
        || tokens[condition + 2]?.kind !== 'identifier' || tokens[condition + 3]?.value !== '=') continue;
      let separator = condition + 4;
      while (separator < close && tokens[separator].value !== ';') {
        if (matching.has(separator)) separator = matching.get(separator);
        separator += 1;
      }
      if (separator >= close) continue;
      const value = tokens[condition + 2].value;
      const tail = tokens.slice(separator + 1, close).filter(token => !['(', ')'].includes(token.value))
        .map(token => source.slice(token.start, token.end)).join('');
      if (tail !== value + '!=0f64&&!' + value + '.is_nan') continue;
      const node = readFamilyLibraryIf(tokens, matching, index);
      if (!node) continue;
      let count = 1;
      while (used.has('ml_library_condition_' + count)) count += 1;
      const name = 'ml_library_condition_' + count;
      const originalCondition = source.slice(tokens[index].end, tokens[close].end);
      const originalBranches = source.slice(tokens[close].end, tokens[node.end].end);
      replacement = {start: tokens[index].start, end: tokens[node.end].end,
        text: '{\nlet ' + name + ' = ' + originalCondition + ';\nif ' + name + originalBranches + '\n}'};
      break;
    }
    if (!replacement) return source;
    source = source.slice(0, replacement.start) + replacement.text + source.slice(replacement.end);
  }
  throw new Error('the finite library truthiness syntax must converge');
}

/** Flatten only an else block containing one complete nested if expression. */
export function flattenFamilyLibraryElseIf(source) {
  for (let pass = 0; pass < 100; pass += 1) {
    const {tokens, matching, opaque} = readFamilyLibraryTokens(source);
    let selected;
    for (let index = 0; index < tokens.length; index += 1) {
      if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'else' || tokens[index + 1]?.value !== '{') continue;
      const open = index + 1, close = matching.get(open);
      const nested = readFamilyLibraryIf(tokens, matching, open + 1);
      if (nested && nested.end === close - 1) { selected = {open, close}; break; }
    }
    if (!selected) return source;
    source = source.slice(0, tokens[selected.close].start) + source.slice(tokens[selected.close].end);
    source = source.slice(0, tokens[selected.open].start) + ' ' + source.slice(tokens[selected.open].end);
  }
  throw new Error('the finite library else-if syntax must converge');
}

/** Merge exact sibling bodies; || retains the original conditional order. */
export function mergeFamilyLibraryIdenticalBranches(source) {
  for (let pass = 0; pass < 100; pass += 1) {
    const {tokens, matching, opaque} = readFamilyLibraryTokens(source);
    let replacement;
    for (let index = 0; index < tokens.length; index += 1) {
      if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'if') continue;
      const first = readFamilyLibraryIf(tokens, matching, index);
      if (!first || tokens[first.close + 1]?.value !== 'else') continue;
      const second = readFamilyLibraryIf(tokens, matching, first.close + 2);
      if (!second) continue;
      const firstBody = source.slice(tokens[first.open].end, tokens[first.close].start);
      const secondBody = source.slice(tokens[second.open].end, tokens[second.close].start);
      if (firstBody !== secondBody) continue;
      const gap = source.slice(tokens[first.close].end, tokens[first.close + 1].start)
        + source.slice(tokens[first.close + 1].end, tokens[second.start].start);
      if (gap.trim()) continue;
      // Merging must not discard either occurrence of an original comment.
      let prior = tokens[first.open].end, commented = false;
      for (let at = first.open + 1; at <= first.close; at += 1) {
        if (source.slice(prior, tokens[at].start).trim()) commented = true;
        prior = tokens[at].end;
      }
      if (commented) continue;
      // A condition-local binding cannot be read by a merged sibling body.
      const firstCondition = tokens.slice(index + 1, first.open);
      const secondCondition = tokens.slice(second.start + 1, second.open);
      if ([...firstCondition, ...secondCondition].some(token => ['let', '{', '}'].includes(token.value))) continue;
      const left = source.slice(tokens[index].end, tokens[first.open].start);
      const right = source.slice(tokens[second.start].end, tokens[second.open].start);
      replacement = {start: tokens[index].start, end: tokens[second.open].end,
        text: `if (\n${left}\n) || (\n${right}\n) {`};
      break;
    }
    if (!replacement) return source;
    source = source.slice(0, replacement.start) + replacement.text + source.slice(replacement.end);
  }
  throw new Error('the finite library identical-branch syntax must converge');
}

/** Admit const only when every parameter and body token proves Boolean syntax. */
export function qualifyFamilyLibraryBooleanFunctions(source) {
  const {tokens, matching, opaque} = readFamilyLibraryTokens(source), edits = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (opaque.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'fn' || tokens[index - 1]?.value === 'const') continue;
    if (index > 0 && !['pub', '}', '{', ']', ';'].includes(tokens[index - 1].value)) continue;
    const open = index + 2, close = matching.get(open);
    if (tokens[index + 1]?.kind !== 'identifier' || tokens[open]?.value !== '(' || close === undefined) continue;
    if (tokens[close + 1]?.value !== '->' || tokens[close + 2]?.value !== 'bool' || tokens[close + 3]?.value !== '{') continue;
    const parameters = new Set(); let accepted = true;
    for (let parameter = open + 1; parameter < close;) {
      if (tokens[parameter]?.kind !== 'identifier' || tokens[parameter + 1]?.value !== ':' || tokens[parameter + 2]?.value !== 'bool') { accepted = false; break; }
      parameters.add(tokens[parameter].value); parameter += 3;
      if (parameter < close && tokens[parameter].value !== ',') { accepted = false; break; }
      if (tokens[parameter]?.value === ',') parameter += 1;
    }
    const body = close + 3, end = matching.get(body);
    if (end === undefined) continue;
    const allowed = new Set(['true', 'false', 'if', 'else', 'return', '!', '&&', '||', '(', ')', '{', '}', ';']);
    if (tokens.slice(body + 1, end).some(token => !['identifier', 'punct'].includes(token.kind)
      || !allowed.has(token.value) && !parameters.has(token.value))) accepted = false;
    // Parameter names followed by '(' would be a call, not a Boolean read.
    if (tokens.slice(body + 1, end).some((token, offset) => parameters.has(token.value) && tokens[body + offset + 2]?.value === '(')) accepted = false;
    if (accepted) edits.push(tokens[index].start);
    index = end;
  }
  for (const start of edits.reverse()) source = source.slice(0, start) + 'const ' + source.slice(start);
  return source;
}
