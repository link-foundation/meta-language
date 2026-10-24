// Types of JavaScript functions written without JSDoc. A parameter or result
// without a declared type gets a type variable; the bodies, the calls and the
// top-level statements constrain the variables by unification, and each
// variable takes the one type every use agrees on. A BigInt literal is a
// `bigint` of any integer type, `+` of a string is concatenation, and a type
// nothing constrains is a Number. Two uses that need different types are a
// type error: the function would need a declared type for each use.

import { typeError } from './diagnostics.js';
import { readJavaScriptRecordMethodInferenceStep, readJavaScriptPropertyCallFallbackKind, readJavaScriptSourcePropertyOperationKind } from './javascript-property-call-rules.js';
import { readJavaScriptNumberBitwiseArity } from './number-bitwise-rules.js';
import { readBigIntegerBitwiseArity } from './big-integer-bitwise-rules.js';
import { readArrayStateMethodResult, readArrayStateMethodArgumentRole } from './array-state-method-rules.js';
import { readReferenceMethodResult } from './reference-method-rules.js';
import { readCollectionAccessResult } from './collection-access-rules.js';
import { inferCollectionConstruction } from './collection-construction.js';
import { readArrayFromCallbackConstraints, readArrayFromInferenceInputs, readArrayFromIterableType } from './array-from.js';
import { readJavaScriptArrayDefaultConstraints } from './array-binding-inference.js';
import { checkJavaScriptArrayBindingWrapper } from './array-binding-validation.js';
import { readInferenceTermIdentity, readOptionalAdditionalAbsence, readOptionalCoalescingConstraints, readOptionalFieldInference, readOptionalJoinConstraints, readOptionalTypeConstraints } from './optional-inference.js';
import { mergeOptionalAbsence } from './structured-type-rules.js';
import { readOptionalBigIntConstraint, readOptionalCallBinding, readOptionalJoinBinding, readOptionalNumericTerm, requireInferenceJoinAlias, requireOptionalResultBinding } from './optional-value-rules.js';
import { readRecordFieldInsertionOrder, readRecordFieldPosition, readRecordSpreadRole, readRecordTypeConstraintError, readRecordTypeConstraintStep, readRecordFieldInferenceStep } from './record-value-rules.js';
import { containsRecordTypeOwner } from './record-type-facts.js';
import { constructCallableFunctionType, readCallableApplicationConstraints, readCallableCellOptionBinding, readCallableCellReadInference, readCallableFunctionConstraints, readCallableLambdaInference, readCallableTypeChildren } from './callable-source.js';
import { acceptCallableApplicationInferenceResult, requireCallableNamedInference, requireCallableStructuralInference } from './callable-source-rules.js';
import { acceptArgumentCount, acceptJavaScriptOptionalTupleContext } from './frontend-rules.js';
import { readParameterDefaultArgumentInference } from './parameter-default-rules.js';
import { BOOL, FLOAT, INT, STRING, UNIT, option } from './types.js';
import { markSharedReferenceTypes } from './javascript-cells.js';
import { readJavaScriptProductIndex, requiresJavaScriptProductProof } from './javascript-products.js';
import { acceptJavaScriptArrayTupleContext } from './product-source-rules.js';
import { readJavaScriptMethodReceiverKind, readJavaScriptMethodArgumentKind,
  readJavaScriptStringMethodResultForm, readJavaScriptIncludesReceiverForm,
  readJavaScriptEqualityInferenceStep } from './javascript-surface-rules.js';
import {readPrimitiveMethodMinimumArguments,readPrimitiveMethodArgumentCount,
  readPrimitiveMethodCheckedArgumentKind,readPrimitiveMethodCheckedResultForm} from './primitive-method-rules.js';

const ROOT = 'crate';
const LOGICAL = new Set(['and', 'or']);
const ORDER = new Set(['eq', 'ne', 'lt', 'le', 'gt', 'ge']);
const ARITHMETIC = new Set(['add', 'sub', 'mul', 'div', 'rem']);
const recordNameUnits = fields => fields.map(field => Array.from({ length: field.name.length }, (_, index) => field.name.charCodeAt(index)));

/** Fills in the missing parameter and result types of the functions of a parsed JavaScript program. */
export function inferJavaScriptTypes(program) {
  const functions = [];
  collectFunctions(program.items, [ROOT], functions);
  const untyped = program.items.some((item) => item.k === 'data' && item.ctors.some((ctor) => ctor.fields.some((field) => !field.type)));
  if (!untyped && functions.every(({ fn }) => fn.ret && fn.params.every((param) => param.type)) && !hasStructuralInference(program)) return markSharedReferenceTypes(program);
  const inference = new Inference(program.items, requiresJavaScriptProductProof(program));
  inference.run(functions, program.main, program.externals ?? []);
  if (inference.recordExpressionTypes) Object.defineProperty(program, 'inferredExpressionTypes', { value: inference.expressionTypes });
  return markSharedReferenceTypes(program);
}

function collectFunctions(items, path, into) {
  for (const item of items) {
    if (item.k === 'fn') into.push({ path: [...path, item.name], fn: item });
    else if (item.k === 'module') collectFunctions(item.items, [...path, item.name], into);
  }
}

function hasStructuralInference(node) {
  if (!node || typeof node !== 'object') return false;
  if (Array.isArray(node)) return node.some(hasStructuralInference);
  if (node.k === 'arrayBindingRest' || node.k === 'arrayBindingDefault') return true;
  if (requireCallableStructuralInference(node.k ?? '', node.kind ?? '', node.op ?? '')) return true;
  return Object.entries(node).some(([key, value]) => key !== 'span' && hasStructuralInference(value));
}

class Inference {
  constructor(items, recordExpressionTypes = false) {
    // Constructors of every data type, by tag.
    this.data = new Map();
    for (const item of items) {
      if (item.k === 'data') this.data.set(item.name, item.ctors);
    }
    this.bindings = [];
    this.unificationChanges = 0;
    this.bigints = new Set();
    this.plus = [];
    this.fields = [];
    this.signatures = new Map();
    // The types of the fields the translator makes up, which are inferred like parameters.
    this.fieldTerms = new Map();
    // Every homogeneous array keeps its solved element type for contextual checking.
    this.emptyArrays = new Map();
    this.lambdas = new Map();
    this.records = new Map();
    this.methods = [];
    this.propertyMethods = new Map();
    this.includes = [];
    this.coalesces = [];
    this.arrayBindingDefaults = [];
    this.recordSpreads = [];
    this.collections = new Map();
    this.iterables = [];
    this.indices = [];
    this.cells = new Map();
    this.arithmetic = [];
    this.equalities = [];
    this.conditions = [];
    this.joins = [];
    this.results = [];
    this.calls = [];
    this.expressionTerms = new Map();
    this.expressionTypes = new WeakMap();
    this.recordExpressionTypes = recordExpressionTypes;
  }

  fresh(bigint = false) {
    const id = this.bindings.length;
    this.bindings.push(null);
    if (bigint) this.bigints.add(id);
    return { kind: 'var', id };
  }

  resolve(term) {
    let current = term;
    while (current.kind === 'var' && this.bindings[current.id]) current = this.bindings[current.id];
    return current;
  }

