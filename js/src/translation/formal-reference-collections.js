import { readProgramExpressionEntryOperandField } from '../typed-program-link-rules.js';
import { collectSourceIdentifierTexts, readSourceIdentifierSupply } from './source-identifiers.js';
import { referenceMethodOperation } from './reference-model.js';
import { readOptionalOutputAbsences } from './optional-value-rules.js';
import { prepareFormalClassMethods } from './formal-class-methods.js';
import { readJavaScriptClassOwnerRecords } from './javascript-class-owners.js';
import { acceptFormalLocalCellElement, acceptFormalReferenceStoreLowering, acceptFormalReferenceType, observeFormalStructuralState,
  planFormalReferenceStoreFunctions, readFormalReferenceStoreOperation, requireFormalReferenceStore } from './callable-emission-rules.js';
import { readRecordFieldWriteName, readRecordFieldWriteOperator } from './record-write-rules.js';
import { BOOL, NAT, UNIT, array, callable, tuple, typeKey } from './types.js';

const typeFields = new Set(['type', 'domain', 'from', 'to', 'ret', 'annotatedRet', 'semanticRet']);
const declined = Symbol('unrepresented formal reference operation');
function declineReferenceStore() {
  if (globalThis.process?.env?.META_LANGUAGE_TRACE_FORMAL_STATE === '1') {
    console.error(new Error(declined.description).stack);
  }
  return declined;
}
const variable = (name, type) => ({ k: 'var', name, type });
const truth = value => ({ k: 'lit', value, type: BOOL });
const collection = (kind, items, type) => ({ k: kind, items, type });
const pack = items => collection('tuple', items, tuple(items.map(item => item.type)));
const index = (object, position) => ({ k: 'tupleIndex', object, index: position, type: object.type.elements[position] });
const length = value => ({ k: 'length', array: value, type: NAT });
const condition = (cond, then, otherwise) => ({ k: 'if', cond, then, else: otherwise, type: then.type });
const binary = (op, left, right) => ({ k: 'binary', op, left, right, domain: left.type, type: BOOL });
const negate = arg => ({ k: 'unary', op: 'not', arg, type: BOOL });
const append = (left, right) => ({ k: 'append', left, right, type: left.type });

function withoutReference(type) {
  const { reference, ...payload } = type;
  return payload;
}

function mapTypeChildren(type, transform) {
  if (type.kind === 'record') return { ...type, fields: type.fields.map(field => ({ ...field, type: transform(field.type) })) };
  if (type.kind === 'map') return { ...type, key: transform(type.key), value: transform(type.value) };
  if (type.kind === 'function') return { ...type, params: type.params.map(transform), ret: transform(type.ret) };
  if (type.elements) return { ...type, elements: type.elements.map(transform) };
  if (type.alternatives) return { ...type, alternatives: type.alternatives.map(transform) };
  if (type.element) return { ...type, element: transform(type.element) };
  return type;
}

function inspect(value, visit) {
  const pending = [value];
  const seen = new WeakSet();
  while (pending.length) {
    const current = pending.pop();
    if (!current || typeof current !== 'object' || seen.has(current)) continue;
    seen.add(current);
    visit(current);
    for (const child of Object.values(current)) pending.push(child);
  }
}

function needsState(node) {
  return requireFormalReferenceStore(node.k ?? '', node.object?.type?.reference === true);
}

function targetAddress(type, structuralState) {
  return type.kind === 'cell' || structuralState && type.reference === true;
}

function referenceTruth(type) {
  return type.reference === true || type.kind === 'option' && type.element.reference === true;
}

function absent(value) {
  return readOptionalOutputAbsences(value.type.absence).map(absence => ({
    k: 'strictEqual', left: value,
    right: { k: 'none', absence, type: { kind: 'option', element: UNIT, absence } },
    type: BOOL, negate: false,
  })).reduce((left, right) => binary('or', left, right));
}

function sameCollectionKey(left, right) {
  const equal = binary('eq', left, right);
  if (left.type.kind !== 'float') return equal;
  const nan = value => ({ k: 'math', op: 'isNaN', args: [value], type: BOOL });
  return binary('or', equal, binary('and', nan(left), nan(right)));
}

