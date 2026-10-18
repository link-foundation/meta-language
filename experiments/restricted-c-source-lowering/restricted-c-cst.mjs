// SOURCE-ONLY EXPERIMENT. It lowers the repository's real C CST to
// typed address/value/control-flow instructions; it does NOT execute C or
// pretend that these instructions are a checked portable Program.
import {createHash} from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {encodeStructuredDataLinks, decodeStructuredDataLinks} from '../../js/src/structured-data-links.js';

const unknownType = reason => ({kind: 'unresolved C type', reason});
const pointerTo = to => ({kind: 'C pointer', to});
const booleanType = Object.freeze({kind: 'C boolean'});
const voidType = Object.freeze({kind: 'C void'});
const unqualified = type => type?.kind === 'C qualified' ? unqualified(type.type) : type;
const sameType = (a, b) => a === b || (a?.kind === 'C integer' && b?.kind === 'C integer'
  && a.bits === b.bits && a.signed === b.signed);
const expressionTerms = new Set(['identifier', 'field_identifier', 'number_literal', 'char_literal',
  'character_literal', 'string_literal', 'true', 'false', 'null', 'parenthesized_expression',
  'binary_expression', 'unary_expression', 'pointer_expression', 'assignment_expression',
  'update_expression', 'call_expression', 'field_expression', 'subscript_expression',
  'cast_expression', 'conditional_expression', 'sizeof_expression', 'size_of_expression']);

function sameSourceDataGraph(left, right) {
  const pending = [[left, right]], forward = new WeakMap(), backward = new WeakMap();
  while (pending.length) {
    const [actual, expected] = pending.pop();
    if (actual === null || expected === null || typeof actual !== 'object' || typeof expected !== 'object') {
      if (!Object.is(actual, expected)) return false;
      continue;
    }
    if (Array.isArray(actual) !== Array.isArray(expected)) return false;
    if (forward.has(actual)) { if (forward.get(actual) !== expected) return false; continue; }
    if (backward.has(expected)) return false;
    forward.set(actual, expected); backward.set(expected, actual);
    const names = Reflect.ownKeys(actual), expectedNames = Reflect.ownKeys(expected);
    if (names.length !== expectedNames.length || names.some((name, index) => name !== expectedNames[index])) return false;
    for (const name of names) {
      const first = Object.getOwnPropertyDescriptor(actual, name), second = Object.getOwnPropertyDescriptor(expected, name);
      if (!Object.hasOwn(first, 'value') || !Object.hasOwn(second, 'value') || first.enumerable !== second.enumerable) return false;
      pending.push([first.value, second.value]);
    }
  }
  return true;
}

function copyRestrictedCSourceData(value, seen = new WeakMap(), active = new WeakSet()) {
  if (['undefined', 'function', 'symbol', 'bigint'].includes(typeof value)
    || (typeof value === 'number' && !Number.isFinite(value))) throw new TypeError('source-only C data requires supported data values');
  if (!value || typeof value !== 'object') return value;
  if (active.has(value)) throw new TypeError('cyclic source-only C containers require explicit link references');
  if (seen.has(value)) return seen.get(value);
  const array = Array.isArray(value), names = Reflect.ownKeys(value);
  if (names.some(name => typeof name !== 'string')) throw new TypeError('source-only C data cannot hide symbol fields');
  if (array && (names.length !== value.length + 1 || !Array.from({length: value.length}, (_, index) => Object.hasOwn(value, index)).every(Boolean))) throw new TypeError('source-only C data requires dense arrays without extra fields');
  const result = array ? [] : {};
  seen.set(value, result);
  active.add(value);
  for (const key of names) {
    if (array && key === 'length') continue;
    const descriptor = Object.getOwnPropertyDescriptor(value, key);
    if (!Object.hasOwn(descriptor, 'value') || !descriptor.enumerable) throw new TypeError('source-only C data cannot contain host accessor or hidden fields');
    Object.defineProperty(result, key, {value: copyRestrictedCSourceData(descriptor.value, seen, active), enumerable: true, writable: true, configurable: true});
  }
  active.delete(value);
  return result;
}

export function lowerRestrictedCst(parsed, source, options = {}) {
  if (typeof source !== 'string') throw new TypeError('source-only C lowering requires source text');
  const inputs = copyRestrictedCSourceData({parsed, options});
  parsed = inputs.parsed; options = inputs.options;
  if (parsed?.canonical !== 'C' || parsed.tree?.term !== 'translation_unit') {
    throw new TypeError('the restricted lowerer requires the repository C translation-unit CST');
  }
  const bytes = Buffer.from(source, 'utf8');
  let at = 0;
  for (const token of parsed.tokens) {
    const range = token.span?.byteRange;
    if (!range || !Number.isSafeInteger(range.start) || !Number.isSafeInteger(range.end)
      || typeof token.text !== 'string' || range.start !== at || range.end < at || range.end > bytes.length
      || !Buffer.from(token.text, 'utf8').equals(bytes.subarray(at, range.end))) {
      throw new TypeError('C CST tokens must preserve every original source byte in order');
    }
    at = range.end;
  }
  if (at !== bytes.length) throw new TypeError('C CST token coverage is incomplete');
  const state = new Lowering(parsed, bytes, options);
  return state.lower();
}

