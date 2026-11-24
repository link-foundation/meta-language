import { readFileSync } from 'node:fs';
import { isDeepStrictEqual } from 'node:util';
import { GrammarBuilder as Expression } from '../grammar.js';
import { collectReferences } from '../grammar-emitters/structural.js';
import { parseGrammarLinks } from '../grammar-links.js';
import { jsonDocumentGrammar, pointerChild, readJsonDocument } from '../json-document.js';
import { jsonDocumentPolicy } from '../json-document-policy.js';
import { analyzeJsonSchema, validateJsonDocument } from '../json-schema-validation.js';
import { anyJsonPatternCharacter, lowerJsonStringPattern } from './json-schema-patterns.js';
import { additionalPropertyName, jsonStringCharacter } from './json-schema-property-names.js';
import { intersectJsonStringExpressions, selectExclusiveJsonStringExpressions } from './json-schema-string-intersection.js';
import { stringSetsAreDisjoint } from './json-schema-tag-policy.js';
import { exclusiveDecimalRange, exclusiveIntegerRange, integerBounds, numericConstant, numericLimitKeywords, signedDecimalRange, signedIntegerRange } from './json-schema-numeric-ranges.js';

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
const assertionTypes = new Map([
  ['object', jsonDocumentPolicy.objectAssertion], ['array', jsonDocumentPolicy.arrayAssertion],
  ['string', jsonDocumentPolicy.stringAssertion], ['number', jsonDocumentPolicy.limit],
].flatMap(([type, keywords]) => keywords.map(keyword => [keyword, jsonTypes.get(type)])));
const allJsonTypes = (1 << 7) - 1;
const [maximumNestedCollectionDepth, maximumNestedCollections] = jsonDocumentPolicy.nestedCollectionLimits.map(Number);
const [maximumFiniteObjectProperties, maximumFiniteArrayLength, maximumFiniteCollectionValues] = jsonDocumentPolicy.finiteCollectionLimits.map(Number);
const finiteScalarTypes = jsonTypes.get('boolean') | jsonTypes.get('null');
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

function chargeCollectionValue(value, copies, budget, depth = 0) {
  const cost = copies * (typeof value === 'string' ? 1 + Buffer.byteLength(value) : 1);
  if (cost > budget.remaining) return false;
  budget.remaining -= cost;
  if (depth > 128) return false;
  if (Array.isArray(value)) return value.every(item => chargeCollectionValue(item, copies, budget, depth + 1));
  if (value !== null && typeof value === 'object') {
    for (const key in value) {
      if (Object.hasOwn(value, key) && (!chargeCollectionValue(key, copies, budget, depth)
          || !chargeCollectionValue(value[key], copies, budget, depth + 1))) return false;
    }
  }
  return true;
}

function extendCollectionValues(prefixes, members, keepPrefixes, budget, append) {
  const size = prefixes.length * (members.length + Number(keepPrefixes));
  if (size > maximumFiniteCollectionValues || size > budget.remaining) return undefined;
  budget.remaining -= size;
  if (members.length && (!prefixes.every(prefix => chargeCollectionValue(prefix, members.length, budget))
      || !members.every(member => chargeCollectionValue(member, prefixes.length, budget)))) return undefined;
  const extended = prefixes.flatMap(prefix => members.map(member => append(prefix, member)));
  return keepPrefixes ? [...prefixes, ...extended] : extended;
}

function finiteCollectionValues(rules, membership, collectMember, budget) {
  if (membership === jsonTypes.get('object') && rules.additionalProperties === false) {
    const properties = rules.properties ?? {};
    const required = new Set(rules.required ?? []);
    if ([...required].some(name => !Object.hasOwn(properties, name))) return [];
    const fields = Object.entries(properties);
    if (fields.length > maximumFiniteObjectProperties) return undefined;
    let values = [{}];
    for (const [name, child] of fields) {
      const candidate = collectMember(child);
      if (!candidate) return undefined;
      if (!chargeCollectionValue(name, values.length * candidate.values.length, budget)) return undefined;
      values = extendCollectionValues(values, candidate.values, !required.has(name), budget,
        (prefix, member) => ({ ...prefix, [name]: member }));
      if (!values || !values.length) return values;
    }
    return values;
  }
  if (membership === jsonTypes.get('array')) {
    const [minimum, maximum] = arrayLengthBounds([rules]);
    if (maximum < minimum) return [];
    if (!Number.isSafeInteger(maximum) || maximum > maximumFiniteArrayLength) return undefined;
    let values = [[]];
    const candidates = [];
    for (let length = 0; length <= maximum; length += 1) {
      if (length >= minimum) {
        if (candidates.length + values.length > maximumFiniteCollectionValues
            || !values.every(value => chargeCollectionValue(value, 1, budget))) return undefined;
        candidates.push(...values);
      }
      if (length === maximum) break;
      const candidate = collectMember(rules.prefixItems?.[length] ?? rules.items ?? true);
      if (!candidate) return undefined;
      values = extendCollectionValues(values, candidate.values, false, budget,
        (prefix, member) => [...prefix, member]);
      if (!values) return undefined;
      if (!values.length) break;
    }
    return candidates;
  }
  return undefined;
}