  /** Compare logical types, retaining distinct unresolved variable identities. */
  sameTerm(left, right) {
    if (this.resolve(left) === this.resolve(right)) return true;
    const normalize = (term, active = new Set()) => {
      const type = this.resolve(term);
      if (active.has(type)) throw typeError('a recursive inferred type needs an explicit data type');
      active.add(type);
      let normalized = type;
      const child = (value) => normalize(value, active);
      if (['array', 'option', 'set', 'cell'].includes(type.kind)) normalized = { ...type, element: child(type.element) };
      else if (type.kind === 'tuple') normalized = { ...type, elements: type.elements.map(child) };
      else if (type.kind === 'union') normalized = { ...type, alternatives: type.alternatives.map(child) };
      else if (type.kind === 'record') normalized = { ...type, fields: type.fields.map((field) => ({ ...field, type: child(field.type) })) };
      else if (type.kind === 'map') normalized = { ...type, key: child(type.key), value: child(type.value) };
      else if (type.kind === 'function') normalized = { ...type, params: type.params.map(child), ret: child(type.ret) };
      active.delete(type);
      return normalized;
    };
    const compare = (first, second) => {
      const pairs = readInferenceTermIdentity(first, second);
      return pairs !== null && pairs.every(([value, expected]) => compare(value, expected));
    };
    return compare(normalize(left), normalize(right));
  }

  /** The type of a constructor field as a term, one per field even when it is not declared. */
  field(field) {
    if (field.type) return this.declared(field.type);
    if (!this.fieldTerms.has(field)) this.fieldTerms.set(field, this.fresh());
    return this.fieldTerms.get(field);
  }

  /** A declared surface type as a term; the frontend writes `bigint` as `int`. */
  declared(type) {
    if (!type) return this.fresh();
    if (type.kind === 'named') return { kind: 'data', name: type.path.at(-1) };
    if (type.kind === 'nat') return INT;
    if (type.kind === 'array') return { kind: 'array', element: this.declared(type.element) };
    if (type.kind === 'option' || type.kind === 'set' || type.kind === 'cell') return { ...type, element: this.declared(type.element) };
    if (type.kind === 'tuple') return { ...type, elements: type.elements.map((element) => this.declared(element)) };
    if (type.kind === 'union') return { ...type, alternatives: type.alternatives.map(alternative => this.declared(alternative)) };
    if (type.kind === 'record') return { ...type, fields: type.fields.map((field) => ({ ...field, type: this.declared(field.type) })) };
    if (type.kind === 'map') return { ...type, key: this.declared(type.key), value: this.declared(type.value) };
    if (type.kind === 'function') return { ...type, params: type.params.map((param) => this.declared(param.type ?? param)), ret: this.declared(type.ret) };
    return type;
  }

  unify(left, right, where) {
    const a = this.resolve(left);
    const b = this.resolve(right);
    if (a === b) return;
    // Only a function that finishes without a return value makes undefined, the unit value.
    const other = a.kind === 'unit' ? b : b.kind === 'unit' ? a : null;
    if (other && other.kind !== 'unit' && (other.kind !== 'var' || this.bigints.has(other.id))) {
      throw typeError(`the function returns ${other.kind === 'var' ? 'a bigint' : describe(other)} on one path and finishes without a return value, returning undefined, on another; return a value on every path`, where);
    }
    if (a.kind === 'var' && b.kind === 'var') {
      if (a.id === b.id) return;
      if (this.bigints.has(a.id)) this.bigints.add(b.id);
      this.bindings[a.id] = b;
      this.unificationChanges += 1;
      return;
    }
    if (a.kind === 'var' || b.kind === 'var') {
      const [variable, type] = a.kind === 'var' ? [a, b] : [b, a];
      if (this.bigints.has(variable.id) && !isInteger(type)) {
        const value = type.kind === 'option' ? this.resolve(type.element) : type;
        const operation = readOptionalBigIntConstraint(type.kind, value.kind, isInteger(value));
        if (operation === 'mark') this.bigints.add(value.id);
        else if (operation === 'error') {
          throw typeError(`a BigInt is used as ${describe(type)}; JavaScript does not mix BigInt with other types`, where);
        }
      }
      if (this.contains(type, variable.id)) throw typeError('a recursive inferred type needs an explicit data type', where);
      this.bindings[variable.id] = type;
      this.unificationChanges += 1;
      return;
    }
    if (isInteger(a) && isInteger(b)) return;
    if ((a.kind === 'array' && b.kind === 'array') || (a.kind === 'cell' && b.kind === 'cell')) {
      this.unify(a.element, b.element, where);
      return;
    }
    if (a.kind === 'option' || b.kind === 'option') {
      const constraints = readOptionalTypeConstraints(a, b);
      this.unify(...constraints.pair, where);
      if (constraints.absenceOwners.length) {
        const [first, second] = constraints.absenceOwners;
        const absence = mergeOptionalAbsence(first.absence, second.absence);
        if (first.absence !== absence || second.absence !== absence) this.unificationChanges += 1;
        first.absence = absence;
        second.absence = absence;
      }
      return;
    }
    if (a.kind === 'tuple' && b.kind === 'tuple') {
      if (a.elements.length !== b.elements.length) throw typeError('one tuple is used with two different lengths', where);
      a.elements.forEach((element, index) => this.unify(element, b.elements[index], where));
      return;
    }
    if (a.kind === 'record' && b.kind === 'record') {
      let phase = 0;
      let index = 0;
      for (;;) {
        const step = readRecordTypeConstraintStep(recordNameUnits(a.fields), recordNameUnits(b.fields), Boolean(a.open), Boolean(b.open), phase, index);
        const [action, left, right] = step;
        if (action === 6) return;
        if (action === 0) this.unify(a.fields[left].type, b.fields[right].type, where);
        else if (action === 1) this.addInferredField(b, a.fields[left], where);
        else if (action === 2) this.addInferredField(a, b.fields[right], where);
        const error = readRecordTypeConstraintError(action, action === 3 ? a.fields[left].name : action === 4 ? b.fields[right].name : '');
        if (error) throw typeError(error, where);
        phase = step[3];
        index = step[4];
      }
    }
    if (a.kind === 'map' && b.kind === 'map') {
      this.unify(a.key, b.key, where);
      this.unify(a.value, b.value, where);
      return;
    }
    if (a.kind === 'set' && b.kind === 'set') {
      this.unify(a.element, b.element, where);
      return;
    }
    if (a.kind === 'function' && b.kind === 'function') {
      const constraints = readCallableFunctionConstraints(a, b);
      if (!constraints) throw typeError('one function is used with two different parameter counts', where);
      for (const [left, right] of constraints) this.unify(left, right, where);
      return;
    }
    if (a.kind !== b.kind || (a.kind === 'data' && a.name !== b.name) || (a.kind === 'fixed' && (a.bits !== b.bits || a.signed !== b.signed))
      || (a.kind === 'union' && !this.sameTerm(a, b))) {
      throw typeError(`one value is used as ${describe(a)} and as ${describe(b)}; declare the types of the function with JSDoc`, where);
    }
  }

  contains(term, id, seen = new Set()) {
    // A rebinding must inspect the variable itself before following its old
    // binding: pointing it back through that binding would create a cycle.
    if (term.kind === 'var' && term.id === id) return true;
    const type = this.resolve(term);
    if (type === id) return true;
    if (type.kind === 'var') return type.id === id;
    if (seen.has(type)) return false;
    seen.add(type);
    const children = type.kind === 'array' || type.kind === 'option' || type.kind === 'set' || type.kind === 'cell' ? [type.element]
      : type.kind === 'tuple' ? type.elements
      : type.kind === 'union' ? type.alternatives
      : type.kind === 'record' ? type.fields.map((field) => field.type)
      : type.kind === 'map' ? [type.key, type.value]
      : type.kind === 'function' ? readCallableTypeChildren(type) : [];
    return children.some((child) => this.contains(child, id, seen));
  }

  addInferredField(record, field, where) {
    if (containsRecordTypeOwner(field.type, record, this.bindings)) throw typeError('a recursive inferred type needs an explicit data type', where);
    record.fields.push({ ...field });
    this.unificationChanges += 1;
  }

