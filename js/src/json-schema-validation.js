import { jsonDocumentError, pointerChild, readJsonDocument, readJsonValue } from './json-document.js';

const annotations = new Set([
  '$id', '$schema', '$comment', 'title', 'description', 'default', 'examples',
  'deprecated', 'readOnly', 'writeOnly',
]);
const schemaMaps = ['properties', '$defs'];
const schemaLists = ['prefixItems', 'anyOf', 'oneOf'];
const schemaValues = ['items', 'additionalProperties'];
const numericLimits = new Map([
  ['minimum', (actual, limit) => actual < limit], ['maximum', (actual, limit) => actual > limit],
  ['exclusiveMinimum', (actual, limit) => actual <= limit], ['exclusiveMaximum', (actual, limit) => actual >= limit],
]);
const schemaTypes = new Set(['object', 'array', 'string', 'integer', 'number', 'boolean', 'null']);
const typeSet = value => Array.isArray(value) ? value : [value];
const object = value => value !== null && typeof value === 'object' && !Array.isArray(value);

function dateMatch(value) {
  const match = /^(\d{4})-(\d{2})-(\d{2})$/u.exec(value);
  if (!match) return false;
  const [, year, month, day] = match.map(Number);
  if (month < 1 || month > 12 || day < 1) return false;
  const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
  return day <= [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][month - 1];
}

function dateTimeMatch(value) {
  const match = /^(\d{4}-\d{2}-\d{2})[Tt](\d{2}):(\d{2}):(\d{2})(?:\.\d+)?(?:[Zz]|[+-](\d{2}):(\d{2}))$/u.exec(value);
  if (!match || !dateMatch(match[1])) return false;
  const [, , hour, minute, seconds, offsetHour = '0', offsetMinute = '0'] = match;
  return Number(hour) <= 23 && Number(minute) <= 59 && Number(seconds) <= 60
    && Number(offsetHour) <= 23 && Number(offsetMinute) <= 59;
}