export function archiveRestrictedCLowering(unit) {
  return encodeStructuredDataLinks(unit);
}

export function readRestrictedCLowering(network, root) {
  const value = decodeStructuredDataLinks(network, root);
  if (value?.kind !== 'restricted C source lowering' || value.checkedPortableProgram !== null) {
    throw new TypeError('this prototype archive is lowered C data, not a checked portable Program');
  }
  const tokens = value.sourceCst?.tokens;
  if (!Array.isArray(tokens) || !value.loweringOptions || typeof value.loweringOptions !== 'object') {
    throw new TypeError('the source-only C archive requires its complete CST and recorded lowering inputs');
  }
  const source = tokens.map(token => token.text).join('');
  const bytes = Buffer.from(source, 'utf8');
  if (bytes.length !== value.sourceBytes || createHash('sha256').update(bytes).digest('hex') !== value.sourceSha256) {
    throw new TypeError('the source-only C archive source bytes differ from their receipt');
  }
  const replay = lowerRestrictedCst(value.sourceCst, source, value.loweringOptions);
  if (!isDeepStrictEqual(replay, value)) {
    throw new TypeError('the source-only C archive differs from complete actual source lowering');
  }
  if (!sameSourceDataGraph(value, replay)) {
    throw new TypeError('the source-only C archive differs from complete actual source lowering owner relationships');
  }
  return value;
}

class Lowering {
  constructor(parsed, bytes, options) {
    this.parsed = parsed;
    this.bytes = bytes;
    this.options = options;
    this.abi = options.abi ?? null;
    this.types = new Map();
    this.records = new Map();
    this.enums = [];
    this.signatures = new Map();
    this.functionNodes = new Map();
    this.globals = new Map();
    this.obligations = [];
    this.temporary = 0;
    this.binding = 0;
    this.scope = [];
    this.functionResult = voidType;
    this.loopDepth = 0;
    for (const [name, type] of Object.entries(options.types ?? {})) this.types.set(name, type);
  }

  span(node) {
    const value = node?.span?.byteRange;
    if (!value || !Number.isSafeInteger(value.start) || !Number.isSafeInteger(value.end)
      || value.start < 0 || value.end < value.start || value.end > this.bytes.length) {
      throw new TypeError('C CST nodes require actual UTF-8 source byte ranges');
    }
    return {start: value.start, end: value.end};
  }

  text(node) { const {start, end} = this.span(node); return this.bytes.subarray(start, end).toString('utf8'); }
  children(node) { return (node.children ?? []).filter(child => child.named && !child.flags?.isExtra && child.term !== 'comment'); }
  fields(node, name) { return (node.children ?? []).filter(child => child.field === name); }
  field(node, name, optional = false) {
    const values = this.fields(node, name);
    if (values.length === 1) return values[0];
    if (values.length === 0 && optional) return null;
    throw new TypeError(`${node.term} requires exactly one actual ${name} CST field`);
  }
  obligation(category, node, detail) {
    const record = {category, span: this.span(node), detail};
    this.obligations.push(record);
    return record;
  }
  emit(out, operation, fields, node) {
    const instruction = {operation, ...fields, span: this.span(node)};
    out.push(instruction);
    return instruction;
  }
  temporaryValue(out, operation, fields, type, node) {
    const result = {temporary: ++this.temporary, type};
    this.emit(out, operation, {...fields, result}, node);
    return result;
  }
  integer(bits, signed) { return {kind: 'C integer', bits, signed}; }
  intType(node) {
    if (!Number.isSafeInteger(this.abi?.intBits) || this.abi.intBits < 16 || this.abi.intBits > 64) {
      this.obligation('C ABI', node, 'an explicit C int width is required');
      return unknownType('C int ABI');
    }
    return this.integer(this.abi.intBits, true);
  }
  convert(out, value, type, reason, node) {
    return this.temporaryValue(out, 'convert C value', {value, target: type, reason,
      conversion: type.kind === 'C integer' ? {bits: type.bits, signed: type.signed,
        rule: type.signed ? 'signed representability requires proof' : 'modulo two to bits'}
        : {rule: type.kind === 'C boolean' ? 'C scalar truth conversion' : 'declared C conversion'}}, type, node);
  }
  promote(out, value, node) {
    if (value.type.kind === 'C integer' && value.type.bits < (this.abi?.intBits ?? 0)) {
      return this.convert(out, value, this.intType(node), 'C integer promotion', node);
    }
    return value;
  }
  truth(out, value, node) { return this.convert(out, value, booleanType, 'C controlling expression', node); }
  qualified(type, node) {
    const qualifiers = this.children(node).filter(child => child.term === 'type_qualifier').map(child => this.text(child));
    return qualifiers.length ? {kind: 'C qualified', qualifiers, type} : type;
  }
  baseType(node) { return this.qualified(this.type(this.field(node, 'type')), node); }
  hasEffect(node) {
    if (['call_expression', 'update_expression', 'assignment_expression'].includes(node.term)) return true;
    return (node.children ?? []).some(child => this.hasEffect(child));
  }

