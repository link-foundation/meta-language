import { readFileSync } from 'node:fs';
import { GrammarBuilder as Expression } from '../grammar.js';
import { parseGrammarLinks } from '../grammar-links.js';
import { jsonDocumentGrammar, pointerChild, readJsonDocument } from '../json-document.js';
import { analyzeJsonSchema, validateJsonDocument } from '../json-schema-validation.js';
import { lowerLarkRegex } from './lark-regex.js';

const { charClass, charRange, seq, choice, literal, ref, repeat, repeat0, optional, empty } = Expression;
const space = ref('json_space');
const value = ref('value');
const referenceKeys = new Set(['$ref', '$defs', '$id', '$schema', '$comment', 'title', 'description', 'default', 'examples', 'deprecated', 'readOnly', 'writeOnly']);
const jsonTypes = ['object', 'array', 'string', 'number', 'boolean', 'null'];
const characters = (value) => charClass(Array.from(value, character => ({ kind: 'char', value: character })));
const importedSchemas = new WeakMap();
const baseFile = new URL('../data/json-document/decoder.lino', import.meta.url);
let decoderBase;

export function originalJsonSchema(grammar) {
  const saved = importedSchemas.get(grammar);
  if (!saved || saved.grammar !== JSON.stringify(grammar.normalized())) return undefined;
  return structuredClone(saved.schema);
}

function emitterExpression(expression) {
  if (['alias', 'token', 'immediateToken', 'capture'].includes(expression.kind)) return emitterExpression(expression.item);
  if (expression.kind === 'choice') return choice(...expression.items.map(emitterExpression));
  if (expression.items && expression.kind === 'seq') return seq(...expression.items.map(emitterExpression));
  return expression.item ? { ...expression, item: emitterExpression(expression.item) } : expression;
}

function separated(expression) {
  return seq(space, literal(','), space, expression);
}

function collection(left, right, members) {
  return seq(literal(left), space, members, space, literal(right));
}

function unboundedList(item) {
  return optional(seq(item, repeat0(separated(item))));
}

function installJsonRules(builder) {
  builder.rule('json_space', repeat0(characters(' \t\n\r')));
  const base = jsonDocumentGrammar();
  for (const rule of base.rules.values()) {
    if (['document', 'object', 'pair', 'array'].includes(rule.name)) continue;
    builder.rule(rule.name, emitterExpression(rule.expression));
  }
  builder.rule('object', collection('{', '}', unboundedList(ref('pair'))));
  builder.rule('pair', seq(ref('string'), space, literal(':'), space, value));
  builder.rule('array', collection('[', ']', unboundedList(value)));
  builder.rule('integer', seq(optional(literal('-')), choice(literal('0'), seq(charRange('1', '9'), repeat0(charRange('0', '9'))))));
  builder.rule('boolean', choice(ref('true'), ref('false')));
}

function hexUnit(unit) {
  return seq(literal('\\u'), ...Array.from(unit.toString(16).padStart(4, '0'), digit => (
    /^[a-f]$/u.test(digit) ? characters(digit + digit.toUpperCase()) : literal(digit)
  )));
}

function quoted(value) {
  const characters = Array.from(value, character => {
    const escaped = seq(...Array.from({ length: character.length }, (_, index) => hexUnit(character.charCodeAt(index))));
    const ordinary = JSON.stringify(character).slice(1, -1);
    const alternatives = [literal(ordinary), escaped];
    if (character === '/') alternatives.push(literal('\\/'));
    return choice(...alternatives);
  });
  return seq(literal('"'), ...characters, literal('"'));
}

function property(key, expression) {
  return seq(quoted(key), space, literal(':'), space, expression);
}

function boundedTail(expression, min, max, hasPrefix) {
  if (max === 0) return empty();
  if (hasPrefix) return repeat(separated(expression), min, max);
  const body = seq(expression, repeat(separated(expression), Math.max(0, min - 1), max === null ? null : max - 1));
  return min === 0 ? optional(body) : body;
}