  run(functions, main, externals) {
    // The other items of the module keep the types their own translation checked.
    for (const external of externals) {
      this.signatures.set(`${ROOT}.${external.name}`, external.k === 'fn'
        ? { params: external.params.map((param) => this.declared(param.type)), defaults: external.params.map(() => false), optionalParams: external.params.map(() => false), ret: this.declared(external.ret) }
        : { params: [], defaults: [], optionalParams: [], ret: this.declared(external.type) });
    }
    for (const { path, fn } of functions) {
      const ret = this.fresh();
      const declaredRet = fn.ret ? this.declared(fn.ret) : null;
      if (declaredRet) this.unify(ret, declaredRet, fn.span);
      this.signatures.set(path.join('.'), {
        params: fn.params.map((param) => this.declared(param.type)),
        defaults: fn.params.map((param) => Boolean(param.default || param.optional)),
        optionalParams: fn.params.map((param) => Boolean(param.optional)),
        ret,
        declaredRet,
      });
    }
    for (const { path, fn } of functions) {
      const signature = this.signatures.get(path.join('.'));
      this.context = path.slice(0, -1);
      fn.params.forEach((param, index) => {
        if (param.default) this.unify(this.expr(param.default, new Map()), signature.params[index], param.span);
      });
    }
    for (const { path, fn } of functions) {
      const signature = this.signatures.get(path.join('.'));
      const env = new Map(fn.params.map((param, index) => [param.name, signature.params[index]]));
      this.context = path.slice(0, -1);
      const body = this.expr(fn.body, env, signature.ret);
      this.unify(body, signature.ret, fn.span);
      this.results.push({ fn, signature, body });
      if (!fn.ret && signature.ret.kind === 'var' && this.resolve(body).kind === 'option') this.bindings[signature.ret.id] = body;
    }
    this.context = [ROOT];
    const env = new Map();
    for (const effect of main?.effects ?? []) {
      if (effect.k === 'print') this.expr(effect.expr, env);
      else if (effect.k === 'let') env.set(effect.name, this.expr(effect.value, env));
      else if (effect.k === 'assert') this.prop(effect.prop, env);
    }
    this.solve();
    for(const[node,form]of this.propertyMethods){
      const object=node.object,args=node.args;
      delete node.object;delete node.args;
      if(form==='test'){node.k='stringTest';node.object=object;node.search=args[0];}
      else if(form==='map'){node.k='stringMap';node.object=object;}
      else{node.k='toString';node.arg=object;delete node.op;}
    }
    for (const [node, term] of this.expressionTerms) this.expressionTypes.set(node, this.surface(term, node.span));
    for (const [node] of this.expressionTerms) {
      const value = node.k === 'let' && !node.type ? this.expressionTypes.get(node.value) : null;
      if (value?.kind === 'option' && value.element.kind === 'tuple') node.type = value;
    }
    for (const { path, fn } of functions) {
      const signature = this.signatures.get(path.join('.'));
      fn.params.forEach((param, index) => {
        if (!param.type) param.type = this.surface(signature.params[index], fn.span);
      });
      if (!fn.ret) fn.ret = this.surface(signature.ret, fn.span);
      const observed = this.results.find((result) => result.fn === fn);
      if (observed?.signature.declaredRet && this.additionalAbsence(observed.body, observed.signature.declaredRet)) fn.semanticRet = this.surface(observed.body, fn.span);
    }
    for (const [field, term] of this.fieldTerms) field.type = this.surface(term, field.span);
    for (const [node, term] of this.emptyArrays) node.element = this.surface(term, node.span);
    for (const [node, term] of this.records) node.type = this.surface(term, node.span);
    for (const [node, term] of this.collections) node.type = this.surface(term, node.span);
    for (const [node, term] of this.cells) node.type = this.surface(term, node.span);
    for (const [node, term] of this.lambdas) {
      node.params.forEach((param, index) => { if (!param.type) param.type = this.surface(term.params[index], param.span ?? node.span); });
      node.ret = this.surface(term.ret, node.span);
    }
  }

