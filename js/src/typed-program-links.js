import {readRustSourceVersion} from './translation/rust-source-versions.js';
// The checked semantic program, represented entirely by inspectable links.
// No source text, source envelope, JSON AST payload or cached emission is stored.
import { readRuntimeProjectData } from './project-data-loader.js';
import {readTypedProgramRecordShapeProfile,selectTypedProgramRecordShapes,readTypedProgramRecordShapeFields,compactTypedProgramRecordShapes} from './typed-program-record-shapes.js';
import { LinkNetwork } from './network.js';
import { LinkId, LinkMetadata, LinkType } from './primitives.js';
import { parseJavaScript } from './translation/javascript.js';
import { parseRust } from './translation/rust.js';
import { checkProgram } from './translation/check.js';
import { emitJavaScript } from './translation/emit-javascript.js';
import { emitRust } from './translation/emit-rust.js';
import { emitLean } from './translation/emit-lean.js';
import { emitRocq } from './translation/emit-rocq.js';
import { validateLinkedTypedProgram } from './typed-program-links-validation.js';
import { recheckLinkedTypedProgram } from './typed-program-checked-projection.js';
import { readLinkedPatternBindingScope } from './typed-program-pattern-bindings.js';
import { readCallableOwnedParameters } from './translation/callable-source.js';
import { acceptProgramContainerFields, acceptProgramReferenceField, acceptProgramSchemaRoot, isUnsafeProgramField, readProgramContainerKind, readProgramScalarKind, readProgramReferenceSourceField, readProgramReferenceTargetField, readProgramExpressionOperandFields, readProgramExpressionEntryOperandField, readProgramPropositionOperandFields, readProgramPropositionBindingForm } from './typed-program-link-rules.js';

export const TYPED_PROGRAM_LINKS_SCHEMA = Object.freeze(readRuntimeProjectData(new URL('./data/typed-program-links-schema.json', import.meta.url)));
const schema = TYPED_PROGRAM_LINKS_SCHEMA;
const sharingProfile=Object.freeze(readRuntimeProjectData(new URL('./data/typed-program-value-sharing.lino',import.meta.url)));
const idOf = (id) => id.asU64();
const labelOf = (name) => schema.fields[name] ?? name.replace(/[A-Z]/gu, (letter) => ` ${letter.toLowerCase()}`);
const fieldOf = (label) => Object.entries(schema.fields).find(([, value]) => value === label)?.[0]
  ?? label.replace(/ ([a-z])/gu, (_, letter) => letter.toUpperCase());
const metadata = (term, definition) => LinkMetadata.new().withLinkType(LinkType.Semantic)
  .withLanguage(schema.language).withNamed(true).withTerm(term).withDefinition(definition);

