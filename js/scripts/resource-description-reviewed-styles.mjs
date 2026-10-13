// Reviewed source-structure corrections for the CI14 resource policy modules.
// These are human-reviewed transformations, not machine-applicable diagnostics.
import assert from 'node:assert/strict';
import { parseProgrammingLanguage } from '../src/programming-language-parser.js';
import { tokenize } from '../src/translation/lexer.js';

const field = (node, name) => node.children?.find(child => child.field === name);
const children = node => (node.children ?? []).filter(child => child.term !== 'whitespace' && !child.flags?.isExtra);
const unwrap = node => node?.term === 'parenthesized_expression' ? unwrap(children(node).find(child => child.term !== '(' && child.term !== ')')) : node;
const blockChildren = node => children(node).filter(child => !['{', '}'].includes(child.term));
const inner = node => node?.term === 'expression_statement' ? children(node).find(child => child.term !== ';') : node;
const alternative = node => inner(children(field(node, 'alternative') ?? {}).find(child => child.term !== 'else'));
const operator = node => children(node).find(child => child.field === 'operator' || ['&&', '||', '>=', '<=', '<', '>', '==', '!='].includes(child.term))?.term;
const tokenText = text => tokenize(text, 'Rust').tokens.map(token => token.kind === 'number' ? token.raw.replaceAll('_', '') : token.raw).join('');