function finiteSchemaCandidates(schema, resolve, budget = { remaining: 16_384 }, allowedTypes = allJsonTypes) {
  const active = new Set();
  function collect(rules, depth, allowed) {
    budget.remaining -= 1;
    if (budget.remaining < 0 || depth > 128 || active.has(rules)) return undefined;
    const values = finiteConstants(rules);
    if (values) return { values, keyword: Object.hasOwn(rules, 'const') ? 'const' : 'enum' };
    const membership = allowed & declaredTypeMembership(rules);
    if (!membership) return { values: [], keyword: 'type' };
    if ((membership & finiteScalarTypes) === membership) {
      return { values: [false, true, null].filter(value => valueTypeMembership(value) & membership), keyword: 'type' };
    }
    if (!rules || typeof rules !== 'object') return undefined;
    active.add(rules);
    try {
      if (typeof rules.$ref === 'string') {
        const target = collect(resolve(rules.$ref), depth + 1, membership);
        if (target) return { values: target.values, keyword: '$ref' };
      }
      for (const keyword of ['anyOf', 'oneOf']) {
        if (!Array.isArray(rules[keyword])) continue;
        const values = [];
        let complete = true;
        for (const child of rules[keyword]) {
          const candidate = collect(child, depth + 1, membership);
          if (!candidate || candidate.values.length > budget.remaining) {
            complete = false;
            break;
          }
          budget.remaining -= candidate.values.length;
          for (const value of candidate.values) values.push(value);
        }
        if (complete) return { values, keyword };
      }
      const collection = finiteCollectionValues(rules, membership, child => collect(child, depth + 1, allJsonTypes), budget);
      if (collection) return { values: collection, keyword: membership === jsonTypes.get('object') ? 'properties' : 'items' };
    } finally {
      active.delete(rules);
    }
    return undefined;
  }
  return collect(schema, 0, allowedTypes);
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

function referencedTypeMembership(branch, allowedTypes, referenceLayers) {
  return (referenceLayers(branch) ?? [branch]).reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
}

function appliesToType(keyword, membership) {
  return !assertionTypes.has(keyword) || (assertionTypes.get(keyword) & membership) !== 0;
}

function activeConjunctionKeyword(keyword, index, combination, membership) {
  return keyword !== 'type' && !referenceKeys.has(keyword)
    && !(index === 0 && keyword === combination) && appliesToType(keyword, membership);
}

function combineNumericConstraints(layers, membership, combination) {
  if (membership !== numberType && membership !== integerType) return undefined;
  const limits = {};
  for (const [index, layer] of layers.entries()) {
    for (const [key, value] of Object.entries(layer)) {
      if (!activeConjunctionKeyword(key, index, combination, membership)) continue;
      if (!numericLimitKeywords.includes(key) || !Number.isFinite(value)) return undefined;
      const tighten = key === 'minimum' || key === 'exclusiveMinimum' ? Math.max : Math.min;
      limits[key] = tighten(limits[key] ?? value, value);
    }
  }
  return { type: membership === integerType ? 'integer' : 'number', limits };
}

function compatibleChildSchema(schemas) {
  if (schemas.includes(false)) return false;
  const rule = schemas.find(schema => schema !== true) ?? true;
  if (schemas.every(schema => schema === true || isDeepStrictEqual(schema, rule))) return rule;
  const types = schemas.reduce((types, schema) => types & declaredTypeMembership(schema), allJsonTypes);
  return types === 0 ? false : undefined;
}

function arrayLengthBounds(layers) {
  const minimum = layers.reduce((limit, layer) => Math.max(limit, layer.minItems ?? 0), 0);
  const maximum = layers.reduce((limit, layer) => {
    const itemLimit = layer.items === false ? layer.prefixItems?.length ?? 0 : Infinity;
    return Math.min(limit, layer.maxItems ?? Infinity, itemLimit);
  }, Infinity);
  return [minimum, maximum];
}

function arrayLengthProfile(layers, combination) {
  const array = jsonTypes.get('array');
  if (layers.some((layer, index) => Object.keys(layer).some(key => (
    activeConjunctionKeyword(key, index, combination, array) && !jsonDocumentPolicy.arrayAssertion.includes(key)
  )))) return undefined;
  const [minimum, maximum] = arrayLengthBounds(layers);
  if (!Number.isSafeInteger(minimum) || (maximum !== Infinity && !Number.isSafeInteger(maximum + 1))) return undefined;
  let layout;
  for (const rules of layers) {
    const prefixItems = rules.prefixItems ?? [];
    // A closed tail already contributes its limit to the length interval.
    const items = rules.items === false ? true : rules.items ?? true;
    if (prefixItems.length === 0 && items === true) continue;
    const candidate = { prefixItems, items };
    if (layout && !isDeepStrictEqual(layout, candidate)) return undefined;
    layout = candidate;
  }
  return { minimum, maximum, layout };
}

function branchesUseDisjointTypes(branches, allowedTypes, referenceLayers) {
  if (branches.length < 2) return true;
  let seen = 0;
  for (const branch of branches) {
    if (branch === null || !['object', 'boolean'].includes(typeof branch) || Array.isArray(branch)) return false;
    const membership = referencedTypeMembership(branch, allowedTypes, referenceLayers);
    if (seen & membership) return false;
    seen |= membership;
  }
  return true;
}

function finiteStringValues(schema) {
  const values = finiteConstants(schema);
  return values?.every(value => typeof value === 'string' && value.isWellFormed()) ? values : undefined;
}

function requiredStringTags(schema) {
  const tags = new Map();
  const declared = Array.isArray(schema?.type) ? schema.type : [schema?.type];
  if (declared.length !== 1 || declared[0] !== 'object' || !Array.isArray(schema.required)
      || ['$ref', 'anyOf', 'oneOf'].some(key => Object.hasOwn(schema, key))) return tags;
  for (const key of schema.required) {
    if (!schema.properties || !Object.hasOwn(schema.properties, key)) continue;
    const values = finiteStringValues(schema.properties[key]);
    if (values) tags.set(key, values);
  }
  return tags;
}

function requiredPrefixTags(layers, minimum) {
  const tags = new Map();
  for (const layer of layers) {
    if (!Array.isArray(layer.prefixItems)) continue;
    const prefix = layer.prefixItems.slice(0, minimum);
    for (const [position, schema] of prefix.entries()) {
      if (tags.has(position)) continue;
      const values = finiteStringValues(schema);
      if (values) tags.set(position, values);
    }
  }
  return tags;
}

function branchesUseDisjointCollectionLayouts(branches, allowedTypes, referenceLayers) {
  const references = branches.map(referenceLayers);
  const tags = references.map(layers => requiredStringTags(layers?.at(-1)));
  const arrays = references.map(layers => {
    const membership = layers?.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
    if (membership !== jsonTypes.get('array')) return undefined;
    const [minimum, maximum] = arrayLengthBounds(layers);
    return { minimum, maximum, tags: requiredPrefixTags(layers, minimum) };
  });
  const disjointTags = (left, right) => [...left].some(([key, values]) => (
    right.has(key) && stringSetsAreDisjoint(values, right.get(key))
  ));
  return branches.every((branch, index) => branches.slice(0, index).every((previous, other) => {
    const left = arrays[index];
    const right = arrays[other];
    return branchesUseDisjointTypes([branch, previous], allowedTypes, referenceLayers)
      || (left && right && (Math.min(left.maximum, right.maximum) < Math.max(left.minimum, right.minimum)
        || disjointTags(left.tags, right.tags)))
      || disjointTags(tags[index], tags[other]);
  }));
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
  const numericConstantDiagnostics = new Set();
  const report = (pointer, keyword, message) => diagnostics.push({
    keyword, pointer: pointerChild(pointer, keyword), message,
  });
  let next = 1;
  let remainingNestedCollections = maximumNestedCollections;
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
  function childExpression(schema, pointer) {
    return typeof schema === 'function' ? schema() : named(schema, pointer);
  }
  function constantExpression(instance, pointer, keyword) {
    if (typeof instance === 'string') return quoted(instance);
    if (typeof instance === 'number' && Number.isFinite(instance)) {
      const location = pointerChild(pointer, keyword);
      if (instance !== 0 && !numericConstantDiagnostics.has(location)) {
        numericConstantDiagnostics.add(location);
        report(pointer, keyword, 'Validation accepts additional equivalent number spellings with shifted decimal points and compensating exponents.');
      }
      return numericConstant(instance);
    }
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
      const other = childExpression(rules.additionalProperties, pointerChild(pointer, 'additionalProperties'));
      return other ? collection('{', '}', unboundedList(seq(ref('string'), space, literal(':'), space, other))) : collection('{', '}', empty());
    }
    let additional;
    let additionalValue;
    if (rules.additionalProperties !== false) {
      const child = childExpression(rules.additionalProperties ?? true, pointerChild(pointer, 'additionalProperties'));
      additionalValue = child;
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
      const expression = Object.hasOwn(fields, key)
        ? childExpression(fields[key], pointerChild(pointerChild(pointer, 'properties'), key)) : additionalValue;
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
    const min = rules.minItems ?? 0;
    const max = rules.maxItems ?? null;
    if (max !== null && max < min) return null;
    if (max === 0) return collection('[', ']', empty());
    const prefix = (rules.prefixItems ?? []).map((item, index) => childExpression(item, pointerChild(pointerChild(pointer, 'prefixItems'), index)));
    const tail = childExpression(rules.items ?? true, pointerChild(pointer, 'items'));
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
    if (Object.keys(rules).some(key => key !== combination && appliesToType(key, jsonTypes.get('string')) && (key === '$ref'
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
  function regularStringChain(layers, combination) {
    const conjunction = [];
    for (const [index, layer] of layers.entries()) {
      const expressions = regularStringConstraints(layer, (index === 0 && combination) || '$ref');
      if (!expressions) return expressions;
      conjunction.push(...expressions);
    }
    return conjunction;
  }
  function combineChildSchemas(schemas, pointer, depth) {
    const compatible = compatibleChildSchema(schemas);
    if (compatible !== undefined) return compatible;
    const budget = { remaining: 16_384 };
    for (const schema of schemas) {
      const candidates = finiteSchemaCandidates(schema, analysis.target, budget);
      if (!candidates || candidates.values.length * schemas.length > budget.remaining) continue;
      const entries = candidates.values.filter(item => {
        const document = readJsonDocument(JSON.stringify(item));
        return schemas.every(rule => validateJsonDocument(document, rule, analysis).errors.length === 0);
      });
      return entries.length ? () => choice(...entries.map(item => constantExpression(item, pointer, candidates.keyword))) : false;
    }
    const layers = [];
    for (const schema of schemas) {
      if (schema === true) continue;
      const reference = referenceLayers(schema);
      if (!reference || layers.length + reference.length > 128) return undefined;
      layers.push(...reference);
    }
    const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allJsonTypes);
    const collections = membership & (jsonTypes.get('object') | jsonTypes.get('array'));
    // Recursive definitions can revisit the same collection through different member paths.
    if (collections && layers.some(layer => Object.keys(layer).some(keyword => (assertionTypes.get(keyword) ?? 0) & collections))) {
      if (depth >= maximumNestedCollectionDepth || remainingNestedCollections === 0) return undefined;
      remainingNestedCollections -= 1;
      depth += 1;
    }
    const entries = conjunctionEntries(layers, membership, pointer, undefined, depth);
    if (!entries) return undefined;
    return entries.length ? () => lowerConjunction(entries) : false;
  }
  function collectionConstraints(layers, membership, combination, pointer, depth, selectedProperties) {
    const keywords = membership === jsonTypes.get('object') ? jsonDocumentPolicy.objectAssertion : jsonDocumentPolicy.arrayAssertion;
    if (layers.some((layer, index) => Object.keys(layer).some(key => (
      activeConjunctionKeyword(key, index, combination, membership) && !keywords.includes(key)
    )))) return undefined;
    if (membership === jsonTypes.get('object')) {
      const fields = layers.map(layer => layer.properties ?? {});
      const defaults = layers.map(layer => layer.additionalProperties ?? true);
      const additionalProperties = combineChildSchemas(defaults, pointerChild(pointer, 'additionalProperties'), depth);
      if (additionalProperties === undefined) return undefined;
      const keys = new Set(fields.flatMap(Object.keys));
      const entries = [];
      for (const name of keys) {
        // Defaults apply only in the layers that leave this property undeclared.
        const rule = selectedProperties?.has(name) ? true
          : combineChildSchemas(fields.map((properties, index) => (
            Object.hasOwn(properties, name) ? properties[name] : defaults[index]
          )), pointerChild(pointerChild(pointer, 'properties'), name), depth);
        if (rule === undefined) return undefined;
        entries.push([name, rule]);
      }
      return { lower: objectExpression, rules: {
        properties: Object.fromEntries(entries),
        additionalProperties,
        required: layers.flatMap(layer => layer.required ?? []),
      } };
    }
    if (membership === jsonTypes.get('array')) {
      const [minimum, maximum] = arrayLengthBounds(layers);
      const layouts = maximum === 0 || maximum < minimum ? [] : layers.filter(layer => (
        (layer.prefixItems?.length ?? 0) > 0 || (layer.items !== undefined && typeof layer.items !== 'boolean')
      ));
      const bounds = { minItems: minimum, maxItems: Number.isFinite(maximum) ? maximum : null };
      if (layouts.length < 2) {
        const { prefixItems, items } = layouts[0] ?? {};
        return { lower: arrayExpression, rules: { prefixItems, items, ...bounds } };
      }
      const prefixLength = layouts.reduce((length, layer) => Math.max(length, layer.prefixItems?.length ?? 0), 0);
      const prefixItems = [];
      for (let index = 0; index < Math.min(prefixLength, maximum); index += 1) {
        const item = combineChildSchemas(layouts.map(layer => layer.prefixItems?.[index] ?? layer.items ?? true),
          pointerChild(pointerChild(pointer, 'prefixItems'), index), depth);
        if (item === undefined) return undefined;
        // An array cannot reach any position after a forbidden item.
        if (item === false) return { lower: arrayExpression, rules: { prefixItems, ...bounds, maxItems: index } };
        prefixItems.push(item);
      }
      const items = maximum <= prefixLength ? true : combineChildSchemas(layouts.map(layer => layer.items ?? true), pointerChild(pointer, 'items'), depth);
      if (items === undefined) return undefined;
      return { lower: arrayExpression, rules: { prefixItems, items, ...bounds } };
    }
    return undefined;
  }
  function referencedExpression(rules, allowedTypes, pointer) {
    const layers = referenceLayers(rules);
    if (!layers) return undefined;
    const entries = conjunctionEntries(layers, allowedTypes, pointer);
    return entries && lowerConjunction(entries);
  }
  function lowerConjunction(entries) {
    const expressions = entries.map(lower => lower()).filter(Boolean);
    return expressions.length ? choice(...expressions) : null;
  }
  function conjunctionEntries(layers, allowedTypes, pointer, combination, depth = 0) {
    const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
    if ((membership & numberType) === fractionalType) return undefined;
    const entries = [];
    for (const [type, mask] of jsonTypes) {
      if ((membership & mask) !== mask || (type === 'integer' && (membership & fractionalType))) continue;
      const entry = prepareType(type, mask);
      if (entry === undefined) return undefined;
      if (entry) entries.push(entry);
    }
    return entries;

    function prepareType(type, mask) {
      if (type === 'string') {
        const conjunction = regularStringChain(layers, combination);
        if (!conjunction) return conjunction;
        const expression = conjunction.length ? stringBranchProduct([conjunction]) : ref('string');
        return expression && (() => expression);
      }
      if (type === 'number' || type === 'integer') {
        const constraints = combineNumericConstraints(layers, mask, combination);
        return constraints && (() => numberExpression(type, constraints.limits, pointer));
      }
      if (type === 'object' || type === 'array') {
        const collection = collectionConstraints(layers, mask, combination, pointer, depth);
        return collection && (() => collection.lower(collection.rules, pointer));
      }
      const residual = layers.some((layer, index) => Object.keys(layer).some(key => (
        activeConjunctionKeyword(key, index, combination, mask)
      )));
      return residual ? undefined : () => ref(type);
    }
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
    const finiteWork = { remaining: 16_384 };
    for (const [index, branch] of alternatives.entries()) {
      if (possibleTypeMembership(branch, allowedTypes) === 0) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
      if (membership === 0) continue;
      const finite = keyword === 'oneOf' && (membership & numberType) === membership
        && finiteSchemaCandidates(branch, analysis.target, finiteWork);
      if (finite) {
        if (finite.values.length > finiteWork.remaining) return undefined;
        finiteWork.remaining -= finite.values.length;
        const common = combineNumericConstraints([rules], numberType, keyword);
        if (!common) return undefined;
        // Distinct values within one branch contribute disjoint point intervals.
        for (const item of new Set(finite.values)) {
          if (typeof item !== 'number' || !(membership & valueTypeMembership(item))) continue;
          const document = readJsonDocument(JSON.stringify(item));
          if ([branch, common.limits].every(rule => validateJsonDocument(document, rule, analysis).errors.length === 0)) {
            branches.push({ index, type: 'number', limits: { minimum: item, maximum: item } });
          }
        }
        continue;
      }
      const constraints = combineNumericConstraints([rules, ...layers], membership, keyword);
      if (!constraints) return undefined;
      branches.push({ index, ...constraints });
    }
    if (keyword === 'oneOf') {
      const numberBranches = branches.filter(branch => branch.type === 'number');
      if (numberBranches.length) {
        const mixed = numberBranches.length !== branches.length;
        let decimal = exclusiveDecimalRange(numberBranches.map(branch => branch.limits), mixed);
        if (decimal === undefined) return undefined;
        if (mixed) {
          // Count both schema types for an integer, including its zero-padded decimal spellings.
          const integer = exclusiveIntegerRange(branches.map(branch => integerBounds(branch.limits)));
          if (integer === undefined) return undefined;
          const integerSpellings = integer && seq(integer, optional(seq(literal('.'), repeat(literal('0'), 1))));
          const entries = [decimal, integerSpellings].filter(Boolean);
          decimal = entries.length ? choice(...entries) : null;
        }
        if (decimal && decimal.kind !== 'ref') {
          report(pointer, keyword, 'The exclusive number decoder uses decimal spelling. Validation also accepts equivalent exponent spellings.');
        }
        return decimal;
      }
      const integer = exclusiveIntegerRange(branches.map(branch => integerBounds(branch.limits)));
      if (integer) report(pointer, keyword, 'The exclusive integer decoder uses integer spelling. Validation also accepts equivalent decimal or exponent spellings.');
      return integer;
    }
    const location = pointerChild(pointer, keyword);
    const entries = branches.map(({ index, type, limits }) => numberExpression(type, limits, pointerChild(location, index))).filter(Boolean);
    return entries.length ? choice(...entries) : null;
  }
  function lowerExclusiveArrayAlternatives(rules, alternatives, allowedTypes, pointer) {
    const array = jsonTypes.get('array');
    const guard = arrayLengthProfile([rules], 'oneOf');
    if (!guard) return undefined;
    const profiles = [];
    for (const branch of alternatives) {
      if (possibleTypeMembership(branch, allowedTypes) === 0) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
      if (membership === 0) continue;
      if (membership !== array) return undefined;
      const profile = arrayLengthProfile(layers);
      if (!profile) return undefined;
      const minimum = Math.max(guard.minimum, profile.minimum);
      const maximum = Math.min(guard.maximum, profile.maximum);
      if (minimum <= maximum) profiles.push({ minimum, maximum, layout: profile.layout });
    }
    const lengths = lowerExclusiveArrayLengths(guard.layout, profiles, pointer);
    return lengths === undefined ? lowerExclusiveArrayItems(guard.layout, profiles, pointer) : lengths;
  }
  function lowerExclusiveArrayLengths(guard, profiles, pointer) {
    const groups = [];
    let emptyCoverage = 0;
    for (const { minimum, maximum, layout: items } of profiles) {
      if (minimum === 0) emptyCoverage += 1;
      if (maximum === 0) continue;
      if (guard && items && !isDeepStrictEqual(guard, items)) return undefined;
      const layout = guard ?? items;
      let group = groups.find(candidate => isDeepStrictEqual(candidate.layout, layout));
      if (!group) {
        group = { layout, boundaries: new Map() };
        groups.push(group);
      }
      const { boundaries } = group;
      const changeAt = (length, change) => boundaries.set(length, (boundaries.get(length) ?? 0) + change);
      // Coverage changes at the first included length and the first length after the interval.
      changeAt(Math.max(minimum, 1), 1);
      if (Number.isFinite(maximum)) changeAt(maximum + 1, -1);
    }
    // The first item distinguishes nonempty arrays with disjoint first-item types.
    if (groups.length > 1 && !branchesUseDisjointTypes(
      groups.map(({ layout }) => layout?.prefixItems[0] ?? layout?.items ?? true), allJsonTypes, referenceLayers,
    )) return undefined;
    const expressions = emptyCoverage === 1 ? [arrayExpression({ maxItems: 0 }, pointer)] : [];
    for (const { layout, boundaries } of groups) {
      const addInterval = (minItems, maxItems) => {
        const expression = arrayExpression({ ...layout, minItems, maxItems }, pointer);
        if (expression) expressions.push(expression);
      };
      let count = 0;
      let minimum;
      for (const [length, change] of [...boundaries].sort(([left], [right]) => left - right)) {
        const nextCount = count + change;
        if (count === 1 && nextCount !== 1) addInterval(minimum, length - 1);
        if (count !== 1 && nextCount === 1) minimum = length;
        count = nextCount;
      }
      if (count === 1) addInterval(minimum, null);
    }
    return expressions.length ? choice(...expressions) : null;
  }
  function lowerExclusiveArrayItems(guard, profiles, pointer) {
    const lastPosition = profiles.reduce((position, { layout, minimum, maximum }) => Math.max(
      position, minimum, Number.isFinite(maximum) ? maximum : 0, layout?.prefixItems.length ?? 0,
    ), Math.max(1, guard?.prefixItems.length ?? 0));
    const budget = { remaining: Number(jsonDocumentPolicy.exclusiveArrayWorkLimit[0]) };
    const states = new Map();
    const itemSchema = (layout, position) => layout?.prefixItems[position] ?? layout?.items ?? true;
    const itemPointer = pointerChild(pointer, 'items');
    const start = stateAt(0, profiles.map((_, index) => index));
    // After the final prefix or length boundary, only branch membership can change.
    for (const state of states.values()) {
      const { position, branches, transitions } = state;
      budget.remaining -= 1 + branches.length;
      if (budget.remaining < 0) return undefined;
      const children = branches.map(index => {
        const { layout, maximum } = profiles[index];
        return position < maximum ? itemSchema(layout, position) : false;
      });
      const variants = selectExclusiveCollectionValueVariants(children, itemSchema(guard, position), itemPointer, budget, true);
      if (!variants) return undefined;
      for (const variant of variants) {
        const remaining = variant.matches.map(index => branches[index]);
        transitions.push({ variant, target: stateAt(Math.min(position + 1, lastPosition), remaining) });
      }
    }
    // Work backward from possible ends. The only cycles consume a member and stay in the same state.
    const ordered = [...states.values()].sort((left, right) => right.position - left.position || left.branches.length - right.branches.length);
    for (const state of ordered) {
      const matching = state.branches.filter(index => state.position >= profiles[index].minimum);
      const expressions = matching.length === 1 ? [empty()] : [];
      const loops = [];
      for (const { variant, target } of state.transitions) {
        if (target !== state && !target.expression) continue;
        const item = variant.schema === undefined
          ? choice(...variant.values.map(value => constantExpression(value, itemPointer, 'oneOf')))
          : childExpression(variant.schema, itemPointer);
        if (!item) continue;
        const member = state.position ? separated(item) : item;
        if (target === state) loops.push(member);
        else expressions.push(seq(member, target.expression));
      }
      if (!expressions.length) continue;
      const name = `json_exclusive_array_${next++}`;
      state.expression = ref(name);
      builder.rule(name, choice(...expressions, ...loops.map(item => seq(item, state.expression))));
    }
    return start.expression ? collection('[', ']', start.expression) : null;

    function stateAt(position, branches) {
      const key = `${position}:${branches.join(',')}`;
      if (!states.has(key)) states.set(key, { position, branches, transitions: [] });
      return states.get(key);
    }
  }
  function exclusiveObjectFieldExpression(rules, alternatives, allowedTypes, pointer) {
    const object = jsonTypes.get('object');
    const siblingAssertions = Object.keys(rules).filter(key => activeConjunctionKeyword(key, 0, 'oneOf', object));
    if (siblingAssertions.some(key => !jsonDocumentPolicy.objectAssertion.includes(key))) return undefined;
    const layouts = [];
    for (const [index, branch] of alternatives.entries()) {
      if (!possibleTypeMembership(branch, allowedTypes)) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const membership = layers.reduce((types, layer) => possibleTypeMembership(layer, types), allowedTypes);
      if (membership === 0) continue;
      if (membership !== object || layers.some((layer, index) => Object.keys(layer).some(key => (
        activeConjunctionKeyword(key, index, undefined, object)
        && (index !== layers.length - 1 || !jsonDocumentPolicy.objectAssertion.includes(key))
      )))) return undefined;
      const leaf = layers.at(-1);
      const reference = layers.at(-2)?.$ref;
      layouts.push({
        rules: leaf,
        properties: leaf.properties ?? {}, required: new Set(leaf.required ?? []),
        additionalProperties: leaf.additionalProperties ?? true,
        pointer: reference ? reference.slice(1) : pointerChild(pointerChild(pointer, 'oneOf'), index),
      });
    }
    if (layouts.length < 2) return undefined;
    const [base, ...others] = layouts;
    if (others.some(layout => !isDeepStrictEqual(base.additionalProperties, layout.additionalProperties))) return undefined;
    const names = new Set(layouts.flatMap(layout => Object.keys(layout.properties)));
    const propertySchema = (layout, name) => Object.hasOwn(layout.properties, name)
      ? layout.properties[name] : layout.additionalProperties;
    const differing = new Set([...names].filter(name => others.some(layout => (
      !isDeepStrictEqual(propertySchema(base, name), propertySchema(layout, name))
    ))));
    const branchRequired = new Set(layouts.flatMap(layout => [...layout.required]));
    const differingRequired = new Set([...branchRequired].filter(name => layouts.some(layout => !layout.required.has(name))));
    const affected = new Set([...differing, ...differingRequired]);
    if (!affected.size) return null;
    const required = new Set([...(rules.required ?? []), ...[...branchRequired].filter(name => !differingRequired.has(name))]);
    const allBranches = layouts.map((_, index) => index);
    const budget = { remaining: 16_384 };
    const dimensions = [];
    const propertiesPointer = pointerChild(pointer, 'properties');
    for (const name of affected) {
      let variants = [{ matches: allBranches, useCommonSchema: true }];
      if (differing.has(name)) {
        const schemas = layouts.map(layout => propertySchema(layout, name));
        const sibling = Object.hasOwn(rules.properties ?? {}, name)
          ? rules.properties[name] : rules.additionalProperties ?? true;
        variants = selectExclusiveCollectionValueVariants(schemas, sibling, pointerChild(propertiesPointer, name), budget);
        if (!variants) return undefined;
      }
      if (!required.has(name)) {
        const matches = allBranches.filter(index => !layouts[index].required.has(name));
        if (matches.length) variants.push({ matches, values: [] });
      }
      dimensions.push({ name, variants });
    }

    let selections;
    if (dimensions.length === 1 && differingRequired.size === 0) {
      const [dimension] = dimensions;
      const exclusive = dimension.variants.filter(variant => variant.matches.length === 1);
      const shared = exclusive.find(variant => variant.schema !== undefined);
      const values = exclusive.flatMap(variant => variant.values ?? []);
      dimension.variants = [shared ?? { values }];
      selections = shared || values.length ? [[0]] : [];
    } else {
      // An absent field matches only branches that leave it optional.
      // The ordinary object emitter tracks each field subset for every selection.
      const keys = new Set([...names, ...required, ...branchRequired, ...Object.keys(rules.properties ?? {})]);
      if (keys.size > 12) return undefined;
      const workPerObject = (2 ** keys.size + 1) * (2 + keys.size);
      selections = [];
      if (!selectObjectFields(0, allBranches, [])) return undefined;

      function selectObjectFields(depth, matches, selected) {
        if (depth === dimensions.length) {
          if (matches.length !== 1) return true;
          if (budget.remaining < workPerObject) return false;
          budget.remaining -= workPerObject;
          selections.push(selected.slice());
          return true;
        }
        for (const [index, variant] of dimensions[depth].variants.entries()) {
          budget.remaining -= 1 + matches.length * variant.matches.length;
          if (budget.remaining < 0) return false;
          const remaining = matches.filter(index => variant.matches.includes(index));
          if (remaining.length === 0) continue;
          selected.push(index);
          if (!selectObjectFields(depth + 1, remaining, selected)) return false;
          selected.pop();
        }
        return true;
      }
    }
    if (!selections.length) return null;
    const layout = siblingAssertions.length
      ? collectionConstraints([rules, base.rules], object, 'oneOf', pointer, 0, differing)?.rules
      : { properties: base.properties, additionalProperties: base.additionalProperties };
    if (!layout) return undefined;
    const location = siblingAssertions.length ? pointer : base.pointer;
    const expressions = selections.map(selection => {
      const properties = Object.assign(Object.create(null), layout.properties);
      const requiredNames = [...required];
      for (const [index, { name, variants }] of dimensions.entries()) {
        const variant = variants[selection[index]];
        if (variant.useCommonSchema) {
          requiredNames.push(name);
          continue;
        }
        if (variant.schema !== undefined) {
          properties[name] = variant.schema;
          requiredNames.push(name);
          continue;
        }
        const { values } = variant;
        if (values.length && !variant.expression) {
          const rule = `json_exclusive_property_${next++}`;
          builder.rule(rule, choice(...values.map(item => constantExpression(item, pointer, 'oneOf'))));
          variant.expression = ref(rule);
        }
        properties[name] = values.length ? () => variant.expression : false;
        if (values.length) requiredNames.push(name);
      }
      return objectExpression({ properties, required: requiredNames, additionalProperties: layout.additionalProperties }, location);
    }).filter(Boolean);
    return expressions.length ? choice(...expressions) : null;
  }
  function selectExclusiveCollectionValueVariants(schemas, sibling, pointer, budget, allowSharedSchema = false) {
    if (sibling === false) return [];
    const allowed = schemas.filter(schema => schema !== false);
    if (!allowed.length) return [];
    // A shared schema gives every permitted present value the same branch membership.
    if ((allowSharedSchema || allowed.length < schemas.length) && allowed.every(schema => isDeepStrictEqual(schema, allowed[0]))) {
      const schema = combineChildSchemas([allowed[0], sibling], pointer, 0);
      if (schema === undefined) return undefined;
      if (schema === false) return [];
      return [{ matches: schemas.flatMap((rule, index) => rule === false ? [] : [index]), schema }];
    }
    const checks = schemas.length + Number(sibling !== true);
    const candidates = new Map();
    const shared = allowSharedSchema && sibling !== true && finiteSchemaCandidates(sibling, analysis.target, budget);
    for (const schema of shared ? [sibling] : allowed) {
      const finite = shared || finiteSchemaCandidates(schema, analysis.target, budget);
      if (!finite) return undefined;
      for (const item of finite.values) {
        if (!chargeCollectionValue(item, checks, budget)) return undefined;
        candidates.set(JSON.stringify(item), item);
      }
    }
    const variants = new Map();
    for (const [source, item] of candidates) {
      const document = readJsonDocument(source);
      if (sibling !== true && validateJsonDocument(document, sibling, analysis).errors.length !== 0) continue;
      const matches = [];
      for (const [index, rule] of schemas.entries()) {
        if (validateJsonDocument(document, rule, analysis).errors.length === 0) matches.push(index);
      }
      if (!matches.length) continue;
      const key = matches.join(',');
      if (!variants.has(key)) variants.set(key, { matches, values: [] });
      variants.get(key).values.push(item);
    }
    return [...variants.values()];
  }
  function alternativeConjunctionExpression(rules, alternatives, allowedTypes, keyword, pointer) {
    const entries = [];
    const location = pointerChild(pointer, keyword);
    for (const [index, branch] of alternatives.entries()) {
      if (possibleTypeMembership(branch, allowedTypes) === 0) continue;
      const layers = referenceLayers(branch);
      if (!layers) return undefined;
      const branchEntries = conjunctionEntries([rules, ...layers], allowedTypes, pointerChild(location, index), keyword);
      if (!branchEntries) return undefined;
      entries.push(...branchEntries);
    }
    return lowerConjunction(entries);
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
    let candidates;
    if (constants) {
      candidates = { values: constants, keyword: ['const', 'enum', 'anyOf', 'oneOf'].find(key => Object.hasOwn(rules, key)) };
    } else if (referenceHasSiblings || alternatives) {
      candidates = finiteSchemaCandidates(rules, analysis.target, undefined, allowedTypes);
    }
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
      const overlaps = keyword === 'oneOf' && !branchesUseDisjointTypes(alternatives, allowedTypes, referenceLayers)
        && !branchesUseDisjointCollectionLayouts(alternatives, allowedTypes, referenceLayers);
      if (overlaps || hasSiblingAssertions) {
        const combined = alternativeStringExpression(rules, alternatives, allowedTypes, keyword);
        if (combined !== undefined) return combined;
        const numeric = alternativeNumericExpression(rules, alternatives, allowedTypes, keyword, pointer);
        if (numeric !== undefined) return numeric;
        if (overlaps) {
          const array = lowerExclusiveArrayAlternatives(rules, alternatives, allowedTypes, pointer);
          if (array !== undefined) return array;
          const object = exclusiveObjectFieldExpression(rules, alternatives, allowedTypes, pointer);
          if (object !== undefined) return object;
        }
        if (!overlaps) {
          const conjunction = alternativeConjunctionExpression(rules, alternatives, allowedTypes, keyword, pointer);
          if (conjunction !== undefined) return conjunction;
        }
      }
      if (overlaps) {
        // An overlap can affect one JSON instance type without affecting the others.
        const reachable = alternatives.reduce((types, branch) => types | referencedTypeMembership(branch, allowedTypes, referenceLayers), 0);
        const families = [...jsonTypes].filter(([type, mask]) => type !== 'integer' && (mask & reachable));
        if (families.length > 1) {
          const entries = families.map(([, mask]) => named(rules, pointer, mask & allowedTypes)).filter(Boolean);
          return entries.length ? choice(...entries) : null;
        }
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
