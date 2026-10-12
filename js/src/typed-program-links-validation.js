// Validate the source-free checked IR boundary before any target emission.
import { fixedBounds, sameType } from './translation/types.js';
import { acceptProgramDivisionRounding, acceptProgramExpressionOperation, acceptProgramMachineIntegerWidth, acceptProgramOptionalAbsence } from './typed-program-link-rules.js';

const primitives = new Set(['nat', 'int', 'bool', 'string', 'unit', 'float', 'output']);
const operationKinds = new Set(['math', 'stringMap', 'stringTest', 'method', 'collectionRead', 'collectionItems', 'optionalPredicate']);

export function validateLinkedTypedProgram(program, references = new WeakMap()) {
  if (!program || program.schemaVersion !== 1 || !['JavaScript', 'Rust', 'Lean', 'Rocq'].includes(program.sourceLanguage)
    || !(program.declarations instanceof Map) || !Array.isArray(program.items)) throw new TypeError('the linked value is not a checked typed program');
  const active = new Set();
  const visited = new Set();
  function bounded(value, depth = 0) {
    if (!value || typeof value !== 'object') {
      if (typeof value === 'function' || typeof value === 'symbol') throw new TypeError('typed programs contain data, not host callbacks');
      return;
    }
    if (active.has(value)) throw new TypeError('the checked program contains a structural cycle');
    if (visited.has(value)) return;
    if (depth > 256 || visited.size >= 100_000) throw new TypeError('the checked program exceeds its structural resource limit');
    visited.add(value); active.add(value);
    const values = value instanceof Map ? [...value].flat() : Object.values(value);
    for (const child of values) bounded(child, depth + 1);
    active.delete(value);
  }
  bounded(program);
  // Shared records can describe exponentially many expression paths. Bound
  // the semantic walk as well as the number of unique structural records.
  let semanticVisits = 0;
  function visit() {
    if (++semanticVisits > 100_000) throw new TypeError('the linked program exceeds its semantic resource limit');
  }
  function type(value) {
    visit();
    if (!value || typeof value !== 'object' || typeof value.kind !== 'string') throw new TypeError('the linked program contains a malformed type');
    if (primitives.has(value.kind)) return;
    if (value.kind === 'fixed') {
      if (!Number.isSafeInteger(value.bits) || !acceptProgramMachineIntegerWidth(String(value.bits)) || typeof value.signed !== 'boolean') throw new TypeError('the linked program contains an invalid machine integer type');
    } else if (value.kind === 'data') {
      if (typeof value.name !== 'string' || !value.name) throw new TypeError('a linked data type has no name');
    } else if (['array', 'set', 'option', 'cell'].includes(value.kind)) {
      type(value.element);
      if (value.kind === 'option' && !acceptProgramOptionalAbsence(value.absence ?? 'both')) throw new TypeError('a linked optional type has an invalid absence policy');
    } else if (value.kind === 'tuple') value.elements.forEach(type);
    else if (value.kind === 'record') value.fields.forEach((field) => type(field.type));
    else if (value.kind === 'map') { type(value.key); type(value.value); }
    else if (value.kind === 'function') { value.params.forEach(type); type(value.ret); }
    else throw new TypeError(`the linked program contains unsupported type ${value.kind}`);
  }
  const declarations = new Map(program.declarations);
  for (const external of program.externals ?? []) declarations.set(external.name, external);
  for (const [name, entry] of program.declarations) {
    if (!entry || entry.fullName !== name || typeof entry.name !== 'string' || !Array.isArray(entry.modulePath)) throw new TypeError('the linked declaration mapping is inconsistent');
    if (entry.k === 'fn') { entry.params.forEach((parameter) => type(parameter.type)); type(entry.ret); }
    if (entry.k === 'const') type(entry.type);
  }
  const same = (left, right, detail) => {
    type(left); type(right);
    if (!sameType(left, right)) throw new TypeError(`the linked program has inconsistent ${detail}`);
  };
  function expression(node, environment) {
    if (!node || typeof node !== 'object') return;
    visit();
    if (operationKinds.has(node.k) && !acceptProgramExpressionOperation(node.k, node.op)) throw new TypeError('a linked expression operation is unsupported');
    if (node.k === 'ctor' && Object.hasOwn(node, 'frozen') && typeof node.frozen !== 'boolean') throw new TypeError('a linked constructor immutability flag must be a boolean');
    if (node.type && node.k) type(node.type);
    if (node.span && (!Number.isSafeInteger(node.span.start) || !Number.isSafeInteger(node.span.end)
      || node.span.start < 0 || node.span.end < node.span.start)) throw new TypeError('a linked source span is invalid');
    if (node.k === 'lit') {
      if (node.type.kind === 'bool' && typeof node.value !== 'boolean') throw new TypeError('a linked boolean literal is invalid');
      if (node.type.kind === 'string' && typeof node.value !== 'string') throw new TypeError('a linked string literal is invalid');
      if (node.type.kind === 'float' && (typeof node.value !== 'string' || !/^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:[eE][+-]?[0-9]+)?|NaN|Infinity|-Infinity)$/u.test(node.value))) throw new TypeError('a linked floating-point literal is invalid');
      if (['nat', 'int', 'fixed'].includes(node.type.kind)) {
        if (typeof node.value !== 'string' || !/^-?[0-9]+$/u.test(node.value)) throw new TypeError('a linked integer literal is invalid');
        const value = BigInt(node.value);
        if (node.type.kind === 'nat' && value < 0n) throw new TypeError('a linked natural literal is negative');
        if (node.type.kind === 'fixed') {
          const bounds = fixedBounds(node.type);
          if (value < bounds.min || value > bounds.max) throw new TypeError('a linked integer literal is out of range');
        }
      }
    }
    if (Array.isArray(node.effects)) {
      const inner = new Map(environment);
      for (const effect of node.effects) {
        if (effect.k === 'let') { expression(effect.value, inner); inner.set(effect.name, effect); }
        else expression(effect, inner);
      }
      return;
    }
    if (node.p === 'forall') {
      const inner = new Map(environment);
      for (const binder of node.binders) { type(binder.type); inner.set(binder.name, binder); }
      expression(node.body, inner);
      return;
    }
    if (node.k === 'var') {
      const binder = environment.get(node.name);
      if (!binder) throw new TypeError(`the linked variable ${node.name} is unbound`);
      if (references.get(node)?.target && references.get(node).target !== binder) throw new TypeError('the linked variable targets a declaration outside its scope');
      // A let expression has its body's result type; the variable is bound to
      // the initializer, including when an escaping closure captures a cell.
      same(node.type, binder.k === 'let' ? binder.value?.type : binder.type, 'variable type');
      return;
    }
    if (node.k === 'call' || node.k === 'functionRef') {
      const declaration = declarations.get(node.fn);
      if (!declaration) throw new TypeError(`the linked declaration reference ${node.fn} is unresolved`);
      if (references.get(node)?.target && references.get(node).target !== declaration) throw new TypeError('the linked declaration reference targets a different declaration');
      if (node.k === 'functionRef') {
        if (declaration.k === 'const') throw new TypeError('a linked function reference targets a constant');
        same(node.type, { kind: 'function', params: declaration.params.map((parameter) => parameter.type), ret: declaration.ret }, 'function reference type');
      } else {
        const arguments_ = node.args ?? [];
        if (declaration.k === 'const') {
          if (arguments_.length) throw new TypeError('a linked constant reference has arguments');
          same(node.type, declaration.type, 'constant reference type');
        } else {
          if (arguments_.length !== declaration.params.length) throw new TypeError('a linked call has the wrong number of arguments');
          arguments_.forEach((argument, index) => { expression(argument, environment); same(argument.type, declaration.params[index].type, 'call argument type'); });
          same(node.type, declaration.ret, 'call return type');
        }
      }
      return;
    }
    if (node.k === 'let') {
      expression(node.value, environment);
      expression(node.body, new Map(environment).set(node.name, node));
      same(node.type, node.body.type, 'let result type');
      return;
    }
    if (node.k === 'lambda') {
      const inner = new Map(environment);
      for (const parameter of node.params) { type(parameter.type); inner.set(parameter.name, parameter); }
      expression(node.body, inner);
      same(node.type, { kind: 'function', params: node.params.map((parameter) => parameter.type), ret: node.body.type }, 'lambda type');
      return;
    }
    if (node.k === 'apply') {
      if (node.fn.type?.kind !== 'function') throw new TypeError('a linked application does not target a function');
      if (node.args.length !== node.fn.type.params.length) throw new TypeError('a linked application has the wrong number of arguments');
      node.args.forEach((argument, index) => same(argument.type, node.fn.type.params[index], 'application argument type'));
      same(node.type, node.fn.type.ret, 'application return type');
    }
    if (node.k === 'cellAllocate') {
      if (node.type?.kind !== 'cell') throw new TypeError('a linked cell allocation does not produce a reference cell');
      same(node.value.type, node.type.element, 'cell initializer type');
    }
    if (node.k === 'cellRead' || node.k === 'cellWrite') {
      if (node.cell.type?.kind !== 'cell') throw new TypeError('a linked cell operation does not target a reference cell');
      if (node.k === 'cellRead') same(node.type, node.cell.type.element, 'cell read type');
      else {
        same(node.value.type, node.cell.type.element, 'cell write type');
        same(node.type, node.body.type, 'cell write result type');
      }
    }
    if (node.k === 'if') {
      same(node.cond.type, { kind: 'bool' }, 'conditional condition type');
      same(node.then.type, node.else.type, 'conditional branch types');
      same(node.type, node.then.type, 'conditional result type');
    }
    if (node.k === 'unary') {
      same(node.type, node.arg.type, 'unary result type');
      if (node.op === 'not') {
        if (node.type.kind !== 'bool') throw new TypeError('a linked logical negation has a nonboolean type');
      } else if (node.op === 'neg') {
        const policy = { float: 'ieee', int: 'exact', fixed: 'checked' }[node.type.kind];
        if (!policy || node.semantics !== policy || (node.type.kind === 'fixed' && !node.type.signed)) throw new TypeError('a linked negation has inconsistent numeric semantics');
      } else throw new TypeError('a linked unary operation has no checked semantics');
    }
    if (node.k === 'toString') {
      same(node.type, { kind: 'string' }, 'text conversion result type');
    }
    if (node.k === 'math') {
      const spread = node.op === 'maxOf' || node.op === 'minOf';
      const variadic = node.op === 'max' || node.op === 'min';
      if (!Array.isArray(node.args) || (!variadic && node.args.length !== 1)) throw new TypeError('a linked math operation has the wrong number of arguments');
      for (const argument of node.args) same(argument.type, spread ? { kind: 'array', element: { kind: 'float' } } : { kind: 'float' }, 'math argument type');
      same(node.type, { kind: acceptProgramExpressionOperation('optionalPredicate', node.op) ? 'bool' : 'float' }, 'math result type');
    }
    if (node.k === 'stringMap' || node.k === 'stringTest') {
      same(node.string.type, { kind: 'string' }, 'string operation receiver type');
      if (node.k === 'stringTest') same(node.search.type, { kind: 'string' }, 'string search type');
      same(node.type, { kind: node.k === 'stringMap' ? 'string' : 'bool' }, 'string operation result type');
    }
    if (node.k === 'method' && !['string', 'array'].includes(node.object.type?.kind)) throw new TypeError('a linked method has an unsupported receiver type');
    if (node.k === 'optionalPredicate') {
      if (node.arg.type?.kind !== 'option') throw new TypeError('a linked optional predicate requires an optional value');
      same(node.type, { kind: 'bool' }, 'optional predicate result type');
    }
    if (node.k === 'collectionRead' || node.k === 'collectionItems') {
      const object = node.object.type;
      if (!['map', 'set'].includes(object?.kind)) throw new TypeError('a linked collection operation has a noncollection receiver');
      const key = object.kind === 'map' ? object.key : object.element;
      const value = object.kind === 'map' ? object.value : key;
      if (node.k === 'collectionRead') {
        same(node.key.type, key, 'collection key type');
        if (node.op === 'get' && object.kind !== 'map') throw new TypeError('a linked collection lookup requires a map');
        same(node.type, node.op === 'has' ? { kind: 'bool' } : { kind: 'option', element: value, absence: 'undefined' }, 'collection read result type');
      } else {
        const element = node.op === 'keys' ? key : node.op === 'values' ? value : { kind: 'tuple', elements: [key, value] };
        same(node.type, { kind: 'array', element }, 'collection items result type');
      }
    }
    if (node.k === 'tupleIndex' || node.k === 'recordField' || node.k === 'optionalField') {
      if (!Number.isSafeInteger(node.index) || node.index < 0) throw new TypeError('a linked positional field index is invalid');
      const object = node.k === 'optionalField' ? node.object.type?.element : node.object.type;
      const entry = node.k === 'tupleIndex' ? object?.elements?.[node.index] : object?.fields?.[node.index];
      if (!entry || (node.k !== 'tupleIndex' && entry.name !== node.field)) throw new TypeError('a linked positional field index differs from its layout');
      same(node.type, node.k === 'tupleIndex' ? entry : node.k === 'optionalField'
        ? { kind: 'option', element: entry.type, absence: 'undefined' } : entry.type, 'positional field type');
    }
    if (node.k === 'binary') {
      if (node.rounding !== undefined && !acceptProgramDivisionRounding(node.rounding)) throw new TypeError('a linked division has an unsupported rounding policy');
      same(node.left.type, node.right.type, 'binary operand types');
      if (node.domain) same(node.domain, node.left.type, 'binary operation domain');
      const comparison = ['eq', 'ne', 'lt', 'le', 'gt', 'ge'].includes(node.op);
      if (comparison && node.type.kind !== 'bool') throw new TypeError('a linked comparison does not return a boolean');
      if (['add', 'sub', 'mul', 'div', 'rem'].includes(node.op)) {
        if (!['nat', 'int', 'float', 'fixed'].includes(node.domain?.kind)) throw new TypeError('a linked arithmetic operation has a nonnumeric domain');
        same(node.type, node.domain, 'binary result type');
        const policies = node.domain.kind === 'float' ? ['ieee'] : node.domain.kind === 'fixed' ? ['checked'] : ['exact', 'truncated'];
        if (!policies.includes(node.semantics)) throw new TypeError('a linked arithmetic operation has inconsistent numeric semantics');
      } else if (['and', 'or'].includes(node.op)) {
        if (node.left.type.kind !== 'bool' || node.type.kind !== 'bool') throw new TypeError('a linked logical operation has a nonboolean type');
      } else if (node.op === 'concat') {
        if (node.left.type.kind !== 'string' || node.type.kind !== 'string') throw new TypeError('a linked concatenation has a nonstring type');
      } else if (!comparison) throw new TypeError('a linked binary operation has no checked semantics');
    }
    if (node.k === 'match') {
      expression(node.scrutinee, environment);
      for (const kase of node.cases) {
        const inner = new Map(environment);
        if (kase.pattern.name) inner.set(kase.pattern.name, { type: node.scrutinee.type });
        for (const bind of kase.pattern.binds ?? []) if (bind) inner.set(bind, { type: findVariableType(kase.body, bind) });
        expression(kase.body, inner);
      }
      return;
    }
    for (const [key, value] of Object.entries(node)) {
      if (['type', 'domain', 'span', 'from', 'to'].includes(key)) continue;
      if (Array.isArray(value)) for (const child of value) expression(child, environment);
      else if (value && typeof value === 'object') expression(value, environment);
    }
  }
  function findVariableType(node, name) {
    if (!node || typeof node !== 'object') return undefined;
    visit();
    if (node.k === 'var' && node.name === name) return node.type;
    for (const value of Object.values(node)) {
      if (Array.isArray(value)) { for (const child of value) { const found = findVariableType(child, name); if (found) return found; } }
      else if (value && typeof value === 'object') { const found = findVariableType(value, name); if (found) return found; }
    }
    return undefined;
  }
  for (const entry of program.declarations.values()) {
    const environment = new Map((entry.params ?? []).map((parameter) => [parameter.name, parameter]));
    expression(entry.body ?? entry.value, environment);
    if (entry.k === 'fn') same(entry.body.type, entry.ret, 'function return type');
  }
  expression(program.main, new Map());
  return program;
}