export function reuseResourceArrayPrelude(source, provider) {
  const normalize = text => tokenText(text.replace(/#\[must_use\]\s*/gu, '').replace(/\bconst fn integer\b/gu, 'fn integer'))
    .replace(/,\)->/gu, ')->')
    .replace('{matchindex.and_then(|index|values.get(index)){Some(value)=>value.clone(),None=>panic!("array index out of range"),}}', '{checked_optional_element_read}')
    .replace('{index.and_then(|index|values.get(index)).map_or_else(||panic!("array index out of range"),Clone::clone)}', '{checked_optional_element_read}');
  const tree = parseProgrammingLanguage(provider, 'Rust')?.tree;
  const module = tree?.children.find(node => node.term === 'mod_item' && field(node, 'name')?.term === 'identifier'
    && provider.slice(field(node, 'name').span.byteRange.start, field(node, 'name').span.byteRange.end) === 'ml_array');
  assert.ok(module, 'the reviewed array provider must retain its complete module');
  const range = module.span.byteRange;
  const expected = Buffer.from(provider).subarray(range.start, range.end).toString();
  assert.ok(normalize(source) === normalize(expected), 'every array helper signature and operation must match the checked provider');
  return 'pub use crate::translation::frontend_rules::ml_array;\n';
}

function reviewedPass(source) {
  const bytes = Buffer.from(source);
  const tree = parseProgrammingLanguage(source, 'Rust')?.tree;
  assert.ok(tree && !tree.flags.hasError, 'reviewed styles require a valid Rust snapshot');
  const text = node => bytes.subarray(node.span.byteRange.start, node.span.byteRange.end).toString();
  const candidates = [];
  const add = (node, replacement, reason) => {
    if (replacement !== text(node)) candidates.push({ start: node.span.byteRange.start, end: node.span.byteRange.end, replacement, reason });
  };
  const terminal = node => {
    const value = inner(node);
    if (!value) return false;
    if (['continue_expression', 'return_expression', 'break_expression'].includes(value.term)) return true;
    if (value.term === 'block') return terminal(blockChildren(value).at(-1));
    if (value.term === 'if_expression') return terminal(field(value, 'consequence')) && terminal(alternative(value));
    return false;
  };
  const number = node => {
    const value = tokenText(text(unwrap(node)));
    return /^-?(?:[0-9][0-9_]*)(?:\.[0-9_]*)?(?:f64)?$/u.test(value) ? Number(value.replaceAll('_', '').replace(/f64$/u, '')) : null;
  };
  const constraints = node => {
    const value = unwrap(node);
    if (value?.term !== 'binary_expression') return null;
    const op = operator(value), left = unwrap(field(value, 'left')), right = unwrap(field(value, 'right'));
    if (op === '==' && left?.term === 'identifier' && number(right) !== null) return [{ [text(left)]: number(right) }];
    const a = constraints(left), b = constraints(right);
    if (!a || !b) return null;
    if (op === '||') return [...a, ...b];
    if (op !== '&&') return null;
    return a.flatMap(x => b.filter(y => Object.keys(x).every(key => y[key] === undefined || y[key] === x[key])).map(y => ({ ...x, ...y })));
  };
  const visit = (node, parent) => {
    for (const child of node.children ?? []) visit(child, node);
    if (node.term === 'function_item') {
      const original = text(node);
      const tokens = tokenize(original, 'Rust').tokens;
      let changed = original;
      for (const name of ['starts', 'lengths', 'variants', 'singletons']) {
        if (tokens.filter(token => token.kind === 'identifier' && token.raw === name).length === 2) changed = changed.replace(new RegExp('\\b' + name + '\\.clone\\(\\)', 'gu'), name);
      }
      // The loop's Done arm returns its two owned position arrays immediately.
      if (text(field(node, 'name')) === 'ml_accept_resource_description_language_loop11') changed = changed.replace('MlAcceptResourceDescriptionLanguageLoop11Done(starts.clone(), lengths.clone())', 'MlAcceptResourceDescriptionLanguageLoop11Done(starts, lengths)');
      add(node, changed, 'move single-use owned scalar arrays and terminal continuation payloads');
    }
    if (node.term === 'parenthesized_expression') {
      const value = children(node).find(child => !['(', ')'].includes(child.term));
      const contents = text(node).slice(1, -1).trim();
      if (value?.term === 'parenthesized_expression' && contents === text(value)) add(node, text(value), 'retain one complete inner grouping when removing a duplicated outer pair');
      else if (parent?.term === 'arguments' && ['identifier', 'integer_literal', 'float_literal', 'unary_expression', 'binary_expression', 'type_cast_expression'].includes(value?.term)
        && contents === text(value)) add(node, contents, 'the argument list already delimits its complete scalar expression');
    }
    if (node.term === 'binary_expression' && operator(node) === '&&') {
      const left = unwrap(field(node, 'left')), right = unwrap(field(node, 'right'));
      if (left?.term === 'binary_expression' && right?.term === 'binary_expression') {
        const parts = [left, right].map(value => ({ op: operator(value), operand: unwrap(field(value, 'left')), bound: unwrap(field(value, 'right')) }));
        const low = parts.find(value => value.op === '>='), high = parts.find(value => ['<', '<='].includes(value.op));
        if (low && high && low.operand?.term === 'identifier' && tokenText(text(low.operand)) === tokenText(text(high.operand)) && number(low.bound) !== null && number(high.bound) !== null) {
          add(node, '(' + text(low.bound) + (high.op === '<=' ? '..=' : '..') + text(high.bound) + ').contains(&' + text(low.operand) + ')', 'positive finite-bound comparisons and contains both reject NaN');
        }
      }
    }
    if (node.term === 'binary_expression' && operator(node) === '+') {
      const left = field(node, 'left'), right = field(node, 'right');
      const product = [left, right].map(unwrap).find(value => value?.term === 'binary_expression' && operator(value) === '*'
        && /f64$/u.test(tokenText(text(field(value, 'right')))));
      if (product && (unwrap(left) === product || number(left) !== null)) {
        const operand = value => unwrap(value) === product ? 'separately_rounded_product' : text(value);
        add(node, '{ let separately_rounded_product = ' + text(product) + '; ' + operand(left) + ' + ' + operand(right) + ' }', 'retain separately rounded multiplication and addition, without fused arithmetic');
      }
    }
    if (node.term === 'block') {
      const values = blockChildren(node), declaration = values[0], tail = inner(values[1]);
      if (values.length === 2 && declaration?.term === 'let_declaration' && field(declaration, 'pattern')?.term === 'identifier' && tail?.term === 'identifier'
        && text(field(declaration, 'pattern')) === text(tail) && !children(declaration).some(child => child.term === 'mut')) {
        add(node, '{ ' + text(field(declaration, 'value')) + ' }', 'move the single-use binding expression directly into its original result');
      }
    }
    if (node.term !== 'if_expression') return;
    const condition = unwrap(field(node, 'condition')), consequence = field(node, 'consequence'), alternate = alternative(node);
    if (alternate?.term === 'block') {
      let positive;
      if (condition.term === 'unary_expression' && children(condition)[0]?.term === '!') positive = text(children(condition)[1]);
      else if (condition.term === 'binary_expression' && operator(condition) === '!=') positive = text(field(condition, 'left')) + ' == ' + text(field(condition, 'right'));
      if (positive) add(node, 'if ' + positive + ' ' + text(alternate) + ' else ' + text(consequence), 'evaluate the same predicate once and exchange complementary branches');
      if (terminal(consequence)) add(node, '{ if ' + text(field(node, 'condition')) + ' ' + text(consequence) + '\n' + text(alternate) + ' }', 'an abrupt branch cannot reach the following original alternative');
      const nested = blockChildren(alternate);
      if (nested.length === 1 && inner(nested[0])?.term === 'if_expression' && !text(nested[0]).trimEnd().endsWith(';')) add(node, 'if ' + text(field(node, 'condition')) + ' ' + text(consequence) + ' else ' + text(inner(nested[0])), 'retain the complete nested conditional as an else-if branch');
    }
    // These scalar transition tables have mutually exclusive equality premises.
    // Grouping equal results preserves all values, NaN, and signed-zero equality.
    const branches = [];
    let cursor = node;
    while (cursor?.term === 'if_expression') {
      branches.push({ condition: field(cursor, 'condition'), result: field(cursor, 'consequence') });
      cursor = alternative(cursor);
    }
    if (branches.length > 1 && cursor?.term === 'block') {
      const premises = branches.map(branch => constraints(branch.condition));
      const disjoint = premises.every(Boolean) && premises.every((a, index) => premises.slice(index + 1).every(b => a.every(x => b.every(y => Object.keys(x).some(key => y[key] !== undefined && y[key] !== x[key])))));
      const grouped = new Map();
      for (const branch of branches) {
        const signature = tokenText(text(branch.result));
        const entry = grouped.get(signature) ?? { result: branch.result, conditions: [] };
        entry.conditions.push(branch.condition); grouped.set(signature, entry);
      }
      if (disjoint && grouped.size < branches.length) add(node, [...grouped.values()].map((branch, index) => (index ? 'else if ' : 'if ') + branch.conditions.map(value => '(' + text(value) + ')').join(' || ') + ' ' + text(branch.result)).join(' ') + ' else ' + text(cursor), 'group only proved-disjoint scalar equality branches with identical results');
    }
  };
  visit(tree);
  const selected = [];
  for (const correction of candidates.sort((a, b) => (a.end - a.start) - (b.end - b.start))) if (!selected.some(other => correction.start < other.end && other.start < correction.end)) selected.push(correction);
  let result = bytes;
  for (const correction of selected.sort((a, b) => b.start - a.start)) result = Buffer.concat([result.subarray(0, correction.start), Buffer.from(correction.replacement), result.subarray(correction.end)]);
  return { source: result.toString(), reasons: selected.map(value => value.reason) };
}

export function reviewResourceDescriptionStyles(source, format) {
  const reasons = [];
  // Macro token trees do not expose their expanded arithmetic as CST binary
  // expressions. This reviewed scalar expression uses exactly the same two
  // operations; the temporary prevents a suggestion to fuse their rounding.
  const scalarProduct = '65536f64 + ((code - 55296f64) * 1024f64)';
  if (source.includes(scalarProduct)) {
    source = format(source.replaceAll(scalarProduct, '{ let separately_rounded_product = (code - 55296f64) * 1024f64; 65536f64 + separately_rounded_product }'));
    reasons.push('retain separately rounded scalar multiplication and addition inside the macro');
  }
  for (let round = 0; round < 30; round += 1) {
    const result = reviewedPass(source);
    if (result.source === source) return { source, reasons };
    source = format(result.source); reasons.push(...result.reasons);
  }
  throw new Error('reviewed resource styles did not reach a fixed point');
}
