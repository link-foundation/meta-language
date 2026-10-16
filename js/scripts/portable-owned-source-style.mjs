// Recorded ownership styles over the bounded syntax emitted by the checked core.
// Unsupported source keeps its original copies; no native algorithm is supplied.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {STRING_PRELUDE} from '../src/translation/string-runtime.js';

function readTokens(source) {
  const {tokens, comments} = tokenize(source, 'Rust'), matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    if (['(', '[', '{'].includes(tokens[index].value)) stack.push(index);
    else if ([')', ']', '}'].includes(tokens[index].value)) {
      const open = stack.pop();
      assert.notEqual(open, undefined, 'an owned source style retains its opening delimiter');
      assert.equal(tokens[open].value, {')': '(', ']': '[', '}': '{'}[tokens[index].value]);
      matching.set(open, index); matching.set(index, open);
    }
  }
  assert.equal(stack.length, 0, 'an owned source style retains its closing delimiters');
  return {tokens, comments, matching};
}

function normalizeType(type) { return type.replace(/\s+/gu, ''); }
function textType(type) { return /^(?:self::|crate::)?ml_string::Text$/u.test(type ?? ''); }
function ownedType(type, authenticatedText = false) {
  if (!type) return false;
  if (type === 'String' || authenticatedText && textType(type)) return true;
  if (!type.startsWith('Vec<') || !type.endsWith('>')) return false;
  const element = type.slice(4, -1);
  return ['f64', 'bool', 'u16', 'usize', 'i64', 'u64'].includes(element) || ownedType(element, authenticatedText);
}

function runtimeTokenKey(source) {
  const tokens = tokenize(source, 'Rust').tokens, selected = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].value === '#' && tokens[at + 1]?.value === '['
      && tokens[at + 2]?.value === 'must_use' && tokens[at + 3]?.value === ']') at += 3;
    else if (tokens[at].kind !== 'eof') selected.push([tokens[at].kind,
      tokens[at].kind === 'number' ? tokens[at].raw.replaceAll('_', '') : tokens[at].raw]);
  }
  return JSON.stringify(selected);
}

function readTextProvider(source, whole) {
  const {tokens, matching} = whole, providers = [];
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].value === 'pub' && tokens[at + 1]?.value === 'mod'
      && tokens[at + 2]?.value === 'ml_string' && tokens[at + 3]?.value === '{') {
      providers.push([at, matching.get(at + 3)]); at = matching.get(at + 3);
    } else if (tokens[at].kind === 'punct' && tokens[at].value === '{') at = matching.get(at);
  }
  if (providers.length !== 1) return undefined;
  const [start, end] = providers[0];
  if (runtimeTokenKey(source.slice(tokens[start].start, tokens[end].end)) !== runtimeTokenKey(STRING_PRELUDE)) return undefined;
  for (let at = 0; at < tokens.length; at += 1) {
    if (at === start) { at = end; continue; }
    if (tokens[at].kind === 'macro' && !['format', 'vec', 'panic', 'assert'].includes(tokens[at].value)) return undefined;
    if (tokens[at].kind === 'identifier' && tokens[at].value === 'extern') return undefined;
    if (tokens[at].kind !== 'identifier' || tokens[at].value !== 'impl') continue;
    let finish = at + 1;
    while (finish < tokens.length && tokens[finish].value !== '{') finish += 1;
    if (tokens.slice(at + 1, finish).some(token => token.value === 'Text')) return undefined;
  }
  // These facts are read from the authenticated provider's actual &self
  // signatures. Its complete context proves derived Clone over Vec<u16>.
  const provider = readTokens(STRING_PRELUDE), methods = new Map();
  for (let at = 0; at < provider.tokens.length; at += 1) {
    if (provider.tokens[at].kind !== 'identifier' || provider.tokens[at].value !== 'fn' || provider.tokens[at + 2]?.value !== '(') continue;
    const close = provider.matching.get(at + 2);
    if (provider.tokens[at + 3]?.value !== '&' || provider.tokens[at + 4]?.value !== 'self'
      || provider.tokens[close + 1]?.value !== '->') continue;
    let body = close + 2;
    while (body < provider.tokens.length && provider.tokens[body].value !== '{') body += 1;
    const type = normalizeType(STRING_PRELUDE.slice(provider.tokens[close + 2].start, provider.tokens[body].start));
    if (['Self', 'Vec<Self>', 'f64', 'bool', 'Option<f64>'].includes(type)) methods.set(provider.tokens[at + 1].value, type.replaceAll('Self', 'self::ml_string::Text'));
  }
  return methods;
}

