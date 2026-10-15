// Packaging changes concrete syntax only. Every change is replayed by the
// ordinary Links decorator engine; the original complete artifact is retained.
// Conservation applies to the supported automatically generated pure-decision
// subset. Token counts alone do not prove unrestricted Rust equivalence for
// custom Drop implementations, temporary scopes or other observable lifetimes.
import { tokenize } from '../src/translation/lexer.js';
import { DecoratorSet } from '../src/decorators.js';
import { acceptRustPureUnitName } from '../src/translation/rust-pure-unit-call-rules.js';

function syntax(source) {
  const { tokens, comments } = tokenize(source, 'Rust');
  const matching = new Map(), stack = [];
  for (let index = 0; index < tokens.length; index += 1) {
    if (tokens[index].kind !== 'punct') continue;
    const value = tokens[index].value;
    if (['(', '[', '{'].includes(value)) stack.push(index);
    else if ([')', ']', '}'].includes(value)) {
      const open = stack.pop();
      if (open === undefined || tokens[open].value !== ({ ')': '(', ']': '[', '}': '{' })[value]) {
        throw new Error('Rust library source has unbalanced delimiters');
      }
      matching.set(open, index); matching.set(index, open);
    }
  }
  if (stack.length) throw new Error('Rust library source has unbalanced delimiters');
  const macroTokens = new Set(), macroOwners = new Map();
  for (let index = 0; index < tokens.length - 1; index += 1) {
    if (tokens[index].kind !== 'macro' || !['(', '[', '{'].includes(tokens[index + 1].value)) continue;
    const close = matching.get(index + 1);
    for (let at = index + 1; at <= close; at += 1) { macroTokens.add(at); macroOwners.set(at, tokens[index].value); }
  }
  return { tokens, comments, matching, macroTokens, macroOwners };
}

/** Structural counts do not invoke a compiler or a formatter. */
export function readRustLibraryNesting(source) {
  const { tokens } = syntax(source);
  let depth = 0, blockDepth = 0, maximumDelimiterDepth = 0, maximumBlockDepth = 0, conditionals = 0, alternatives = 0;
  for (const token of tokens) {
    if (token.kind === 'identifier' && token.value === 'if') conditionals += 1;
    if (token.kind === 'identifier' && token.value === 'else') alternatives += 1;
    if (token.kind !== 'punct') continue;
    if (['(', '[', '{'].includes(token.value)) { depth += 1; maximumDelimiterDepth = Math.max(maximumDelimiterDepth, depth); }
    if (token.value === '{') { blockDepth += 1; maximumBlockDepth = Math.max(maximumBlockDepth, blockDepth); }
    if ([')', ']', '}'].includes(token.value)) depth -= 1;
    if (token.value === '}') blockDepth -= 1;
  }
  return { maximumDelimiterDepth, maximumBlockDepth, conditionals, alternatives };
}

/** Inventory complete top-level items, including every leading trivia byte. */
export function readRustLibraryItems(source) {
  const { tokens, matching } = syntax(source), items = [];
  let cursor = 0, start = 0;
  while (tokens[cursor]?.kind !== 'eof') {
    const first = cursor;
    let header = false;
    while (tokens[cursor]?.value === '#') {
      const inner = tokens[cursor + 1]?.value === '!';
      const open = cursor + (inner ? 2 : 1);
      if (tokens[open]?.value !== '[' || !matching.has(open)) throw new Error('Rust library attribute has no balanced body');
      cursor = matching.get(open) + 1;
      if (inner) { header = true; break; }
    }
    let kind, name;
    if (header) { kind = 'inner attribute'; name = `attribute_${items.length}`; }
    else {
      let head = cursor;
      if (tokens[head]?.value === 'pub') {
        head += 1;
        if (tokens[head]?.value === '(') head = matching.get(head) + 1;
      }
      while (['unsafe', 'async', 'const'].includes(tokens[head]?.value) && tokens[head + 1]?.value === 'fn') head += 1;
      kind = tokens[head]?.value; name = tokens[head + 1]?.value;
      if (!['fn', 'mod', 'enum', 'struct', 'use', 'type', 'const', 'static'].includes(kind)) {
        throw new Error(`Rust library item is not represented: ${kind}`);
      }
      let end = cursor, terminated = false;
      while (tokens[end]?.kind !== 'eof') {
        if (tokens[end].kind === 'punct' && tokens[end].value === ';') { end += 1; terminated = true; break; }
        if (tokens[end].kind === 'punct' && tokens[end].value === '{') { end = matching.get(end) + 1; terminated = true; break; }
        if (tokens[end].kind === 'punct' && ['(', '['].includes(tokens[end].value)) end = matching.get(end) + 1;
        else end += 1;
      }
      if (!terminated) throw new Error(`Rust library item ${name} has no complete boundary`);
      cursor = end;
    }
    const end = tokens[cursor - 1].end;
    items.push({ id: `item_${items.length}`, kind, name, start, end,
      tokenStart: tokens[first].start, source: source.slice(start, end) });
    start = end;
  }
  return { items, trailing: source.slice(start) };
}

