import { readFileSync } from 'node:fs';
import { GrammarBuilder as Expression } from '../grammar.js';
import { collectReferences } from '../grammar-emitters/structural.js';
import { parseGrammarLinks } from '../grammar-links.js';
import { jsonDocumentGrammar, pointerChild, readJsonDocument } from '../json-document.js';
import { analyzeJsonSchema, validateJsonDocument } from '../json-schema-validation.js';
import { anyJsonPatternCharacter, lowerJsonStringPattern } from './json-schema-patterns.js';
import { additionalPropertyName, jsonStringCharacter } from './json-schema-property-names.js';
import { intersectJsonStringExpressions, selectExclusiveJsonStringExpressions } from './json-schema-string-intersection.js';
import { stringSetsAreDisjoint } from './json-schema-tag-policy.js';
import { exclusiveIntegerRange, integerBounds, numericLimitKeywords, signedDecimalRange, signedIntegerRange } from './json-schema-numeric-ranges.js';

const { charClass, charRange, seq, choice, literal, ref, repeat, repeat0, optional, empty } = Expression;
const space = ref('json_space');
const value = ref('value');
const referenceKeys = new Set(['$ref', '$defs', '$id', '$schema', '$comment', 'title', 'description', 'default', 'examples', 'deprecated', 'readOnly', 'writeOnly']);
const integerType = 1 << 3;
const fractionalType = 1 << 4;
const numberType = integerType | fractionalType;
const jsonTypes = new Map([
  ['object', 1], ['array', 1 << 1], ['string', 1 << 2], ['number', numberType],
  ['integer', integerType], ['boolean', 1 << 5], ['null', 1 << 6],
]);
const allJsonTypes = (1 << 7) - 1;
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

function member(key, expression) {
  return seq(key, space, literal(':'), space, expression);
}

function property(key, expression) {
  return member(quoted(key), expression);
}

function boundedTail(expression, min, max, hasPrefix) {
  if (max === 0) return empty();
  if (hasPrefix) return repeat(separated(expression), min, max);
  const body = seq(expression, repeat(separated(expression), Math.max(0, min - 1), max === null ? null : max - 1));
  return min === 0 ? optional(body) : body;
}

function uuidExpression() {
  const hex = charClass([{ kind: 'range', start: '0', end: '9' }, { kind: 'range', start: 'a', end: 'f' }, { kind: 'range', start: 'A', end: 'F' }]);
  return seq(repeat(hex, 8, 8), ...[4, 4, 4, 12].map(length => seq(literal('-'), repeat(hex, length, length))));
}

function dateCharacters(text) {
  return seq(...Array.from(text, digit => characters(digit)));
}

function decimalWidth(min, max, width = 2) {
  const alternatives = [];
  for (let prefix = Math.floor(min / 10); prefix <= Math.floor(max / 10); prefix += 1) {
    const first = Math.max(0, min - prefix * 10);
    const last = Math.min(9, max - prefix * 10);
    alternatives.push(seq(dateCharacters(String(prefix).padStart(width - 1, '0')), first === last ? characters(String(first)) : charRange(String(first), String(last))));
  }
  return choice(...alternatives);
}