const formats = new Map([
  ['date', dateMatch], ['date-time', dateTimeMatch],
  ['uuid', value => /^[0-9a-f]{8}-(?:[0-9a-f]{4}-){3}[0-9a-f]{12}$/iu.test(value)],
  ['email', value => /^(?:[a-z\d!#$%&'*+\-/=?^_`{|}~]+(?:\.[a-z\d!#$%&'*+\-/=?^_`{|}~]+)*|"(?:[\x20-\x21\x23-\x5b\x5d-\x7e]|\\[\x20-\x7e])*")@(?:[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?)(?:\.[a-z\d](?:[a-z\d-]{0,61}[a-z\d])?)*$/iu.test(value)],
]);

function referenceTarget(root, ref) {
  if (typeof ref !== 'string' || !ref.startsWith('#')) return undefined;
  let pointer;
  try { pointer = decodeURIComponent(ref.slice(1)); } catch { return undefined; }
  if (pointer === '') return root;
  if (!pointer.startsWith('/')) return undefined;
  let target = root;
  for (const token of pointer.slice(1).split('/')) {
    if (/~(?![01])/u.test(token)) return undefined;
    const key = token.replaceAll('~1', '/').replaceAll('~0', '~');
    if (!target || typeof target !== 'object' || !Object.hasOwn(target, key)) return undefined;
    target = target[key];
  }
  return target;
}

export function analyzeJsonSchema(root) {
  const issues = [];
  const patterns = new Map();
  const visited = new Set();
  function visit(schema, pointer) {
    if (typeof schema === 'boolean') return;
    if (!object(schema)) throw new TypeError(`Expected a JSON Schema object or boolean at ${pointer || '/'}`);
    if (visited.has(schema)) return;
    visited.add(schema);
    const report = (keyword, message) => issues.push({ keyword, pointer: pointerChild(pointer, keyword), message });
    for (const [keyword, value] of Object.entries(schema)) {
      const location = pointerChild(pointer, keyword);
      const require = (condition, expectation) => {
        if (!condition) throw new TypeError(`Expected ${expectation} at ${location}`);
      };
      if (annotations.has(keyword)) continue;
      if (schemaMaps.includes(keyword)) {
        require(object(value), 'an object of schemas');
        for (const [key, child] of Object.entries(value)) visit(child, pointerChild(location, key));
      } else if (schemaLists.includes(keyword)) {
        require(Array.isArray(value) && value.length > 0, 'a nonempty array of schemas');
        value.forEach((child, index) => visit(child, pointerChild(location, index)));
      } else if (schemaValues.includes(keyword)) {
        visit(value, location);
      } else if (keyword === 'type') {
        const types = typeSet(value);
        require(types.length > 0 && new Set(types).size === types.length && types.every(item => schemaTypes.has(item)), 'a JSON type or a nonempty list of unique JSON types');
      } else if (keyword === 'enum') {
        require(Array.isArray(value) && value.length > 0, 'a nonempty array of permitted values');
      } else if (keyword === 'required') {
        require(Array.isArray(value) && new Set(value).size === value.length && value.every(key => typeof key === 'string'), 'an array of unique property names');
      } else if (keyword === 'minItems' || keyword === 'maxItems') {
        require(Number.isSafeInteger(value) && value >= 0, 'a nonnegative safe integer');
      } else if (numericLimits.has(keyword)) {
        require(Number.isFinite(value), 'a finite number');
      } else if (keyword === 'pattern') {
        require(typeof value === 'string', 'an ECMA-262 regular expression');
        patterns.set(schema, new RegExp(value, 'u'));
      } else if (keyword === 'format') {
        require(typeof value === 'string', 'a format name');
        if (!formats.has(value)) report(keyword, `Unsupported string format ${value}`);
      } else if (keyword === '$ref') {
        const target = referenceTarget(root, value);
        if (typeof target !== 'boolean' && !object(target)) report(keyword, `Unresolved local schema reference ${value}`);
      } else if (keyword !== 'const') {
        report(keyword, `Unsupported JSON Schema keyword ${keyword}`);
      }
    }
  }
  visit(root, '');
  return { issues, patterns, target: ref => referenceTarget(root, ref) };
}

function jsonEqual(left, right) {
  if (left === right) return true;
  if (typeof left !== 'object' || typeof right !== 'object' || left === null || right === null
    || Array.isArray(left) !== Array.isArray(right)) return false;
  const keys = Object.keys(left);
  return keys.length === Object.keys(right).length && keys.every(key => Object.hasOwn(right, key) && jsonEqual(left[key], right[key]));
}

function instanceType(value) {
  if (value === null) return 'null';
  if (Array.isArray(value)) return 'array';
  return typeof value;
}

function objectLocations(document) {
  const byPointer = new Map();
  const keys = new Map();
  for (const node of document.nodes) {
    if (node.field === 'key' && node.kind === 'string') keys.set(node.pointer, node);
    else if (!node.flags.isMissing && ['array', 'object', 'string', 'number', 'true', 'false', 'null'].includes(node.kind)) {
      byPointer.set(node.pointer, node);
    }
  }
  return { byPointer, keys };
}

export function validateJsonDocument(document, schema, analysis = analyzeJsonSchema(schema)) {
  const syntax = jsonDocumentError(document);
  const value = readJsonValue(document.value);
  const { byPointer, keys } = objectLocations(document);
  const active = new Map();
  const errors = syntax ? [syntax] : [];
  function visit(rules, instance, pointer) {
    if (instance === undefined) return;
    const type = instanceType(instance);
    const node = byPointer.get(pointer) ?? document.root;
    const complete = !node.flags.hasError;
    const report = (keyword, expected, at = pointer, node = byPointer.get(at) ?? document.root) => {
      errors.push({ keyword, expected, pointer: at, span: node.span, linkId: node.linkId });
    };
    if (rules === true) return;
    if (rules === false) {
      report('false', []);
      return;
    }
    const activePointers = active.get(rules) ?? new Set();
    if (activePointers.has(pointer)) {
      report('$ref', ['a reference that advances into the instance']);
      return;
    }
    active.set(rules, activePointers);
    activePointers.add(pointer);
    if (rules.$ref !== undefined) {
      const target = analysis.target(rules.$ref);
      if (typeof target === 'boolean' || object(target)) visit(target, instance, pointer);
    }
    if (rules.type !== undefined && !typeSet(rules.type).some(expected => expected === type
      || (expected === 'integer' && type === 'number' && Number.isInteger(instance)))) {
      report('type', typeSet(rules.type));
    }
    if (complete && rules.enum && !rules.enum.some(candidate => jsonEqual(candidate, instance))) report('enum', rules.enum);
    if (complete && Object.hasOwn(rules, 'const') && !jsonEqual(rules.const, instance)) report('const', [rules.const]);
    if (type === 'object') {
      for (const key of rules.required ?? []) {
        if (complete && !Object.hasOwn(instance, key)) report('required', [key], pointerChild(pointer, key), node);
      }
      for (const [key, child] of Object.entries(instance)) {
        const location = pointerChild(pointer, key);
        if (rules.properties && Object.hasOwn(rules.properties, key)) visit(rules.properties[key], child, location);
        else if (rules.additionalProperties === false) report('additionalProperties', Object.keys(rules.properties ?? {}), location, keys.get(location));
        else if (object(rules.additionalProperties)) visit(rules.additionalProperties, child, location);
      }
    } else if (type === 'array') {
      if (complete && rules.minItems !== undefined && instance.length < rules.minItems) report('minItems', [rules.minItems]);
      if (complete && rules.maxItems !== undefined && instance.length > rules.maxItems) report('maxItems', [rules.maxItems]);
      instance.forEach((child, index) => {
        const childSchema = rules.prefixItems?.[index] ?? rules.items;
        if (child !== undefined && childSchema === false && index >= (rules.prefixItems?.length ?? 0)) {
          report('items', [], pointerChild(pointer, index));
        } else if (childSchema !== undefined) visit(childSchema, child, pointerChild(pointer, index));
      });
    } else if (type === 'string') {
      if (analysis.patterns.has(rules) && !analysis.patterns.get(rules).test(instance)) report('pattern', [rules.pattern]);
      if (formats.has(rules.format) && !formats.get(rules.format)(instance)) report('format', [rules.format]);
    } else if (type === 'number') {
      for (const [keyword, rejects] of numericLimits) {
        if (rules[keyword] !== undefined && rejects(instance, rules[keyword])) report(keyword, [rules[keyword]]);
      }
    }
    for (const combination of ['anyOf', 'oneOf']) {
      if (!complete || !rules[combination]) continue;
      let accepted = 0;
      const alternatives = [];
      for (const item of rules[combination]) {
        const first = errors.length;
        visit(item, instance, pointer);
        if (errors.length === first) accepted += 1;
        alternatives.push(errors.splice(first));
      }
      if (accepted === 0 || (combination === 'oneOf' && accepted !== 1)) {
        errors.push({ keyword: combination, expected: combination === 'oneOf' ? ['one schema'] : ['at least one schema'],
          pointer, span: node.span, linkId: node.linkId,
          alternatives, accepted });
      }
    }
    activePointers.delete(pointer);
  }
  visit(schema, value, '');
  return { valid: errors.length === 0 && analysis.issues.length === 0, value: syntax ? undefined : value, errors,
    schemaIssues: analysis.issues, network: document.network };
}

/** Validate a JSON document using draft 2020-12 keywords and report CST locations. */
export function validateJson(source, schema) {
  return validateJsonDocument(readJsonDocument(source), schema);
}