function constantExpression(instance) {
  if (typeof instance === 'string') return quoted(instance);
  if (Array.isArray(instance)) return collection('[', ']', seq(...instance.map((item, i) => {
    const expression = constantExpression(item);
    return i ? separated(expression) : expression;
  })));
  if (instance !== null && typeof instance === 'object') {
    return collection('{', '}', seq(...Object.entries(instance).map(([key, item], i) => {
      const expression = property(key, constantExpression(item));
      return i ? separated(expression) : expression;
    })));
  }
  return literal(JSON.stringify(instance));
}

function signedIntegerRange(first, last) {
  function rangeWithWidth(a, b) {
    const left = Number(a[0]);
    const right = Number(b[0]);
    const width = a.length - 1;
    if (width === 0) return left === right ? literal(a) : charRange(a, b);
    if (left === right) return seq(literal(a[0]), rangeWithWidth(a.slice(1), b.slice(1)));
    const alternatives = [seq(literal(a[0]), rangeWithWidth(a.slice(1), '9'.repeat(width)))];
    if (right - left > 1) alternatives.push(seq(charRange(String(left + 1), String(right - 1)), repeat(charRange('0', '9'), width, width)));
    alternatives.push(seq(literal(b[0]), rangeWithWidth('0'.repeat(width), b.slice(1))));
    return choice(...alternatives);
  }
  function unsigned(min, max) {
    const widths = [];
    const low = String(min);
    const high = String(max ?? min);
    for (let width = low.length; width <= high.length; width += 1) {
      const start = width === low.length ? low : `1${'0'.repeat(width - 1)}`;
      const end = max !== null && width === high.length ? high : '9'.repeat(width);
      widths.push(rangeWithWidth(start, end));
    }
    if (max === null) widths.push(seq(charRange('1', '9'), repeat(charRange('0', '9'), high.length)));
    return choice(...widths);
  }
  const parts = [];
  if (first === null || first <= 0) parts.push(seq(literal('-'), unsigned(last === null ? 0 : Math.max(0, -last), first === null ? null : -first)));
  if (last === null || last >= 0) parts.push(unsigned(Math.max(0, first ?? 0), last));
  return choice(...parts);
}

function uuidExpression() {
  const hex = charClass([{ kind: 'range', start: '0', end: '9' }, { kind: 'range', start: 'a', end: 'f' }, { kind: 'range', start: 'A', end: 'F' }]);
  return seq(repeat(hex, 8, 8), ...[4, 4, 4, 12].map(length => seq(literal('-'), repeat(hex, length, length))));
}

function decimalWidth(min, max, width = 2) {
  const alternatives = [];
  for (let prefix = Math.floor(min / 10); prefix <= Math.floor(max / 10); prefix += 1) {
    const first = Math.max(0, min - prefix * 10);
    const last = Math.min(9, max - prefix * 10);
    alternatives.push(seq(literal(String(prefix).padStart(width - 1, '0')), first === last ? literal(String(first)) : charRange(String(first), String(last))));
  }
  return choice(...alternatives);
}

function dateExpression() {
  const digit = charRange('0', '9');
  const normal = seq(repeat(digit, 4, 4), literal('-'), choice(
    ...['01', '03', '05', '07', '08', '10', '12'].map(month => seq(literal(`${month}-`), decimalWidth(1, 31))),
    ...['04', '06', '09', '11'].map(month => seq(literal(`${month}-`), decimalWidth(1, 30))),
    seq(literal('02-'), decimalWidth(1, 28)),
  ));
  const multiples = Array.from({ length: 25 }, (_, index) => literal(String(index * 4).padStart(2, '0')));
  const leap = seq(choice(seq(repeat(digit, 2, 2), choice(...multiples.slice(1))), seq(choice(...multiples), literal('00'))), literal('-02-29'));
  return choice(normal, leap);
}

function dateTimeExpression() {
  const hour = decimalWidth(0, 23);
  const minute = decimalWidth(0, 59);
  return seq(dateExpression(), characters('Tt'), hour, literal(':'), minute, literal(':'), decimalWidth(0, 60),
    optional(seq(literal('.'), repeat(charRange('0', '9'), 1))),
    choice(characters('Zz'), seq(characters('+-'), hour, literal(':'), minute)));
}

