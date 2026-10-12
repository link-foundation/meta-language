// The checked semantic program, represented entirely by inspectable links.
// No source text, source envelope, JSON AST payload or cached emission is stored.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { LinkId, LinkMetadata, LinkType } from './primitives.js';
import { parseJavaScript } from './translation/javascript.js';
import { parseRust } from './translation/rust.js';
import { checkProgram } from './translation/check.js';
import { emitJavaScript } from './translation/emit-javascript.js';
import { emitRust } from './translation/emit-rust.js';
import { validateLinkedTypedProgram } from './typed-program-links-validation.js';
import { acceptProgramReferenceField, acceptProgramSchemaRoot, isUnsafeProgramField, readProgramContainerKind, readProgramScalarKind } from './typed-program-link-rules.js';

export const TYPED_PROGRAM_LINKS_SCHEMA = Object.freeze(JSON.parse(readFileSync(new URL('./data/typed-program-links-schema.json', import.meta.url), 'utf8')));
const schema = TYPED_PROGRAM_LINKS_SCHEMA;
const idOf = (id) => id.asU64();
const labelOf = (name) => schema.fields[name] ?? name.replace(/[A-Z]/gu, (letter) => ` ${letter.toLowerCase()}`);
const fieldOf = (label) => Object.entries(schema.fields).find(([, value]) => value === label)?.[0]
  ?? label.replace(/ ([a-z])/gu, (_, letter) => letter.toUpperCase());
const metadata = (term, definition) => LinkMetadata.new().withLinkType(LinkType.Semantic)
  .withLanguage(schema.language).withNamed(true).withTerm(term).withDefinition(definition);

function bindingTargets(program) {
  const targets = new WeakMap();
  const declarations = new Map(program.declarations);
  for (const external of program.externals ?? []) declarations.set(external.name, external);
  function walk(node, environment) {
    if (!node || typeof node !== 'object') return;
    if (node.k === 'var' && environment.has(node.name)) targets.set(node, { label: schema.references.binding, target: environment.get(node.name), field: 'name' });
    if (typeof node.fn === 'string' && declarations.has(node.fn)) targets.set(node, { label: schema.references.declaration, target: declarations.get(node.fn), field: 'fn' });
    if (Array.isArray(node.effects)) {
      const inner = new Map(environment);
      for (const effect of node.effects) {
        if (effect.k === 'let') { walk(effect.value, inner); inner.set(effect.name, effect); }
        else walk(effect, inner);
      }
      return;
    }
    if (node.p === 'forall') {
      const inner = new Map(environment);
      for (const binder of node.binders) inner.set(binder.name, binder);
      walk(node.body, inner);
      return;
    }
    if (node.k === 'let') {
      walk(node.value, environment);
      walk(node.body, new Map(environment).set(node.name, node));
      return;
    }
    if (node.k === 'lambda') {
      const inner = new Map(environment);
      for (const parameter of node.params) inner.set(parameter.name, parameter);
      walk(node.body, inner);
      return;
    }
    for (const [key, value] of Object.entries(node)) {
      if (['type', 'domain', 'span', 'from', 'to'].includes(key)) continue;
      if (Array.isArray(value)) for (const child of value) walk(child, environment);
      else if (value && typeof value === 'object') walk(value, environment);
    }
  }
  for (const entry of program.declarations.values()) {
    const environment = new Map((entry.params ?? []).map((parameter) => [parameter.name, parameter]));
    walk(entry.body ?? entry.value, environment);
  }
  walk(program.main, new Map());
  return targets;
}

export function encodeTypedProgramLinks(program, { provenance } = {}) {
  validateLinkedTypedProgram(program);
  const network = new LinkNetwork();
  const identities = new WeakMap();
  const targets = bindingTargets(program);
  const pending = [];
  function encode(value) {
    if (value && typeof value === 'object') {
      if (identities.has(value)) return identities.get(value);
      if (!(Array.isArray(value) || value instanceof Map || Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null)) throw new TypeError('a typed program contains a non-data host object');
      const kind = Array.isArray(value) ? 'array' : value instanceof Map ? 'map' : 'record';
      const id = network.insertLink([], metadata(schema.containers[kind]));
      identities.set(value, id);
      if (kind === 'array') network.link(id).setReferences(value.map(encode));
      else if (kind === 'map') network.link(id).setReferences([...value].map(([key, item]) => encode([key, item])));
      else {
        const target = targets.get(value);
        for (const [name, item] of Object.entries(value)) {
          if (target?.field === name) pending.push({ parent: id, ...target });
          else network.insertFieldRelation(id, labelOf(name), encode(item));
        }
      }
      return id;
    }
    const kind = value === null ? 'null' : typeof value;
    if (!schema.scalars[kind]) throw new TypeError(`a typed program contains a ${kind} host value`);
    const definition = kind === 'null' || kind === 'undefined' ? undefined
      : kind === 'number' && Object.is(value, -0) ? '-0' : String(value);
    return network.insertLink([], metadata(schema.scalars[kind], definition));
  }
  const body = encode(program);
  for (const reference of pending) network.insertFieldRelation(reference.parent, reference.label, encode(reference.target));
  const root = network.insertLink([body], metadata(schema.root, String(schema.schemaVersion)));
  if (provenance !== undefined) network.insertFieldRelation(root, 'provenance', encode(provenance));
  return { network, root };
}

