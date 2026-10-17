// Binding forms share one evaluation order: read the value once, then bind
// its selected fields or elements from left to right.
import { unsupported, typeError } from './diagnostics.js';
import { imperative, lowerImperative } from './javascript-lower.js';
import { readTypeSeparatorPositions } from './type-syntax-rules.js';
import { acceptRecordTypeBoundary, acceptRecordTypeOptionalName, readRecordTypeFieldColon } from './record-type-syntax-rules.js';
import { readNullableTypeAbsence, readNullableTypeSyntax } from './nullable-type-syntax-rules.js';
import { constructCallableSurfaceParameters, readCallableClosureConflicts, readCallableClosureScope, readCallableStatementCaptures } from './callable-source.js';
import { planCallableTypeSyntax, readCallableClosureParameterError, readCallableClosurePrefixError, readCallableSurfaceConstructor, requireCallableAssignedCaptureField } from './callable-source-rules.js';
import { acceptJavaScriptArrayBindingName, planJavaScriptArrayBindingNames, readJavaScriptArrayBindingValueForm, readJavaScriptArrayRestBindingError } from './array-binding-rules.js';
import { JAVASCRIPT_ARRAY_BINDING_POLICY } from './array-binding-policy.js';
import { acceptJavaScriptLiteralBinding } from './javascript-scoped-binding-emission-rules.js';

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

// Declaration discovery and parsing use the same actual token positions. A
// malformed plan stays with the ordinary parser for its precise syntax error.
export function arrayBindingNames(parser, start) {
  const tokens = parser.cursor.tokens;
  const plan = planJavaScriptArrayBindingNames(tokens.map(token => token.kind), tokens.map(token => token.value), start);
  return plan[0] < 0 ? [] : plan.slice(1).map(position => tokens[position]);
}

export function readArrayBindingInitializer(parser, bindings, read) {
  const outer = parser.scope;
  parser.scope = {
    locals: new Set(outer.locals), mutable: new Set(outer.mutable),
    tdz: new Set([...outer.tdz, ...bindings.map(binding => binding.name)]),
  };
  try { return read(); } finally { parser.scope = outer; }
}

export function readArrayBinding(parser, path = [], state = null) {
  const c = parser.cursor;
  const outer = parser.scope;
  const root = state === null;
  if (root) {
    const names = new Set();
    for (const token of arrayBindingNames(parser, c.index)) {
      if (!acceptJavaScriptArrayBindingName(token.kind, token.value, names.has(token.value), JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings)) {
        throw unsupported('array binding name', 'every binding needs a distinct admitted lexical identifier', token);
      }
      names.add(token.value);
    }
    state = { seen: new Set() };
    parser.scope = { locals: new Set(outer.locals), mutable: new Set(outer.mutable), tdz: new Set([...outer.tdz, ...names]) };
  }
  try {
    c.expect('[', 'array binding');
    const bindings = [];
    let index = 0;
    while (!c.is(']')) {
      if (c.eat(',')) { index += 1; continue; }
      const rest = Boolean(c.eat('...'));
      if (rest && path.length) throw unsupported('nested array rest binding', 'a nested rest binding needs its product iterator proof', c.peek());
      if (c.is('[')) {
        if (rest) throw unsupported('array rest pattern', 'the selected rest binding needs a lexical name', c.peek());
        bindings.push(...readArrayBinding(parser, [...path, index], state));
        index += 1;
        if (!c.eat(',')) break;
        continue;
      }
      const token = c.identifier('array binding');
      if (!acceptJavaScriptArrayBindingName(token.kind, token.value, state.seen.has(token.value), JAVASCRIPT_ARRAY_BINDING_POLICY.reservedBindings)) {
        throw unsupported('array binding name', 'every binding needs a distinct admitted lexical identifier', token);
      }
      state.seen.add(token.value);
      let defaultValue;
      if (rest) {
        const reason = readJavaScriptArrayRestBindingError(c.peek().value);
        if (reason) throw unsupported('array rest binding', reason, c.peek());
      } else if (c.eat('=')) {
        if (path.length) throw unsupported('nested array binding default', 'a nested default needs its product iterator proof', token);
        defaultValue = parser.expr();
      }
      bindings.push({ name: token.value, index, ...(path.length ? { path: [...path, index] } : {}),
        ...(rest ? { rest: true } : {}), ...(defaultValue ? { defaultValue } : {}), span: { start: token.start, end: token.end } });
      parser.declareLocal(token.value);
      index += 1;
      if (rest) break;
      if (!c.eat(',')) break;
    }
    c.expect(']', 'array binding');
    return bindings;
  } finally {
    if (root) parser.scope = outer;
  }
}

export function bindArrayElements(bindings, object, binding, where) {
  return bindings.map(entry => {
    const form = readJavaScriptArrayBindingValueForm(entry.rest === true, Object.hasOwn(entry, 'defaultValue'));
    const value = (entry.path ?? [entry.index]).reduce((value, index) => ({ k: 'index', object: value,
      index: { k: 'num', value: String(index), span: entry.span }, ...(entry.path && { productBinding: true }), span: entry.span }), object);
    return { s: binding, name: entry.name, span: where,
      value: form === 'suffix' ? { k: 'arrayBindingRest', object, start: { k: 'num', value: String(entry.index), span: entry.span }, span: entry.span }
        : form === 'default' ? { k: 'arrayBindingDefault', value, fallback: entry.defaultValue, span: entry.span } : value };
  });
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
  const entries = value.k === 'record' ? value.entries : [];
  let hasSpread = false;
  const allChildrenAdmitted = entries.every((entry) => {
    hasSpread = Boolean(entry.spread);
    return !hasSpread && isLiteralBinding(entry.value);
  });
  return acceptJavaScriptLiteralBinding(value.k, hasSpread, allChildrenAdmitted);
}