function bindingTargets(program) {
  const targets = new WeakMap();
  const patternOverrides = new WeakMap();
  const declarations = new Map(program.declarations);
  let referenceVisits = 0;
  for (const external of program.externals ?? []) declarations.set(external.name, external);
  function reference(node, field, label, target) {
    const references = targets.get(node) ?? [];
    if (references.some(value => value.field === field && value.label === label && value.target === target)) return;
    if (references.some(value => value.field === field || value.label === label)) throw new TypeError('a typed program reference has duplicate field ownership');
    references.push({ field, label, target });
    targets.set(node, references);
  }
  const seenTypes = new WeakSet();
  function walkType(type) {
    if (!type || typeof type !== 'object' || seenTypes.has(type)) return;
    seenTypes.add(type);
    if (type.kind === 'data') reference(type, 'name', schema.references.datatype, declarations.get(type.name));
    else if (['array', 'option', 'set', 'cell'].includes(type.kind)) walkType(type.element);
    else if (type.kind === 'tuple') type.elements.forEach(walkType);
    else if (type.kind === 'union') type.alternatives.forEach(walkType);
    else if (type.kind === 'record') type.fields.forEach(field => walkType(field.type));
    else if (type.kind === 'map') { walkType(type.key); walkType(type.value); }
    else if (type.kind === 'function') { type.params.forEach(walkType); walkType(type.ret); }
  }
  function walk(node, environment) {
    if (!node || typeof node !== 'object') return;
    if (++referenceVisits > 100_000) throw new TypeError('the linked program exceeds its reference resource limit');
    for (const field of ['type', 'domain', 'from', 'to']) walkType(node[field]);
    if (node.k === 'var' && environment.has(node.name)) reference(node, 'name', schema.references.binding, environment.get(node.name));
    if (['call', 'functionRef'].includes(node.k) && declarations.has(node.fn)) reference(node, 'fn', schema.references.declaration, declarations.get(node.fn));
    if (node.k === 'ctor') {
      const datatype = declarations.get(node.data);
      reference(node, 'data', schema.references.datatype, datatype);
      reference(node, 'ctor', schema.references.constructor, datatype?.ctors.find(ctor => ctor.name === node.ctor));
    }
    if (node === program.main) {
      const inner = new Map(environment);
      for (const effect of node.effects) {
        if (effect.k === 'let') { walk(effect.value, inner); inner.set(effect.name, effect); }
        else walk(effect, inner);
      }
      return;
    }
    if (readProgramPropositionBindingForm(node.p ?? '') === 'binders') {
      const inner = new Map(environment);
      for (const binder of node.binders) inner.set(binder.name, binder);
      for (const field of readProgramPropositionOperandFields(node.p).split(' ').filter(Boolean)) walk(node[field], inner);
      return;
    }
    if (node.k === 'let') {
      walk(node.value, environment);
      walk(node.body, new Map(environment).set(node.name, node));
      return;
    }
    if (node.k === 'lambda') {
      const inner = new Map(environment);
      for (const { parameter } of readCallableOwnedParameters(node)) inner.set(parameter.name, parameter);
      walk(node.body, inner);
      return;
    }
    if (node.k === 'match') {
      walk(node.scrutinee, environment);
      for (const kase of node.cases) {
        const previous = patternOverrides.get(kase.pattern);
        const scope = readLinkedPatternBindingScope(previous ?? kase.pattern, node.scrutinee.type, declarations, { graph: previous !== undefined });
        patternOverrides.set(kase.pattern, scope.graphPattern);
        if (kase.pattern.k === 'ctor') walk(kase.pattern, environment);
        const inner = new Map(environment);
        for (const binding of scope.bindings) { walkType(binding.type); inner.set(binding.name, binding); }
        walk(kase.body, inner);
      }
      return;
    }
    for (const [key, value] of Object.entries(node)) {
      if (['type', 'domain', 'span', 'from', 'to'].includes(key)) continue;
      if (Array.isArray(value)) for (const child of value) walk(child, environment);
      else if (value && typeof value === 'object') walk(value, environment);
    }
  }
  for (const entry of program.declarations.values()) {
    for (const parameter of entry.params ?? entry.binders ?? []) walkType(parameter.type);
    walkType(entry.ret ?? entry.type);
    if (entry.k === 'fn' && entry.sourceSignature !== undefined) {
      for (const position of [...entry.sourceSignature.parameters, entry.sourceSignature.result]) walkType(position.type);
    }
    for (const constructor of entry.ctors ?? []) for (const field of constructor.fields) walkType(field.type);
    for(const constructor of entry.sourceFieldSignatures?.ctors??[])for(const field of constructor.fields)walkType(field.type);
    const environment = new Map((entry.params ?? []).map((parameter) => [parameter.name, parameter]));
    walk(entry.body ?? entry.value, environment);
  }
  walk(program.main, new Map());
  return { targets, patternOverrides, typeNodes:seenTypes };
}