/**
 * Reference addresses are indices in separate immutable arrays of payloads.
 * The address has a single source domain before translation. Changing an
 * outer collection never copies a referenced element's payload.
 */
class FormalReferenceCollections {
  constructor(program, structuralState) {
    this.program = program;
    this.structuralState = structuralState;
    this.standalone = !program.main?.effects.length;
    const classOwners = readJavaScriptClassOwnerRecords(program.sourceClassOwners);
    this.closedClassLibrary = this.standalone && classOwners.length > 0;
    this.prototypeOwners = new Set(classOwners
      .filter(owner => owner.representation === 'prototype helpers').map(owner => owner.constructorName));
    const classConstructors = new Set(classOwners.map(owner => owner.constructorName));
    this.sharedFunctions = new Map();
    this.localEntries = new Set();
    this.localClosures = new Map();
    this.prefix = readSourceIdentifierSupply(collectSourceIdentifierTexts(program));
    this.count = 0;
    this.slots = new Map();
    this.types = new WeakMap();
    const roots = new Set();
    const callers = [];
    const callees = [];
    for (const [name, entry] of program.declarations) {
      inspect(entry, node => this.admitType(node));
      let addressBoundary = false;
      let privateClassBoundary = false;
      inspect(entry.k === 'fn' ? [entry.params, entry.ret, entry.annotatedRet, entry.semanticRet] : entry, node => {
        addressBoundary ||= targetAddress(node, this.standalone && structuralState);
        privateClassBoundary ||= node.kind === 'record' && classConstructors.has(node.sourceClassOwner);
      });
      if (addressBoundary) {
        if (!this.closedClassLibrary || entry.k !== 'fn' || !(entry.generated || privateClassBoundary)) throw declineReferenceStore();
        roots.add(name);
      } else if (this.closedClassLibrary && entry.k === 'fn') {
        this.localEntries.add(name);
      }
      if (entry.k !== 'fn') continue;
      inspect(entry.body, node => {
        if (needsState(node)) roots.add(name);
        if (node.k === 'call') {
          if (this.standalone && structuralState && !program.declarations.has(node.fn)) {
            inspect([node.type, ...node.args.map(argument => argument.type)], type => {
              if (targetAddress(type, true)) throw declineReferenceStore();
            });
          }
          callers.push(name);
          callees.push(node.fn);
        }
      });
    }
    inspect(program.main, node => this.admitType(node));
    // Closed class helpers pass private addresses along proven local call edges.
    // Ordinary standalone functions retain independent stores and signatures.
    this.effectful = this.standalone && !this.closedClassLibrary ? roots
      : new Set(planFormalReferenceStoreFunctions([...roots], callers, callees));
    if (this.closedClassLibrary) {
      for (const [name, entry] of program.declarations) {
        if (!this.effectful.has(name)) continue;
        const local = this.name('referenceStoreFunction');
        this.sharedFunctions.set(name, {name:local,fullName:[...entry.modulePath,local].join('.')});
      }
    }
    this.slots = new Map([...this.slots.keys()].sort().map((key, index) =>
      [key, { index, type: this.slots.get(key) }]));
    this.stateType = tuple([...this.slots.values()].map(({ type }) => array(this.targetType(type))));
  }

  admitType(type) {
    const callableClassSlot = type.kind === 'record' && type.sourceClassOwner !== undefined
      && !this.prototypeOwners.has(type.sourceClassOwner)
      && type.fields.some(field => field.type.kind === 'function');
    if (!acceptFormalReferenceType(callableClassSlot)) throw declineReferenceStore();
    if (!targetAddress(type, this.structuralState)) return;
    if (type.kind === 'cell' ? !acceptFormalLocalCellElement(type.element.kind)
      : !['array', 'set', 'map', 'record'].includes(type.kind)) throw declineReferenceStore();
    const key = typeKey(type);
    if (!this.slots.has(key)) this.slots.set(key, type.kind === 'cell' ? type.element : withoutReference(type));
  }

  targetType(type) {
    if (targetAddress(type, this.structuralState)) return NAT;
    if (!this.types.has(type)) this.types.set(type, mapTypeChildren(
      type.reference === true ? withoutReference(type) : type, child => this.targetType(child)));
    return this.types.get(type);
  }

  sharesStore(name) {
    return (!this.standalone || this.closedClassLibrary) && this.effectful.has(name);
  }

