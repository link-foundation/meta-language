import { createHash } from 'node:crypto';
import { tokenize } from '../src/translation/lexer.js';

const maximumLines = 900;
const maximumParts = 64;
const countLines = source => source.split('\n').length;
const bodyDeclarations = new Set(['enum', 'fn', 'impl', 'mod', 'struct', 'trait', 'union']);
const terminatedDeclarations = new Set(['const', 'static', 'type', 'use']);

// include! expands complete items in their existing module. Splitting an
// inline module's body in place preserves self, super and crate resolution.
export function partitionNativePolicyAssertions(source, moduleName) {
  if (!/^[a-z][a-z_0-9]*$/u.test(moduleName)) throw new TypeError('invalid native assertion module name');
  const output = 'rust/tests/generated/' + moduleName;
  const root = {path: output + '.rs', source};
  if (countLines(source) <= 1000) return [root];

  const tokens = tokenize(source, 'Rust').tokens;
  const opening = new Map([['(', ')'], ['[', ']'], ['{', '}']]);
  const closing = new Set(opening.values());
  const ends = new Map(), stack = [];
  const parts = [];
  for (const [at, token] of tokens.entries()) {
    if (token.kind !== 'punct') continue;
    if (opening.has(token.value)) stack.push(at);
    if (closing.has(token.value)) {
      const start = stack.pop();
      if (opening.get(tokens[start]?.value) !== token.value) {
        throw new TypeError('unbalanced native assertion source delimiters');
      }
      ends.set(start, at);
    }
  }
  if (stack.length) throw new TypeError('unclosed native assertion source delimiter');
  const is = (at, value) => tokens[at]?.raw === value;
  function skipAttribute(at, inner) {
    if (is(at, '#') && is(at + 1, inner ? '!' : '[') && is(at + (inner ? 2 : 1), '[')) {
      return ends.get(at + (inner ? 2 : 1)) + 1;
    }
    return at;
  }
  function declaration(first, last) {
    let at = first;
    while (skipAttribute(at, false) !== at) at = skipAttribute(at, false);
    if (is(at, 'pub')) {
      at += 1;
      if (is(at, '(')) at = ends.get(at) + 1;
    }
    // Only the emitted Rust declaration vocabulary is accepted here.
    if (is(at, 'const') && is(at + 1, 'fn') || is(at, 'unsafe') && is(at + 1, 'fn')) at += 1;
    const kind = tokens[at]?.value;
    if (!bodyDeclarations.has(kind) && !terminatedDeclarations.has(kind)) {
      throw new TypeError('unrepresented native assertion declaration at ' + tokens[at]?.start + ': ' + kind);
    }
    for (let next = at + 1; next < last; next += 1) {
      if (is(next, ';')) return {end: next};
      if (is(next, '{') && bodyDeclarations.has(kind)) {
        return {end: ends.get(next), body: kind === 'mod' ? next : undefined};
      }
      if (ends.has(next)) next = ends.get(next);
    }
    throw new TypeError('incomplete native assertion declaration at ' + tokens[at].start);
  }
  function scope(first, last, start, finish, entry) {
    let at = first, statements = '';
    while (skipAttribute(at, true) !== at) at = skipAttribute(at, true);
    const attributeEnd = at === first ? start : tokens[at - 1].end;
    let cursor = attributeEnd, group = '';
    function flush() {
      if (!group.trim()) return;
      if (parts.length >= maximumParts) throw new RangeError('too many native assertion source parts');
      const part = 'part_' + (parts.length + 1) + '.rs';
      parts.push({path: output + '/' + part, source: group.trim() + '\n'});
      statements += 'include!("' + (entry ? moduleName + '/' : '') + part + '");\n';
      group = '';
    }
    while (at < last) {
      const {end, body} = declaration(at, last);
      let item = source.slice(cursor, tokens[end].end);
      if (countLines(item) > maximumLines && body !== undefined) {
        item = source.slice(cursor, tokens[body].end) + '\n'
          + scope(body + 1, end, tokens[body].end, tokens[end].start, false)
          + '}\n';
      }
      if (countLines(item) > maximumLines) {
        throw new RangeError('native assertion declaration exceeds its source limit at ' + tokens[at].start);
      }
      if (countLines(group) + countLines(item) > maximumLines) flush();
      group += item;
      cursor = tokens[end].end;
      at = end + 1;
    }
    group += source.slice(cursor, finish);
    flush();
    return source.slice(start, attributeEnd).trimEnd() + '\n\n' + statements;
  }
  root.source = scope(0, tokens.length - 1, 0, source.length, true).trimStart();
  return [root, ...parts];
}

export function pinNativePolicyAssertionFiles(files) {
  return files.map(({path, source}) => ({
    path, bytes: Buffer.byteLength(source), sha256: createHash('sha256').update(source).digest('hex'),
  }));
}