function splitTypes(tokens, matching, start, end) {
  const parts = []; let begin = start, depth = 0;
  for (let at = start; at < end; at += 1) {
    if (matching.get(at) > at) { at = matching.get(at); continue; }
    if (tokens[at].value === '<') depth += 1;
    else if (tokens[at].value === '>') depth -= 1;
    else if (tokens[at].value === '>>') depth -= 2;
    if (tokens[at].value === ',' && depth === 0) { parts.push([begin, at]); begin = at + 1; }
  }
  if (begin < end) parts.push([begin, end]);
  return parts;
}

function readDeclarations(source) {
  const functions = [], returns = new Map(), variants = new Map();
  const whole = readTokens(source);
  let depth = 0, shadowed = false;
  for (let at = 0; at < whole.tokens.length; at += 1) {
    const token = whole.tokens[at];
    if (token.kind === 'punct' && token.value === '{') depth += 1;
    else if (token.kind === 'punct' && token.value === '}') depth -= 1;
    if (depth !== 0) continue;
    if (token.kind === 'macro' && !['format', 'vec', 'panic', 'assert'].includes(token.value)) shadowed = true;
    if (token.kind === 'punct' && token.value === '#') {
      const open = whole.tokens[at + 1]?.value === '!' ? at + 2 : at + 1;
      const attribute = whole.tokens[open + 1]?.value;
      if (!['allow', 'expect', 'must_use', 'derive'].includes(attribute)) shadowed = true;
      if (attribute === 'derive' && whole.tokens.slice(open + 2, whole.matching.get(open))
        .some(item => item.kind === 'identifier' && !['Clone', 'Debug', 'PartialEq', 'Eq', 'PartialOrd', 'Ord'].includes(item.value))) shadowed = true;
    }
    if (token.kind !== 'identifier') continue;
    if (token.value === 'use') {
      const alias = [['identifier', 'pub'], ['identifier', 'use'], ['identifier', 'crate'], ['punct', '::'],
        ['identifier', 'translation'], ['punct', '::'], ['identifier', 'frontend_rules'], ['punct', '::'],
        ['identifier', 'ml_array'], ['punct', ';']];
      // This generated relocation binds only the existing array provider. It
      // imports no owned type, conversion trait or formatting macro.
      if (alias.every(([kind, value], offset) => whole.tokens[at - 1 + offset]?.kind === kind
        && whole.tokens[at - 1 + offset]?.value === value)) { at += alias.length - 2; continue; }
    }
    if (token.value === 'use' || ['type', 'struct', 'enum', 'union', 'mod'].includes(token.value)
      && ['String', 'Vec', 'f64', 'bool', 'u16', 'usize', 'i64', 'u64', 'std'].includes(whole.tokens[at + 1]?.value)) shadowed = true;
    if (token.value !== 'pub' || whole.tokens[at + 1]?.value !== 'fn' || whole.tokens[at + 3]?.value !== '(') continue;
    const name = whole.tokens[at + 2].value;
    let open = whole.matching.get(at + 3) + 1;
    while (open < whole.tokens.length && whole.tokens[open].value !== '{') open += 1;
    if (whole.matching.get(open) === undefined) continue;
    const block = source.slice(token.start, whole.tokens[whole.matching.get(open)].end);
    const parsed = readTokens(block), {tokens, matching} = parsed;
    const parameterOpen = tokens.findIndex(token => token.value === '(');
    const parameterClose = matching.get(parameterOpen);
    let body = parameterClose + 1;
    while (body < tokens.length && tokens[body].value !== '{') body += 1;
    if (matching.get(body) === undefined) continue;
    const type = tokens[parameterClose + 1]?.value === '->'
      ? normalizeType(block.slice(tokens[parameterClose + 2].start, tokens[body].start)) : '';
    returns.set(name, returns.has(name) ? undefined : type);
    functions.push({name, block, ...parsed, parameterOpen, parameterClose, body, end: matching.get(body)});
    at = whole.matching.get(open);
  }
  const {tokens, matching} = whole;
  for (let at = 0; at < tokens.length; at += 1) {
    if (tokens[at].kind === 'punct' && tokens[at].value === '{') { at = matching.get(at); continue; }
    if (tokens[at].value !== 'enum' || tokens[at + 1]?.kind !== 'identifier' || tokens[at + 2]?.value !== '{') continue;
    const name = tokens[at + 1].value, open = at + 2, end = matching.get(open);
    for (const [from, to] of splitTypes(tokens, matching, open + 1, end)) {
      if (tokens[from]?.kind !== 'identifier') continue;
      let types;
      if (from + 1 === to) types = [];
      else if (tokens[from + 1]?.value === '(' && matching.get(from + 1) === to - 1) {
        types = splitTypes(tokens, matching, from + 2, to - 1).map(([start, finish]) =>
          normalizeType(source.slice(tokens[start].start, tokens[finish - 1].end)));
      } else continue;
      for (const root of ['self', 'crate']) {
        const path = root + '::' + name + '::' + tokens[from].value;
        variants.set(path, variants.has(path) ? undefined : types);
      }
    }
    at = end;
  }
  return {functions: shadowed ? [] : functions, returns, variants, textMethods: readTextProvider(source, whole)};
}

