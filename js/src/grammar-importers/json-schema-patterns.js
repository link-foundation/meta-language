import { GrammarBuilder as Expression } from '../grammar.js';
import { parseNative } from '../native-grammar-parser.js';
import { jsonStringCharacter } from './json-schema-property-names.js';
import { boundedPatternRepetition, consumingPatternLowerBound, scalarFromSurrogatePair } from './json-schema-pattern-policy.js';

const { choice, empty, optional, repeat, repeat0, seq } = Expression;
const scalars = [[0, 0xd7ff], [0xe000, 0x10ffff]];
const digit = [[48, 57]];
const word = [[48, 57], [65, 90], [95, 95], [97, 122]];
const whitespace = [[9, 13], [32, 32], [160, 160], [0x1680, 0x1680], [0x2000, 0x200a],
  [0x2028, 0x2029], [0x202f, 0x202f], [0x205f, 0x205f], [0x3000, 0x3000], [0xfeff, 0xfeff]];
const classes = new Map([['d', digit], ['w', word], ['s', whitespace]]);
const controls = new Map([['0', 0], ['b', 8], ['t', 9], ['n', 10], ['v', 11], ['f', 12], ['r', 13]]);
const wild = repeat0(Expression.ref('json_pattern_character'));

function union(ranges) {
  const result = [];
  for (const [start, end] of [...ranges].sort((left, right) => left[0] - right[0])) {
    const previous = result.at(-1);
    if (previous && previous[1] >= start - 1) previous[1] = Math.max(previous[1], end);
    else result.push([start, end]);
  }
  return result;
}

export function complementScalarRanges(ranges) {
  const result = [];
  for (const [start, end] of scalars) {
    let offset = start;
    for (const [first, last] of ranges) {
      if (last < offset || first > end) continue;
      if (offset < first) result.push([offset, first - 1]);
      offset = last + 1;
    }
    if (offset <= end) result.push([offset, end]);
  }
  return result;
}

function single(value) {
  return value === undefined || (value >= 0xd800 && value <= 0xdfff) || value > 0x10ffff
    ? null : [[value, value]];
}

function characterRanges(term, spelling) {
  if (['pattern_character', 'class_character'].includes(term)) return single(spelling.codePointAt(0));
  if (term === 'any_character') return complementScalarRanges([[10, 10], [13, 13], [0x2028, 0x2029]]);
  if (term === 'identity_escape' && /^\\[\^$\\.*+?()[\]{}|/\-]$/u.test(spelling)) return single(spelling.codePointAt(1));
  if (term === 'control_escape') {
    return single(controls.get(spelling[1]) ?? (/^\\x[0-9a-f]{2}$/iu.test(spelling) ? Number.parseInt(spelling.slice(2), 16) : undefined));
  }
  if (term === 'control_letter_escape') return single(spelling.charCodeAt(2) % 32);
  if (term !== 'character_class_escape') return null;
  const escape = spelling[1];
  if (classes.has(escape.toLowerCase())) {
    const ranges = classes.get(escape.toLowerCase());
    return escape === escape.toLowerCase() ? ranges : complementScalarRanges(ranges);
  }
  const unicode = /^\\u(?:([0-9a-f]{4})|\{([0-9a-f]+)\})$/iu.exec(spelling);
  return unicode ? single(Number.parseInt(unicode[1] ?? unicode[2], 16)) : null;
}

const nothing = { nullable: false, consuming: null };
const epsilon = { nullable: true, consuming: null };
const alternatives = entries => entries.length ? choice(...entries) : null;
const complete = ({ nullable, consuming }) => nullable ? (consuming ? optional(consuming) : empty()) : consuming;

function concatenate(left, right) {
  const tail = complete(right);
  return {
    nullable: left.nullable && right.nullable,
    consuming: alternatives([
      left.consuming && tail && seq(left.consuming, tail),
      left.nullable && right.consuming,
    ].filter(Boolean)),
  };
}

function quantify(atom, spelling) {
  const counted = /^\{([0-9]+)(?:,([0-9]*))?\}\??$/u.exec(spelling);
  const minimum = counted ? Number(counted[1]) : Number(spelling.startsWith('+'));
  const maximum = counted ? (counted[2] === '' ? null : Number(counted[2] ?? minimum)) : (spelling.startsWith('?') ? 1 : null);
  if (!boundedPatternRepetition(minimum, maximum ?? -1)) return null;
  const min = consumingPatternLowerBound(atom.nullable, minimum);
  if (maximum === 0 || (!atom.consuming && min === 0)) return epsilon;
  if (!atom.consuming) return nothing;
  return { nullable: min === 0, consuming: repeat(atom.consuming, Math.max(1, min), maximum) };
}