  name(role) {
    return this.prefix + role + (++this.count);
  }

  bind(value, continuation, name = this.name('referenceOperand')) {
    const body = continuation(variable(name, value.type));
    return { k: 'let', name, value, body, type: body.type };
  }

  read(state, sourceType, address) {
    const values = index(state, this.slots.get(typeKey(sourceType)).index);
    return { k: 'index', array: values, index: address, type: values.type.element, provenInBounds: true };
  }

  write(state, sourceType, update) {
    const position = this.slots.get(typeKey(sourceType)).index;
    const values = state.type.elements.map((_, at) => index(state, at));
    values[position] = update(values[position]);
    return pack(values);
  }

  callback(types, body) {
    const params = types.map(type => variable(this.name('referenceCallback'), type));
    const value = body(...params);
    return { k: 'lambda', params: params.map(({ name, type }) => ({ name, type })),
      body: value, type: callable(types, value.type) };
  }

  replace(values, address, replacement) {
    const empty = collection('array', [], values.type);
    const initial = pack([{ k: 'lit', value: '0', type: NAT }, empty]);
    const callback = this.callback([initial.type, values.type.element], (previous, value) => {
      const position = index(previous, 0);
      const item = condition(binary('eq', address, position), replacement, value);
      return pack([{ k: 'binary', op: 'add', left: position, right: { k: 'lit', value: '1', type: NAT },
        type: NAT, domain: NAT }, append(index(previous, 1), collection('array', [item], values.type))]);
    });
    return index({ k: 'method', op: 'reduce', object: values, args: [callback, initial], type: initial.type }, 1);
  }

  closureType(node) {
    let effectful = false;
    inspect(node.body, entry => { effectful ||= needsState(entry) || entry.k === 'call' && this.sharesStore(entry.fn)
      || entry.k === 'var' && this.localClosures.has(entry.name); });
    if (!effectful) return null;
    if (node.params.some(parameter => parameter.default || parameter.optional || parameter.guard)) throw declineReferenceStore();
    const type = this.targetType(node.type);
    return callable([...type.params, this.stateType], tuple([type.ret, this.stateType]));
  }

  withBindings(names, transform) {
    const shadowed = names.map(name => [name, this.localClosures.get(name)]);
    for (const name of names) this.localClosures.delete(name);
    try { return transform(); }
    finally {
      for (const [name, value] of shadowed) {
        if (value) this.localClosures.set(name, value);
        else this.localClosures.delete(name);
      }
    }
  }

  lowerClosure(node, type) {
    const state = variable(this.name('referenceStore'), this.stateType);
    const params = node.params.map(parameter => ({ ...parameter, type: this.targetType(parameter.type) }));
    const body = this.withBindings(params.map(parameter => parameter.name),
      () => this.expression(node.body, state, (next, value) => pack([value, next])));
    return { ...node, params: [...params, { name: state.name, type: state.type }], body, type };
  }