function edits(source, replacements) {
  let previous = source.length;
  for (const { start, end, text } of [...replacements].sort((left, right) => right.start - left.start)) {
    if (end > previous || start < 0 || start > end) throw new Error('Rust library source styles overlap');
    source = source.slice(0, start) + text + source.slice(end); previous = start;
  }
  return source;
}

// Read only the if-expression grammar needed for the redundant outer block.
// A candidate with trailing statements, a semicolon or attributes is retained.
function conditional(tokens, matching, start, cache) {
  if (cache.has(start)) return cache.get(start);
  let body = start + 1;
  while (body < tokens.length) {
    if (tokens[body].kind === 'punct' && tokens[body].value === '{') break;
    if (tokens[body].kind === 'punct' && ['(', '['].includes(tokens[body].value)) body = matching.get(body) + 1;
    else if (tokens[body].value === 'match') return null;
    else body += 1;
  }
  if (!matching.has(body)) return null;
  let end = matching.get(body);
  if (tokens[end + 1]?.value === 'else') {
    const next = end + 2;
    if (tokens[next]?.value === '{') end = matching.get(next);
    else if (tokens[next]?.value === 'if') {
      const child = conditional(tokens, matching, next, cache);
      if (!child) return null;
      end = child.end;
    } else return null;
  }
  const result = { end }; cache.set(start, result); return result;
}

function flattenBranches(source) {
  const { tokens, matching, macroTokens } = syntax(source), removed = new Set(), cache = new Map();
  for (let index = 0; index < tokens.length; index += 1) {
    if (macroTokens.has(index) || tokens[index].kind !== 'identifier' || tokens[index].value !== 'else'
      || tokens[index + 1]?.value !== '{' || tokens[index + 2]?.kind !== 'identifier' || tokens[index + 2]?.value !== 'if') continue;
    const open = index + 1, close = matching.get(open), child = conditional(tokens, matching, open + 1, cache);
    if (child && child.end + 1 === close) { removed.add(open); removed.add(close); }
  }
  const result = edits(source, [...removed].map(index => ({ start: tokens[index].start, end: tokens[index].end, text: '' })));
  const after = syntax(result);
  const key = token => [token.kind, token.raw ?? token.value];
  if (JSON.stringify(after.tokens.map(key)) !== JSON.stringify(tokens.filter((_, index) => !removed.has(index)).map(key))
    || JSON.stringify(after.comments.map(comment => comment.text)) !== JSON.stringify(syntax(source).comments.map(comment => comment.text))) {
    throw new Error('Rust library branch style changed an unapproved token or comment');
  }
  return { source: result, count: removed.size / 2 };
}

function compactIndentation(source) {
  const { tokens, comments } = syntax(source), opaque = [...tokens.filter(token => token.kind !== 'eof'), ...comments]
    .sort((left, right) => left.start - right.start);
  const replacements = [];
  let interval = 0;
  for (const match of source.matchAll(/^[\t ]+/gmu)) {
    const start = match.index, end = start + match[0].length;
    while (opaque[interval]?.end <= start) interval += 1;
    // Whitespace inside multi-line strings/comments is meaningful and retained.
    if (opaque[interval] && opaque[interval].start < end && opaque[interval].end > start) continue;
    replacements.push({ start, end, text: '' });
  }
  const result = edits(source, replacements), after = syntax(result);
  const key = token => [token.kind, token.raw ?? token.value];
  if (JSON.stringify(after.tokens.map(key)) !== JSON.stringify(tokens.map(key))
    || JSON.stringify(after.comments.map(comment => comment.text)) !== JSON.stringify(comments.map(comment => comment.text))) {
    throw new Error('Rust library indentation style changed a token or comment');
  }
  return result;
}