export function lowerJsonStringPattern(source, registerPatternRule) {
  if (typeof source !== 'string' || Buffer.byteLength(source) > 1024 || !source.isWellFormed()) return null;
  try { new RegExp(source, 'u'); } catch { return null; }
  if (source === '') return { expression: wild };
  // Consume escaped backslashes before looking for Unicode-mode pairs.
  source = source.replace(/\\\\|\\u([dD][89a-bA-B][0-9a-fA-F]{2})\\u([dD][c-fC-F][0-9a-fA-F]{2})/gu,
    (escape, lead, trail) => lead
      ? '\\u{' + scalarFromSurrogatePair(Number.parseInt(lead, 16), Number.parseInt(trail, 16)).toString(16) + '}'
      : escape);
  const root = parseNative('native-regex', source);
  if (root.hasError || root.start !== 0 || root.end !== Buffer.byteLength(source)) return null;
  const bytes = Buffer.from(source);
  const text = node => bytes.subarray(node.start, node.end).toString();
  const children = node => node.children.map(child => child.node);
  const quanta = new Set(['optional', 'one_or_more', 'zero_or_more', 'count_quantifier']);
  const characters = new Map();
  function atom(node, depth) {
    if (['anonymous_capturing_group', 'non_capturing_group'].includes(node.term)) {
      const inner = children(node).find(child => child.term === 'pattern');
      const result = inner ? pattern(inner, false, depth + 1) : null;
      // A counted repetition or an optional prefix can reference this group
      // more than once. Keep one production for the group's consuming body.
      return result?.consuming ? { ...result, consuming: registerPatternRule(result.consuming) } : result;
    }
    let ranges;
    if (node.term === 'character_class') {
      const items = [];
      for (const child of children(node)) {
        if (['[', ']', '^'].includes(child.term)) continue;
        if (child.term === 'class_range') {
          const [start, , end] = children(child);
          const lower = characterRanges(start.term, text(start));
          const upper = characterRanges(end.term, text(end));
          if (lower?.length !== 1 || upper?.length !== 1 || lower[0][0] !== lower[0][1]
            || upper[0][0] !== upper[0][1] || lower[0][0] > upper[0][1]) return null;
          items.push([lower[0][0], upper[0][1]]);
        } else {
          const item = characterRanges(child.term, text(child));
          if (!item) return null;
          items.push(...item);
        }
      }
      ranges = union(items);
      if (text(node).startsWith('[^')) ranges = complementScalarRanges(ranges);
    } else ranges = characterRanges(node.term, text(node));
    if (!ranges) return null;
    const key = JSON.stringify(ranges);
    if (!characters.has(key)) {
      const expression = jsonStringCharacter(ranges);
      characters.set(key, expression && registerPatternRule(expression));
    }
    const consuming = characters.get(key);
    return consuming ? { nullable: false, consuming } : nothing;
  }
  function branch(nodes, outer, depth) {
    const start = nodes[0]?.term === 'start_assertion';
    const end = nodes.at(-1)?.term === 'end_assertion';
    if (!outer && (start || end)) return null;
    const body = nodes.slice(Number(start), end ? -1 : undefined);
    let result = epsilon;
    for (let index = 0; index < body.length; index++) {
      let value = atom(body[index], depth);
      if (!value) return null;
      if (quanta.has(body[index + 1]?.term)) value = quantify(value, text(body[++index]));
      if (!value) return null;
      result = concatenate(result, value);
    }
    const longer = { nullable: true, consuming: repeat(Expression.ref('json_pattern_character'), 1, null) };
    if (outer && !start) result = concatenate(longer, result);
    if (outer && !end) result = concatenate(result, longer);
    return result;
  }
  function pattern(node, outer, depth) {
    if (depth > 32) return null;
    const [body] = children(node);
    if (body?.term === 'term') return branch(children(body), outer, depth);
    if (body?.term !== 'alternation') return null;
    const parts = [[]];
    for (const child of children(body)) {
      if (child.term === '|') parts.push([]);
      else if (child.term === 'term') parts.at(-1).push(...children(child));
      else return null;
    }
    const results = parts.map(part => branch(part, outer, depth));
    if (results.some(part => !part)) return null;
    return { nullable: results.some(part => part.nullable), consuming: alternatives(results.map(part => part.consuming).filter(Boolean)) };
  }
  const result = pattern(root, true, 0);
  return result && { expression: complete(result) };
}

export function anyJsonPatternCharacter() {
  return jsonStringCharacter(scalars);
}