  collectionMethod(node, object, args, state, done) {
    const sourceType = node.object.type;
    const kind = sourceType.kind;
    const operation = readFormalReferenceStoreOperation(kind, node.op,
      referenceMethodOperation(kind, node.op)?.id ?? '', args.length,
      sourceType.ordered === true, readRecordFieldWriteOperator(node.op));
    if (!operation) throw declineReferenceStore();
    return this.bind(this.read(state, sourceType, object), value => {
      const finish = (updated, result) => this.bind(
        this.write(state, sourceType, values => this.replace(values, object, updated)),
        next => done(next, result));
      if (operation === 'write field') {
        const field = readRecordFieldWriteName(node.op);
        const entries = value.type.fields.map(({ name, type }, at) => ({
          name, value: name === field ? args[0] : { k: 'recordField', index: at, field: name, object: value, type },
        }));
        return finish({ k: 'record', entries, type: value.type }, { k: 'unit', type: UNIT });
      }
      if (operation === 'fill') {
        const callback = this.callback([value.type.element], () => args[0]);
        return finish({ k: 'method', op: 'map', object: value, args: [callback], type: value.type }, object);
      }
      if (operation === 'append array') {
        const appended = append(value, collection('array', args, value.type));
        return this.bind(appended, updated => finish(updated,
          { k: 'length', array: updated, type: this.targetType(node.type) }));
      }
      const isMap = kind === 'map';
      const keyType = isMap ? value.type.key : value.type.element;
      const element = isMap ? tuple([keyType, value.type.value]) : keyType;
      const entriesType = array(element);
      const construct = entries => ({ k: kind, [isMap ? 'entries' : 'items']: entries, type: value.type });
      if (operation === 'clear') return finish(construct(collection('array', [], entriesType)), { k: 'unit', type: UNIT });
      const [key, newValue] = args;
      const member = { k: 'collectionRead', op: 'has', object: value, key, type: BOOL };
      if (operation === 'has') return done(state, member);
      if (operation === 'get') return done(state, { ...member, op: 'get', type: this.targetType(node.type) });
      return this.bind(member, exists => {
        const entries = { k: 'collectionItems', op: isMap ? 'entries' : 'values', object: value, type: entriesType };
        const matches = item => sameCollectionKey(isMap ? index(item, 0) : item, key);
        if (operation === 'delete') {
          const callback = this.callback([element], item => negate(matches(item)));
          const remaining = { k: 'method', op: 'filter', object: entries, args: [callback], type: entriesType };
          return finish(construct(remaining), exists);
        }
        const inserted = isMap ? pack([key, newValue]) : key;
        const added = construct(append(entries, collection('array', [inserted], entriesType)));
        let existing = value;
        if (isMap) {
          const callback = this.callback([element], item => condition(matches(item), pack([index(item, 0), newValue]), item));
          existing = construct({ k: 'method', op: 'map', object: entries, args: [callback], type: entriesType });
        }
        return finish(condition(exists, existing, added), object);
      });
    });
  }