function emailExpression() {
  const alphaNumeric = [{ kind: 'range', start: 'a', end: 'z' }, { kind: 'range', start: 'A', end: 'Z' }, { kind: 'range', start: '0', end: '9' }];
  const atext = charClass([...alphaNumeric, ...Array.from("!#$%&'*+-/=?^_`{|}~", value => ({ kind: 'char', value }))]);
  const localPart = repeat(atext, 1);
  const domainStart = charClass(alphaNumeric);
  const domainInner = charClass([...alphaNumeric, { kind: 'char', value: '-' }]);
  const label = seq(domainStart, optional(seq(repeat(domainInner, 0, 61), domainStart)));
  return seq(localPart, repeat0(seq(literal('.'), localPart)), literal('@'), label, repeat0(seq(literal('.'), label)));
}

const formatExpressions = new Map([
  ['uuid', uuidExpression], ['date', dateExpression], ['date-time', dateTimeExpression], ['email', emailExpression],
]);

/** Shared decoder productions for the JavaScript and Rust importers. */
export function buildJsonDecoderBase() {
  const builder = new Expression('value');
  installJsonRules(builder);
  for (const [format, expression] of formatExpressions) {
    builder.rule(formatRule(format), seq(literal('"'), expression(), literal('"')));
  }
  return builder.build();
}

function finiteConstants(rules) {
  if (rules === false) return [];
  if (Object.hasOwn(rules, 'const')) return [rules.const];
  return rules.enum;
}

function formatRule(format) {
  return `json_format_${format.replaceAll('-', '_')}`;
}

function safeStringPattern(expression) {
  const escaped = /["\\\u0000-\u001f\ud800-\udfff]/u;
  const safeRange = ({ start, end }) => {
    const first = start.codePointAt(0);
    const last = end.codePointAt(0);
    return ![[0, 31], [34, 34], [92, 92], [0xd800, 0xdfff]].some(([low, high]) => first <= high && last >= low);
  };
  if (expression.kind === 'literal') return !escaped.test(expression.value);
  if (expression.kind === 'charRange') return safeRange(expression);
  if (expression.kind === 'charClass' && !expression.negated) {
    return expression.items?.every(item => {
      if (item.kind === 'char') return !escaped.test(item.value);
      return item.kind === 'range' && safeRange(item);
    });
  }
  if (expression.items) return expression.items.every(safeStringPattern);
  return expression.item ? safeStringPattern(expression.item) : expression.kind === 'empty';
}

function lowerStringPattern(pattern) {
  if (!pattern.startsWith('^') || !pattern.endsWith('$')) return null;
  const source = pattern.slice(1, -1);
  const atom = /(\[(?:\\.|[^\\\]])*\]|[^()[\]{}*+?.|^$\\])(?:(\{(\d+)(?:,(\d*))?\})|([?+*]))?/uy;
  const terms = [];
  for (let offset = 0; offset < source.length;) {
    atom.lastIndex = offset;
    const match = atom.exec(source);
    if (!match) return null;
    const [, spelling, counted, minimum, maximum, suffix] = match;
    const term = emitterExpression(lowerLarkRegex(spelling + (suffix ?? ''), '', offset));
    if (!safeStringPattern(term)) return null;
    const min = Number(minimum);
    const max = maximum === undefined ? min : maximum === '' ? null : Number(maximum);
    if (counted && (min > 1024 || (max !== null && (max > 1024 || min > max)))) return null;
    terms.push(counted ? repeat(term, min, max) : term);
    offset = atom.lastIndex;
  }
  return seq(...terms);
}