  type(node) {
    if (!node) return unknownType('missing source type');
    if (node.term === 'type_descriptor') {
      const base = this.baseType(node);
      const declarator = this.field(node, 'declarator', true);
      return declarator ? this.declarator(declarator, base).type : base;
    }
    if (node.term === 'struct_specifier' || node.term === 'structure_specifier') {
      const nameNode = this.field(node, 'name', true), body = this.field(node, 'body', true);
      const identity = nameNode ? `struct:${this.text(nameNode)}` : `struct-at:${this.span(node).start}`;
      if (this.records.has(identity)) return {kind: 'C record', identity};
      const fields = [];
      this.records.set(identity, {identity, fields, span: this.span(node)});
      if (!body) this.obligation('C record layout', node, `the declaration of ${identity} has no field body`);
      else for (const declaration of this.children(body)) {
        if (declaration.term !== 'field_declaration') {
          this.obligation('C declaration form', declaration, declaration.term); continue;
        }
        const base = this.baseType(declaration);
        for (const item of this.fields(declaration, 'declarator')) {
          const field = this.declarator(item, base);
          if (!field.name) this.obligation('C field name', item, 'a named field is required');
          else fields.push({name: field.name, type: field.type, span: this.span(item)});
        }
      }
      return {kind: 'C record', identity};
    }
    if (node.term === 'enum_specifier' || node.term === 'enumeration_specifier') {
      const body = this.field(node, 'body', true), entries = [];
      let next = 0;
      if (!body) { this.obligation('C enum definition', node, 'an enum body is required'); return unknownType('enum body'); }
      for (const item of this.children(body)) {
        if (item.term !== 'enumerator') continue;
        const name = this.text(this.field(item, 'name'));
        const explicit = this.field(item, 'value', true);
        if (explicit) {
          this.obligation('C constant evaluation', explicit, 'explicit enum constant evaluation is not implemented');
          next = null;
        }
        const bindingOwner = {identity: `enum:${name}`, name, type: this.intType(item), constant: next, span: this.span(item)};
        entries.push({name, value: next, span: this.span(item), bindingOwner});
        this.globals.set(name, bindingOwner);
        if (next !== null) next += 1;
      }
      this.enums.push({span: this.span(node), entries});
      return this.intType(node);
    }
    const name = this.text(node).trim();
    if (this.types.has(name)) return this.types.get(name);
    if (name === 'void') return voidType;
    if (name === 'bool' || name === '_Bool') return booleanType;
    if (/^uint(?:8|16|32|64)_t$/u.test(name)) return this.integer(Number(name.slice(4, -2)), false);
    if (/^int(?:8|16|32|64)_t$/u.test(name)) return this.integer(Number(name.slice(3, -2)), true);
    if (name === 'int') return this.intType(node);
    if (name === 'char' && typeof this.abi?.charSigned === 'boolean') return this.integer(8, this.abi.charSigned);
    if (name === 'size_t' && Number.isSafeInteger(this.abi?.sizeBits)) return this.integer(this.abi.sizeBits, false);
    this.obligation('C type declaration', node, name);
    return unknownType(name);
  }