  /**
   * The continuation consumes an expression once with the store left by
   * its last operand. Branches return one packet before joining so a later
   * sibling is never copied into both branches.
   */
  expression(node, state, done) {
    const lower = (operand, then) => this.expression(operand, state, then);
    const packet = (next, value) => pack([value, next]);
    const unpack = result => this.bind(result, bound => done(index(bound, 1), index(bound, 0)));
    const type = this.targetType(node.type);
    if (node.k === 'var' && this.localClosures.has(node.name)) throw declineReferenceStore();
    if (node.k === 'var' || node.k === 'lit' || node.k === 'none' || node.k === 'unit') {
      return done(state, { ...node, type });
    }
    if (node.k === 'let') {
      const closure = node.value.k === 'lambda' && this.closureType(node.value);
      if (closure) {
        const value = this.lowerClosure(node.value, closure);
        const scoped = this.withBindings([node.name], () => {
          this.localClosures.set(node.name, closure);
          return this.expression(node.body, state, packet);
        });
        return unpack(this.bind(value, () => scoped, node.name));
      }
      return lower(node.value, (next, value) => unpack(this.bind(value,
        () => this.withBindings([node.name], () => this.expression(node.body, next, packet)), node.name)));
    }
    if (node.k === 'apply' && node.fn.k === 'var' && this.localClosures.has(node.fn.name)) {
      const closure = this.localClosures.get(node.fn.name);
      if (node.args.length + 1 !== closure.params.length) throw declineReferenceStore();
      return this.operands({ ...node, fn: variable(node.fn.name, closure) }, state, (next, operands) =>
        unpack({ ...operands, args: [...operands.args, next], type: closure.ret }), ['fn']);
    }
    if (node.k === 'if') {
      return lower(node.cond, (next, cond) => unpack(condition(cond,
        this.expression(node.then, next, packet), this.expression(node.else, next, packet))));
    }
    if (node.k === 'binary' && ['and', 'or'].includes(node.op)) {
      return lower(node.left, (next, left) => {
        const right = this.expression(node.right, next, packet);
        const immediate = packet(next, truth(node.op === 'or'));
        return unpack(node.op === 'and' ? condition(left, right, immediate) : condition(left, immediate, right));
      });
    }
    if (node.k === 'coalesce') {
      return lower(node.left, (next, left) => this.bind(left, value => unpack(condition(absent(value),
        this.expression(node.right, next, packet), packet(next, { k: 'unwrapOption', arg: value, type })))));
    }
    if (node.k === 'match') {
      return lower(node.scrutinee, (next, scrutinee) => {
        const cases = node.cases.map(entry => ({ ...entry, body: this.withBindings(
          entry.pattern.binds?.filter(Boolean) ?? (entry.pattern.name ? [entry.pattern.name] : []),
          () => this.expression(entry.body, next, packet)) }));
        return unpack({ ...node, scrutinee, cases, type: cases[0].body.type });
      });
    }
    if (node.k === 'print') {
      return lower(node.text, (next, text) => {
        const body = this.expression(node.body, next, done);
        return { ...node, text, body, type: body.type };
      });
    }
    if (node.k === 'referenceAllocate' || node.k === 'cellAllocate') {
      return lower(node.value, (next, value) => this.bind(value, bound => {
        const slot = this.slots.get(typeKey(node.type)).index;
        return this.bind(length(index(next, slot)), address => this.bind(this.write(next, node.type,
          values => append(values, collection('array', [bound], values.type))), updated => done(updated, address)));
      }));
    }
    if (node.k === 'referenceRead' || node.k === 'cellRead') {
      const reference = node.reference ?? node.cell;
      return lower(reference, (next, address) => done(next, this.read(next, reference.type, address)));
    }
    if (node.k === 'cellWrite') {
      return lower(node.cell, (next, address) => this.bind(address, bound =>
        this.expression(node.value, next, (updated, value) => this.bind(value, replacement =>
          this.bind(this.write(updated, node.cell.type, values => this.replace(values, bound, replacement)),
            after => this.expression(node.body, after, done))))));
    }
    if (node.k === 'lambda' || node.k === 'functionRef') {
      if (node.k === 'functionRef' && this.sharesStore(node.fn)) throw declineReferenceStore();
      if (node.k === 'lambda') {
        const params = node.params.map(parameter => ({ ...parameter, type: this.targetType(parameter.type) }));
        return done(state, { ...node, params, body: this.withoutEffects(node.body), type });
      }
      return done(state, { ...node, type });
    }
    if (node.k === 'referenceIdentity' || node.k === 'referenceWrite') throw declineReferenceStore();
    if (node.k === 'truthy' && referenceTruth(node.arg.type)) {
      return lower(node.arg, (next, arg) => this.bind(arg, bound =>
        done(next, node.arg.type.reference ? truth(true) : negate(absent(bound)))));
    }
    return this.operands(node, state, (next, operands) => {
      if (node.k === 'call' && this.sharesStore(node.fn)) {
        const result = { ...operands, fn: this.sharedFunctions.get(node.fn)?.fullName ?? node.fn,
          args: [...operands.args, next], type: tuple([type, this.stateType]) };
        return unpack(result);
      }
      if (node.k === 'method' && node.object.type.reference) {
        return this.collectionMethod(node, operands.object, operands.args, next, done);
      }
      return done(next, operands);
    });
  }

  operands(node, state, done, exclude = []) {
    const keys = Object.keys(node).filter(key => !typeFields.has(key) && !exclude.includes(key)
      && (node[key]?.k || Array.isArray(node[key])));
    const output = { ...node };
    for (const key of typeFields) if (node[key]) output[key] = this.targetType(node[key]);
    const groups = keys.flatMap(key => {
      if (!Array.isArray(node[key])) return [{ value: node[key], put: value => { output[key] = value; } }];
      const field = readProgramExpressionEntryOperandField(node.k, key);
      output[key] = [...node[key]];
      return node[key].flatMap((entry, at) => {
        const value = field ? entry[field] : entry;
        if (!value?.k) return [];
        return [{ value, put: result => { output[key][at] = field ? { ...entry, [field]: result } : result; } }];
      });
    });
    const visit = (position, next) => {
      if (position === groups.length) return done(next, output);
      const { value, put } = groups[position];
      return this.expression(value, next, (updated, result) => {
        const use = bound => {
          put(bound);
          return visit(position + 1, updated);
        };
        return ['var', 'lit', 'unit'].includes(result.k) ? use(result) : this.bind(result, use);
      });
    };
    return visit(0, state);
  }