function decodeTypedProgramLinksContext(network, root, { nodeLimit = 100_000, depthLimit = 256 } = {}) {
  const rootLink = network.link(root);
  if (!rootLink || !acceptProgramSchemaRoot(rootLink.metadata().term ?? '', rootLink.metadata().definition ?? '', String(rootLink.references().length))) throw new TypeError('the linked program root or schema is invalid');
  const fields = new Map();
  for (const relation of network.fieldRelations()) {
    const key = idOf(relation.parent);
    if (!fields.has(key)) fields.set(key, []);
    fields.get(key).push(relation);
  }
  const objects = new Map();
  const active = new Set();
  const pending = [];
  let visited = 0;
  function decode(id, depth = 0) {
    const key = idOf(id);
    if (active.has(key)) throw new TypeError('a linked program contains a structural cycle');
    if (objects.has(key)) return objects.get(key);
    if (++visited > nodeLimit || depth > depthLimit) throw new TypeError('the linked program exceeds its structural resource limit');
    const link = network.link(id);
    if (!link || link.metadata().language !== schema.language) throw new TypeError('a linked program contains a dangling or foreign value');
    const { term, definition } = link.metadata();
    const kind = readProgramContainerKind(term ?? '');
    if (kind) {
      const value = kind === 'array' ? [] : kind === 'map' ? new Map() : {};
      objects.set(key, value);
      active.add(key);
      if (kind === 'record') {
        if (link.references().length) throw new TypeError('a program record has positional references');
        const labels = new Set();
        for (const relation of fields.get(key) ?? []) {
          if (labels.has(relation.label)) throw new TypeError('a program record contains a duplicate field');
          labels.add(relation.label);
          if (acceptProgramReferenceField(relation.label)) pending.push({ owner: value, label: relation.label, target: relation.child });
          else {
            const name = fieldOf(relation.label);
            if (isUnsafeProgramField(name)) throw new TypeError('a linked program contains an unsafe property name');
            value[name] = decode(relation.child, depth + 1);
          }
        }
      } else if (kind === 'array') for (const child of link.references()) value.push(decode(child, depth + 1));
      else for (const child of link.references()) {
        const pair = decode(child, depth + 1);
        if (!Array.isArray(pair) || pair.length !== 2 || value.has(pair[0])) throw new TypeError('a program mapping contains an invalid or duplicate entry');
        value.set(pair[0], pair[1]);
      }
      active.delete(key);
      return value;
    }
    if (link.references().length || fields.has(key)) throw new TypeError('a program scalar contains structural children');
    const scalar = readProgramScalarKind(term ?? '');
    if (!scalar || (definition?.length ?? 0) > 1_000_000) throw new TypeError('a linked program contains an invalid scalar');
    let value;
    if (scalar === 'string') value = definition ?? '';
    else if (scalar === 'null') value = null;
    else if (scalar === 'undefined') value = undefined;
    else if (scalar === 'boolean') {
      if (!['true', 'false'].includes(definition)) throw new TypeError('a linked program contains an invalid boolean');
      value = definition === 'true';
    } else if (scalar === 'bigint') {
      if (!/^-?[0-9]+$/u.test(definition ?? '')) throw new TypeError('a linked program contains an invalid integer');
      value = BigInt(definition);
    } else {
      if (!/^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:e[+-]?[0-9]+)?|NaN|Infinity|-Infinity)$/iu.test(definition ?? '')) throw new TypeError('a linked program contains an invalid number');
      value = Number(definition);
    }
    objects.set(key, value);
    return value;
  }
  const program = decode(rootLink.references()[0]);
  const references = new WeakMap();
  for (const reference of pending) {
    const target = objects.get(idOf(reference.target));
    if (!target || typeof target !== 'object') throw new TypeError('a program reference does not target an owned declaration');
    if (reference.label === schema.references.binding) reference.owner.name = target.name;
    else reference.owner.fn = target.fullName ?? target.name;
    references.set(reference.owner, { target, label: reference.label });
  }
  validateLinkedTypedProgram(program, references);
  return { program, objects };
}

export function decodeTypedProgramLinks(network, root, options) {
  return decodeTypedProgramLinksContext(network, root, options).program;
}