  declarator(node, base) {
    if (['identifier', 'field_identifier', 'type_identifier'].includes(node.term)) return {name: this.text(node), type: base};
    if (node.term === 'pointer_declarator' || node.term === 'abstract_pointer_declarator') {
      const child = this.field(node, 'declarator', true);
      const pointer = this.qualified(pointerTo(base), node);
      return child ? this.declarator(child, pointer) : {name: null, type: pointer};
    }
    if (node.term === 'array_declarator') {
      this.obligation('C memory layout', node, 'C arrays require an explicit layout and allocation model');
      return this.declarator(this.field(node, 'declarator'), {kind: 'C array', element: base,
        sizeSource: this.field(node, 'size', true) ? this.span(this.field(node, 'size')) : null});
    }
    if (node.term === 'function_declarator') {
      const child = this.declarator(this.field(node, 'declarator'), base);
      const list = this.field(node, 'parameters');
      const parameters = this.children(list).filter(item => item.term === 'parameter_declaration').map(item => {
        const type = this.baseType(item);
        const declaration = this.field(item, 'declarator', true);
        return declaration ? this.declarator(declaration, type) : {name: null, type};
      }).filter(item => item.type.kind !== 'C void');
      return {name: child.name, type: {kind: 'C function', result: child.type, parameters}};
    }
    this.obligation('C declarator form', node, node.term);
    return {name: null, type: unknownType(node.term)};
  }

  lookup(name) {
    for (let index = this.scope.length - 1; index >= 0; index -= 1) if (this.scope[index].has(name)) return this.scope[index].get(name);
    return this.globals.get(name) ?? null;
  }
  bind(name, type, node) {
    const scope = this.scope.at(-1);
    if (scope.has(name)) throw new TypeError(`duplicate C binding ${name}`);
    const record = {identity: `binding:${++this.binding}`, name, type, span: this.span(node)};
    scope.set(name, record);
    return record;
  }

  lvalue(node, out) {
    if (node.term === 'parenthesized_expression') return this.lvalue(this.onlyExpression(node), out);
    if (node.term === 'identifier') {
      const binding = this.lookup(this.text(node));
      if (!binding) {
        this.obligation('C binding', node, this.text(node));
        return this.temporaryValue(out, 'unresolved C address', {}, pointerTo(unknownType('binding')), node);
      }
      if (Object.hasOwn(binding, 'constant')) {
        this.obligation('C enum constant lvalue', node, 'an enumerator is a value, not an addressable or assignable C object');
        return this.temporaryValue(out, 'unresolved C address', {sourceEnumerator: binding.identity, sourceEnumeratorOwner: binding}, pointerTo(binding.type), node);
      }
      return this.temporaryValue(out, 'address C binding', {binding: binding.identity, bindingOwner: binding}, pointerTo(binding.type), node);
    }
    if (node.term === 'field_expression') {
      const operator = this.text(this.field(node, 'operator'));
      const argument = this.field(node, 'argument');
      const base = operator === '->' ? this.expression(argument, out) : this.lvalue(argument, out);
      const baseType = unqualified(base.type);
      const owner = baseType.kind === 'C pointer' ? baseType.to : unknownType('field owner');
      const name = this.text(this.field(node, 'field'));
      const recordType = unqualified(owner);
      const record = recordType.kind === 'C record' ? this.records.get(recordType.identity) : null;
      let type = record?.fields.find(item => item.name === name)?.type ?? unknownType(`field ${name}`);
      if (owner.kind === 'C qualified') type = {kind: 'C qualified', qualifiers: owner.qualifiers, type};
      this.obligation('C memory addressing', node, `typed ${operator} field address requires memory/layout primitives`);
      return this.temporaryValue(out, 'address C field', {base, owner, field: name, operator}, pointerTo(type), node);
    }
    if (node.term === 'subscript_expression') {
      const base = this.expression(this.field(node, 'argument'), out);
      const index = this.expression(this.field(node, 'index'), out);
      const baseType = unqualified(base.type);
      const element = baseType.kind === 'C pointer' ? baseType.to : baseType.element ?? unknownType('array element');
      if (this.hasEffect(this.field(node, 'argument')) || this.hasEffect(this.field(node, 'index'))) {
        this.obligation('C operand sequencing', node, 'index/base side effects require a disjointness proof before selecting an evaluation order');
      }
      this.obligation('C memory addressing', node, 'typed indexed pointer/array address requires bounds/layout primitives');
      return this.temporaryValue(out, 'address C element', {base, index}, pointerTo(element), node);
    }
    if (node.term === 'pointer_expression' && this.text(this.field(node, 'operator')) === '*') {
      const pointer = this.expression(this.field(node, 'argument'), out);
      this.obligation('C memory addressing', node, 'dereference requires a declared pointer memory model');
      return pointer;
    }
    this.obligation('C lvalue form', node, node.term);
    return this.temporaryValue(out, 'unresolved C address', {}, pointerTo(unknownType(node.term)), node);
  }

