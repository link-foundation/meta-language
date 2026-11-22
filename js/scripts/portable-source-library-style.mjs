// Mechanical library ABI and module placement for complete checked output.
// These styles retain emitted operations, evaluation order and source bodies.
import assert from 'node:assert/strict';
import {spawnSync} from 'node:child_process';
import {tokenize} from '../src/translation/lexer.js';
import {readSourceTextRustBlock} from './source-text-byte-library-style.mjs';

export {readPortableSourceLintStyles} from './portable-source-lint-style.mjs';

/** Borrow immutable primitive parameters; retain actual assigned parameters. */
function readPortableSourceParameterDeclarations(source, declarations, ownedParameters) {
  const {tokens: sourceTokens} = tokenize(source, 'Rust');
  const tokenPositions = new Map(sourceTokens.map((token, index) => [token.start, index]));
  const delimiters = new Map(), openings = [];
  for (const [index, token] of sourceTokens.entries()) {
    if (token.kind !== 'punct') continue;
    if (['(', '{', '['].includes(token.value)) openings.push(index);
    else if ([')', '}', ']'].includes(token.value)) delimiters.set(openings.pop(), index);
  }
  for (const match of source.matchAll(/^pub fn (\w+)\(/gmu)) {
    const [, name] = match;
    const parameterStart = tokenPositions.get(match.index + match[0].length - 1);
    const parameterEnd = delimiters.get(parameterStart);
    assert.notEqual(parameterEnd, undefined, 'a generated library function retains its complete signature: ' + name);
    let bodyStart = parameterEnd + 1;
    while (bodyStart < sourceTokens.length && !(sourceTokens[bodyStart].kind === 'punct' && sourceTokens[bodyStart].value === '{')) {
      bodyStart = (delimiters.get(bodyStart) ?? bodyStart) + 1;
    }
    const bodyEnd = delimiters.get(bodyStart);
    assert.notEqual(bodyEnd, undefined, 'a generated library function retains its complete body: ' + name);
    const block = source.slice(match.index, sourceTokens[bodyEnd].end);
    const body = sourceTokens.slice(bodyStart, bodyEnd + 1);
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
    const parameterSources = [];
    let parameterFrom = sourceTokens[parameterStart].end;
    let genericDepth = 0;
    for (let index = parameterStart + 1; index < parameterEnd; index += 1) {
      if (delimiters.has(index)) {
        index = delimiters.get(index);
        continue;
      }
      const token = sourceTokens[index];
      if (token.kind !== 'punct') continue;
      if (token.value === '<' || token.value === '<<') genericDepth += token.value.length;
      if (token.value === '>' || token.value === '>>') genericDepth -= token.value.length;
      if (token.value === ',' && genericDepth === 0) {
        parameterSources.push(source.slice(parameterFrom, token.start));
        parameterFrom = token.end;
      }
    }
    parameterSources.push(source.slice(parameterFrom, sourceTokens[parameterEnd].start));
    const params = parameterSources.map(value => /^\s*(?:mut )?(\w+):\s*([\s\S]+?)\s*$/u.exec(value)).filter(Boolean)
      .map(([, parameter, type]) => ({name: parameter, type}));
    for (const param of params) {
      let element = param.type;
      let depth = 0;
      while (element.startsWith('Vec<') && element.endsWith('>')) {
        element = element.slice(4, -1);
        depth += 1;
      }
      const textLeaf = /^(?:[A-Za-z_][A-Za-z_0-9]*::)*ml_string::Text$/u.test(element);
      param.text = depth === 0 && textLeaf;
      param.borrow = (param.type === 'String' || param.text || depth > 0 && (['String', 'f64', 'bool'].includes(element) || textLeaf))
        && !assigned.has(param.name)
        && !(Object.hasOwn(ownedParameters, name) && ownedParameters[name].includes(param.name));
    }
    declarations.set(name, {block, params, assigned, unique: !declarations.has(name)});
  }
}

export function readPortableSourceParameterStyles(source, {publicNames = [], ownedParameters = {}, normalizeBorrowedArguments = false, completeItems} = {}) {
  const declarations = new Map();
  for (const item of completeItems ?? [source]) readPortableSourceParameterDeclarations(item, declarations, ownedParameters);
  const adaptedItems = completeItems && [...completeItems];
  // A complete emitter item contains each retained top-level function it owns.
  // Preserve global call resolution while releasing the lexer for each item.
  const itemPositions = adaptedItems && new Map(adaptedItems.flatMap((item, position) =>
    [...item.matchAll(/^pub fn (\w+)\(/gmu)].map(match => [match[1], position])));
  let adapted = source;
  const actions = [];
  for (const [name, declaration] of declarations) {
    let block = declaration.block;
    for (const param of declaration.params) if (param.borrow) {
      const type = param.type === 'String' ? '&str' : param.text ? '&' + param.type : '&[' + param.type.slice(4, -1) + ']';
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
      const copy = parameter.type === 'String' ? '.to_owned()' : parameter.text ? '.clone()' : '.to_vec()';
      let left = index, right = index;
      while (before[left - 1]?.value === '(' && parentheses.get(left - 1) === right + 1) {
        const previous = before[left - 2];
        if (previous?.kind === 'macro' || previous?.kind === 'identifier'
          && !['if', 'while', 'return', 'match', 'break'].includes(previous.value)
          || [')', ']', '?'].includes(previous?.value)) break;
        left -= 1; right += 1;
      }
      // Text has value equality/ordering with Text, not with &Text. Comparing
      // borrowed parameters dereferences their actual value; comparison still
      // borrows its operands and never moves a Text out of the parameter.
      if (parameter.text && (['==', '!=', '<', '<=', '>', '>='].includes(before[left - 1]?.value)
        || ['==', '!=', '<', '<=', '>', '>='].includes(before[right + 1]?.value))) {
        copies.push([token.start, token.end, '(*' + token.value + ')']);
        continue;
      }
      if (before[index + 1]?.value === '.' && before[index + 2]?.value === 'clone'
        && before[index + 3]?.value === '(' && before[index + 4]?.value === ')') {
        if (parameter.type !== 'String' && before[index - 1]?.value === '&') {
          copies.push([before[index - 1].start, before[index + 4].end, token.value]);
        } else {
          copies.push([token.start, before[index + 4].end, token.value + copy]);
        }
        index += 4;
      } else if (parameter.text && before[left - 1]?.kind === 'punct' && before[left - 1].value === '&'
        && [',', ')', ']', '}', ';'].includes(before[right + 1]?.value)) {
        // A parenthesized leaf remains that same borrowed Text. Keep its
        // grouping and remove only the redundant outer loan, never a loan of
        // a compound operand such as &(text.slice(...)).
        copies.push([before[left - 1].start, before[left - 1].end, '']);
      } else if (before[index - 1]?.value === '&' && [',', ')'].includes(before[index + 1]?.value)) {
        copies.push([before[index - 1].start, before[index - 1].end, '']);
      } else if (index > bodyStart && ['return', '=', ',', '(', '[', '{', '=>'].includes(before[index - 1]?.value)
        && [',', ';', ')', ']', '}'].includes(before[index + 1]?.value)) {
        // Final-use emission may transfer an owned parameter directly in a
        // return, binding, tuple/array slot or owned call. Once its interface
        // is borrowed, retain that original owned value with one copy here.
        // Actual borrowed callees remove these leaf copies below, and an
        // explicit reference (including parenthesized ones) stays borrowed.
        if (before[left - 1]?.value !== '&') {
          copies.push([token.start, token.end, token.value + copy]);
        }
      }
    }
    for (const [start, end, text] of copies.sort((left, right) => right[0] - left[0])) {
      block = block.slice(0, start) + text + block.slice(end);
    }
    const {tokens, comments} = tokenize(block, 'Rust');
    const body = tokens.findIndex(token => token.kind === 'punct' && token.value === '{');
    const existingLoans = normalizeBorrowedArguments && declaration.unique && source.split(declaration.block).length === 2
      && !tokens.slice(body + 1).some(token => token.kind === 'macro'
        || token.kind === 'identifier' && ['let', 'match', 'for', 'fn', 'move', 'ref', 'unsafe', 'use'].includes(token.value)
        || token.kind === 'punct' && ['|', '||'].includes(token.value));
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
        if (from === end || !target.params[position]) continue;
        const targetParameter = target.params[position];
        if (existingLoans && target.unique && (targetParameter.type === '&str'
          || targetParameter.borrow && targetParameter.type === 'String')
          && tokens[index - 1]?.value === '::' && ['self', 'crate'].includes(tokens[index - 2]?.value)
          && tokens[index - 3]?.value !== '::') {
          let first = from, last = end;
          while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
          if (tokens[first]?.value === '&') {
            first += 1;
            while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
            const name = tokens[first]?.value;
            if (last === first + 1 && tokens[first]?.kind === 'identifier'
              && declaration.params.filter(parameter => parameter.name === name && parameter.type === '&str').length === 1
              && !declaration.assigned.has(name)
              && !comments.some(comment => comment.start >= tokens[from].start && comment.end <= tokens[end - 1].end)) {
              edits.push([tokens[from].start, tokens[end - 1].end, name]);
              continue;
            }
          }
        }
        if (!targetParameter.borrow) continue;
        const text = block.slice(tokens[from].start, tokens[end - 1].end);
        let first = from, last = end;
        while (tokens[first]?.value === '(' && matching.get(first) === last - 1) { first += 1; last -= 1; }
        const grouped = block.slice(tokens[first].start, tokens[last - 1].end);
        const leaf = /^\w+(?:\.(?:to_vec|to_owned|clone)\(\))?$/u.test(grouped)
          || /^String::from\("[^"\\]*"\)$/u.test(grouped);
        const borrowed = (leaf ? grouped : text).replace(/^(\w+)\.(?:to_vec|to_owned|clone)\(\)$/u, '$1');
        const parameter = declaration.params.find(value => value.name === borrowed);
        const literal = target.params[position].type === 'String' && /^String::from\(("[^"\\]*")\)$/u.exec(borrowed);
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
      if (adaptedItems && declaration.unique) {
        const position = itemPositions.get(name);
        adaptedItems[position] = adaptedItems[position].split(declaration.block).join(block);
      } else if (adaptedItems) {
        for (const [position, item] of adaptedItems.entries()) adaptedItems[position] = item.split(declaration.block).join(block);
      } else adapted = adapted.split(declaration.block).join(block);
    }
  }
  return adaptedItems ? {source: adaptedItems.join('\n\n'), actions, completeItems: adaptedItems} : {source: adapted, actions};
}

/** Place complete top-level items in bounded child modules with the same API. */
export function partitionPortableRustLibrary(source, {notice = '', directory, maximumLines = 900, completeItems} = {}) {
  assert.match(directory, /^[a-z][a-z_0-9]*$/u, 'a portable library has one explicit child-module directory');
  assert.ok(Number.isSafeInteger(maximumLines) && maximumLines > 0 && maximumLines < 990);
  const lines = source.trimEnd().split('\n');
  // Attributes belong to their following declaration, including attributes
  // rustfmt expands over multiple lines. Read balanced token spans so quoted
  // brackets and nested attribute arguments cannot create module boundaries.
  const attributeLines = new Map();
  const publicDeclarations = new Map();
  let lineOffset = 0;
  for (const item of completeItems ?? [source]) {
    const tokens = tokenize(item, 'Rust').tokens;
    const lineStarts = [0];
    for (let newline = item.indexOf('\n'); newline !== -1; newline = item.indexOf('\n', newline + 1)) {
      lineStarts.push(newline + 1);
    }
    const lineAt = offset => {
      let first = 0, last = lineStarts.length;
      while (first + 1 < last) {
        const middle = first + Math.floor((last - first) / 2);
        if (lineStarts[middle] <= offset) first = middle;
        else last = middle;
      }
      return lineOffset + first;
    };
    let braceDepth = 0;
    for (let index = 0; index < tokens.length; index += 1) {
      const token = tokens[index];
      if (token.kind === 'punct' && token.value === '{') braceDepth += 1;
      if (token.kind === 'punct' && token.value === '}') braceDepth -= 1;
      if (braceDepth !== 0) continue;
      if (token.kind === 'identifier' && token.value === 'pub') {
        const declaration = tokens[index + 1]?.value === 'const' ? index + 2 : index + 1;
        const kind = tokens[declaration]?.value, name = tokens[declaration + 1]?.value;
        if (['fn', 'enum', 'struct', 'mod'].includes(kind) && tokens[declaration + 1]?.kind === 'identifier') {
          const line = lineAt(token.start);
          if (/^pub (?:(?:const )?fn|enum|struct|mod) /u.test(lines[line])) publicDeclarations.set(line, {kind, name});
        }
      }
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
    assert.equal(braceDepth, 0, 'complete Rust source items retain balanced body delimiters');
    lineOffset += lineStarts.length + 1;
  }
  const starts = [];
  for (const [index, declaration] of publicDeclarations) {
    let start = index;
    while (start > (starts.at(-1)?.start ?? 0)) {
      if (attributeLines.has(start - 1)) start = attributeLines.get(start - 1);
      else if (/^(?:\s*|\/\/.*)$/u.test(lines[start - 1])) start -= 1;
      else break;
    }
    starts.push({start, declaration});
  }
  assert.ok(starts.length, 'a complete portable library has emitted public declarations');
  const prefix = lines.slice(0, starts[0].start).join('\n');
  starts.push({start: lines.length});
  const oversizedDeclarations = [];
  const parts = [{source: '', names: []}], nestedFiles=[];
  for (let index = 0; index + 1 < starts.length; index += 1) {
    const {start, declaration} = starts[index];
    let item = lines.slice(start, starts[index + 1].start).join('\n').trim();
    const itemLines = item.split('\n').length;
    if(itemLines>maximumLines){
      if (declaration.kind !== 'mod' || !/^[a-z][a-z_0-9]*$/u.test(declaration.name)) {
        oversizedDeclarations.push(`pub ${declaration.kind} ${declaration.name} (${itemLines} lines)`);
        continue;
      }
      const {name} = declaration;
      const itemTokens=tokenize(item,'Rust').tokens,first=itemTokens.findIndex(token=>token.kind==='identifier'&&token.value==='pub');
      assert.ok(first>=0&&itemTokens[first+1]?.value==='mod'&&itemTokens[first+2]?.value===name,'nested placement cannot split an enclosing function/type body');
      const block=readSourceTextRustBlock(item,'mod',name);
      const blockTokens=tokenize(block,'Rust').tokens;
      const opening=blockTokens.find(token=>token.kind==='punct'&&token.value==='{');
      const closing=blockTokens.filter(token=>token.kind==='punct'&&token.value==='}').at(-1);
      assert.ok(opening&&closing&&item.split(block).length===2,'nested placement needs one whole actual inline module');
      const body=block.slice(opening.end,closing.start);
      const bodyTokens=tokenize(body,'Rust').tokens;
      assert.ok(!bodyTokens.some((token,index)=>token.kind==='identifier'&&token.value==='super'&&bodyTokens[index+1]?.value==='::'),'nested placement needs an explicit relocation proof before moving parent-relative paths');
      const formatted=spawnSync('rustfmt',['--edition','2024','--emit','stdout'],{input:body,encoding:'utf8',maxBuffer:8*1024*1024});
      assert.equal(formatted.status,0,formatted.stderr??formatted.error?.message);
      const children=partitionPortableRustLibrary(formatted.stdout,{notice,directory:name,maximumLines});
      for(const child of children)nestedFiles.push({path:directory+'/'+(child.path??name+'.rs'),source:child.source});
      item=item.replace(block,'#[path = "'+name+'.rs"]\npub mod '+name+';');
    }
    assert.ok(item.split('\n').length <= maximumLines, 'a generated declaration must fit a bounded source module without splitting its body');
    const previous = parts.at(-1);
    if (previous.source && previous.source.split('\n').length + item.split('\n').length + 2 > maximumLines) {
      parts.push({source: item, names: [declaration.name]});
    } else {
      previous.source += (previous.source ? '\n\n' : '') + item;
      previous.names.push(declaration.name);
    }
  }
  assert.equal(oversizedDeclarations.length, 0, 'generated functions and types must fit the '
    + maximumLines + '-line limit without splitting their bodies:\n' + oversizedDeclarations.join('\n'));
  const root = notice + prefix + '\n' + parts.map((_, index) => `#[path = "${directory}/part_${index + 1}.rs"]\nmod part_${index + 1};\npub use part_${index + 1}::*;\n`).join('');
  assert.ok(root.split('\n').length < 1000, 'the portable library root retains its bounded size');
  const declarations = new Set([...publicDeclarations.values()].map(({name}) => name));
  return [{path: null, source: root}, ...parts.map(({source: part, names: localNames}, index) => {
    const local = new Set(localNames);
    const partTokens = tokenize(part, 'Rust').tokens;
    const needed = new Set();
    for (let at = 0; at < partTokens.length; at += 1) {
      const token = partTokens[at];
      if (token.kind !== 'identifier' || !declarations.has(token.value) || local.has(token.value)) continue;
      // Relocated emitted references use self::Name; ordinary unqualified
      // references need the same sibling import. A different path or field
      // with a coincidentally equal name does not import a declaration.
      const previous = partTokens[at - 1]?.value;
      if (previous === '.' || previous === '::' && partTokens[at - 2]?.value !== 'self') continue;
      if (previous !== '::' && !['(', '::'].includes(partTokens[at + 1]?.value)
        && ![':', '->', '<'].includes(previous)) continue;
      needed.add(token.value);
    }
    const names = [...needed].sort();
    // Keep one explicit import per sibling and let the selected edition's
    // formatter order numeric continuation names as well as ordinary names.
    const imports = names.length ? spawnSync('rustfmt', ['--edition', '2024', '--emit', 'stdout'], {
      input: names.map(name => `use super::${name};\n`).join(''), encoding: 'utf8',
    }) : {status: 0, stdout: ''};
    assert.equal(imports.status, 0, imports.stderr ?? imports.error?.message);
    const text = notice + imports.stdout + (imports.stdout ? '\n' : '') + part + '\n';
    assert.ok(text.split('\n').length < 1000, 'explicit sibling imports retain a bounded child module');
    return {path: `${directory}/part_${index + 1}.rs`, source: text};
  }),...nestedFiles];
}