  withoutEffects(value) {
    inspect(value, node => {
      if (needsState(node) || node.k === 'var' && this.localClosures.has(node.name)
        || (node.k === 'call' || node.k === 'functionRef') && this.sharesStore(node.fn)
        || node.k === 'truthy' && referenceTruth(node.arg.type)) throw declineReferenceStore();
    });
    const rewrite = node => {
      if (!node || typeof node !== 'object') return node;
      if (Array.isArray(node)) return node.map(rewrite);
      return Object.fromEntries(Object.entries(node).map(([key, child]) =>
        [key, child && typeFields.has(key) ? this.targetType(child) : rewrite(child)]));
    };
    return rewrite(value);
  }

  lowerProgram() {
    const initial = pack(this.stateType.elements.map(type => collection('array', [], type)));
    const declarations = new Map();
    for (const [name, entry] of this.program.declarations) {
      if (entry.k !== 'fn' || !this.effectful.has(name)) {
        declarations.set(name, this.withoutEffects(entry));
        continue;
      }
      const state = variable(this.name('referenceStore'), this.stateType);
      const params = entry.params.map(parameter => this.withoutEffects(parameter));
      const body = this.expression(entry.body, state, (next, value) => pack([value, next]));
      if (this.sharesStore(name)) {
        const implementation = this.sharedFunctions.get(name);
        const {annotatedRet,semanticRet,...original} = entry;
        const lowered = { ...original, ...implementation,
          params: [...params, { name: state.name, type: state.type, guard: null }], ret: body.type, body };
        declarations.set(implementation?.fullName ?? name, lowered);
        if (implementation && this.localEntries.has(name)) {
          const call = { k:'call', fn:implementation.fullName,
            args:[...params.map(parameter => variable(parameter.name,parameter.type)),initial], type:body.type };
          declarations.set(name,{...entry,params,recursive:false,decreasing:null,mutual:false,body:index(call,0)});
        }
      } else {
        const output = this.bind(initial, () => index(body, 0), state.name);
        declarations.set(name, { ...entry, params, ret: output.type, body: output });
      }
    }
    if (this.standalone) {
      if (!this.closedClassLibrary) return { ...this.program, declarations };
      const lowerItems = items => items.flatMap(item => {
        if (item.k === 'module') return [{ ...item, items: lowerItems(item.items) }];
        return [this.sharedFunctions.get(item.fullName)?.fullName, item.fullName]
          .filter(name => declarations.has(name)).map(name => declarations.get(name));
      });
      return { ...this.program, declarations, items: lowerItems(this.program.items) };
    }
    let state = variable(this.name('referenceStore'), this.stateType);
    const effects = [{ k: 'let', name: state.name, value: initial }];
    for (const effect of this.program.main.effects) {
      if (effect.k === 'assert') {
        effects.push(this.withoutEffects(effect));
        continue;
      }
      const field = effect.k === 'let' ? 'value' : effect.k === 'print' ? 'expr' : null;
      if (!field) throw declineReferenceStore();
      const packet = this.expression(effect[field], state, (next, value) => pack([value, next]));
      const bound = variable(this.name('referenceStep'), packet.type);
      state = variable(this.name('referenceStore'), this.stateType);
      effects.push({ k: 'let', name: bound.name, value: packet },
        { k: 'let', name: state.name, value: index(bound, 1) },
        { ...effect, [field]: index(bound, 0) });
    }
    return { ...this.program, declarations, main: { ...this.program.main, effects } };
  }
}

export function prepareFormalReferenceCollections(program) {
  if (program.sourceLanguage !== 'JavaScript') return program;
  try {
    let observedCollection = false;
    let observedCell = false;
    let structuralState = false;
    const observe = node => {
      observedCollection ||= node.k === 'collectionItems';
      observedCell ||= node.k === 'cellAllocate';
      structuralState ||= observeFormalStructuralState(node.k ?? '',
        (node.k === 'binary' ? node.domain : node.object?.type)?.reference === true);
    };
    for (const entry of program.declarations.values()) inspect(entry, observe);
    inspect(program.main, observe);
    const standalone = !program.main?.effects.length;
    const checkedClass = readJavaScriptClassOwnerRecords(program.sourceClassOwners).length > 0;
    if (!acceptFormalReferenceStoreLowering(observedCollection, observedCell, structuralState, standalone, checkedClass)) return program;
    const lowering = new FormalReferenceCollections(prepareFormalClassMethods(program), structuralState);
    return lowering.slots.size ? lowering.lowerProgram() : program;
  } catch (error) {
    if (error !== declined) throw error;
    return program;
  }
}