function dateExpression() {
  const digit = charRange('0', '9');
  const normal = seq(repeat(digit, 4, 4), literal('-'), choice(
    ...['01', '03', '05', '07', '08', '10', '12'].map(month => seq(dateCharacters(`${month}-`), decimalWidth(1, 31))),
    ...['04', '06', '09', '11'].map(month => seq(dateCharacters(`${month}-`), decimalWidth(1, 30))),
    seq(dateCharacters('02-'), decimalWidth(1, 28)),
  ));
  const multiples = Array.from({ length: 25 }, (_, index) => dateCharacters(String(index * 4).padStart(2, '0')));
  const leap = seq(choice(seq(repeat(digit, 2, 2), choice(...multiples.slice(1))), seq(choice(...multiples), dateCharacters('00'))), dateCharacters('-02-29'));
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
  const atom = repeat(atext, 1);
  const dotAtom = seq(atom, repeat0(seq(literal('.'), atom)));
  const quotedText = choice(charRange(' ', '!'), charRange('#', '['), charRange(']', '~'),
    seq(literal('\\'), charRange(' ', '~')));
  const quotedLocal = seq(literal('"'), repeat0(quotedText), literal('"'));
  const domainStart = charClass(alphaNumeric);
  const domainInner = charClass([...alphaNumeric, { kind: 'char', value: '-' }]);
  const label = seq(domainStart, optional(seq(repeat(domainInner, 0, 61), domainStart)));
  return seq(choice(dotAtom, quotedLocal), literal('@'), label, repeat0(seq(literal('.'), label)));
}

const formatExpressions = new Map([
  ['uuid', uuidExpression], ['date', dateExpression], ['date-time', dateTimeExpression], ['email', emailExpression],
]);

/** Shared decoder productions for the JavaScript and Rust importers. */
export function buildJsonDecoderBase() {
  const builder = new Expression('value');
  installJsonRules(builder);
  const characterRules = new Map();
  function encodedCharacter(ranges) {
    const key = JSON.stringify(ranges);
    if (!characterRules.has(key)) {
      const name = `json_format_character_${characterRules.size + 1}`;
      builder.rule(name, jsonStringCharacter(ranges));
      characterRules.set(key, ref(name));
    }
    return characterRules.get(key);
  }
  const scalar = character => character.codePointAt(0);
  function encoded(expression) {
    switch (expression.kind) {
      case 'literal':
        return seq(...Array.from(expression.value, character => encodedCharacter([[scalar(character), scalar(character)]])));
      case 'charRange':
        return encodedCharacter([[scalar(expression.start), scalar(expression.end)]]);
      case 'charClass':
        return encodedCharacter(expression.items.map(item => item.kind === 'char'
          ? [scalar(item.value), scalar(item.value)]
          : [scalar(item.start), scalar(item.end)]));
      case 'seq':
      case 'choice':
        return { ...expression, items: expression.items.map(encoded) };
      case 'repeat':
      case 'repeat0':
      case 'repeat1':
      case 'optional':
        return { ...expression, item: encoded(expression.item) };
      default:
        throw new Error(`Unsupported JSON Schema format expression ${expression.kind}`);
    }
  }
  for (const [format, expression] of formatExpressions) {
    builder.rule(formatRule(format), seq(literal('"'), encoded(expression()), literal('"')));
  }
  return builder.build();
}

function finiteConstants(rules) {
  if (rules === false) return [];
  if (rules === null || typeof rules !== 'object') return undefined;
  if (Object.hasOwn(rules, 'const')) return [rules.const];
  return Array.isArray(rules.enum) ? rules.enum : undefined;
}

function finiteSchemaCandidates(schema, resolve) {
  const active = new Set();
  let work = 16_384;
  function collect(rules, depth) {
    work -= 1;
    if (work < 0 || depth > 128 || active.has(rules)) return undefined;
    const values = finiteConstants(rules);
    if (values) return { values, keyword: Object.hasOwn(rules, 'const') ? 'const' : 'enum' };
    if (!rules || typeof rules !== 'object') return undefined;
    active.add(rules);
    try {
      if (typeof rules.$ref === 'string') {
        const target = collect(resolve(rules.$ref), depth + 1);
        if (target) return { values: target.values, keyword: '$ref' };
      }
      for (const keyword of ['anyOf', 'oneOf']) {
        if (!Array.isArray(rules[keyword])) continue;
        const values = [];
        let complete = true;
        for (const child of rules[keyword]) {
          const candidate = collect(child, depth + 1);
          if (!candidate || candidate.values.length > work) {
            complete = false;
            break;
          }
          work -= candidate.values.length;
          for (const value of candidate.values) values.push(value);
        }
        if (complete) return { values, keyword };
      }
    } finally {
      active.delete(rules);
    }
    return undefined;
  }
  return collect(schema, 0);
}