function qualify(source, modulePath, names, namespaceExpressionMacros) {
  const { tokens, macroTokens, macroOwners } = syntax(source), replacements = [];
  for (let index = 0; index < tokens.length - 2; index += 1) {
    if (tokens[index].kind === 'identifier' && tokens[index].value === 'crate'
      && tokens[index + 1].kind === 'punct' && tokens[index + 1].value === '::'
      && tokens[index + 2].kind === 'identifier' && names.has(tokens[index + 2].value)) {
      if (macroTokens.has(index) && !namespaceExpressionMacros.includes(macroOwners.get(index))) {
        throw new Error('Rust library path inside an unrepresented macro requires a separate rule');
      }
      replacements.push({ start: tokens[index].start, end: tokens[index + 1].end, text: `crate::${modulePath}::` });
    }
  }
  return edits(source, replacements);
}

/** Produce exact source actions for the ordinary translation-rule decorators. */
export function readRustLibraryStyles(source, { modulePath, namespaceExpressionMacros = [] }) {
  if (typeof modulePath !== 'string' || !/^[a-z][a-z0-9_]*(?:::[a-z][a-z0-9_]*)*$/u.test(modulePath)
    || !modulePath.split('::').every(segment => acceptRustPureUnitName(segment))) {
    throw new TypeError('Rust library packaging needs an explicit module path');
  }
  if (!Array.isArray(namespaceExpressionMacros) || namespaceExpressionMacros.some(name => name !== 'vec')) {
    throw new TypeError('Rust library namespace macros need a represented standard expression rule');
  }
  const inventory = readRustLibraryItems(source), names = new Set(inventory.items.filter(item => item.kind !== 'inner attribute').map(item => item.name));
  const records = [], decorators = [], observations = [];
  for (const item of inventory.items) {
    if (item.kind === 'inner attribute') {
      const { tokens, matching } = syntax(item.source);
      if (tokens[3]?.value !== 'allow' || tokens[4]?.value !== '(' || matching.get(4) !== tokens.length - 3) {
        throw new Error('Rust library inner attribute requires an explicit packaging rule');
      }
      continue;
    }
    const record = { id: item.id, source: item.source }, branches = flattenBranches(item.source);
    const target = qualify(compactIndentation(branches.source), modulePath, names, namespaceExpressionMacros);
    records.push(record);
    if (target !== item.source) decorators.push({ id: `rust-library-${item.id}`, level: 'translation-rule',
      when: [['id', item.id]], actions: [{ op: 'replace', field: 'source', from: item.source, to: target }] });
    observations.push({ id: item.id, kind: item.kind, name: item.name, flattenedBranches: branches.count });
  }
  const set = new DecoratorSet(decorators), restored = DecoratorSet.fromLino(set.toLino());
  const decorated = restored.decorateAll('translation-rule', records);
  if (decorated.length !== records.length || decorated.some((row, index) => row.id !== records[index].id)) {
    throw new Error('Rust library packaging changed item ownership or order');
  }
  return { inventory, observations, records, decorated, decorators: set };
}

/** Partition only at complete-item boundaries. No generated algorithm is split. */
export function packageRustLibrary(source, { modulePath, maximumLines = 900, namespaceExpressionMacros = [] } = {}) {
  if (!Number.isInteger(maximumLines) || maximumLines < 20 || maximumLines >= 1000) {
    throw new TypeError('Rust library line budget must be an integer between 20 and 999');
  }
  const styles = readRustLibraryStyles(source, { modulePath, namespaceExpressionMacros }), byId = new Map(styles.inventory.items.map(item => [item.id, item]));
  const groups = [];
  let group = [], lines = 1;
  for (const record of styles.decorated) {
    const size = record.source.split('\n').length + 1;
    if (size + 2 >= maximumLines) throw new Error(`Rust library item ${byId.get(record.id).name} exceeds its whole-item line budget`);
    if (group.length && lines + size >= maximumLines) { groups.push(group); group = []; lines = 1; }
    group.push(record); lines += size;
  }
  if (group.length) groups.push(group);
  const modules = groups.map((records, index) => ({ file: `part_${index}.rs`, itemIds: records.map(record => record.id),
    source: records.map(record => record.source).join('\n') + '\n' }));
  if (modules.length) modules.at(-1).source += styles.inventory.trailing;
  // include! keeps every original item in one lexical module, so private
  // wrappers and unqualified sibling calls retain their existing visibility.
  const root = modules.map(module => `include!("${module.file}");`).join('\n') + '\n';
  return { ...styles, modulePath, maximumLines, modules: [{ file: 'mod.rs', itemIds: [], source: root }, ...modules] };
}