  /** Deferred `+` and field constraints, until nothing more is known; then `+` of unknowns adds numbers. */
  solve() {
    for (let progress = true; progress;) {
      progress = false;
      for (const constraint of [...this.plus]) {
        if (this.plusStep(constraint, false)) {
          this.plus.splice(this.plus.indexOf(constraint), 1);
          progress = true;
        }
      }
      for (const constraint of [...this.fields]) {
        if (this.fieldStep(constraint)) {
          this.fields.splice(this.fields.indexOf(constraint), 1);
          progress = true;
        }
      }
      for (const constraint of [...this.methods]) {
        if (this.methodStep(constraint)) {
          this.methods.splice(this.methods.indexOf(constraint), 1);
          progress = true;
        }
      }
      for (const constraint of [...this.includes]) {
        if (this.includesStep(constraint, false)) {
          this.includes.splice(this.includes.indexOf(constraint), 1);
          progress = true;
        }
      }
      for (const constraint of [...this.coalesces]) {
        const left = this.resolve(constraint.left);
        const pair = readOptionalCoalescingConstraints(left, left, constraint.right);
        if (!pair) continue;
        this.unify(...pair, constraint.where);
        this.coalesces.splice(this.coalesces.indexOf(constraint), 1);
        progress = true;
      }
      for (const constraint of [...this.iterables]) {
        const object = this.resolve(constraint.object);
        const element = readArrayFromIterableType(object)?.element;
        if (!element) continue;
        if (constraint.mapper) this.callback(constraint.mapper, [element, FLOAT], constraint.result.element, constraint.where);
        else this.unify(constraint.result.element, element, constraint.where);
        this.iterables.splice(this.iterables.indexOf(constraint), 1);
        progress = true;
      }
      for (const constraint of [...this.indices]) {
        const object = this.resolve(constraint.object);
        if (object.kind === 'var') continue;
        if (object.kind === 'array') {
          const value = constraint.node.provenInBounds ? object.element : { kind: 'option', element: object.element, absence: 'undefined' };
          this.unify(constraint.result, value, constraint.where);
          if (constraint.result.kind === 'var') this.bindings[constraint.result.id] = value;
        }
        else if (object.kind === 'tuple' && !Number.isNaN(readJavaScriptProductIndex(constraint.index))) {
          const at = readJavaScriptProductIndex(constraint.index);
          if (!Number.isInteger(at) || at < 0 || at >= object.elements.length) throw typeError('tuple index out of range', constraint.where);
          this.unify(constraint.result, object.elements[at], constraint.where);
        }
        this.indices.splice(this.indices.indexOf(constraint), 1);
        progress = true;
      }
      for (const constraint of [...this.arrayBindingDefaults]) {
        const plan = readJavaScriptArrayDefaultConstraints(this.resolve(constraint.value), constraint.fallback, constraint.result);
        if (!plan) continue;
        if (plan.refused) throw typeError('array binding default needs an undefined-only element read', constraint.where);
        for (const [left, right] of plan.pairs) this.unify(left, right, constraint.where);
        this.arrayBindingDefaults.splice(this.arrayBindingDefaults.indexOf(constraint), 1);
        progress = true;
      }
      for (const constraint of [...this.arithmetic]) {
        if (this.arithmeticStep(constraint, false)) {
          this.arithmetic.splice(this.arithmetic.indexOf(constraint), 1);
          progress = true;
        }
      }
      for (const constraint of this.joins) {
        if (this.joinStep(constraint)) progress = true;
      }
      for (const { signature, body } of this.results) {
        const additional = signature.declaredRet ? this.additionalAbsence(body, signature.declaredRet) : false;
        const actual = this.resolve(body);
        const eligible = (!signature.declaredRet || additional) && actual.kind !== 'var' && signature.ret.kind === 'var';
        const same = eligible ? this.sameTerm(signature.ret, actual) : false;
        const recursive = eligible && !same ? this.contains(actual, signature.ret.id) : false;
        if (requireOptionalResultBinding(Boolean(signature.declaredRet), additional, actual.kind, signature.ret.kind, same, recursive)) {
          this.bindings[signature.ret.id] = actual;
          progress = true;
        }
      }
      for (const { signature, args, where } of this.calls) {
        args.forEach((arg, index) => {
          const param = signature.params[index];
          let actual = this.resolve(arg);
          const defaultInference = readParameterDefaultArgumentInference(signature.defaults[index], signature.optionalParams[index], actual.kind, actual.absence ?? '');
          if (defaultInference === 'defer') return;
          if (defaultInference === 'element') actual = this.resolve(actual.element);
          if (defaultInference !== 'identity') {
            const changes = this.unificationChanges;
            this.unify(actual, param, where);
            if (changes !== this.unificationChanges) progress = true;
          }
          const eligible = param?.kind === 'var' && actual.kind === 'option';
          const current = eligible ? this.resolve(param) : undefined;
          const recursive = eligible && current.kind !== 'option' ? this.contains(actual, param.id) : false;
          const operation = readOptionalCallBinding(param?.kind ?? '', actual.kind, current?.kind ?? '', current?.absence ?? '', actual.absence ?? '', recursive);
          if (operation === 'elements' || operation === 'merge') {
            this.unify(current.element, actual.element, where);
            if (readOptionalCallBinding(param.kind, actual.kind, current.kind, current.absence, actual.absence, false) === 'merge') {
              current.absence = 'both';
              progress = true;
            }
          } else if (operation === 'bind') {
            this.bindings[param.id] = { ...actual };
            progress = true;
          }
        });
      }
      for (const constraint of [...this.recordSpreads]) {
        if (constraint.entries.some((entry) => entry.spread && readRecordSpreadRole(this.resolve(entry.type).kind) === 'open')) continue;
        const fields = [];
        for (const entry of constraint.entries) {
          if (entry.spread) {
            const source = this.resolve(entry.type);
            if (readRecordSpreadRole(source.kind) !== 'fields') throw typeError('an object spread needs a known record shape', constraint.where);
            fields.push(...source.fields);
          } else fields.push({ name: entry.name, type: entry.type });
        }
        const shape = { kind: 'record', fields: readRecordFieldInsertionOrder(recordNameUnits(fields)).map(index => ({ name: fields[index].name, type: fields[index].type })) };
        this.unify(constraint.term, shape, constraint.where);
        constraint.term.open = false;
        this.recordSpreads.splice(this.recordSpreads.indexOf(constraint), 1);
        progress = true;
      }
      for (const constraint of [...this.equalities]) {
        if (this.equalityStep(constraint, false)) {
          this.equalities.splice(this.equalities.indexOf(constraint), 1);
          progress = true;
        }
      }
      // Preserve the original string default only after actual call arguments,
      // annotations and other constraints have selected the receiver.
      if (!progress && this.includes.length) {
        this.includesStep(this.includes.shift(), true);
        progress = true;
      }
      if(!progress){
        const method=this.methods.find(item=>this.resolve(item.object).kind==='var'&&readJavaScriptPropertyCallFallbackKind(item.op,readJavaScriptMethodReceiverKind(item.op))!=='');
        if(method&&this.methodStep(method,true)){this.methods.splice(this.methods.indexOf(method),1);progress=true;}
      }
      // `.length` of a value nothing else fixes reads an array.
      const length = this.fields.find(({ object, field }) => field === 'length' && this.resolve(object).kind === 'var');
      if (!progress && length) {
        this.unify(length.object, { kind: 'array', element: this.fresh() }, length.where);
        progress = true;
      }
      const field = this.fields.find(({ object, field: name }) => name !== 'length' && this.resolve(object).kind === 'var');
      if (!progress && field) {
        this.unify(field.object, { kind: 'record', fields: [{ name: field.field, type: field.result }], open: true }, field.where);
        progress = true;
      }
      if (!progress && this.coalesces.length) {
        const coalesced = this.coalesces.shift();
        this.unify(...readOptionalCoalescingConstraints(coalesced.left, this.resolve(coalesced.left), coalesced.right, true), coalesced.where);
        progress = true;
      }
      if (!progress && this.iterables.length) {
        const iterable = this.iterables.find((constraint) => this.resolve(constraint.object).kind === 'var');
        if (iterable) {
          this.unify(iterable.object, { kind: 'array', element: this.fresh() }, iterable.where);
          progress = true;
        }
      }
      if (!progress && this.indices.length) {
        const indexed = this.indices.find((constraint) => this.resolve(constraint.object).kind === 'var');
        if (indexed) {
          this.unify(indexed.object, { kind: 'array', element: this.fresh() }, indexed.where);
          progress = true;
        }
      }
      if (!progress && this.plus.length) {
        this.plusStep(this.plus.shift(), true);
        progress = true;
      }
      if (!progress && this.arithmetic.length) {
        this.arithmeticStep(this.arithmetic.shift(), true);
        progress = true;
      }
      if (!progress && this.equalities.length) {
        this.equalityStep(this.equalities.shift(), true);
        progress = true;
      }
      if (!progress) {
        const condition = this.conditions.find(({ term }) => this.resolve(term).kind === 'var');
        if (condition) {
          this.unify(condition.term, BOOL, condition.where);
          progress = true;
        }
      }
    }
    if (this.recordSpreads.length) throw typeError('an object spread needs a known record shape; declare its fields with JSDoc', this.recordSpreads[0].where);
    if (this.arrayBindingDefaults.length) throw typeError('array binding default needs a known array element type', this.arrayBindingDefaults[0].where);
  }

  plusStep({ left, right, result, where }, force) {
    const a = this.resolve(left);
    const b = this.resolve(right);
    if (a.kind === 'string' || b.kind === 'string') {
      this.unify(result, STRING, where);
      return true;
    }
    if (!force && (a.kind === 'var' || b.kind === 'var')) return false;
    const x = this.numericValue(left);
    const y = this.numericValue(right);
    const first = this.resolve(x);
    const second = this.resolve(y);
    if (first.kind === 'string' || second.kind === 'string') throw typeError('nullable string addition needs a string or number result determined by the values', where);
    this.unify(x, y, where);
    this.unify(result, x, where);
    return true;
  }

  numericValue(term) {
    const type = this.resolve(term);
    const operation = readOptionalNumericTerm(type.kind);
    return operation === 'element' ? this.numericValue(type.element) : operation === 'float' ? FLOAT : term;
  }

  arithmeticStep({ left, right, result, where }, force) {
    const a = this.numericValue(left);
    const b = right ? this.numericValue(right) : a;
    if (!force && (this.resolve(a).kind === 'var' || this.resolve(b).kind === 'var')) return false;
    this.unify(a, b, where);
    this.unify(result, a, where);
    return true;
  }

  equalityStep({ operation, left, right, where }, force) {
    const step = readJavaScriptEqualityInferenceStep(operation, this.resolve(left).kind, this.resolve(right).kind, force);
    if (step === 'defer') return false;
    if (step === 'unify') this.unify(left, right, where);
    return true;
  }

  join(left, right, where) {
    const result = this.fresh();
    this.joins.push({ left, right, result, where });
    return result;
  }

  joinStep({ left, right, result, where }) {
    const a = this.resolve(left);
    const b = this.resolve(right);
    const before = this.unificationChanges;
    const constraints = readOptionalJoinConstraints(left, right, a, b, this.resolve(result));
    if (constraints.pair) this.unify(...constraints.pair, where);
    const value = this.resolve(constraints.value);
    const current = this.resolve(result);
    if (requireInferenceJoinAlias(current.kind, current.id ?? -1, result.id)) this.unify(current, value, where);
    const original = value === result || (value.kind === 'var' && value.id === result.id);
    const same = !original && this.sameTerm(result, value);
    const operation = readOptionalJoinBinding(original, same, !original && !same && this.contains(value, result.id));
    if (operation === 'identity') return false;
    if (operation === 'changes') return this.unificationChanges !== before;
    this.bindings[result.id] = value;
    return true;
  }