function declaredTypeMembership(rules) {
  if (rules?.type === undefined) return allJsonTypes;
  const names = Array.isArray(rules.type) ? rules.type : [rules.type];
  return names.reduce((membership, name) => membership | (jsonTypes.get(name) ?? 0), 0);
}

function valueTypeMembership(value) {
  if (typeof value === 'number') return Number.isInteger(value) ? integerType : fractionalType;
  const name = value === null ? 'null' : Array.isArray(value) ? 'array' : typeof value;
  return jsonTypes.get(name) ?? 0;
}

function possibleTypeMembership(rules, allowedTypes) {
  const declared = allowedTypes & declaredTypeMembership(rules);
  const constants = finiteConstants(rules);
  return constants ? constants.reduce((types, value) => types | valueTypeMembership(value), 0) & declared : declared;
}

function combineNumericConstraints(layers, membership, combination) {
  if (membership !== numberType && membership !== integerType) return undefined;
  const limits = {};
  for (const [index, layer] of layers.entries()) {
    for (const [key, value] of Object.entries(layer)) {
      if (key === 'type' || referenceKeys.has(key) || (index === 0 && key === combination)) continue;
      if (!numericLimitKeywords.includes(key) || !Number.isFinite(value)) return undefined;
      const tighten = key === 'minimum' || key === 'exclusiveMinimum' ? Math.max : Math.min;
      limits[key] = tighten(limits[key] ?? value, value);
    }
  }
  return { type: membership === integerType ? 'integer' : 'number', limits };
}

function branchesUseDisjointTypes(branches, allowedTypes) {
  if (branches.length < 2) return true;
  let seen = 0;
  for (const branch of branches) {
    if (branch === null || !['object', 'boolean'].includes(typeof branch) || Array.isArray(branch)) return false;
    const membership = possibleTypeMembership(branch, allowedTypes);
    if (seen & membership) return false;
    seen |= membership;
  }
  return true;
}

function requiredStringTags(schema) {
  const tags = new Map();
  const declared = Array.isArray(schema?.type) ? schema.type : [schema?.type];
  if (declared.length !== 1 || declared[0] !== 'object' || !Array.isArray(schema.required)
      || ['$ref', 'anyOf', 'oneOf'].some(key => Object.hasOwn(schema, key))) return tags;
  for (const key of schema.required) {
    if (!schema.properties || !Object.hasOwn(schema.properties, key)) continue;
    const values = finiteConstants(schema.properties[key]);
    if (Array.isArray(values) && values.every(value => typeof value === 'string' && value.isWellFormed())) {
      tags.set(key, values);
    }
  }
  return tags;
}

function branchesUseDisjointRequiredTags(branches, allowedTypes, referenceLayers) {
  const tags = branches.map(branch => requiredStringTags(referenceLayers(branch)?.at(-1)));
  return branches.every((branch, index) => branches.slice(0, index).every((previous, other) => (
    branchesUseDisjointTypes([branch, previous], allowedTypes) || [...tags[index]].some(([key, values]) => (
      tags[other].has(key) && stringSetsAreDisjoint(values, tags[other].get(key))
    ))
  )));
}

function formatRule(format) {
  return `json_format_${format.replaceAll('-', '_')}`;
}

