// The checked semantic program, represented entirely by inspectable links.
// No source text, source envelope, JSON AST payload or cached emission is stored.
import { readFileSync } from 'node:fs';
import { LinkNetwork } from './network.js';
import { LinkMetadata, LinkType } from './primitives.js';
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

export function decodeTypedProgramLinks(network, root, { nodeLimit = 100_000, depthLimit = 256 } = {}) {
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
  return program;
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