  additionalAbsence(actualTerm, declaredTerm) {
    const actual = this.resolve(actualTerm);
    const declared = declaredTerm ? this.resolve(declaredTerm) : undefined;
    const plan = readOptionalAdditionalAbsence(actual, declared);
    return plan.additional || plan.pairs.some(([value, expected]) => this.additionalAbsence(value, expected));
  }

  fieldStep({ object, field, result, where }) {
    const type = this.resolve(object);
    if ((type.kind === 'array' || type.kind === 'string' || type.kind === 'tuple') && field === 'length') {
      this.unify(result, FLOAT, where);
      return true;
    }
    if (type.kind === 'record') {
      const [action, position] = readRecordFieldInferenceStep(recordNameUnits(type.fields), Array.from({ length: field.length }, (_, index) => field.charCodeAt(index)), type.open === true);
      let declared = action === 0 ? type.fields[position] : undefined;
      if (action === 1) {
        declared = { name: field, type: result };
        this.addInferredField(type, declared, where);
      }
      if (declared) this.unify(result, declared.type, where);
      return true;
    }
    if ((type.kind === 'map' || type.kind === 'set') && field === 'size') {
      this.unify(result, FLOAT, where);
      return true;
    }
    if (type.kind !== 'data') {
      const owners = [...this.data].filter(([, ctors]) => ctors.some((ctor) => ctor.fields.some((candidate) => candidate.name === field)));
      if (type.kind !== 'var') return false;
      if (owners.length !== 1) return false;
      this.unify(object, { kind: 'data', name: owners[0][0] }, where);
      return this.fieldStep({ object, field, result, where });
    }
    const declared = (this.data.get(type.name) ?? []).flatMap((ctor) => ctor.fields).find((candidate) => candidate.name === field);
    if (declared) this.unify(result, this.field(declared), where);
    return true;
  }

  /** The inferred type as a surface type; an unconstrained BigInt is `bigint`, anything else unconstrained a Number. */
  surface(term, where, active = new Set()) {
    const type = this.resolve(term);
    if (type.kind === 'var') return this.bigints.has(type.id) ? INT : FLOAT;
    if (type.kind === 'data') return { kind: 'named', path: [ROOT, type.name], span: where };
    if (active.has(type)) throw typeError('a recursive inferred type needs an explicit data type', where);
    active.add(type);
    try {
      if (type.kind === 'array') return { kind: 'array', element: this.surface(type.element, where, active) };
      if (type.kind === 'option') return option(this.surface(type.element, where, active), type.absence ?? 'both');
      if (type.kind === 'set' || type.kind === 'cell') return { ...type, element: this.surface(type.element, where, active) };
      if (type.kind === 'tuple') return { ...type, elements: type.elements.map((element) => this.surface(element, where, active)) };
      if (type.kind === 'union') return { ...type, alternatives: type.alternatives.map(alternative => this.surface(alternative, where, active)) };
      if (type.kind === 'record') return { ...type, fields: type.fields.map((field) => ({ ...field, type: this.surface(field.type, where, active) })) };
      if (type.kind === 'map') return { ...type, key: this.surface(type.key, where, active), value: this.surface(type.value, where, active) };
      if (type.kind === 'function') return { ...type, params: type.params.map((param) => this.surface(param, where, active)), ret: this.surface(type.ret, where, active) };
      return type;
    } finally {
      active.delete(type);
    }
  }

  signature(path) {
    const name = path.join('.');
    if (this.signatures.has(name)) return this.signatures.get(name);
    // A call inside a namespace may name a sibling relative to it.
    for (let depth = this.context.length; depth >= 1; depth -= 1) {
      const candidate = [...this.context.slice(0, depth), ...path.slice(1)].join('.');
      if (this.signatures.has(candidate)) return this.signatures.get(candidate);
    }
    return null;
  }

  expr(node, env, expected = undefined) {
    const term = this.inferExpression(node, env, expected);
    if (this.recordExpressionTypes) this.expressionTerms.set(node, term);
    return term;
  }