/** Import a draft 2020-12 schema into a constrained-decoding grammar. */
export function importJsonSchema(schema) {
  const analysis = analyzeJsonSchema(schema);
  const builder = new Expression('document').source('json-schema');
  decoderBase ??= parseGrammarLinks(readFileSync(baseFile, 'utf8'));
  for (const rule of decoderBase.rules.values()) builder.rule(rule.name, rule.expression);
  const compiled = new Map();
  const diagnostics = [...analysis.issues];
  const report = (pointer, keyword, message) => diagnostics.push({
    keyword, pointer: pointerChild(pointer, keyword), message,
  });
  let next = 1;
  function named(rules, pointer) {
    if (compiled.has(rules)) return ref(compiled.get(rules));
    const name = `schema_${next++}`;
    compiled.set(rules, name);
    const expression = lower(rules, pointer);
    if (!expression) {
      compiled.delete(rules);
      return null;
    }
    builder.rule(name, expression);
    return ref(name);
  }
  function objectExpression(rules, pointer) {
    const fields = rules.properties ?? {};
    const required = new Set(rules.required ?? []);
    for (const key of required) if (!Object.hasOwn(fields, key) && rules.additionalProperties === false) return null;
    const keys = [...new Set([...Object.keys(fields), ...required])];
    if (keys.length === 0) {
      if (rules.additionalProperties === false) return collection('{', '}', empty());
      if (!rules.additionalProperties || rules.additionalProperties === true) return ref('object');
      const other = named(rules.additionalProperties, pointerChild(pointer, 'additionalProperties'));
      return other ? collection('{', '}', unboundedList(seq(ref('string'), space, literal(':'), space, other))) : collection('{', '}', empty());
    }
    if (rules.additionalProperties !== false) report(pointer, 'additionalProperties', 'The decoder generates declared fields and required fields with schemas. Validation accepts permitted additional names.');
    const members = [];
    for (const key of keys) {
      const child = Object.hasOwn(fields, key) ? fields[key] : rules.additionalProperties ?? true;
      const expression = named(child, pointerChild(pointerChild(pointer, 'properties'), key));
      if (expression) members.push({ key, expression: property(key, expression) });
      else if (required.has(key)) return null;
    }
    const group = next++;
    const states = new Map();
    if (members.length > 12) {
      report(pointer, 'properties', 'The decoder emits large objects in schema field order. Validation accepts all field orders.');
      function ordered(index, started) {
        if (index === members.length) return empty();
        const key = `${index}_${Number(started)}`;
        if (states.has(key)) return ref(states.get(key));
        const name = `object_${group}_${key}`;
        states.set(key, name);
        const member = members[index];
        const entry = seq(started ? separated(member.expression) : member.expression, ordered(index + 1, true));
        builder.rule(name, required.has(member.key) ? entry : choice(entry, ordered(index + 1, started)));
        return ref(name);
      }
      return collection('{', '}', ordered(0, false));
    }
    let requiredMask = 0;
    members.forEach((member, index) => { if (required.has(member.key)) requiredMask |= 1 << index; });
    function unordered(seen) {
      if (states.has(seen)) return ref(states.get(seen));
      const name = `object_${group}_${seen}`;
      states.set(seen, name);
      const alternatives = (seen & requiredMask) === requiredMask ? [empty()] : [];
      members.forEach((member, index) => {
        const bit = 1 << index;
        if ((seen & bit) === 0) alternatives.push(seq(seen === 0 ? member.expression : separated(member.expression), unordered(seen | bit)));
      });
      builder.rule(name, choice(...alternatives));
      return ref(name);
    }
    return collection('{', '}', unordered(0));
  }
  function arrayExpression(rules, pointer) {
    const prefix = (rules.prefixItems ?? []).map((item, index) => named(item, pointerChild(pointerChild(pointer, 'prefixItems'), index)));
    const tail = named(rules.items ?? true, pointerChild(pointer, 'items'));
    const min = rules.minItems ?? 0;
    const max = rules.maxItems ?? null;
    if (max !== null && max < min) return null;
    const alternatives = [];
    const members = [];
    for (let length = 0; length <= prefix.length && (max === null || length <= max); length += 1) {
      if (length < prefix.length && length >= min) alternatives.push(seq(...members));
      if (length === prefix.length && tail) {
        alternatives.push(seq(...members, boundedTail(tail, Math.max(0, min - length), max === null ? null : max - length, length !== 0)));
      } else if (length === prefix.length && length >= min) alternatives.push(seq(...members));
      if (length === prefix.length || !prefix[length]) break;
      members.push(length === 0 ? prefix[length] : separated(prefix[length]));
    }
    return alternatives.length ? collection('[', ']', choice(...alternatives)) : null;
  }
  function stringExpression(rules, pointer) {
    if (formatExpressions.has(rules.format) && rules.pattern === undefined) {
      report(pointer, 'format', 'The decoder generates direct character spellings in the supported format. Validation also checks escaped strings.');
      return ref(formatRule(rules.format));
    }
    if (rules.format !== undefined) report(pointer, 'format', 'The format constraint combined with a pattern is checked during document validation.');
    if (rules.pattern !== undefined) {
      try {
        const expression = lowerStringPattern(rules.pattern);
        if (expression) {
          report(pointer, 'pattern', 'The decoder generates direct character spellings. Validation also accepts equivalent JSON escapes.');
          return seq(literal('"'), expression, literal('"'));
        }
      } catch {
        // The constrained grammar must stay in the emitters' shared subset.
      }
      report(pointer, 'pattern', 'This pattern is checked during document validation.');
    }
    return ref('string');
  }
  function numberExpression(type, rules, pointer) {
    const keywords = ['minimum', 'maximum', 'exclusiveMinimum', 'exclusiveMaximum'].filter(key => rules[key] !== undefined);
    if (type === 'integer') report(pointer, 'type', 'The decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.');
    if (keywords.length === 0) return ref(type);
    const first = Math.max(Math.ceil(rules.minimum ?? -Infinity), Math.floor(rules.exclusiveMinimum ?? -Infinity) + 1);
    const last = Math.min(Math.floor(rules.maximum ?? Infinity), Math.ceil(rules.exclusiveMaximum ?? Infinity) - 1);
    if (type === 'integer' && (first === -Infinity || Number.isSafeInteger(first)) && (last === Infinity || Number.isSafeInteger(last))) {
      return first <= last ? signedIntegerRange(Number.isFinite(first) ? first : null, Number.isFinite(last) ? last : null) : null;
    }
    for (const keyword of keywords) report(pointer, keyword, 'This bound is checked during document validation.');
    return ref(type);
  }
  function lower(rules, pointer) {
    if (rules === true) return value;
    if (rules === false) return null;
    const constants = finiteConstants(rules)
      ?? ((rules.anyOf ?? rules.oneOf)?.every(child => finiteConstants(child))
        ? (rules.anyOf ?? rules.oneOf).flatMap(finiteConstants) : undefined);
    if (constants) {
      const entries = constants.filter(item => validateJsonDocument(readJsonDocument(JSON.stringify(item)), rules, analysis).errors.length === 0);
      return entries.length ? choice(...entries.map(constantExpression)) : null;
    }
    if (rules.$ref !== undefined) {
      const target = analysis.target(rules.$ref);
      if (typeof target === 'boolean' || (target && typeof target === 'object' && !Array.isArray(target))) {
        if (Object.keys(rules).some(key => !referenceKeys.has(key))) report(pointer, '$ref', 'Sibling reference assertions are checked during document validation.');
        return named(target, rules.$ref.slice(1));
      }
    }
    const alternatives = rules.anyOf ?? rules.oneOf;
    if (alternatives) {
      const keyword = rules.anyOf ? 'anyOf' : 'oneOf';
      const hasSiblingAssertions = Object.keys(rules).some(key => key !== keyword && !referenceKeys.has(key));
      if (hasSiblingAssertions) report(pointer, keyword, 'Sibling assertions on alternatives are checked during document validation.');
      if (keyword === 'oneOf') report(pointer, keyword, 'Exclusive branch selection is checked during document validation.');
      const entries = alternatives.map((child, index) => named(child, pointerChild(pointerChild(pointer, keyword), index))).filter(Boolean);
      return entries.length > 0 ? choice(...entries) : null;
    }
    const types = rules.type === undefined ? jsonTypes : Array.isArray(rules.type) ? rules.type : [rules.type];
    const expressions = types.map(type => {
      if (type === 'object') return objectExpression(rules, pointer);
      if (type === 'array') return arrayExpression(rules, pointer);
      if (type === 'string') return stringExpression(rules, pointer);
      if (type === 'integer' || type === 'number') return numberExpression(type, rules, pointer);
      return ref(type);
    }).filter(Boolean);
    return expressions.length ? choice(...expressions) : null;
  }
  const start = named(schema, '');
  if (!start) throw new RangeError('The JSON Schema has no generatable document');
  builder.rule('document', seq(space, start, space));
  const grammar = builder.build();
  grammar.schemaDiagnostics = diagnostics;
  importedSchemas.set(grammar, { schema: structuredClone(schema), grammar: JSON.stringify(grammar.normalized()) });
  return grammar;
}