// Read only complete generated control constructs. The graph overapproximates
// short-circuit expressions, so an extra possible use retains a copy.
function readFlow(fn) {
  const {tokens, matching, body, end} = fn, patterns = [], matches = new Map(), controlEnds = new Map();
  const word = (at, text) => tokens[at]?.kind === 'identifier' && tokens[at].value === text;
  const fail = () => { throw new Error('unsupported owned source flow'); };
  const findBody = (start, finish) => {
    let at = start;
    if (tokens[at]?.value === '{') at = matching.get(at) + 1;
    for (; at < finish; at += 1) {
      if (tokens[at].value === '{') {
        if (tokens[at - 1]?.kind === 'punct' && ![')', ']', '}'].includes(tokens[at - 1].value)) fail();
        return at;
      }
      if (matching.get(at) > at) at = matching.get(at);
      else if ([';', '=>'].includes(tokens[at].value)) fail();
      else if (tokens[at].kind === 'identifier' && ['if', 'match', 'loop', 'while', 'let', 'return', 'break', 'continue'].includes(tokens[at].value)) fail();
    }
    fail();
  };
  const readIf = (start, finish) => {
    if (word(start + 1, 'let')) fail();
    const open = findBody(start + 1, finish), close = matching.get(open);
    let alternate, next = close + 1;
    if (word(next, 'else')) {
      if (word(next + 1, 'if')) { alternate = readIf(next + 1, finish); next = alternate.next; }
      else if (tokens[next + 1]?.value === '{') {
        alternate = {kind: 'group', start: next + 1, end: matching.get(next + 1)};
        next = alternate.end + 1;
      } else fail();
    }
    return {kind: 'if', start, open, close, alternate, next};
  };
  const readUnits = (start, finish) => {
    const units = [];
    for (let at = start; at < finish;) {
      let unit;
      if (word(at, 'if')) unit = readIf(at, finish);
      else if (word(at, 'loop') || word(at, 'while')) {
        const open = word(at, 'loop') ? at + 1 : findBody(at + 1, finish);
        if (tokens[open]?.value !== '{' || word(at + 1, 'let')) fail();
        unit = {kind: tokens[at].value, start: at, open, close: matching.get(open), next: matching.get(open) + 1};
      } else if (word(at, 'return')) {
        let next = at + 1;
        while (next < finish && tokens[next].value !== ';') next = matching.get(next) > next ? matching.get(next) + 1 : next + 1;
        unit = {kind: 'return', start: at, end: next, next: next < finish ? next + 1 : next};
      } else if (word(at, 'continue') || word(at, 'break')) {
        if (at + 1 < finish && tokens[at + 1].value !== ';') fail();
        unit = {kind: tokens[at].value, start: at, next: at + 1 < finish ? at + 2 : at + 1};
      } else if (word(at, 'match')) {
        const open = findBody(at + 1, finish), close = matching.get(open), arms = [];
        let from = open + 1;
        while (from < close) {
          let arrow = from;
          while (arrow < close && tokens[arrow].value !== '=>') arrow = matching.get(arrow) > arrow ? matching.get(arrow) + 1 : arrow + 1;
          if (arrow >= close || tokens.slice(from, arrow).some(token => token.value === 'if')) fail();
          let to = arrow + 1;
          if (tokens[to]?.value === '{') to = matching.get(to) + 1;
          else if (word(to, 'if')) to = readIf(to, close).next;
          else while (to < close && tokens[to].value !== ',') {
            if (tokens[to].value === '=>') fail();
            to = matching.get(to) > to ? matching.get(to) + 1 : to + 1;
          }
          if (to === arrow + 1) fail();
          const pattern = {from, to: arrow, owner: at, open};
          patterns.push(pattern); arms.push({pattern, from: arrow + 1, to}); from = tokens[to]?.value === ',' ? to + 1 : to;
        }
        if (!arms.length) fail();
        unit = {kind: 'match', start: at, open, close, arms, next: close + 1};
        matches.set(at, unit);
      } else if (matching.get(at) > at) unit = {kind: 'group', start: at, end: matching.get(at), next: matching.get(at) + 1};
      else unit = {kind: 'token', start: at, next: at + 1};
      if (['if', 'match', 'loop', 'while'].includes(unit.kind)) controlEnds.set(unit.next, unit.start);
      units.push(unit); at = unit.next;
    }
    return units;
  };
  const edges = new Map();
  const buildRange = (start, finish, continuation, loop) => {
    const units = readUnits(start, finish);
    let next = continuation;
    for (const unit of units.reverse()) next = build(unit, next, loop);
    return next;
  };
  const build = (unit, next, loop) => {
    let following;
    if (unit.kind === 'group') {
      edges.set(unit.end, next);
      following = buildRange(unit.start + 1, unit.end, [unit.end], loop);
    } else if (unit.kind === 'if') {
      const then = build({kind: 'group', start: unit.open, end: unit.close}, next, loop);
      const otherwise = unit.alternate ? build(unit.alternate, next, loop) : next;
      following = buildRange(unit.start + 1, unit.open, [...then, ...otherwise], loop);
    } else if (unit.kind === 'loop' || unit.kind === 'while') {
      const inner = {continue: [unit.start], break: next};
      const child = build({kind: 'group', start: unit.open, end: unit.close}, [unit.start], inner);
      following = unit.kind === 'loop' ? child : buildRange(unit.start + 1, unit.open, [...child, ...next], loop);
    } else if (unit.kind === 'return') following = buildRange(unit.start + 1, unit.end, [], loop);
    else if (unit.kind === 'continue' || unit.kind === 'break') { if (!loop) fail(); following = loop[unit.kind]; }
    else if (unit.kind === 'match') {
      const arms = unit.arms.flatMap(arm => {
        let entry = buildRange(arm.from, arm.to, next, loop);
        // A selected arm binds its actual fields before evaluating its value.
        // The type adapter identifies which pattern tokens are definitions.
        for (let at = arm.pattern.to - 1; at >= arm.pattern.from; at -= 1) {
          edges.set(at, entry); entry = [at];
        }
        return entry;
      });
      following = buildRange(unit.start + 1, unit.open, arms, loop);
    } else following = next;
    edges.set(unit.start, following);
    return [unit.start];
  };
  buildRange(body + 1, end, [], undefined);
  return {edges, patterns, matches, controlEnds, readIf};
}