/** Import a draft 2020-12 schema into a constrained-decoding grammar. */
export function importJsonSchema(schema) {
  const analysis = analyzeJsonSchema(schema);
  const builder = new Expression('start').source('json-schema');
  decoderBase ??= parseGrammarLinks(readFileSync(baseFile, 'utf8'));
  for (const rule of decoderBase.rules.values()) builder.rule(rule.name, rule.expression);
  builder.rule('json_pattern_character', anyJsonPatternCharacter());
  const compiled = new Map();
  const diagnostics = [...analysis.issues];
  const report = (pointer, keyword, message) => diagnostics.push({
    keyword, pointer: pointerChild(pointer, keyword), message,
  });
  let next = 1;
  const patternRules = new Map();
  function patternRule(expression) {
    const key = JSON.stringify(expression);
    if (!patternRules.has(key)) {
      const name = `json_pattern_part_${patternRules.size + 1}`;
      builder.rule(name, expression);
      patternRules.set(key, ref(name));
    }
    return patternRules.get(key);
  }
  function named(rules, pointer, allowedTypes = allJsonTypes) {
    const membership = allowedTypes & declaredTypeMembership(rules);
    if (!membership) return null;
    const variants = compiled.get(rules) ?? new Map();
    if (variants.has(membership)) return ref(variants.get(membership));
    const name = `schema_${next++}`;
    compiled.set(rules, variants);
    variants.set(membership, name);
    const expression = lower(rules, pointer, membership);
    if (!expression) {
      variants.delete(membership);
      return null;
    }
    builder.rule(name, expression);
    return ref(name);
  }
  function constantExpression(instance, pointer, keyword) {
    if (typeof instance === 'string') return quoted(instance);
    if (Array.isArray(instance)) return collection('[', ']', seq(...instance.map((item, index) => {
      const expression = constantExpression(item, pointer, keyword);
      return index ? separated(expression) : expression;
    })));
    if (instance !== null && typeof instance === 'object') {
      const members = Object.entries(instance).map(([key, item]) => ({
        required: true,
        expression: property(key, constantExpression(item, pointer, keyword)),
      }));
      return objectMembers(members, undefined, pointer, keyword);
    }
    return literal(JSON.stringify(instance));
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
    let additional;
    if (rules.additionalProperties !== false) {
      const child = named(rules.additionalProperties ?? true, pointerChild(pointer, 'additionalProperties'));
      const keyUnits = keys.reduce((size, key) => size + key.length, 0);
      if (child && (keyUnits > 1024 || keys.some(key => key.length > 128 || !key.isWellFormed()))) {
        report(pointer, 'additionalProperties', 'The decoder bounds declared key spellings for additional properties. Validation checks all permitted names.');
      } else if (child) {
        const name = `additional_property_${next++}`;
        builder.rule(name, member(additionalPropertyName(keys), child));
        additional = ref(name);
      }
    }
    const members = [];
    for (const key of keys) {
      const child = Object.hasOwn(fields, key) ? fields[key] : rules.additionalProperties ?? true;
      const expression = named(child, pointerChild(pointerChild(pointer, 'properties'), key));
      if (expression) members.push({ required: required.has(key), expression: property(key, expression) });
      else if (required.has(key)) return null;
    }
    return objectMembers(members, additional, pointer, 'properties');
  }
  function objectMembers(members, additional, pointer, keyword) {
    const group = next++;
    const states = new Map();
    if (members.length > 12) {
      report(pointer, keyword, 'The decoder emits large objects in schema field order. Validation accepts all field orders.');
      function ordered(index, started) {
        const key = `${index}_${Number(started)}`;
        if (states.has(key)) return ref(states.get(key));
        const name = `object_${group}_${key}`;
        states.set(key, name);
        const field = members[index];
        const alternatives = [];
        if (additional) alternatives.push(seq(started ? separated(additional) : additional, ordered(index, true)));
        if (field) {
          alternatives.push(seq(started ? separated(field.expression) : field.expression, ordered(index + 1, true)));
          if (!field.required) alternatives.push(ordered(index + 1, started));
        } else alternatives.push(empty());
        builder.rule(name, choice(...alternatives));
        return ref(name);
      }
      return collection('{', '}', ordered(0, false));
    }
    let requiredMask = 0;
    members.forEach((member, index) => { if (member.required) requiredMask |= 1 << index; });
    function unordered(seen, started) {
      const state = `${seen}_${Number(started)}`;
      if (states.has(state)) return ref(states.get(state));
      const name = `object_${group}_${state}`;
      states.set(state, name);
      const alternatives = (seen & requiredMask) === requiredMask ? [empty()] : [];
      if (additional) alternatives.push(seq(started ? separated(additional) : additional, unordered(seen, true)));
      members.forEach((member, index) => {
        const bit = 1 << index;
        if ((seen & bit) === 0) alternatives.push(seq(started ? separated(member.expression) : member.expression, unordered(seen | bit, true)));
      });
      builder.rule(name, choice(...alternatives));
      return ref(name);
    }
    return collection('{', '}', unordered(0, false));
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
    const format = formatExpressions.has(rules.format) ? ref(formatRule(rules.format)) : null;
    if (rules.pattern !== undefined) {
      const result = lowerJsonStringPattern(rules.pattern, patternRule);
      if (result) {
        if (!result.expression) return null;
        const pattern = seq(literal('"'), result.expression, literal('"'));
        if (!format) return pattern;
        const intersection = intersectJsonStringExpressions(format, pattern, name => builder.rules.get(name)?.expression,
          `json_string_intersection_${next++}`);
        if (intersection === null) return null;
        if (intersection) {
          for (const [name, expression] of intersection.rules) builder.rule(name, expression);
          return intersection.expression;
        }
        report(pointer, 'pattern', 'The format and pattern intersection exceeds the decoder work bound. The pattern is checked during document validation.');
      } else report(pointer, 'pattern', 'This pattern is checked during document validation.');
    }
    return format ?? ref('string');
  }
  function regularStringConstraints(rules, combination) {
    if (Object.keys(rules).some(key => key !== combination && (key === '$ref'
      || (!referenceKeys.has(key) && !['type', 'const', 'enum', 'pattern', 'format'].includes(key))))) return undefined;
    const expressions = [];
    if (rules.format !== undefined) {
      if (!formatExpressions.has(rules.format)) return undefined;
      expressions.push(ref(formatRule(rules.format)));
    }
    if (rules.pattern !== undefined) {
      const result = lowerJsonStringPattern(rules.pattern, patternRule);
      if (!result) return undefined;
      if (!result.expression) return null;
      expressions.push(seq(literal('"'), result.expression, literal('"')));
    }
    const constants = finiteConstants(rules);
    if (constants) {
      const strings = constants.filter(item => typeof item === 'string');
      if (!strings.length) return null;
      if (strings.some(item => !item.isWellFormed())) return undefined;
      expressions.push(choice(...strings.map(quoted)));
    }
    return expressions;
  }
  function stringBranchProduct(branches) {
    const product = selectExclusiveJsonStringExpressions(branches, name => builder.rules.get(name)?.expression,
      `json_string_exclusive_${next++}`);
    if (!product) return product;
    for (const [name, expression] of product.rules) builder.rule(name, expression);
    return product.expression;
  }
  function referenceLayers(schema) {
    const layers = [];
    const visited = new Set();
    let rules = schema;
    while (typeof rules === 'boolean' || (rules && typeof rules === 'object' && !Array.isArray(rules))) {
      layers.push(rules);
      const name = rules.$ref;
      if (name === undefined) return layers;
      if (typeof name !== 'string' || visited.has(name) || layers.length > 128) return undefined;
      visited.add(name);
      rules = analysis.target(name);
    }
    return undefined;
  }
  function regularStringChain(layers) {
    const conjunction = [];
    for (const layer of layers) {
      const expressions = regularStringConstraints(layer, '$ref');
      if (!expressions) return expressions;
      conjunction.push(...expressions);
    }
    return conjunction;
  }
  function referencedExpression(rules, allowedTypes, pointer) {
    const layers = referenceLayers(rules);
    if (!layers) return undefined;
    const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
    if (membership === 0) return null;
    const constraints = combineNumericConstraints(layers, membership);
    if (constraints) return numberExpression(constraints.type, constraints.limits, pointer);
    if (membership !== jsonTypes.get('string')) return undefined;
    const conjunction = regularStringChain(layers);
    if (!conjunction) return conjunction;
    return conjunction.length ? stringBranchProduct([conjunction]) : ref('string');
  }
  function alternativeStringExpression(rules, alternatives, allowedTypes, keyword) {
    const string = jsonTypes.get('string');
    const guard = regularStringConstraints(rules, keyword);
    if (!guard) return guard;
    const branches = [];
    for (const branch of alternatives) {
      if (possibleTypeMembership(branch, allowedTypes) === 0) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
      if (membership === 0) continue;
      if (membership !== string) return undefined;
      const constraints = regularStringChain(layers);
      if (constraints === undefined) return undefined;
      if (constraints) {
        const conjunction = [...guard, ...constraints];
        branches.push(conjunction.length ? conjunction : [ref('string')]);
      }
    }
    if (keyword === 'oneOf') return stringBranchProduct(branches);
    // Distribute the common assertions to bound each intersection independently.
    const entries = [];
    for (const branch of branches) {
      const entry = stringBranchProduct([branch]);
      if (entry === undefined) return undefined;
      if (entry) entries.push(entry);
    }
    return entries.length ? choice(...entries) : null;
  }
  function alternativeNumericExpression(rules, alternatives, allowedTypes, keyword, pointer) {
    const branches = [];
    for (const [index, branch] of alternatives.entries()) {
      if (possibleTypeMembership(branch, allowedTypes) === 0) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
      if (membership === 0) continue;
      const constraints = combineNumericConstraints([rules, ...layers], membership, keyword);
      if (!constraints) return undefined;
      branches.push({ index, ...constraints });
    }
    if (keyword === 'oneOf') {
      if (branches.some(branch => branch.type !== 'integer')) return undefined;
      const integer = exclusiveIntegerRange(branches.map(branch => integerBounds(branch.limits)));
      if (integer) report(pointer, keyword, 'The exclusive integer decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.');
      return integer;
    }
    const location = pointerChild(pointer, keyword);
    const entries = branches.map(({ index, type, limits }) => numberExpression(type, limits, pointerChild(location, index))).filter(Boolean);
    return entries.length ? choice(...entries) : null;
  }
  function numberExpression(type, rules, pointer) {
    const keywords = numericLimitKeywords.filter(key => rules[key] !== undefined);
    if (type === 'integer') report(pointer, 'type', 'The decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.');
    if (keywords.length === 0) return ref(type);
    if (type === 'number') {
      const decimal = signedDecimalRange(rules);
      if (decimal !== undefined) {
        if (decimal) report(pointer, 'type', 'The bounded decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.');
        return decimal;
      }
    }
    const [first, last] = integerBounds(rules);
    if (type === 'integer' && (first === -Infinity || Number.isSafeInteger(first)) && (last === Infinity || Number.isSafeInteger(last))) {
      return first <= last ? signedIntegerRange(Number.isFinite(first) ? first : null, Number.isFinite(last) ? last : null) : null;
    }
    for (const keyword of keywords) report(pointer, keyword, 'This bound is checked during document validation.');
    return ref(type);
  }
  function lower(rules, pointer, allowedTypes) {
    if (rules === true && allowedTypes === allJsonTypes) return value;
    if (rules === false) return null;
    const alternatives = [rules.anyOf, rules.oneOf].find(Array.isArray);
    const referenceHasSiblings = rules.$ref !== undefined && Object.keys(rules).some(key => !referenceKeys.has(key));
    const constants = finiteConstants(rules)
      ?? (alternatives?.every(child => finiteConstants(child)) ? alternatives.flatMap(finiteConstants) : undefined);
    const candidates = constants
      ? { values: constants, keyword: ['const', 'enum', 'anyOf', 'oneOf'].find(key => Object.hasOwn(rules, key)) }
      : (referenceHasSiblings || alternatives) && finiteSchemaCandidates(rules, analysis.target);
    if (candidates) {
      const { values, keyword } = candidates;
      const entries = values.filter(item => (allowedTypes & valueTypeMembership(item))
        && validateJsonDocument(readJsonDocument(JSON.stringify(item)), rules, analysis).errors.length === 0);
      return entries.length ? choice(...entries.map(item => constantExpression(item, pointer, keyword))) : null;
    }
    if (rules.$ref !== undefined) {
      const target = analysis.target(rules.$ref);
      if (typeof target === 'boolean' || (target && typeof target === 'object' && !Array.isArray(target))) {
        if (Object.keys(rules).some(key => key !== 'type' && !referenceKeys.has(key))) {
          const conjunction = referencedExpression(rules, allowedTypes, pointer);
          if (conjunction !== undefined) return conjunction;
          report(pointer, '$ref', 'Sibling reference assertions are checked during document validation.');
        }
        return named(target, rules.$ref.slice(1), allowedTypes);
      }
    }
    if (alternatives) {
      const keyword = alternatives === rules.anyOf ? 'anyOf' : 'oneOf';
      const hasSiblingAssertions = Object.keys(rules).some(key => key !== 'type' && key !== keyword && !referenceKeys.has(key));
      const overlaps = keyword === 'oneOf' && !branchesUseDisjointTypes(alternatives, allowedTypes)
        && !branchesUseDisjointRequiredTags(alternatives, allowedTypes, referenceLayers);
      if (overlaps || hasSiblingAssertions) {
        const combined = alternativeStringExpression(rules, alternatives, allowedTypes, keyword);
        if (combined !== undefined) return combined;
        const numeric = alternativeNumericExpression(rules, alternatives, allowedTypes, keyword, pointer);
        if (numeric !== undefined) return numeric;
      }
      if (hasSiblingAssertions) report(pointer, keyword, 'Sibling assertions on alternatives are checked during document validation.');
      if (overlaps) {
        report(pointer, keyword, 'Exclusive branch selection is checked during document validation.');
      }
      const entries = alternatives.map((child, index) => named(child, pointerChild(pointerChild(pointer, keyword), index), allowedTypes)).filter(Boolean);
      return entries.length > 0 ? choice(...entries) : null;
    }
    const expressions = [];
    for (const [type, membership] of jsonTypes) {
      if ((allowedTypes & membership) !== membership || (type === 'integer' && (allowedTypes & fractionalType))) continue;
      const expression = lowerType(type);
      if (expression) expressions.push(expression);
    }
    return expressions.length ? choice(...expressions) : null;

    function lowerType(type) {
      if (type === 'object') return objectExpression(rules, pointer);
      if (type === 'array') return arrayExpression(rules, pointer);
      if (type === 'string') return stringExpression(rules, pointer);
      if (type === 'integer' || type === 'number') return numberExpression(type, rules, pointer);
      return ref(type);
    }
  }
  const start = named(schema, '');
  if (!start) throw new RangeError('The JSON Schema has no generatable document');
  builder.rule('start', seq(space, start, space));
  const grammar = builder.build();
  const reachable = new Set();
  const pending = ['start'];
  while (pending.length) {
    const name = pending.pop();
    if (reachable.has(name)) continue;
    reachable.add(name);
    const rule = grammar.rule(name);
    if (rule) pending.push(...collectReferences(rule.expression));
  }
  for (const name of grammar.rules.keys()) {
    if (!reachable.has(name)) grammar.rules.delete(name);
  }
  grammar.schemaDiagnostics = diagnostics;
  importedSchemas.set(grammar, { schema: structuredClone(schema), grammar: JSON.stringify(grammar.normalized()) });
  return grammar;
}
