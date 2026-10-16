// Binding forms share one evaluation order: read the value once, then bind
// its selected fields or elements from left to right.
import { unsupported, typeError } from './diagnostics.js';
import { imperative, lowerImperative } from './javascript-lower.js';
import { readTypeSeparatorPositions } from './type-syntax-rules.js';
import { acceptRecordTypeBoundary, acceptRecordTypeOptionalName, readRecordTypeFieldColon } from './record-type-syntax-rules.js';
import { readNullableTypeAbsence, readNullableTypeSyntax } from './nullable-type-syntax-rules.js';
import { constructCallableSurfaceParameters, readCallableClosureConflicts, readCallableClosureScope, readCallableStatementCaptures } from './callable-source.js';
import { planCallableTypeSyntax, readCallableClosureParameterError, readCallableClosurePrefixError, readCallableSurfaceConstructor, requireCallableAssignedCaptureField } from './callable-source-rules.js';

export function readClosure(parser, method = false) {
  const c = parser.cursor;
  const start = c.peek();
  const expression = method || Boolean(c.eat('function'));
  const prefixError = readCallableClosurePrefixError(expression, c.peek().kind, c.peek().value);
  if (prefixError === 'named function expression') throw unsupported(prefixError, 'the function name needs a recursive local binding', c.peek());
  if (prefixError === 'generator function') throw unsupported(prefixError, 'generators require a suspended execution state', c.peek());
  const tokens = c.eat('(') ? parser.parameters('closure') : [c.identifier('closure parameter')];
  if (!expression) c.expect('=>', 'closure');
  const names = tokens.map((token) => token.value);
  const parameterError = readCallableClosureParameterError(names, tokens.map(token => Boolean(token.defaultValue)));
  if (parameterError === 'duplicate closure parameter') throw typeError(parameterError, start);
  if (parameterError === 'closure parameter default') throw unsupported(parameterError, 'a function value needs a callable default-argument signature', start);
  const params = constructCallableSurfaceParameters(tokens);
  const outer = parser.scope;
  const outerJumps = parser.jumps;
  const outerTop = parser.topLevel;
  const outerAsync = parser.inAsync;
  parser.scope = readCallableClosureScope(outer, tokens);
  parser.jumps = { loops: 0, switches: 0 };
  parser.topLevel = false;
  parser.inAsync = false;
  try {
    const statements = c.is('{') ? parser.blockStatements() : [{ s: 'return', expr: parser.expr(), span: start }];
    const where = { start: start.start, end: c.peek().start };
    const { assignedCaptures, captures } = readCallableStatementCaptures(statements, names, outer.locals);
    if (assignedCaptures.length && !parser.referenceSemantics) throw unsupported('mutable closure capture', 'captured assignments require a shared cell across calls', where);
    const body = imperative(statements) ? lowerImperative(parser, `closure${++parser.generatedCount}`, params, statements, where, new Set(assignedCaptures), new Set(captures)) : parser.lower(statements, where);
    return { k: readCallableSurfaceConstructor('closure', false), params, ret: null, captures, ...(requireCallableAssignedCaptureField(assignedCaptures.length) ? { assignedCaptures } : {}), body, span: where };
  } finally {
    parser.scope = outer;
    parser.jumps = outerJumps;
    parser.topLevel = outerTop;
    parser.inAsync = outerAsync;
  }
}

export function validateClosureCaptures(statements, references = false) {
  for (const { closure, capture } of readCallableClosureConflicts(statements)) {
    if (capture && !references) throw unsupported(`mutable closure capture ${capture}`, 'captured assignments require a shared cell across calls', closure.span);
  }
}

export function readArrayBinding(parser, path = []) {
  const c = parser.cursor;
  c.expect('[', 'array binding');
  const bindings = [];
  let index = 0;
  while (!c.is(']')) {
    if (c.eat(',')) { index += 1; continue; }
    if (c.is('...')) throw unsupported('array rest binding', 'rest binding needs an array suffix operation', c.peek());
    if (c.is('[')) {
      bindings.push(...readArrayBinding(parser, [...path, index]));
      index += 1;
      if (!c.eat(',')) break;
      continue;
    }
    const token = c.identifier('array binding');
    if (token.value.startsWith('ml_')) throw unsupported('reserved identifier', `${token.value} uses the translator's reserved ml_ prefix`, token);
    if (c.is('=')) throw unsupported('array binding default', 'a missing element needs an undefined-aware default', token);
    bindings.push({ name: token.value, index, ...(path.length ? { path: [...path, index] } : {}), span: { start: token.start, end: token.end } });
    index += 1;
    if (!c.eat(',')) break;
  }
  c.expect(']', 'array binding');
  return bindings;
}