function acceptFunction(fn, implicitReads) {
  const {tokens, matching, body, end} = fn;
  for (let at = body + 1; at < end; at += 1) {
    const token = tokens[at], previous = tokens[at - 1]?.value;
    if (token.kind === 'identifier' && ['fn', 'for', 'unsafe', 'async', 'await', 'yield', 'move', 'ref',
      'const', 'static', 'type', 'struct', 'enum', 'impl', 'use', 'mod'].includes(token.value)) return false;
    if (token.kind === 'punct' && ['#', '?', '|'].includes(token.value)) return false;
    if (token.value === '||' && ['=', '(', '[', ',', '{', 'return', '=>'].includes(previous)) return false;
    if (token.kind !== 'macro') continue;
    if (!['format', 'vec'].includes(token.value) || matching.get(at + 1) === undefined) return false;
    if (previous === '::' && !(tokens[at - 2]?.kind === 'identifier' && tokens[at - 2].value === 'std'
      && tokens[at - 3]?.value !== '::')) return false;
    if (token.value === 'format') {
      const literal = tokens[at + 2];
      if (literal?.kind !== 'string') return false;
      const fields = literal.value.replaceAll('{{', '').replaceAll('}}', '').matchAll(/\{([^{}]*)\}/gu);
      for (const [, field] of fields) {
        if (/^[0-9]*$/u.test(field)) continue;
        if (!/^[A-Za-z_][A-Za-z_0-9]*$/u.test(field)) return false;
        // Captured fields read the actual lexical binding. Retain that read
        // through completion of every explicit argument, including a call
        // which consumes a clone of the same binding later in this macro.
        for (const index of [at + 2, matching.get(at + 1)]) {
          if (!implicitReads.has(index)) implicitReads.set(index, new Set());
          implicitReads.get(index).add(field);
        }
      }
      if (/[{}]/u.test(literal.value.replaceAll('{{', '').replaceAll('}}', '').replace(/\{[^{}]*\}/gu, ''))) return false;
    }
  }
  return true;
}