  onlyExpression(node) {
    const values = this.children(node).filter(child => expressionTerms.has(child.term));
    if (values.length !== 1) throw new TypeError(`${node.term} must contain one actual C expression`);
    return values[0];
  }
  load(out, address, node) {
    const sourceType = unqualified(address.type).to ?? unknownType('load');
    if (sourceType.kind === 'C qualified' && sourceType.qualifiers.includes('volatile')) {
      this.obligation('C volatile access', node, 'volatile memory events require an explicit operational primitive');
    }
    return this.temporaryValue(out, 'load C lvalue', {address, sourceType}, unqualified(sourceType), node);
  }
  store(out, address, value, node) {
    const sourceType = unqualified(address.type).to ?? unknownType('store');
    if (sourceType.kind === 'C qualified' && sourceType.qualifiers.includes('const')) {
      this.obligation('C const write', node, 'a const-qualified lvalue cannot receive an ordinary C store');
    }
    const converted = this.convert(out, value, unqualified(sourceType), 'C assignment conversion', node);
    this.emit(out, 'store C lvalue', {address, value: converted}, node);
    return converted;
  }
  commonArithmetic(out, left, right, node) {
    left = this.promote(out, left, node); right = this.promote(out, right, node);
    if (sameType(left.type, right.type)) return {left, right, type: left.type};
    this.obligation('C usual arithmetic conversions', node, 'mixed arithmetic types need the complete selected C ABI conversion rules');
    return {left, right, type: unknownType('usual arithmetic conversions')};
  }