  inferExpression(node, env, expected = undefined) {
    switch (node.k) {
      case 'num':
        // The 1 of `x++` has the type of `x`, a Number or a BigInt.
        if (node.unit) return this.fresh();
        return node.type ? this.declared(node.type) : this.fresh(true);
      case 'bool':
        return BOOL;
      case 'str':
        return STRING;
      case 'unit':
        return UNIT;
      case 'none':
        return { kind: 'option', element: this.fresh(), absence: node.absence ?? 'both' };
      case 'cellAllocate': {
        const element = this.fresh();
        this.unify(element, this.expr(node.value, env), node.span);
        const type = { kind: 'cell', element };
        this.cells.set(node, type);
        return type;
      }
      case 'cellRead': {
        const cell = this.expr(node.cell, env);
        const resolved = this.resolve(cell);
        const plan = readCallableCellReadInference(cell, resolved);
        if (plan.element !== undefined) return plan.element;
        const element = this.fresh();
        this.unify(plan.bind, { kind: 'cell', element }, node.span);
        return element;
      }
      case 'cellWrite': {
        const value = this.expr(node.value, env);
        const cell = this.expr(node.cell, env);
        this.unify(cell, { kind: 'cell', element: value }, node.span);
        const resolved = this.resolve(cell);
        const binding = readCallableCellOptionBinding(resolved, this.resolve(value));
        if (binding) this.bindings[binding.variable] = binding.value;
        return this.expr(node.body, env, expected);
      }
      case 'referenceWrite': {
        const object = this.expr(node.object, env);
        // Unary updates have no right-hand expression. Infer the selected
        // numeric slot without inventing a value or reading a missing node.
        const updating = node.op === '++' || node.op === '--';
        const value = updating ? this.fresh() : this.expr(node.value, env);
        if (node.index) {
          this.expr(node.index, env);
          this.unify(object, { kind: 'array', element: value }, node.span);
        } else this.fields.push({ object, field: node.field, result: value, where: node.span });
        if (updating) this.arithmetic.push({ left: value, result: value, where: node.span });
        return value;
      }
      case 'name': {
        if (node.path.length === 1 && env.has(node.path[0])) return env.get(node.path[0]);
        return this.call(node.path, [], env, node.span);
      }
      case 'functionValue': {
        const signature = this.signature(node.path);
        return signature ? constructCallableFunctionType(signature.params.map(type => ({ type })), signature.ret) : this.fresh();
      }
      case 'app': {
        const local = node.fn.k === 'name' && node.fn.path.length === 1 ? env.get(node.fn.path[0]) : null;
        if (requireCallableNamedInference(node.fn.k, node.fn.path?.length ?? 0, Boolean(local))) return this.call(node.fn.path, node.args, env, node.span);
        const fn = local ?? this.expr(node.fn, env);
        const args = node.args.map((arg) => this.expr(arg, env));
        const resolved = this.resolve(fn);
        if (acceptCallableApplicationInferenceResult(resolved.kind, resolved.params?.length ?? 0, args.length)) {
          const constraints = readCallableApplicationConstraints(resolved, args, undefined);
          for (const [left, right] of constraints.pairs) this.unify(left, right, node.span);
          return constraints.ret;
        }
        const ret = this.fresh();
        const candidate = constructCallableFunctionType(args.map(type => ({ type })), ret);
        const constraints = readCallableApplicationConstraints(resolved, args, candidate);
        for (const [left, right] of constraints.pairs) this.unify(left, right, node.span);
        return ret;
      }
      case 'field': {
        const result = this.fresh();
        this.fields.push({ object: this.expr(node.object, env), field: node.field, result, where: node.span });
        return result;
      }
      case 'optionalField': {
        const object = this.expr(node.object, env);
        const resolved = this.resolve(object);
        const plan = readOptionalFieldInference(object, resolved);
        const element = plan.element ?? this.fresh();
        if (plan.bind) this.unify(plan.bind, { kind: 'option', element, absence: 'both' }, node.span);
        const field = this.fresh();
        this.fields.push({ object: element, field: node.field, result: field, where: node.span });
        return { kind: 'option', element: field, absence: 'undefined' };
      }
      case 'arrayBindingRest': {
        checkJavaScriptArrayBindingWrapper(node);
        const object = this.expr(node.object, env);
        const result = { kind: 'array', element: this.fresh() };
        this.unify(object, result, node.span);
        return result;
      }
      case 'arrayBindingDefault': {
        checkJavaScriptArrayBindingWrapper(node);
        const value = this.expr(node.value, env);
        const fallback = this.expr(node.fallback, env, expected);
        const result = this.fresh();
        this.arrayBindingDefaults.push({ value, fallback, result, where: node.span });
        return result;
      }
      case 'coalesce': {
        const left = this.expr(node.left, env);
        const right = this.expr(node.right, env, expected);
        const resolved = this.resolve(left);
        const pair = readOptionalCoalescingConstraints(left, resolved, right);
        if (pair) this.unify(...pair, node.span);
        else this.coalesces.push({ left, right, where: node.span });
        return right;
      }
      case 'lambda': {
        const want = expected && this.resolve(expected);
        const plan = readCallableLambdaInference(node, want);
        const params = plan.params.map(({ type, declared }) => declared ? this.declared(type) : type ?? this.fresh());
        const scope = new Map(env);
        node.params.forEach((param, index) => scope.set(param.name, params[index]));
        const ret = plan.declaredRet ? this.declared(plan.ret) : plan.ret ?? this.fresh();
        const body = this.expr(node.body, scope, ret);
        this.unify(body, ret, node.span);
        const term = constructCallableFunctionType(params.map(type => ({ type })), plan.declaredRet ? ret : body);
        this.lambdas.set(node, term);
        return term;
      }
      case 'typeOf':
        this.expr(node.arg, env);
        return STRING;
      case 'unary': {
        const arg = this.expr(node.arg, env);
        if (node.op === 'not') {
          this.conditions.push({ term: arg, where: node.span });
          return BOOL;
        }
        const result = this.fresh();
        this.arithmetic.push({ left: arg, result, where: node.span });
        return result;
      }
      case 'binary':
        return this.binary(node, env);
      case 'if': {
        this.conditions.push({ term: this.expr(node.cond, env), where: node.cond.span ?? node.span });
        const then = this.expr(node.then, env, expected);
        const otherwise = this.expr(node.else, env, expected);
        return this.join(then, otherwise, node.span);
      }
      case 'let': {
        const want = node.type ? this.declared(node.type) : undefined;
        const value = this.expr(node.value, env, want);
        if (want) this.unify(value, want, node.span);
        return this.expr(node.body, new Map(env).set(node.name, want ?? value), expected);
      }
      case 'match':
        return this.match(node, env);
      case 'stringMap':
        this.unify(this.expr(node.object, env), STRING, node.span);
        return STRING;
      case 'stringTest':
        if (node.op === 'includes') {
          this.includes.push({ node, object: this.expr(node.object, env), search: this.expr(node.search, env), where: node.span });
          return BOOL;
        }
        this.unify(this.expr(node.object, env), STRING, node.span);
        this.unify(this.expr(node.search, env), STRING, node.span);
        return BOOL;
      case 'toString':
      case 'show':
        this.expr(node.arg, env);
        return STRING;
      case 'abort':
        return this.fresh();
      case 'print':
        this.expr(node.expr, env);
        return this.expr(node.body, env, expected);
      case 'ctorObject':
        return this.ctorObject(node, env);
      case 'tuple': {
        const want = expected && this.resolve(expected);
        const term = { kind: 'tuple', elements: node.items.map((item, index) => this.expr(item, env, want?.kind === 'tuple' ? want.elements[index] : undefined)) };
        if (want) this.unify(term, want, node.span);
        return term;
      }
      case 'array': {
        const context = expected && this.resolve(expected);
        const present = context?.element && this.resolve(context.element);
        const want = acceptJavaScriptOptionalTupleContext('JavaScript', node.k, context?.kind ?? '', present?.kind ?? '') ? present : context;
        if (acceptJavaScriptArrayTupleContext(node.k, want?.kind ?? '', node.items.some((item) => item.spread))) {
          const elements = node.items.map((item, index) => this.expr(item.value, env, want.elements[index]));
          const term = { kind: 'tuple', elements };
          this.unify(term, want, node.span);
          return term;
        }
        const element = this.fresh();
        this.emptyArrays.set(node, element);
        for (const item of node.items) {
          const wanted = want?.kind === 'array' ? item.spread ? want : want.element : undefined;
          const value = this.expr(item.value, env, wanted);
          this.unify(value, item.spread ? { kind: 'array', element } : element, item.value.span ?? node.span);
          const contained = item.spread ? this.resolve(value).element : this.resolve(value);
          if (contained && this.resolve(contained).kind === 'option') this.bindings[element.id] = this.resolve(contained);
        }
        return { kind: 'array', element };
      }
      case 'record':
        return this.record(node, env, expected);
      case 'set':
      case 'map':
        return inferCollectionConstruction(this, node, env, expected);
      case 'method': {
        const object = this.expr(node.object, env);
        const args = node.args.map((arg) => this.expr(arg, env));
        const result = this.fresh();
        this.methods.push({ node, object, args, op: node.op, result, where: node.span });
        return result;
      }
      case 'stringPoints':
        this.unify(this.expr(node.object, env), STRING, node.span);
        return { kind: 'array', element: STRING };
      case 'copySlice':
        return this.expr(node.object, env, expected);
      case 'forOfIterable': {
        const object = this.expr(node.object, env);
        // Keep the original array term and its optional-element owner. The
        // existing shared iterable plan already forwards known arrays unchanged.
        if (readArrayFromIterableType(this.resolve(object))?.form === 0) {
          this.collections.set(node, object);
          return object;
        }
        const result = { kind: 'array', element: this.fresh() };
        this.iterables.push({ object, result, where: node.span });
        this.collections.set(node, result);
        return result;
      }
      case 'arrayFrom': {
        const args = node.args.map((arg) => this.expr(arg, env));
        const inputs = readArrayFromInferenceInputs(args, expected);
        const result = { kind: 'array', element: inputs.selected ? inputs.element : this.fresh() };
        if (inputs.object) this.iterables.push({ object: inputs.object, mapper: inputs.mapper, result, where: node.span });
        this.collections.set(node, result);
        return result;
      }
      case 'arrayIsArray':
        this.expr(node.arg, env);
        return BOOL;
      case 'math':
        if ((readJavaScriptNumberBitwiseArity(node.op) || readBigIntegerBitwiseArity(node.op)) && node.args.length > 0) {
          const values = node.args.map(item => this.expr(item.value, env)), result = this.fresh();
          this.arithmetic.push({ left: values[0], right: values[1], result, where: node.span });
          return result;
        }
        // Math takes Numbers: an argument of no known type is one, and the checker refuses any other.
        for (const item of node.args) {
          const type = this.resolve(this.expr(item.value, env));
          const unknown = item.spread ? type.kind === 'var' || (type.kind === 'array' && this.resolve(type.element).kind === 'var') : type.kind === 'var';
          if (unknown && !readNumberPredicateArgumentForm('JavaScript', node.op, '', '')) this.unify(type, item.spread ? { kind: 'array', element: FLOAT } : FLOAT, item.value.span ?? node.span);
        }
        return ['isInteger', 'isSafeInteger', 'isFinite', 'isNaN'].includes(node.op) ? BOOL : FLOAT;
      case 'tupleIndex':
      case 'index': {
        const element = this.fresh();
        const index = node.k === 'tupleIndex' ? { k: 'num', value: String(node.index), span: node.span } : node.index;
        this.indices.push({ object: this.expr(node.object, env), index, node, result: element, where: node.object.span ?? node.span });
        if (node.k === 'index') this.expr(node.index, env);
        return element;
      }
      case 'length':
        this.unify(this.expr(node.object, env), { kind: 'array', element: this.fresh() }, node.span);
        return node.integer ? this.fresh(true) : FLOAT;
      default:
        return this.fresh();
    }
  }