export function encodeTypedProgramLinks(program, { provenance, valueSharing='', recordShapes='' } = {}) {
  if(recordShapes!==''){if(recordShapes!==readTypedProgramRecordShapeProfile().model)throw new TypeError('unregistered typed program record shape profile');}
  if(valueSharing!==''&&(valueSharing!==sharingProfile.model||sharingProfile.schemaVersion!==1||sharingProfile.scalarValues!==true||sharingProfile.typeRecords!==true||sharingProfile.sourceSpans!==true||sharingProfile.occurrences!==false))throw new TypeError('unregistered typed program value sharing profile');
  validateLinkedTypedProgram(program);
  recheckLinkedTypedProgram(program);
  const network = new LinkNetwork();
  const identities = new WeakMap();
  const { targets, patternOverrides, typeNodes } = bindingTargets(program);
  const scalars=new Map(),types=new Map(),shapes=new Map(),shapeIdentities=new WeakMap(),activeShapes=new WeakSet();let shapeVisits=0;
  // Exact shape keys use child identities, retaining ordered fields, complete
  // metadata and signed-zero spellings without expanding a shared type DAG.
  // Only declared type roots use this key; expression/binder owners never do.
  function shape(value,depth=0){
    if(depth>256||++shapeVisits>1000000)throw new TypeError('typed value sharing exceeds its structural resource limit');
    if(value&&typeof value==='object'&&shapeIdentities.has(value))return shapeIdentities.get(value);
    const object=value&&typeof value==='object';if(object&&activeShapes.has(value))throw new TypeError('typed value sharing refuses cyclic type structure');
    if(object&&!Array.isArray(value)&&(Object.getPrototypeOf(value)!==Object.prototype&&Object.getPrototypeOf(value)!==null))throw new TypeError('typed value sharing requires canonical data objects');
    if(object)for(const key of Object.keys(value))if(!Object.hasOwn(Object.getOwnPropertyDescriptor(value,key),'value'))throw new TypeError('typed value sharing refuses accessor-driven type structure');
    if(object)activeShapes.add(value);
    const key=object?Array.isArray(value)?JSON.stringify(['array',value.map(child=>shape(child,depth+1))]):JSON.stringify(['record',Object.entries(value).map(([field,child])=>[field,shape(child,depth+1)])]):JSON.stringify([value===null?'null':typeof value,typeof value==='number'&&Object.is(value,-0)?'-0':String(value)]);
    let identity=shapes.get(key);if(identity===undefined){identity=shapes.size;shapes.set(key,identity);}
    if(object){activeShapes.delete(value);shapeIdentities.set(value,identity);}return identity;
  }
  const pending = [];
  function encode(value,field='',source=true) {
    if (value && typeof value === 'object') {
      if (identities.has(value)) return identities.get(value);
      if (!(Array.isArray(value) || value instanceof Map || Object.getPrototypeOf(value) === Object.prototype || Object.getPrototypeOf(value) === null)) throw new TypeError('a typed program contains a non-data host object');
      const span=source&&field==='span'&&value!==null&&!Array.isArray(value)&&(Object.getPrototypeOf(value)===Object.prototype||Object.getPrototypeOf(value)===null)&&Object.keys(value).length===2&&Object.hasOwn(value,'start')&&Object.hasOwn(value,'end');
      const typeIdentity=valueSharing&&(typeNodes.has(value)||span)?shape(value):null;
      if(typeIdentity!==null&&types.has(typeIdentity)){const id=types.get(typeIdentity);identities.set(value,id);return id;}
      const kind = Array.isArray(value) ? 'array' : value instanceof Map ? 'map' : 'record';
      const id = network.insertLink([], metadata(schema.containers[kind]));
      identities.set(value, id);
      if(typeIdentity!==null)types.set(typeIdentity,id);
      if (kind === 'array') network.link(id).setReferences(value.map(child=>encode(child,'',source)));
      else if (kind === 'map') network.link(id).setReferences([...value].map(([key, item]) => encode([key, item],'',source)));
      else {
        const references = targets.get(value) ?? [];
        for (const [name, item] of Object.entries(patternOverrides.get(value) ?? value)) {
          const target = references.find(reference => reference.field === name);
          if (target) pending.push({ parent: id, ...target });
          else network.insertFieldRelation(id, labelOf(name), encode(item,name,source));
        }
      }
      return id;
    }
    const kind = value === null ? 'null' : typeof value;
    if (!schema.scalars[kind]) throw new TypeError(`a typed program contains a ${kind} host value`);
    const definition = kind === 'null' || kind === 'undefined' ? undefined
      : kind === 'number' && Object.is(value, -0) ? '-0' : String(value);
    const scalarKey=valueSharing?JSON.stringify([kind,definition??null]):null;
    if(valueSharing&&scalars.has(scalarKey))return scalars.get(scalarKey);
    const id=network.insertLink([], metadata(schema.scalars[kind], definition));if(valueSharing)scalars.set(scalarKey,id);return id;
  }
  const body = encode(program);
  for (const reference of pending) network.insertFieldRelation(reference.parent, reference.label, encode(reference.target));
  const root = network.insertLink([body], metadata(schema.root, String(schema.schemaVersion)));
  if (provenance !== undefined) network.insertFieldRelation(root, 'provenance', encode(provenance,'',false));
  return recordShapes?compactTypedProgramRecordShapes(network,root,schema.language,schema.containers.record):{network,root};
}