function readBindingTypes(fn, flow, declarations) {
  const {block, tokens, matching, parameterOpen, parameterClose, body, end} = fn;
  const bindings = new Map(), counts = new Map(), initializers = [], patterns = [];
  const definitions = new Map(), declarationTokens = new Set(), writeTokens = new Set();
  const refused = () => ({bindings: new Map(), definitions: new Map(), declarationTokens: new Set(), writeTokens: new Set()});
  const add = (name, type, position) => {
    counts.set(name, (counts.get(name) ?? 0) + 1);
    if (type) bindings.set(name, type);
    if (position !== undefined) declarationTokens.add(position);
  };
  const define = (position, name) => {
    if (!definitions.has(position)) definitions.set(position, new Set());
    definitions.get(position).add(name);
  };
  const text = (from, to) => from < to ? block.slice(tokens[from].start, tokens[to - 1].end) : '';
  for (const [from, to] of splitTypes(tokens, matching, parameterOpen + 1, parameterClose)) {
    const at = tokens[from]?.value === 'mut' ? from + 1 : from;
    if (tokens[at]?.kind !== 'identifier' || tokens[at + 1]?.value !== ':') return refused();
    add(tokens[at].value, normalizeType(text(at + 2, to)));
  }
  for (let at = body + 1; at < end; at += 1) {
    if (tokens[at].kind !== 'identifier' || tokens[at].value !== 'let') continue;
    let name = tokens[at + 1]?.value === 'mut' ? at + 2 : at + 1;
    if (tokens[name]?.value === '(') {
      const close = matching.get(name);
      for (const [from, to] of splitTypes(tokens, matching, name + 1, close)) {
        if (to !== from + 1 || tokens[from].kind !== 'identifier') return refused();
        add(tokens[from].value, undefined, from);
      }
      continue;
    }
    if (tokens[name]?.kind !== 'identifier') return refused();
    let equal = name + 1, type;
    if (tokens[equal]?.value === ':') {
      equal += 1;
      while (equal < end && !['=', ';'].includes(tokens[equal].value)) equal += 1;
      type = normalizeType(text(name + 2, equal));
    }
    add(tokens[name].value, type, name);
    if (tokens[equal]?.value !== '=') continue;
    let finish = equal + 1;
    while (finish < end && tokens[finish].value !== ';') finish = matching.get(finish) > finish ? matching.get(finish) + 1 : finish + 1;
    initializers.push({name: tokens[name].value, from: equal + 1, to: finish});
    if (finish < end) define(finish, tokens[name].value);
  }
  for (const pattern of flow.patterns) {
    const {from, to} = pattern;
    let open = from;
    while (open < to && tokens[open].value !== '(') open += 1;
    if (open === to) {
      const pattern = normalizeType(text(from, to));
      if (pattern !== '_' && declarations.variants.get(pattern)?.length !== 0) return refused();
      continue;
    }
    const path = normalizeType(text(from, open)), types = declarations.variants.get(path);
    if (!types || matching.get(open) !== to - 1) return refused();
    const slots = [];
    const fields = splitTypes(tokens, matching, open + 1, to - 1);
    if (fields.length !== types.length) return refused();
    for (const [[start, finish], index] of fields.map((part, index) => [part, index])) {
      if (finish !== start + 1 || tokens[start].kind !== 'identifier') return refused();
      if (tokens[start].value !== '_') {
        add(tokens[start].value, undefined, start); define(start, tokens[start].value);
        slots.push({name: tokens[start].value, type: types[index]});
      }
    }
    patterns.push({...pattern, path, slots, verified: false});
  }
  const divergent = Symbol('a branch cannot finish its initializer');
  const commonType = types => {
    const returning = types.filter(type => type !== divergent);
    if (!returning.length) return divergent;
    return returning[0] && returning.every(type => type === returning[0]) ? returning[0] : undefined;
  };
  const infer = (from, to) => {
    while (tokens[from]?.value === '(' && matching.get(from) === to - 1) { from += 1; to -= 1; }
    if (tokens[from]?.kind === 'identifier' && tokens[from].value === 'return') return divergent;
    if (tokens[from]?.kind === 'identifier' && tokens[from].value === 'continue'
      && (to === from + 1 || to === from + 2 && tokens[from + 1].value === ';')) return divergent;
    if (tokens[from]?.value === '{' && matching.get(from) === to - 1) {
      let tail = from + 1;
      for (let at = tail; at < to - 1; at += 1) {
        // Only an actual statement start can prove this block cannot finish.
        // A return hidden in a short-circuit operand supplies no such proof.
        if (at === tail && tokens[at].kind === 'identifier' && tokens[at].value === 'return') return divergent;
        if (at === tail && tokens[at].kind === 'identifier' && tokens[at].value === 'continue'
          && (at + 1 === to - 1 || tokens[at + 1]?.value === ';')) return divergent;
        if (matching.get(at) > at) at = matching.get(at);
        else if (tokens[at].value === ';') tail = at + 1;
      }
      return infer(tail, to - 1);
    }
    if (tokens[from]?.kind === 'identifier' && tokens[from].value === 'if') {
      const node = flow.readIf(from, to);
      if (node.next !== to || !node.alternate) return undefined;
      const first = infer(node.open, node.close + 1);
      const second = infer(node.alternate.start, node.alternate.next ?? node.alternate.end + 1);
      return commonType([first, second]);
    }
    if (tokens[from]?.kind === 'identifier' && tokens[from].value === 'match') {
      const node = flow.matches.get(from);
      if (!node || node.next !== to) return undefined;
      return commonType(node.arms.map(arm => infer(arm.from, arm.to)));
    }
    const expression = normalizeType(text(from, to));
    if (to === from + 1 && counts.get(tokens[from]?.value) === 1) return bindings.get(tokens[from].value);
    if (tokens[from]?.kind === 'macro' && tokens[from].value === 'format' && matching.get(from + 1) === to - 1) return 'String';
    if (/^String::(?:from|new)\(/u.test(expression) && matching.get(from + 3) === to - 1) return 'String';
    const vector = /^Vec::<(.+)>::new\(\)$/u.exec(expression);
    if (vector && ownedType('Vec<' + vector[1] + '>', Boolean(declarations.textMethods))) return 'Vec<' + vector[1] + '>';
    if (declarations.textMethods && /^(?:self::|crate::)?ml_string::Text::from_(?:text|units)\(/u.test(expression)) {
      let open = from;
      while (open < to && tokens[open].value !== '(') open += 1;
      if (matching.get(open) === to - 1) return 'self::ml_string::Text';
    }
    const copied = /^([A-Za-z_][A-Za-z_0-9]*)\.(clone|to_owned|to_vec)\(\)$/u.exec(expression);
    if (copied && counts.get(copied[1]) === 1) {
      const type = bindings.get(copied[1]);
      if (copied[2] === 'clone' && ownedType(type, Boolean(declarations.textMethods))) return type;
      if (copied[2] === 'clone' && type === '&String') return 'String';
      if (copied[2] === 'clone' && declarations.textMethods && type?.startsWith('&') && textType(type.slice(1))) return type.slice(1);
      if (copied[2] === 'to_owned' && ['String', '&str', '&String'].includes(type)) return 'String';
      if (copied[2] === 'to_vec' && type?.startsWith('&[') && type.endsWith(']')) return 'Vec<' + type.slice(2, -1) + '>';
    }
    const method = /^([A-Za-z_][A-Za-z_0-9]*)\.([A-Za-z_][A-Za-z_0-9]*)\(/u.exec(expression);
    if (method && counts.get(method[1]) === 1 && declarations.textMethods && matching.get(from + 3) === to - 1) {
      const type = bindings.get(method[1]);
      if (textType(type) || type?.startsWith('&') && textType(type.slice(1))) return declarations.textMethods.get(method[2]);
    }
    const call = /^(?:self|crate)::([A-Za-z_][A-Za-z_0-9]*)\(/u.exec(expression);
    if (call && matching.get(from + 3) === to - 1) return declarations.returns.get(call[1]);
    let open = from;
    while (open < to && tokens[open].value !== '(') open += 1;
    const constructor = normalizeType(text(from, open));
    const fields = declarations.variants.get(constructor);
    if (fields && (open === to && fields.length === 0 || matching.get(open) === to - 1)) {
      return constructor.slice(0, constructor.lastIndexOf('::'));
    }
    return undefined;
  };
  for (let pass = 0; pass <= initializers.length + patterns.length; pass += 1) {
    let changed = false;
    for (const initializer of initializers) if (counts.get(initializer.name) === 1 && !bindings.has(initializer.name)) {
      const type = infer(initializer.from, initializer.to);
      if (typeof type === 'string' && type) { bindings.set(initializer.name, type); changed = true; }
    }
    for (const pattern of patterns) if (!pattern.verified) {
      const actual = infer(pattern.owner + 1, pattern.open);
      const expected = pattern.path.slice(0, pattern.path.lastIndexOf('::')).replace(/^(?:self|crate)::/u, '');
      if (actual !== 'self::' + expected && actual !== 'crate::' + expected) continue;
      pattern.verified = true; changed = true;
      for (const slot of pattern.slots) if (counts.get(slot.name) === 1 && slot.type) bindings.set(slot.name, slot.type);
    }
    if (!changed) break;
  }
  if (patterns.some(pattern => !pattern.verified)) return refused();
  for (const [name, type] of bindings) if (counts.get(name) !== 1 || !ownedType(type, Boolean(declarations.textMethods))) bindings.delete(name);
  const startsStatement = start => {
    for (let at = start; at > body;) {
      if (['{', ';'].includes(tokens[at - 1]?.value)) return true;
      const previous = flow.controlEnds.get(at);
      if (previous === undefined || previous >= at) return false;
      at = previous;
    }
    return false;
  };
  for (let equal = body + 1; equal < end; equal += 1) {
    if (tokens[equal].kind !== 'punct' || tokens[equal].value !== '=') continue;
    let start = equal - 1, targets;
    if (tokens[start]?.kind === 'identifier') targets = [start];
    else if (tokens[start]?.value === ')' && matching.get(start) < start) {
      const close = start; start = matching.get(start);
      const fields = splitTypes(tokens, matching, start + 1, close);
      if (!fields.length || fields.some(([from, to]) => to !== from + 1 || tokens[from].kind !== 'identifier')) continue;
      targets = fields.map(([from]) => from);
    } else continue;
    // Only complete bare-place statements have side-effect-free left sides.
    // Fields, dereferences, indexing, declarations and compound stores retain
    // their original reads and cannot provide an ownership kill.
    if (!startsStatement(start)
      || new Set(targets.map(at => tokens[at].value)).size !== targets.length
      || targets.some(at => counts.get(tokens[at].value) !== 1)) continue;
    let finish = equal + 1;
    while (finish < end && tokens[finish].value !== ';') {
      if (matching.get(finish) > finish) finish = matching.get(finish) + 1;
      else if ([')', ']', '}'].includes(tokens[finish].value)) break;
      else finish += 1;
    }
    if (tokens[finish]?.value !== ';') continue;
    for (const at of targets) if (bindings.has(tokens[at].value)) {
      writeTokens.add(at);
      // Every RHS operand, including all tuple elements, is evaluated before
      // any old target value dies. A continue or return bypasses this node.
      define(finish, tokens[at].value);
    }
  }
  for (const names of definitions.values()) for (const name of names) if (!bindings.has(name)) names.delete(name);
  return {bindings, definitions, declarationTokens, writeTokens};
}

function acceptUnborrowedBinding(fn, name, type, declarations) {
  const {tokens, matching, body, end} = fn;
  for (let at = body + 1; at < end; at += 1) {
    if (tokens[at].kind === 'punct' && tokens[at].value === '&') {
      let finish = at + 1;
      while (finish < end && ![';', ',', ')', ']', '}'].includes(tokens[finish].value)) {
        finish = matching.get(finish) > finish ? matching.get(finish) + 1 : finish + 1;
      }
      if (tokens.slice(at + 1, finish).some(token => token.kind === 'identifier' && token.value === name)) return false;
    }
    if (tokens[at].kind === 'identifier' && tokens[at].value === name && tokens[at + 1]?.value === '.') {
      const method = tokens[at + 2]?.value;
      if (!['clone', 'len', 'is_empty', 'contains', 'starts_with', 'ends_with'].includes(method)
        && !(textType(type) && declarations.textMethods?.has(method))) return false;
    }
  }
  return true;
}

/** Remove only proven dead owned leaf copies from complete checked functions. */
export function readPortableOwnedSourceStyles(source) {
  const declarations = readDeclarations(source), actions = [];
  let adapted = source;
  for (const fn of declarations.functions) {
    const implicitReads = new Map();
    if (!acceptFunction(fn, implicitReads) || declarations.returns.get(fn.name) === undefined || source.split(fn.block).length !== 2) continue;
    let flow;
    try { flow = readFlow(fn); } catch (error) {
      if (error.message === 'unsupported owned source flow') continue;
      throw error;
    }
    const {bindings, definitions, declarationTokens, writeTokens} = readBindingTypes(fn, flow, declarations), edits = [], safe = new Map();
    const {tokens, comments} = fn;
    for (let at = fn.body + 1; at + 4 < fn.end; at += 1) {
      const token = tokens[at];
      if (token.kind !== 'identifier' || !bindings.has(token.value)
        || ['.', '::'].includes(tokens[at - 1]?.value)
        || tokens[at + 1]?.value !== '.' || tokens[at + 2]?.value !== 'clone'
        || tokens[at + 3]?.value !== '(' || tokens[at + 4]?.value !== ')') continue;
      if (!safe.has(token.value)) safe.set(token.value, acceptUnborrowedBinding(fn, token.value, bindings.get(token.value), declarations));
      if (!safe.get(token.value) || !flow.edges.has(at + 4)
        || comments.some(comment => comment.start >= tokens[at + 1].start && comment.end <= tokens[at + 4].end)) continue;
      const pending = [...flow.edges.get(at + 4)], seen = new Set();
      let live = false;
      while (pending.length && !live) {
        const next = pending.pop();
        if (seen.has(next)) continue;
        seen.add(next);
        if (definitions.get(next)?.has(token.value)) continue;
        if (implicitReads.get(next)?.has(token.value) || !declarationTokens.has(next) && !writeTokens.has(next)
          && tokens[next].kind === 'identifier' && tokens[next].value === token.value) live = true;
        else pending.push(...(flow.edges.get(next) ?? []));
      }
      if (!live) edits.push([tokens[at + 1].start, tokens[at + 4].end]);
    }
    let styled = fn.block;
    for (const [start, end] of edits.reverse()) styled = styled.slice(0, start) + styled.slice(end);
    if (styled !== fn.block) {
      actions.push({op: 'replace', field: 'source', from: fn.block, to: styled});
      adapted = adapted.split(fn.block).join(styled);
    }
  }
  return {source: adapted, actions};
}