// Expression operands consumed by the existing emitters. This intentionally
// does not walk arbitrary metadata, types, source spans or provenance fields.
const expressionOperands = {
  call: ['args'], ctor: ['args'], math: ['args'], recur: ['args'],
  apply: ['fn', 'args'], method: ['object', 'args'],
  cellAllocate: ['value'], cellRead: ['cell'], cellWrite: ['cell', 'value', 'body'],
  referenceAllocate: ['value'], referenceRead: ['reference'],
  recordField: ['object'], tupleIndex: ['object'], tuple: ['items'], array: ['items'],
  some: ['value'], flattenOption: ['value'], nullishNumber: ['arg'], optionalIndexNumber: ['arg'],
  truthy: ['arg'], strictEqual: ['left', 'right'], optionalTypeOf: ['arg'], optionalText: ['arg'],
  unwrapOption: ['arg'], optionalPredicate: ['arg'], coalesce: ['left', 'right'],
  optionalField: ['object'], optionalProject: ['object'], optionalIndex: ['array', 'index'],
  stringPoints: ['object'], set: ['items'], map: ['entries'],
  collectionSize: ['object'], collectionRead: ['object', 'key'], collectionItems: ['object'],
  unary: ['arg'], binary: ['left', 'right'], if: ['cond', 'then', 'else'],
  print: ['text', 'body'], toString: ['arg'], stringMap: ['string'], stringTest: ['string', 'search'],
  cast: ['arg'], append: ['left', 'right'], index: ['array', 'index'], length: ['array'],
};

/** Internal formal bridge: identify owned lexical binder records, including unused binders. */
export function readTypedProgramBindingIdentities(network, root) {
  const { program, objects } = decodeTypedProgramLinksContext(network, root);
  const bindings = new WeakSet();
  const expressions = new WeakSet();
  const propositions = new WeakSet();
  let visits = 0;
  function admit(node, visited) {
    if (!node || typeof node !== 'object' || visited.has(node)) return false;
    if (++visits > 100_000) throw new TypeError('the linked program exceeds its binding resource limit');
    visited.add(node);
    return true;
  }
  function expression(node) {
    if (!admit(node, expressions)) return;
    if (node.k === 'let') {
      bindings.add(node);
      expression(node.value); expression(node.body);
    } else if (node.k === 'lambda') {
      for (const parameter of node.params) bindings.add(parameter);
      expression(node.body);
    } else if (node.k === 'record') {
      for (const entry of node.entries) expression(entry.value);
    } else if (node.k === 'match') {
      expression(node.scrutinee);
      // Pattern variable spelling is still conservative in this codec.
      for (const entry of node.cases) expression(entry.body);
    } else {
      for (const field of expressionOperands[node.k] ?? []) {
        const value = node[field];
        if (Array.isArray(value)) value.forEach(expression);
        else expression(value);
      }
    }
  }
  function proposition(node) {
    if (!admit(node, propositions)) return;
    if (node.p === 'forall') {
      for (const binder of node.binders) bindings.add(binder);
      proposition(node.body);
    } else if (['and', 'or', 'implies'].includes(node.p)) {
      proposition(node.left); proposition(node.right);
    } else if (node.p === 'not') proposition(node.arg);
    else if (node.p === 'bool') expression(node.expr);
    else if (['eq', 'ne', 'lt', 'le', 'gt', 'ge'].includes(node.p)) {
      expression(node.left); expression(node.right);
    }
  }
  for (const entry of program.declarations.values()) {
    if (entry.k === 'fn') {
      for (const parameter of entry.params) bindings.add(parameter);
      expression(entry.body);
    } else if (entry.k === 'const') expression(entry.value);
    else if (entry.k === 'theorem') {
      for (const binder of entry.binders) bindings.add(binder);
      proposition(entry.prop);
    }
  }
  for (const effect of program.main?.effects ?? []) {
    if (effect.k === 'let') { bindings.add(effect); expression(effect.value); }
    else if (effect.k === 'print') expression(effect.expr);
    else if (effect.k === 'assert') proposition(effect.prop);
  }
  return [...objects].filter(([, value]) => bindings.has(value)).map(([id]) => LinkId.fromU64(id));
}

export function createTypedProgramLinks(source, language, options = {}) {
  const canonical = /^(?:rust|rs)$/iu.test(language) ? 'Rust' : /^(?:javascript|js|mjs)$/iu.test(language) ? 'JavaScript' : null;
  if (!canonical) throw new TypeError('typed linked source import currently reads JavaScript and Rust');
  const program = checkProgram(canonical === 'Rust' ? parseRust(source) : parseJavaScript(source, options.context));
  if (options.utf16Strings) program.utf16Strings = true;
  return encodeTypedProgramLinks(program, options);
}

export function emitTypedProgramLinks(network, root, target) {
  const program = decodeTypedProgramLinks(network, root);
  const emit = /^(?:rust|rs)$/iu.test(target) ? emitRust : /^(?:javascript|js|mjs)$/iu.test(target) ? emitJavaScript : null;
  if (!emit) throw new TypeError('typed linked emission currently writes JavaScript and Rust');
  const emitted = emit(program);
  return { code: emitted.text, language: emitted.language, mappings: emitted.mappings,
    assumptions: emitted.assumptions, encodings: emitted.encodings, theorems: emitted.theorems };
}