  expression(node, out) {
    if (node.term === 'parenthesized_expression') return this.expression(this.onlyExpression(node), out);
    if (node.term === 'number_literal') {
      const text = this.text(node);
      if (!/^(?:0|[1-9][0-9]*|0[xX][0-9a-fA-F]+)$/u.test(text)
        || BigInt(text) >= 2n ** BigInt((this.abi?.intBits ?? 32) - 1)) {
        this.obligation('C integer literal type', node, 'suffixes/large/other literal forms require source type-selection rules');
        return this.temporaryValue(out, 'unresolved C literal', {spelling: text}, unknownType('literal'), node);
      }
      return this.temporaryValue(out, 'C integer literal', {value: BigInt(text).toString(), spelling: text}, this.intType(node), node);
    }
    if (node.term === 'char_literal' || node.term === 'character_literal') {
      const spelling = this.text(node), inside = spelling.slice(1, -1);
      const escapes = {'\\n': 10, '\\r': 13, '\\t': 9, '\\0': 0, '\\\\': 92, "\\'": 39, '\\"': 34};
      let value = escapes[inside];
      if (value === undefined && spelling.startsWith("'") && [...inside].length === 1 && inside.codePointAt(0) <= 127) value = inside.codePointAt(0);
      if (value === undefined) this.obligation('C character literal', node, 'only single ASCII scalar/basic escape literals are lowered in this prototype');
      return this.temporaryValue(out, value === undefined ? 'unresolved C literal' : 'C integer literal',
        value === undefined ? {spelling} : {value: String(value), spelling}, this.intType(node), node);
    }
    if (node.term === 'identifier' && ['true', 'false'].includes(this.text(node))) {
      this.obligation('C preprocessing constant', node, 'true/false macro or keyword semantics require an explicit selected C source environment');
      return this.temporaryValue(out, 'unresolved C literal', {spelling: this.text(node)}, unknownType('preprocessing constant'), node);
    }
    if (node.term === 'true' || node.term === 'false') {
      return this.temporaryValue(out, 'C boolean literal', {value: node.term === 'true'}, booleanType, node);
    }
    if (node.term === 'identifier') {
      const binding = this.lookup(this.text(node));
      if (binding && Object.hasOwn(binding, 'constant')) {
        const fields = {spelling: this.text(node), sourceEnumerator: binding.identity, sourceEnumeratorOwner: binding};
        if (Number.isSafeInteger(binding.constant)) {
          return this.temporaryValue(out, 'C integer literal', {...fields, value: String(binding.constant)}, binding.type, node);
        }
        this.obligation('C constant evaluation', node, 'the referenced enumerator has no lowered constant value');
        return this.temporaryValue(out, 'unresolved C constant', fields, binding.type, node);
      }
    }
    if (node.term === 'identifier' && this.signatures.has(this.text(node))) {
      const name = this.text(node);
      return this.temporaryValue(out, 'C function reference', {name}, this.signatures.get(name), node);
    }
    if (['identifier', 'field_expression', 'subscript_expression'].includes(node.term)) return this.load(out, this.lvalue(node, out), node);
    if (node.term === 'pointer_expression') {
      const operator = this.text(this.field(node, 'operator'));
      if (operator === '&') return this.lvalue(this.field(node, 'argument'), out);
      return this.load(out, this.lvalue(node, out), node);
    }
    if (node.term === 'assignment_expression') {
      const address = this.lvalue(this.field(node, 'left'), out);
      const operator = this.text(this.field(node, 'operator'));
      const before = operator === '=' ? null : this.load(out, address, node);
      const right = this.expression(this.field(node, 'right'), out);
      if (this.hasEffect(this.field(node, 'left')) || this.hasEffect(this.field(node, 'right'))) {
        this.obligation('C operand sequencing', node, 'assignment address/read and RHS effects need the actual C sequencing and alias proof');
      }
      if (operator === '=') return this.store(out, address, right, node);
      const common = this.commonArithmetic(out, before, right, node);
      const result = this.temporaryValue(out, 'C binary operation', {operator: operator.slice(0, -1),
        left: common.left, right: common.right}, common.type, node);
      return this.store(out, address, result, node);
    }
    if (node.term === 'update_expression') {
      const address = this.lvalue(this.field(node, 'argument'), out);
      const before = this.load(out, address, node);
      const one = this.temporaryValue(out, 'C integer literal', {value: '1', spelling: '1'}, this.intType(node), node);
      const common = this.commonArithmetic(out, before, one, node);
      const operatorNode = this.field(node, 'operator');
      const operator = this.text(operatorNode) === '++' ? '+' : '-';
      const value = this.temporaryValue(out, 'C binary operation', {operator, left: common.left, right: common.right}, common.type, node);
      const stored = this.store(out, address, value, node);
      return this.span(operatorNode).start < this.span(this.field(node, 'argument')).start ? stored : before;
    }
    if (node.term === 'binary_expression') {
      const operator = this.text(this.field(node, 'operator'));
      const left = this.expression(this.field(node, 'left'), out);
      if (operator === '&&' || operator === '||') {
        const condition = this.truth(out, left, node), rightCode = [], fixedCode = [];
        const rightTruth = this.truth(rightCode, this.expression(this.field(node, 'right'), rightCode), node);
        const right = this.convert(rightCode, rightTruth, this.intType(node), 'C logical result zero or one', node);
        const fixed = this.temporaryValue(fixedCode, 'C integer literal', {value: operator === '||' ? '1' : '0', spelling: operator === '||' ? '1' : '0'}, this.intType(node), node);
        return this.temporaryValue(out, 'C short circuit', {operator, condition,
          whenTrue: operator === '&&' ? {instructions: rightCode, value: right} : {instructions: fixedCode, value: fixed},
          whenFalse: operator === '&&' ? {instructions: fixedCode, value: fixed} : {instructions: rightCode, value: right}}, this.intType(node), node);
      }
      if (this.hasEffect(this.field(node, 'left')) || this.hasEffect(this.field(node, 'right'))) {
        this.obligation('C operand sequencing', node, 'ordinary C binary operands are not generally sequenced left to right');
      }
      const right = this.expression(this.field(node, 'right'), out);
      const common = this.commonArithmetic(out, left, right, node);
      const type = ['==', '!=', '<', '>', '<=', '>='].includes(operator) ? this.intType(node) : common.type;
      return this.temporaryValue(out, 'C binary operation', {operator, left: common.left, right: common.right}, type, node);
    }
    if (node.term === 'unary_expression') {
      const operator = this.text(this.field(node, 'operator'));
      const original = this.expression(this.field(node, 'argument'), out);
      const value = operator === '!' ? this.truth(out, original, node) : this.promote(out, original, node);
      return this.temporaryValue(out, 'C unary operation', {operator, value}, operator === '!' ? this.intType(node) : value.type, node);
    }
    if (node.term === 'cast_expression') {
      const target = this.type(this.field(node, 'type'));
      return this.convert(out, this.expression(this.field(node, 'value'), out), target, 'explicit C cast', node);
    }
    if (node.term === 'call_expression') {
      const calleeNode = this.field(node, 'function'), name = calleeNode.term === 'identifier' ? this.text(calleeNode) : null;
      let callee;
      if (name && !this.signatures.has(name)) {
        const primitive = this.options.primitives?.[name];
        if (primitive?.kind !== 'C function') this.obligation('C external primitive', calleeNode, name);
        callee = this.temporaryValue(out, 'C primitive reference', {name, declaredSignature: primitive ?? null}, primitive ?? unknownType(name), calleeNode);
      } else callee = this.expression(calleeNode, out);
      const argumentsNode = this.field(node, 'arguments');
      if (this.children(argumentsNode).some(argument => this.hasEffect(argument))) {
        this.obligation('C operand sequencing', node, 'argument/callee side effects require a C evaluation-order proof');
      }
      const args = this.children(argumentsNode).map(argument => this.expression(argument, out));
      const signature = callee.type.kind === 'C function' ? callee.type : callee.type.to?.kind === 'C function' ? callee.type.to : null;
      if (!signature) this.obligation('C indirect primitive signature', calleeNode, 'indirect call requires a declared typed function pointer');
      else if (signature.parameters.length !== args.length) this.obligation('C call arity', node, 'source arguments do not match the declared signature');
      const converted = args.map((argument, index) => signature?.parameters[index]
        ? this.convert(out, argument, signature.parameters[index].type, 'C parameter conversion', node) : argument);
      return this.temporaryValue(out, 'C call', {callee, arguments: converted,
        sourceFunction: name && this.functionNodes.has(name) ? name : null}, signature?.result ?? unknownType('call result'), node);
    }
    if (node.term === 'conditional_expression') {
      const condition = this.truth(out, this.expression(this.field(node, 'condition'), out), node);
      const trueCode = [], falseCode = [];
      const whenTrue = this.expression(this.field(node, 'consequence'), trueCode);
      const whenFalse = this.expression(this.field(node, 'alternative'), falseCode);
      if (!sameType(whenTrue.type, whenFalse.type)) this.obligation('C conditional type', node, 'branch conversions require usual C conditional rules');
      return this.temporaryValue(out, 'C conditional value', {condition,
        whenTrue: {instructions: trueCode, value: whenTrue}, whenFalse: {instructions: falseCode, value: whenFalse}}, whenTrue.type, node);
    }
    this.obligation('C expression form', node, node.term);
    return this.temporaryValue(out, 'unresolved C expression', {term: node.term}, unknownType(node.term), node);
  }