  call(path, args, env, where) {
    const signature = this.signature(path);
    const argTypes = args.map((arg, index) => this.expr(arg, env, signature?.params[index]));
    if (!signature || !acceptArgumentCount(signature.defaults, args.length)) return this.fresh();
    this.calls.push({ signature, args: argTypes, where });
    argTypes.forEach((type, index) => {
      const param = signature.params[index];
      const actual = this.resolve(type);
      const operation = readParameterDefaultArgumentInference(signature.defaults[index], signature.optionalParams[index], actual.kind, actual.absence ?? '');
      if (operation === 'defer') return;
      const value = operation === 'element' ? actual.element : type;
      this.unify(value, param, args[index].span ?? where);
      if (param.kind === 'var' && this.resolve(value).kind === 'option') this.bindings[param.id] = this.resolve(value);
    });
    return signature.ret;
  }

  binary(node, env) {
    const left = this.expr(node.left, env);
    const right = this.expr(node.right, env);
    if (LOGICAL.has(node.op)) {
      this.conditions.push({ term: left, where: node.left.span ?? node.span });
      return this.join(left, right, node.span);
    }
    if (ORDER.has(node.op)) {
      if (node.op === 'eq' || node.op === 'ne') this.equalities.push({ operation: node.op, left, right, where: node.span });
      else this.unify(left, right, node.span);
      return BOOL;
    }
    if (ARITHMETIC.has(node.op)) {
      const result = this.fresh();
      this.arithmetic.push({ left, right, result, where: node.span });
      return result;
    }
    if (node.op === 'concat') return STRING;
    const result = this.fresh();
    this.plus.push({ left, right, result, where: node.span });
    return result;
  }

  match(node, env) {
    const scrutinees = node.scrutinees.map((scrutinee) => this.expr(scrutinee, env));
    let result = null;
    for (const row of node.rows) {
      const scope = new Map(env);
      row.patterns.forEach((pattern, index) => this.pattern(pattern, scrutinees[index], scope, row.span ?? node.span));
      const body = this.expr(row.body, scope);
      if (result) {
        result = this.join(body, result, row.span ?? node.span);
      } else result = body;
    }
    return result ?? this.fresh();
  }

  pattern(pattern, type, scope, where) {
    switch (pattern.k) {
      case 'ctor': {
        const [name, tag] = pattern.path.slice(-2);
        this.unify(type, { kind: 'data', name }, where);
        const ctor = (this.data.get(name) ?? []).find((candidate) => candidate.name === tag);
        pattern.args.forEach((arg, index) => {
          const field = ctor?.fields[index];
          this.pattern(arg, field ? this.field(field) : this.fresh(), scope, where);
        });
        return;
      }
      case 'bind':
      case 'bindOrCtor':
        scope.set(pattern.name, type);
        return;
      case 'numLit':
        this.unify(type, this.fresh(true), where);
        return;
      case 'boolLit':
        this.unify(type, BOOL, where);
        return;
      case 'strLit':
        this.unify(type, STRING, where);
        return;
      default:
    }
  }

  ctorObject(node, env) {
    const owners = [...this.data].filter(([, ctors]) => ctors.some((ctor) => ctor.name === node.tag));
    const fieldTypes = Object.entries(node.fields).map(([name, value]) => [name, this.expr(value, env), value.span]);
    if (owners.length !== 1) return this.fresh();
    const [name, ctors] = owners[0];
    const ctor = ctors.find((candidate) => candidate.name === node.tag);
    for (const [field, type, where] of fieldTypes) {
      const declared = ctor.fields.find((candidate) => candidate.name === field);
      if (declared) this.unify(type, this.field(declared), where ?? node.span);
    }
    return { kind: 'data', name };
  }

  record(node, env, expected) {
    const want = expected && this.resolve(expected);
    const fields = [];
    const entries = [];
    let unknownSpread = false;
    for (const entry of node.entries) {
      if (entry.spread) {
        const type = this.expr(entry.value, env);
        entries.push({ spread: true, type });
        const source = this.resolve(type);
        const role = readRecordSpreadRole(source.kind);
        if (role === 'open') unknownSpread = true;
        else {
          if (role !== 'fields') throw typeError('an object spread needs a known record shape', entry.value.span ?? node.span);
          fields.push(...source.fields);
        }
      } else {
        const expectedPosition = want?.kind === 'record' ? readRecordFieldPosition(recordNameUnits(want.fields), Array.from({ length: entry.name.length }, (_, index) => entry.name.charCodeAt(index))) : -1;
        const declared = expectedPosition < 0 ? undefined : want.fields[expectedPosition].type;
        const value = this.expr(entry.value, env, declared);
        if (declared) this.unify(value, declared, entry.value.span ?? node.span);
        fields.push({ name: entry.name, type: declared ?? value });
        entries.push({ spread: false, name: entry.name, type: declared ?? value });
      }
    }
    const term = { kind: 'record', fields: readRecordFieldInsertionOrder(recordNameUnits(fields)).map(index => ({ name: fields[index].name, type: fields[index].type })), ...(unknownSpread && { open: true }) };
    if (unknownSpread) this.recordSpreads.push({ term, entries, where: node.span });
    this.records.set(node, term);
    return term;
  }

  includesStep({ node, object, search, where }, force) {
    const receiver = this.resolve(object);
    const form = readJavaScriptIncludesReceiverForm(receiver.kind, force);
    if (form === 'defer') return false;
    if (form === 'array') {
      this.unify(search, receiver.element, where);
      const argument = node.search;
      node.k = 'method';
      node.args = [argument];
      delete node.search;
    } else {
      this.unify(object, STRING, where);
      this.unify(search, STRING, where);
    }
    return true;
  }