export function encodeTypedProgramLinksWithValueSharing(program,provenance,profile){if(typeof profile!=='string'||profile==='')throw new TypeError('unregistered typed program value sharing profile');return encodeTypedProgramLinks(program,{provenance,valueSharing:profile});}
export function encodeTypedProgramLinksWithRecordShapes(program,provenance,profile){if(typeof profile!=='string'||profile==='')throw new TypeError('unregistered typed program record shape profile');return encodeTypedProgramLinks(program,{provenance,recordShapes:profile});}

function decodeTypedProgramLinksContext(network, root, { nodeLimit = 100_000, depthLimit = 256 } = {}) {
  const rootLink = network.link(root);
  if (!rootLink || !acceptProgramSchemaRoot(rootLink.metadata().term ?? '', rootLink.metadata().definition ?? '', String(rootLink.references().length))) throw new TypeError('the linked program root or schema is invalid');
  const fields = new Map();
  for (const relation of network.fieldRelations()) {
    const key = idOf(relation.parent);
    if (!fields.has(key)) fields.set(key, []);
    fields.get(key).push(relation);
  }
  const recordShapes=selectTypedProgramRecordShapes(network,root,schema.language);
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
      if (!acceptProgramContainerFields(kind, fields.has(key))) throw new TypeError('an ordered program container contains named fields');
      const value = kind === 'array' ? [] : kind === 'map' ? new Map() : {};
      objects.set(key, value);
      active.add(key);
      if (kind === 'record') {
        if(link.references().length&&!recordShapes)throw new TypeError('a program record has positional references');
        const recordFields=link.references().length?readTypedProgramRecordShapeFields(network,link,schema.language,recordShapes,fields):(fields.get(key)??[]);
        const labels = new Set();
        for (const relation of recordFields) {
          if (labels.has(relation.label)) throw new TypeError('a program record contains a duplicate field');
          labels.add(relation.label);
          if (acceptProgramReferenceField(relation.label)) pending.push({ owner: value, label: relation.label, target: relation.child });
          else {
            const name = fieldOf(relation.label);
            if (isUnsafeProgramField(name)) throw new TypeError('a linked program contains an unsafe property name');
            if (Object.hasOwn(value, name)) throw new TypeError('a program record contains a duplicate normalized field');
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
      if (!/^(?:-?(?:[0-9]+(?:\.[0-9]+)?)(?:[eE][+-]?[0-9]+)?|NaN|Infinity|-Infinity)$/u.test(definition ?? '')) throw new TypeError('a linked program contains an invalid number');
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
    const field = readProgramReferenceSourceField(reference.label, reference.owner.k ?? '', reference.owner.kind ?? '');
    if (!field || Object.hasOwn(reference.owner, field)) throw new TypeError('a program reference has an invalid or duplicate source field');
    const targetField = readProgramReferenceTargetField(reference.label);
    const name = target[targetField] ?? (reference.label === schema.references.declaration ? target.name : undefined);
    if (typeof name !== 'string' || !name) throw new TypeError('a program reference does not target a named declaration');
    reference.owner[field] = name;
    const owned = references.get(reference.owner) ?? new Map();
    if (owned.has(reference.label)) throw new TypeError('a program reference has duplicate role ownership');
    owned.set(reference.label, target);
    references.set(reference.owner, owned);
  }
  const patternScopes = new WeakMap();
  validateLinkedTypedProgram(program, references, { graphPatterns: true, patternScopes });
  recheckLinkedTypedProgram(program);
  return { program, objects, patternScopes };
}

export function decodeTypedProgramLinks(network, root, options) {
  return decodeTypedProgramLinksContext(network, root, options).program;
}

/** Internal formal bridge: identify owned lexical binder records, including unused binders. */
export function readTypedProgramBindingIdentities(network, root) {
  const { program, objects, patternScopes } = decodeTypedProgramLinksContext(network, root);
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
      for (const { parameter } of readCallableOwnedParameters(node)) bindings.add(parameter);
      expression(node.body);
    } else if (node.k === 'match') {
      expression(node.scrutinee);
      for (const entry of node.cases) {
        for (const binding of patternScopes.get(entry.pattern)?.bindings ?? []) bindings.add(binding);
        expression(entry.body);
      }
    } else {
      for (const field of readProgramExpressionOperandFields(node.k ?? '').split(' ').filter(Boolean)) {
        const value = node[field];
        const child = readProgramExpressionEntryOperandField(node.k ?? '', field);
        if (child) value.forEach(entry => expression(entry[child]));
        else if (Array.isArray(value)) value.forEach(expression);
        else expression(value);
      }
    }
  }
  function proposition(node) {
    if (!admit(node, propositions)) return;
    const form = readProgramPropositionBindingForm(node.p ?? '');
    if (form === 'binders') {
      for (const binder of node.binders) bindings.add(binder);
    }
    for (const field of readProgramPropositionOperandFields(node.p ?? '').split(' ').filter(Boolean)) {
      if (form === 'expressions') expression(node[field]);
      else proposition(node[field]);
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
  const sourceVersion=options.rustSourceVersion,sourceVersionSelected=sourceVersion!==undefined;
  if(sourceVersionSelected&&canonical!=='Rust')throw new TypeError('a Rust source version requires Rust source');
  const sharedArraySelected = options.rustSharedArrayHandleSourceModel !== undefined;
  const arraySelected = options.rustArrayResultModel !== undefined;
  const completionSelected = options.rustCompletionModel !== undefined;
  const signatureSelected = options.rustSourceSignatureModel !== undefined;
  const loopSelected = options.rustScalarLoopModel !== undefined;
  const javaScriptLoopSelected = options.javaScriptScalarLoopModel !== undefined;
  const emptyOptionSelected=options.rustEmptyOptionSourceModel!==undefined;
  const primitiveSelected=options.rustPrimitiveOwnerModel!==undefined;
  const ownedLocalSelected=options.rustOwnedLocalStateModel!==undefined;
  const constFunctionSelected=options.rustConstFunctionModel!==undefined;
  const namespaceSelected = options.rustTypeNamespaceModel !== undefined;
  const runtimeShellSelected=options.rustRuntimeShellModel!==undefined;
  const pureUnitSelected=options.rustPureUnitCallModel!==undefined;
  const selected = sourceVersionSelected||sharedArraySelected || arraySelected || completionSelected || signatureSelected || loopSelected || javaScriptLoopSelected||primitiveSelected||emptyOptionSelected||namespaceSelected||pureUnitSelected||ownedLocalSelected||constFunctionSelected||runtimeShellSelected;
  if(runtimeShellSelected&&canonical!=='Rust')throw new TypeError('a Rust runtime shell model requires Rust source');
  if(pureUnitSelected&&canonical!=='Rust')throw new TypeError('a Rust pure Unit-call model requires Rust source');
  if(emptyOptionSelected&&canonical!=='Rust')throw new TypeError('a Rust empty option source model requires Rust source');
  if (namespaceSelected && canonical !== 'Rust') throw new TypeError('a Rust type namespace model requires Rust source');
  if(ownedLocalSelected&&canonical!=='Rust')throw new TypeError('a Rust owned local state model requires Rust source');
  if(constFunctionSelected&&canonical!=='Rust')throw new TypeError('a Rust const function model requires Rust source');
  if(primitiveSelected&&canonical!=='Rust')throw new TypeError('a Rust primitive owner model requires Rust source');
  if(sharedArraySelected&&canonical!=='Rust')throw new TypeError('a Rust shared array handle source model requires Rust source');
  if (arraySelected && canonical !== 'Rust') throw new TypeError('a Rust array result model requires Rust source');
  if (completionSelected && canonical !== 'Rust') throw new TypeError('a Rust completion model requires Rust source');
  if (signatureSelected && canonical !== 'Rust') throw new TypeError('a Rust source signature model requires Rust source');
  if (loopSelected && canonical !== 'Rust') throw new TypeError('a Rust scalar loop model requires Rust source');
  if (javaScriptLoopSelected && canonical !== 'JavaScript') throw new TypeError('a JavaScript scalar loop model requires JavaScript source');
  const program = checkProgram(canonical === 'Rust' ? parseRust(source, { sourceVersion,sharedArrayHandleSourceModel: options.rustSharedArrayHandleSourceModel, arrayResultModel: options.rustArrayResultModel, completionModel: options.rustCompletionModel, sourceSignatureModel: options.rustSourceSignatureModel, scalarLoopModel: options.rustScalarLoopModel,primitiveOwnerModel:options.rustPrimitiveOwnerModel,emptyOptionSourceModel:options.rustEmptyOptionSourceModel,typeNamespaceModel:options.rustTypeNamespaceModel,pureUnitCallModel:options.rustPureUnitCallModel,localValueStateModel:options.rustOwnedLocalStateModel,constFunctionModel:options.rustConstFunctionModel,runtimeShellModel:options.rustRuntimeShellModel }) : parseJavaScript(source, {...options.context, ...(javaScriptLoopSelected ? {scalarLoopModel: options.javaScriptScalarLoopModel} : {})}));
  if (options.utf16Strings) program.utf16Strings = true;
  const provenance = selected ? {
    sourceProfile: { language: canonical,...(sourceVersionSelected?{rustSourceVersion:readRustSourceVersion(sourceVersion).spelling}:{}), ...(sharedArraySelected ? {rustSharedArrayHandleSourceModel: options.rustSharedArrayHandleSourceModel} : {}), ...(arraySelected ? { rustArrayResultModel: options.rustArrayResultModel } : {}), ...(completionSelected ? { rustCompletionModel: options.rustCompletionModel } : {}), ...(signatureSelected ? { rustSourceSignatureModel: options.rustSourceSignatureModel } : {}), ...(loopSelected ? {rustScalarLoopModel: options.rustScalarLoopModel} : {}), ...(javaScriptLoopSelected ? {javaScriptScalarLoopModel: options.javaScriptScalarLoopModel} : {}),...(primitiveSelected?{rustPrimitiveOwnerModel:options.rustPrimitiveOwnerModel}:{}),...(emptyOptionSelected?{rustEmptyOptionSourceModel:options.rustEmptyOptionSourceModel}:{}),...(namespaceSelected?{rustTypeNamespaceModel:options.rustTypeNamespaceModel}:{}),...(pureUnitSelected?{rustPureUnitCallModel:options.rustPureUnitCallModel}:{}),...(ownedLocalSelected?{rustOwnedLocalStateModel:options.rustOwnedLocalStateModel}:{}),...(constFunctionSelected?{rustConstFunctionModel:options.rustConstFunctionModel}:{}),...(runtimeShellSelected?{rustRuntimeShellModel:options.rustRuntimeShellModel}:{}) },
    userProvenance: options.provenance ?? null,
  } : options.provenance;
  return encodeTypedProgramLinks(program, { ...options, provenance });
}

export function emitTypedProgramLinks(network, root, target, options = {}) {
  const program = decodeTypedProgramLinks(network, root);
  const emit = /^(?:rust|rs)$/iu.test(target) ? emitRust : /^(?:javascript|js|mjs)$/iu.test(target) ? emitJavaScript
    : /^lean$/iu.test(target) ? emitLean : /^rocq$/iu.test(target) ? emitRocq : null;
  if (!emit) throw new TypeError('typed linked emission writes JavaScript, Rust, Lean and Rocq');
  if (options.rustSourceHeaderEmissionModel !== undefined && emit !== emitRust) throw new TypeError('a Rust source header emission model requires Rust target');
  if (options.javaScriptScopedBindingEmissionModel !== undefined && emit !== emitJavaScript) throw new TypeError('a JavaScript scoped binding emission model requires JavaScript target');
  const emitted = emit === emitRust ? emit(program, { sourceHeaderModel: options.rustSourceHeaderEmissionModel, arrayConsumption: true })
    : emit === emitJavaScript ? emit(program, {scopedBindingModel: options.javaScriptScopedBindingEmissionModel}) : emit(program);
  return { code: emitted.text, language: emitted.language, mappings: emitted.mappings,
    assumptions: emitted.assumptions, encodings: emitted.encodings, theorems: emitted.theorems };
}
