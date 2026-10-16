// Mechanical library ABI and module placement for complete checked output.
// These styles retain emitted operations, evaluation order and source bodies.
import assert from 'node:assert/strict';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

export {readPortableSourceLintStyles} from './portable-source-lint-style.mjs';

/** Borrow immutable primitive parameters; retain actual assigned parameters. */
export function readPortableSourceParameterStyles(source, {publicNames = [], ownedParameters = {}} = {}) {
  const declarations = new Map();
  for (const [, name] of source.matchAll(/^pub fn (\w+)\(/gmu)) {
    const block = readSourceTextRustBlock(source, 'fn', name);
    const header = /^pub fn \w+\(([^)]*)\)/u.exec(block);
    assert.ok(header, 'a generated library function retains its complete signature');
    const body = tokenize(block.slice(header[0].length), 'Rust').tokens;
    const assigned = new Set();
    for (let index = 0; index < body.length; index += 1) {
      if (body[index].kind !== 'punct' || !['=', '+=', '-=', '*=', '/=', '%=', '&=', '|=', '^=', '<<=', '>>='].includes(body[index].value)) continue;
      const previous = body[index - 1];
      if (previous?.kind === 'identifier') assigned.add(previous.value);
      else if (previous?.value === ')') {
        let start = index - 2, depth = 1;
        while (start >= 0 && depth > 0) {
          if (body[start].kind === 'punct' && body[start].value === ')') depth += 1;
          else if (body[start].kind === 'punct' && body[start].value === '(') depth -= 1;
          if (depth > 0 && body[start].kind === 'identifier') assigned.add(body[start].value);
          start -= 1;
        }
        assert.equal(depth, 0, 'an emitted tuple assignment has its complete left-hand side');
      }
    }
    const params = header[1].split(',').map(value => /^\s*(?:mut )?(\w+):\s*(.+?)\s*$/u.exec(value)).filter(Boolean)
      .map(([, parameter, type]) => ({name: parameter, type}));
    for (const param of params) {
      let element = param.type;
      let depth = 0;
      while (element.startsWith('Vec<') && element.endsWith('>')) {
        element = element.slice(4, -1);
        depth += 1;
      }
      const textLeaf = /^(?:[A-Za-z_][A-Za-z_0-9]*::)*ml_string::Text$/u.test(element);
      param.borrow = (param.type === 'String' || depth > 0 && (['String', 'f64', 'bool'].includes(element) || textLeaf))
        && !assigned.has(param.name)
        && !(Object.hasOwn(ownedParameters, name) && ownedParameters[name].includes(param.name));
    }
    declarations.set(name, {block, params});
  }
  let adapted = source;
  const actions = [];
  for (const [name, declaration] of declarations) {
    let block = declaration.block;
    for (const param of declaration.params) if (param.borrow) {
      const type = param.type === 'String' ? '&str' : '&[' + param.type.slice(4, -1) + ']';
      block = block.replace(new RegExp('(?:mut )?' + param.name + ': ' + param.type.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'), param.name + ': ' + type);
    }
    // Edit actual parameter occurrences only. A target-source string or a
    // longer identifier containing the parameter name retains its bytes.
    const copies = [];
    const before = tokenize(block, 'Rust').tokens;
    const bodyStart = before.findIndex(token => token.kind === 'punct' && token.value === '{');
    const parentheses = new Map(), openings = [];
    for (let index = bodyStart; index < before.length; index += 1) {
      if (before[index].kind !== 'punct') continue;
      if (['(', '[', '{'].includes(before[index].value)) openings.push(index);
      else if ([')', ']', '}'].includes(before[index].value)) {
        const opening = openings.pop();
        parentheses.set(opening, index);
      }
    }
    for (let index = 0; index < before.length; index += 1) {
      const token = before[index];
      const parameter = declaration.params.find(value => value.borrow && value.name === token.value);
      if (token.kind !== 'identifier' || !parameter || before[index - 1]?.value === '.') continue;
      if (before[index + 1]?.value === '.' && before[index + 2]?.value === 'clone'
        && before[index + 3]?.value === '(' && before[index + 4]?.value === ')') {
        if (parameter.type !== 'String' && before[index - 1]?.value === '&') {
          copies.push([before[index - 1].start, before[index + 4].end, token.value]);
        } else {
          copies.push([token.start, before[index + 4].end, token.value + (parameter.type === 'String' ? '.to_owned()' : '.to_vec()')]);
        }
        index += 4;
      } else if (before[index - 1]?.value === '&' && [',', ')'].includes(before[index + 1]?.value)) {
        copies.push([before[index - 1].start, before[index - 1].end, '']);
      } else if (index > bodyStart && ['return', '=', ',', '(', '[', '{', '=>'].includes(before[index - 1]?.value)
        && [',', ';', ')', ']', '}'].includes(before[index + 1]?.value)) {
        // Final-use emission may transfer an owned parameter directly in a
        // return, binding, tuple/array slot or owned call. Once its interface
        // is borrowed, retain that original owned value with one copy here.
        // Actual borrowed callees remove these leaf copies below, and an
        // explicit reference (including parenthesized ones) stays borrowed.
        let left = index, right = index;
        while (before[left - 1]?.value === '(' && parentheses.get(left - 1) === right + 1) {
          left -= 1; right += 1;
        }
        if (before[left - 1]?.value !== '&') {
          copies.push([token.start, token.end, token.value + (parameter.type === 'String' ? '.to_owned()' : '.to_vec()')]);
        }
      }
    }
    for (const [start, end, text] of copies.sort((left, right) => right[0] - left[0])) {
      block = block.slice(0, start) + text + block.slice(end);
    }
    const tokens = tokenize(block, 'Rust').tokens;
    const matching = new Map(), stack = [];
    for (let index = 0; index < tokens.length; index += 1) {
      if (['(', '[', '{'].includes(tokens[index].value) && tokens[index].kind === 'punct') stack.push(index);
      else if ([')', ']', '}'].includes(tokens[index].value) && tokens[index].kind === 'punct') matching.set(stack.pop(), index);
    }
    const edits = [];
    for (let index = 0; index < tokens.length; index += 1) {
      const target = declarations.get(tokens[index].value);
      if (!target || tokens[index + 1]?.value !== '(' || tokens[index - 1]?.value === 'fn') continue;
      const closing = matching.get(index + 1);
      assert.notEqual(closing, undefined, 'a generated library call has a closing delimiter');
      const arguments_ = [];
      let begin = index + 2;
      for (let at = begin; at <= closing; at += 1) {
        if (at === closing || tokens[at].value === ',') { arguments_.push([begin, at]); begin = at + 1; }
        else if (matching.has(at)) at = matching.get(at);
      }
      for (const [position, [from, end]] of arguments_.entries()) {
        if (from === end || !target.params[position]?.borrow) continue;
        const text = block.slice(tokens[from].start, tokens[end - 1].end);
        let first = from, last = end;
        while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
        const grouped = block.slice(tokens[first].start, tokens[last - 1].end);
        const leaf = /^\w+(?:\.(?:to_vec|to_owned|clone)\(\))?$/u.test(grouped)
          || /^String::from\("[^"\\]*"\)$/u.test(grouped);
        const borrowed = (leaf ? grouped : text).replace(/^(\w+)\.(?:to_vec|to_owned|clone)\(\)$/u, '$1');
        const parameter = declaration.params.find(value => value.name === borrowed);
        const literal = /^String::from\(("[^"\\]*")\)$/u.exec(borrowed);
        if (literal) edits.push([tokens[from].start, tokens[end - 1].end, literal[1]]);
        else {
          // Leaf-copy replacements cannot overlap nested argument calls. Loan
          // markers are zero-width edits around each original compound span.
          if (borrowed !== text) edits.push([tokens[from].start, tokens[end - 1].end, borrowed]);
          if (!parameter?.borrow && !borrowed.startsWith('&') && !/^"/u.test(borrowed)) {
            edits.push([tokens[from].start, tokens[from].start, '&(']);
            edits.push([tokens[end - 1].end, tokens[end - 1].end, ')']);
          }
        }
      }
    }
    for (const [start, end, text] of edits.sort((left, right) => right[0] - left[0] || right[1] - left[1])) {
      block = block.slice(0, start) + text + block.slice(end);
    }
    if (publicNames.includes(name)) block = '/// Execute the checked shared source algorithm.\n#[must_use]\n' + block;
    if (block !== declaration.block) {
      actions.push({op: 'replace', field: 'source', from: declaration.block, to: block});
      adapted = adapted.split(declaration.block).join(block);
    }
  }
  return {source: adapted, actions};
}

/** Place complete top-level items in bounded child modules with the same API. */
export function partitionPortableRustLibrary(source, {notice = '', directory, maximumLines = 900} = {}) {
  assert.match(directory, /^[a-z][a-z_0-9]*$/u, 'a portable library has one explicit child-module directory');
  assert.ok(Number.isSafeInteger(maximumLines) && maximumLines > 0 && maximumLines < 990);
  const lines = source.trimEnd().split('\n');
  // Attributes belong to their following declaration, including attributes
  // rustfmt expands over multiple lines. Read balanced token spans so quoted
  // brackets and nested attribute arguments cannot create module boundaries.
  const tokens = tokenize(source, 'Rust').tokens;
  const attributeLines = new Map();
  const lineAt = offset => source.slice(0, offset).split('\n').length - 1;
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct' || tokens[index].value !== '#'
      || tokens[index + 1]?.value !== '[') continue;
    let depth = 0, end = index + 1;
    for (; end < tokens.length; end += 1) {
      if (tokens[end].kind !== 'punct') continue;
      if (tokens[end].value === '[') depth += 1;
      if (tokens[end].value === ']' && --depth === 0) break;
    }
    assert.ok(end < tokens.length, 'a generated attribute has its complete bracket span');
    attributeLines.set(lineAt(tokens[end].end), lineAt(tokens[index].start));
    index = end;
  }
  const starts = [];
  for (let index = 0; index < lines.length; index += 1) {
    if (!/^pub (?:fn|enum|struct|mod) /u.test(lines[index])) continue;
    let start = index;
    while (start > (starts.at(-1) ?? 0)) {
      if (attributeLines.has(start - 1)) start = attributeLines.get(start - 1);
      else if (/^(?:\s*|\/\/.*)$/u.test(lines[start - 1])) start -= 1;
      else break;
    }
    starts.push(start);
  }
  assert.ok(starts.length, 'a complete portable library has emitted public declarations');
  const prefix = lines.slice(0, starts[0]).join('\n');
  starts.push(lines.length);
  const parts = [''];
  for (let index = 0; index + 1 < starts.length; index += 1) {
    const item = lines.slice(starts[index], starts[index + 1]).join('\n').trim();
    assert.ok(item.split('\n').length <= maximumLines, 'a generated declaration must fit a bounded source module without splitting its body');
    const previous = parts.at(-1);
    if (previous && previous.split('\n').length + item.split('\n').length + 2 > maximumLines) parts.push(item);
    else parts[parts.length - 1] = previous ? previous + '\n\n' + item : item;
  }
  const root = notice + prefix + '\n' + parts.map((_, index) => `#[path = "${directory}/part_${index + 1}.rs"]\nmod part_${index + 1};\npub use part_${index + 1}::*;\n`).join('');
  assert.ok(root.split('\n').length < 1000, 'the portable library root retains its bounded size');
  return [{path: null, source: root}, ...parts.map((part, index) => ({
    path: `${directory}/part_${index + 1}.rs`, source: notice + 'use super::*;\n\n' + part + '\n',
  }))];
}