  declaration(node, out, global = false) {
    const base = this.baseType(node);
    for (const item of this.fields(node, 'declarator')) {
      const initialized = item.term === 'init_declarator' || item.term === 'initialized_declarator';
      const own = initialized ? this.field(item, 'declarator') : item;
      const declared = this.declarator(own, base);
      if (!declared.name) { this.obligation('C declaration name', node, 'a name is required'); continue; }
      if (declared.type.kind === 'C function') { this.signatures.set(declared.name, declared.type); continue; }
      const binding = global ? {identity: `global:${declared.name}`, name: declared.name, type: declared.type, span: this.span(item)}
        : this.bind(declared.name, declared.type, item);
      if (global) this.globals.set(declared.name, binding);
      this.emit(out, 'declare C binding', {binding}, node);
      if (initialized) {
        const address = this.temporaryValue(out, 'address C binding', {binding: binding.identity, bindingOwner: binding}, pointerTo(binding.type), item);
        this.store(out, address, this.expression(this.field(item, 'value'), out), item);
      }
    }
  }

  statement(node, out) {
    switch (node.term) {
      case 'compound_statement':
        this.scope.push(new Map());
        for (const child of this.children(node)) this.statement(child, out);
        this.scope.pop(); return;
      case 'declaration': this.declaration(node, out); return;
      case 'expression_statement': {
        const expressions = this.children(node);
        if (expressions.length === 1) this.expression(expressions[0], out);
        else if (expressions.length !== 0) this.obligation('C statement expression', node, 'one expression is required');
        return;
      }
      case 'return_statement': {
        const values = this.children(node);
        const value = values.length === 1 ? this.convert(out, this.expression(values[0], out), this.functionResult, 'C return conversion', node) : null;
        if (values.length > 1 || (!value && this.functionResult.kind !== 'C void')) this.obligation('C return form', node, 'return value does not match the source result type');
        this.emit(out, 'return from C call', {value}, node); return;
      }
      case 'if_statement': {
        const condition = this.truth(out, this.expression(this.field(node, 'condition'), out), node);
        const whenTrue = [], whenFalse = [];
        this.statement(this.field(node, 'consequence'), whenTrue);
        const alternative = this.field(node, 'alternative', true);
        if (alternative) this.statement(alternative, whenFalse);
        this.emit(out, 'C branch', {condition, whenTrue, whenFalse}, node); return;
      }
      case 'else_clause': {
        const values = this.children(node);
        if (values.length !== 1) throw new TypeError('a C else clause requires one statement');
        this.statement(values[0], out); return;
      }
      case 'while_statement': {
        const conditionCode = [], body = [];
        const condition = this.truth(conditionCode, this.expression(this.field(node, 'condition'), conditionCode), node);
        this.loopDepth += 1; this.statement(this.field(node, 'body'), body); this.loopDepth -= 1;
        this.emit(out, 'C while', {condition: {instructions: conditionCode, value: condition}, body}, node); return;
      }
      case 'for_statement': {
        this.scope.push(new Map());
        const initializer = this.field(node, 'initializer', true);
        if (initializer?.term === 'declaration') this.declaration(initializer, out);
        else if (initializer) this.expression(initializer, out);
        const conditionCode = [], body = [], update = [];
        const conditionNode = this.field(node, 'condition', true);
        const condition = conditionNode ? this.truth(conditionCode, this.expression(conditionNode, conditionCode), conditionNode)
          : this.temporaryValue(conditionCode, 'C boolean literal', {value: true}, booleanType, node);
        const updateNode = this.field(node, 'update', true);
        if (updateNode) this.expression(updateNode, update);
        this.loopDepth += 1; this.statement(this.field(node, 'body'), body); this.loopDepth -= 1;
        this.emit(out, 'C for', {condition: {instructions: conditionCode, value: condition}, body, update}, node);
        this.scope.pop(); return;
      }
      case 'break_statement': case 'continue_statement':
        if (this.loopDepth === 0) this.obligation('C control target', node, 'no enclosing loop exists in this prototype');
        this.emit(out, node.term === 'break_statement' ? 'break C loop' : 'continue C loop', {}, node); return;
      default:
        this.obligation('C statement form', node, node.term);
        this.emit(out, 'unresolved C statement', {term: node.term}, node);
    }
  }