  methodStep({ node, object, args, op, result, where }, force=false) {
    let receiver = this.resolve(object);
    if (receiver.kind === 'var') {
      if(!force)return false;
      const kind = readJavaScriptPropertyCallFallbackKind(op,readJavaScriptMethodReceiverKind(op));
      if(kind==='scalar'){if(node&&args.length===0)this.propertyMethods.set(node,'text');this.unify(result,STRING,where);return true;}
      if (kind === 'array') {
        this.unify(object, { kind: 'array', element: this.fresh() }, where);
      } else if (kind === 'string') {
        this.unify(object, STRING, where);
      } else return false;
      receiver = this.resolve(object);
    }
    if(receiver.kind==='record'){
      const field=receiver.fields.find(field=>field.name===op);
      if(!field)return true;
      const callable=this.resolve(field.type),step=readJavaScriptRecordMethodInferenceStep(callable.kind,callable.params?.length??0,args.length);
      if(step==='apply'||step==='infer')this.unify(field.type,constructCallableFunctionType(args.map(type=>({type})),result),where);
      return true;
    }
    if(op==='toString'){if(node&&args.length===0)this.propertyMethods.set(node,'text');this.unify(result,STRING,where);return true;}
    if (receiver.kind === 'string') {
      const selected=readJavaScriptSourcePropertyOperationKind(op);
      if(node&&(selected==='test'&&args.length===1||selected==='map'&&args.length===0))this.propertyMethods.set(node,selected);
      const primitive = readPrimitiveMethodMinimumArguments(receiver.kind, op) >= 0;
      args.forEach((arg, index) => {
        const kind = primitive && index < readPrimitiveMethodArgumentCount(receiver.kind, op)
          ? readPrimitiveMethodCheckedArgumentKind(receiver.kind, op, index)
          : readJavaScriptMethodArgumentKind(receiver.kind, op, index);
        if (kind === 'float') this.unify(arg, FLOAT, where);
        else if (kind === 'number-value') this.unify(this.numericValue(arg), FLOAT, where);
        else if (kind === 'string') this.unify(arg, STRING, where);
      });
      const form = primitive ? readPrimitiveMethodCheckedResultForm(receiver.kind, op) : readJavaScriptStringMethodResultForm(op);
      const output = form === 'bool' ? BOOL : form === 'float' ? FLOAT
        : form === 'optional-float' ? { kind: 'option', element: FLOAT, absence: 'undefined' }
        : form === 'optional-string' ? { kind: 'option', element: STRING, absence: 'undefined' }
        : form === 'array-string' ? { kind: 'array', element: STRING } : STRING;
      this.unify(result, output, where);
      return true;
    }
    if (receiver.kind === 'array') {
      if (readArrayStateMethodResult(op)) {
        args.forEach((argument,index) => {
          const role=readArrayStateMethodArgumentRole(op,index), actual=this.resolve(argument);
          if(role==='element')this.unify(argument,receiver.element,where);
          else if(role&&actual.kind==='var')this.unify(argument,FLOAT,where);
          else if(role&&actual.kind==='option'&&this.resolve(actual.element).kind==='var')this.unify(actual.element,FLOAT,where);
        });
        this.unify(result,receiver,where);
        return true;
      }
      const referenceResult = readReferenceMethodResult('array', op);
      if (referenceResult === 'length') {
        for (const arg of args) {
          this.unify(arg, receiver.element, where);
          if (receiver.element.kind === 'var' && this.resolve(arg).kind === 'option') this.bindings[receiver.element.id] = this.resolve(arg);
        }
        this.unify(result, FLOAT, where);
      } else if (referenceResult === 'optional element') {
        this.unify(result, { kind: 'option', element: receiver.element, absence: 'undefined' }, where);
      } else if (op === 'splice') {
        for (const arg of args.slice(0, 2)) this.unify(arg, FLOAT, where);
        for (const arg of args.slice(2)) this.unify(arg, receiver.element, where);
        this.unify(result, receiver, where);
      } else if (op === 'fill') {
        if (args[0]) this.unify(args[0], receiver.element, where);
        for (const arg of args.slice(1)) this.unify(arg, FLOAT, where);
        this.unify(result, receiver, where);
      } else if (op === 'copyWithin') {
        for (const arg of args) this.unify(arg, FLOAT, where);
        this.unify(result, receiver, where);
      } else if (op === 'sort') {
        if (args[0]) this.callback(args[0], [receiver.element, receiver.element], FLOAT, where);
        this.unify(result, receiver, where);
      } else if (['map', 'flatMap', 'filter', 'find', 'findIndex', 'some', 'every', 'forEach'].includes(op)) {
        const ret = ['filter', 'find', 'findIndex', 'some', 'every'].includes(op) ? BOOL : this.fresh();
        if (op === 'flatMap') this.unify(ret, { kind: 'array', element: this.fresh() }, where);
        this.callback(args[0], [receiver.element, FLOAT, receiver], ret, where);
        const output = op === 'map' ? { kind: 'array', element: ret }
          : op === 'flatMap' ? ret
          : op === 'filter' ? receiver
          : op === 'find' ? { kind: 'option', element: receiver.element, absence: 'undefined' }
          : op === 'findIndex' ? FLOAT : op === 'forEach' ? UNIT : BOOL;
        this.unify(result, output, where);
      } else if (op === 'reduce') {
        const accumulator = args[1] ?? receiver.element;
        this.callback(args[0], [accumulator, receiver.element, FLOAT, receiver], accumulator, where);
        this.unify(result, accumulator, where);
      } else if (op === 'join') {
        if (args[0]) this.unify(args[0], STRING, where);
        this.unify(result, STRING, where);
      } else if (['includes', 'indexOf', 'lastIndexOf'].includes(op)) {
        args.forEach((arg, index) => {
          const kind = readJavaScriptMethodArgumentKind(receiver.kind, op, index);
          if (kind === 'element') this.unify(arg, receiver.element, where);
          else if (kind === 'float') this.unify(arg, FLOAT, where);
        });
        this.unify(result, op === 'includes' ? BOOL : FLOAT, where);
      } else if (op === 'at') {
        if (args[0]) this.unify(args[0], FLOAT, where);
        this.unify(result, { kind: 'option', element: receiver.element, absence: 'undefined' }, where);
      } else if (op === 'slice') {
        args.forEach((arg, index) => {
          if (readJavaScriptMethodArgumentKind(receiver.kind, op, index) === 'float') this.unify(arg, FLOAT, where);
        });
        this.unify(result, receiver, where);
      } else this.unify(result, receiver, where);
      return true;
    }
    if (receiver.kind === 'map' || receiver.kind === 'set') {
      const key = receiver.kind === 'map' ? receiver.key : receiver.element;
      if (args[0] && ['get', 'has', 'delete', 'set', 'add'].includes(op)) this.unify(args[0], key, where);
      if (op === 'set' && args[1]) {
        this.unify(args[1], receiver.value, where);
        if (receiver.value.kind === 'var' && this.resolve(args[1]).kind === 'option') this.bindings[receiver.value.id] = this.resolve(args[1]);
      }
      const form = readReferenceMethodResult(receiver.kind, op) || readCollectionAccessResult(receiver.kind, op);
      const output = form === 'optional value' ? { kind: 'option', element: receiver.value, absence: 'undefined' }
        : form === 'boolean' ? BOOL
        : form === 'unit' ? UNIT
        : form === 'keys' ? { kind: 'array', element: key }
        : form === 'values' ? { kind: 'array', element: receiver.kind === 'map' ? receiver.value : key }
        : form === 'entries' ? { kind: 'array', element: { kind: 'tuple', elements: receiver.kind === 'map' ? [receiver.key, receiver.value] : [key, key] } }
        : receiver;
      this.unify(result, output, where);
      return true;
    }
    return true;
  }

  callback(term, params, ret, where) {
    if (!term) return;
    const declared = this.resolve(term);
    const constraints = readArrayFromCallbackConstraints(term, declared, params, ret);
    if (constraints.error) throw typeError(constraints.error, where);
    for (const pair of constraints.pairs) this.unify(...pair, where);
  }

  prop(prop, env) {
    switch (prop.p) {
      case 'eq':
      case 'ne':
      case 'lt':
      case 'le':
      case 'gt':
      case 'ge':
        this.unify(this.expr(prop.left, env), this.expr(prop.right, env), prop.span);
        return;
      case 'and':
      case 'or':
        this.prop(prop.left, env);
        this.prop(prop.right, env);
        return;
      case 'not':
        this.prop(prop.arg, env);
        return;
      case 'bool':
        this.unify(this.expr(prop.expr, env), BOOL, prop.span);
        return;
      default:
    }
  }
}

function isInteger(type) {
  return type.kind === 'int' || type.kind === 'nat' || type.kind === 'fixed';
}

function describe(type) {
  switch (type.kind) {
    case 'float':
      return 'a number';
    case 'int':
    case 'nat':
    case 'fixed':
      return 'a bigint';
    case 'bool':
      return 'a boolean';
    case 'string':
      return 'a string';
    case 'data':
      return `a ${type.name}`;
    case 'array':
      return 'an array';
    default:
      return type.kind;
  }
}

import {readNumberPredicateArgumentForm} from './number-predicate-rules.js';