export function bindArrayElements(bindings, object, binding, where) {
  return bindings.map((entry) => ({
    s: binding, name: entry.name,
    value: (entry.path ?? [entry.index]).reduce((value, index) => ({ k: 'index', object: value, index: { k: 'num', value: String(index), span: entry.span }, ...(entry.path && { productBinding: true }), span: entry.span }), object),
    span: where,
  }));
}

// Split type syntax at its outer level, retaining commas and bars inside
// records, tuples, generic arguments and parentheses.
export function splitType(text, separator) {
  let start = 0;
  const parts = [];
  const units = Array.from({ length: text.length }, (_, index) => text.charCodeAt(index));
  const separatorUnit = typeof separator === 'string' && separator.length === 1 ? separator.charCodeAt(0) : -1;
  for (const at of readTypeSeparatorPositions(units, separatorUnit)) {
    parts.push(text.slice(start, at).trim());
    start = at + 1;
  }
  parts.push(text.slice(start).trim());
  return parts;
}

export function readCompositeType(parser, name, range) {
  const plan = planCallableTypeSyntax(Array.from({ length: name.length }, (_, index) => name.charCodeAt(index)));
  if (plan[0] === -1) throw unsupported(`JSDoc type {${name}}`, 'a function type needs a return type', range);
  if (plan[0] === 0) return parser.type(name.slice(plan[1], plan[2]), range);
  if (plan[0] === 1) {
    const params = [];
    let at = 4;
    for (let parameter = 0; parameter < plan[3]; parameter += 1) {
      const count = plan[at++];
      const pieces = [];
      for (let piece = 0; piece < count; piece += 1) { pieces.push(name.slice(plan[at], plan[at + 1])); at += 2; }
      params.push(parser.type(pieces.join(':'), range));
    }
    return { kind: 'function', params, ret: parser.type(name.slice(plan[1], plan[2]), range) };
  }
  const alternatives = splitType(name, '|');
  const nullable = readNullableTypeSyntax(alternatives, name.charCodeAt(0));
  if (nullable[0] === 1) return { kind: 'option', element: parser.type(alternatives[nullable[1]], range), absence: readNullableTypeAbsence(nullable[2]) };
  if (nullable[0] === 2) {
    return { kind: 'union', alternatives: alternatives.map((part) => parser.type(part, range)) };
  }
  if (nullable[0] === 3) return { kind: 'absence', absence: readNullableTypeAbsence(nullable[2]) };
  if (nullable[0] === 4) return { kind: 'option', element: parser.type(name.slice(1), range), absence: readNullableTypeAbsence(nullable[2]) };
  if (name.startsWith('[') && name.endsWith(']')) return { kind: 'tuple', elements: splitType(name.slice(1, -1), ',').filter(Boolean).map((part) => parser.type(part, range)) };
  if (acceptRecordTypeBoundary(name.charCodeAt(0), name.charCodeAt(name.length - 1))) {
    const fields = splitType(name.slice(1, -1), ',').filter(Boolean).map((part) => {
      const index = readRecordTypeFieldColon(Array.from({ length: part.length }, (_, index) => part.charCodeAt(index)));
      if (index < 0) throw unsupported(`record type {${name}}`, 'every field has a name and a type', range);
      const field = part.slice(0, index).trim();
      const optional = acceptRecordTypeOptionalName(field.charCodeAt(field.length - 1));
      const type = parser.type(part.slice(index + 1), range);
      return { name: optional ? field.slice(0, -1) : field, type: optional ? { kind: 'option', element: type, absence: 'undefined' } : type };
    });
    return { kind: 'record', fields };
  }
  const generic = /^(Set|Map)<([\s\S]+)>$/u.exec(name);
  if (generic) {
    const arguments_ = splitType(generic[2], ',');
    if (generic[1] === 'Set' && arguments_.length === 1) return { kind: 'set', element: parser.type(arguments_[0], range) };
    if (generic[1] === 'Map' && arguments_.length === 2) return { kind: 'map', key: parser.type(arguments_[0], range), value: parser.type(arguments_[1], range) };
    throw unsupported(`JSDoc type {${name}}`, 'Set has one type argument and Map has two', range);
  }
  return null;
}

export function isLiteralBinding(value) {
  if (['num', 'bool', 'str', 'none'].includes(value.k)) return true;
  return value.k === 'record' && value.entries.every((entry) => !entry.spread && isLiteralBinding(entry.value));
}