  sourceData(value) { return copyRestrictedCSourceData(value); }

  lower() {
    const pending = [this.parsed.tree];
    while (pending.length) {
      const node = pending.pop();
      if (node.flags?.isError || node.flags?.isMissing || node.flags?.hasError) throw new TypeError('recovered C CSTs cannot be silently admitted by lowering');
      pending.push(...(node.children ?? []));
    }
    const globalInitializers = [];
    for (const node of this.children(this.parsed.tree)) {
      if (node.term === 'type_definition') {
        const base = this.baseType(node);
        for (const child of this.fields(node, 'declarator')) {
          const declaration = this.declarator(child, base);
          if (declaration.name) this.types.set(declaration.name, declaration.type);
        }
      } else if (node.term === 'function_definition') {
        const signature = this.declarator(this.field(node, 'declarator'), this.baseType(node));
        if (!signature.name || signature.type.kind !== 'C function') throw new TypeError('a C function definition needs an actual function declarator');
        if (this.functionNodes.has(signature.name)) throw new TypeError('duplicate C function definition');
        if (Object.hasOwn(this.options.primitives ?? {}, signature.name)) throw new TypeError('a declared source function cannot be replaced by a primitive adapter');
        this.signatures.set(signature.name, signature.type); this.functionNodes.set(signature.name, node);
      } else if (node.term === 'declaration') this.declaration(node, globalInitializers, true);
      else this.obligation('C top-level form', node, node.term);
    }
    const roots = this.options.roots ?? [...this.functionNodes.keys()];
    const selected = new Set(), queue = [...roots];
    while (queue.length) {
      const name = queue.shift();
      if (selected.has(name)) continue;
      const node = this.functionNodes.get(name);
      if (!node) throw new TypeError(`unknown source entry function ${name}`);
      selected.add(name);
      const nodes = [this.field(node, 'body')];
      while (nodes.length) {
        const child = nodes.pop();
        if (child.term === 'call_expression') {
          const callee = this.field(child, 'function');
          if (callee.term === 'identifier' && this.functionNodes.has(this.text(callee))) queue.push(this.text(callee));
        }
        nodes.push(...(child.children ?? []));
      }
    }
    const functions = [];
    for (const [name, node] of this.functionNodes) {
      if (!selected.has(name)) continue;
      const signature = this.signatures.get(name), body = [];
      this.scope = [new Map()]; this.functionResult = signature.result;
      const parameters = signature.parameters.map(parameter => parameter.name
        ? this.bind(parameter.name, parameter.type, node) : parameter);
      this.statement(this.field(node, 'body'), body);
      functions.push({name, signature, parameters, body, span: this.span(node)});
    }
    this.obligation('C execution boundary', this.parsed.tree,
      'typed C instructions are inspectable lowering data; C memory, ABI conversions, sequencing and execution are not checked portable Program semantics');
    this.obligation('C syntax derivation', this.parsed.tree,
      'the supplied CST is an explicit input assumption; source-only replay does not authenticate its derivation from the source grammar');
    return this.sourceData({kind: 'restricted C source lowering', sourceSha256: createHash('sha256').update(this.bytes).digest('hex'),
      sourceBytes: this.bytes.length, abi: this.abi, roots: [...roots],
      sourceDenominators: this.options.sourceDenominators ?? null,
      sourceCst: this.parsed, loweringOptions: this.options, records: [...this.records.values()], enums: this.enums,
      globalInitializers, functions,
      unselectedSourceFunctions: [...this.functionNodes].filter(([name]) => !selected.has(name)).map(([name, node]) => ({name, span: this.span(node)})),
      obligations: this.obligations, checkedPortableProgram: null});
  }
}
