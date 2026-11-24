import { readJavaScriptClassOwnerRecords } from './javascript-class-owners.js';
import { readRecordFieldWriteName, readRecordFieldWriteOperator } from './record-write-rules.js';
import { UNIT, typeKey } from './types.js';
import { acceptLinkedJavaScriptSourceException } from './source-exception-rules.js';
import { acceptFormalClassMethodFactory, acceptFormalClassMethodInstallation,
  acceptFormalClassMethodPlaceholder } from './callable-emission-rules.js';

const unsupportedClass = Symbol('a class method needs its actual callable slot');
const unit = { k: 'unit', type: UNIT };
function refuseClassSlot(reason) {
  if (globalThis.process?.env?.META_LANGUAGE_TRACE_FORMAL_STATE === '1') {
    console.error(new Error(reason).stack);
  }
  return unsupportedClass;
}

function tail(node) {
  while (node.k === 'let') node = node.body;
  return node;
}

function abrupt(node) {
  return node.k === 'abort' || acceptLinkedJavaScriptSourceException(node.k, node.op ?? '');
}

function placeholderName(value, declarations) {
  const adapter = value?.k === 'lambda' ? tail(value.body) : null;
  const name = value?.k === 'functionRef' ? value.fn
    : adapter?.k === 'call' ? adapter.fn : undefined;
  const declaration = declarations.get(name);
  if (declaration && acceptFormalClassMethodPlaceholder(declaration.body.k, declaration.params.length,
    declaration.body.fn === name, declaration.body.args?.length ?? -1)) return name;
}

function variableUses(node, counts) {
  if (!node || typeof node !== 'object') return;
  if (node.k === 'var') counts.set(node.name, (counts.get(node.name) ?? 0) + 1);
  for (const child of Object.values(node)) variableUses(child, counts);
}

/**
 * The checked class owner already prohibits detached method values and slot
 * assignments outside their constructor. A linear installation prefix also
 * proves that the actual receiver has fully initialized its slots before any
 * use can reach a method. Expose those receiver-retaining calls to the state
 * pass before its ordinary callable analysis.
 */
export function prepareFormalClassMethods(program) {
  const owners = readJavaScriptClassOwnerRecords(program.sourceClassOwners);
  if (!owners.length) return program;
  try {
    const methods = new Map();
    const helpers = new Map();
    const installations = new WeakSet();
    const placeholders = new WeakMap();
    const payloads = new WeakSet();
    for (const owner of owners) {
      if (owner.representation !== 'closure slots' || owner.methods.length === 0) continue;
      const constructor = program.declarations.get(owner.constructorName);
      if (constructor?.k !== 'fn') throw unsupportedClass;
      const closureFields = new Map(owner.methods.map(method => [method.name, method.helper]));
      methods.set(owner.constructorName, closureFields);
      for (const method of owner.methods) {
        const helper = program.declarations.get(method.helper);
        if (helper?.k !== 'fn' || !acceptFormalClassMethodFactory(helper.body.k, helper.params.length, Boolean(helper.params[0]?.guard))
          || helpers.has(method.helper)) throw refuseClassSlot('invalid class method factory');
        helpers.set(method.helper, helper.body);
      }

      let body = constructor.body;
      let allocation = null;
      const installed = new Set();
      while (body.k === 'let' || body.k === 'if') {
        if (body.k === 'if') {
          if (!abrupt(body.then) && !abrupt(body.else)) break;
          body = abrupt(body.then) ? body.else : body.then;
          continue;
        }
        const step = body.value;
        if (step.k === 'referenceAllocate' && step.type.sourceClassOwner === owner.constructorName) {
          if (allocation !== null) throw refuseClassSlot('duplicate constructor allocation');
          allocation = body.name;
          let payload = step.value;
          const locals = new Map();
          const uses = new Map();
          variableUses(payload, uses);
          while (payload.k === 'let') {
            if (locals.has(payload.name)) throw refuseClassSlot('duplicate class payload binding');
            locals.set(payload.name, payload.value);
            payload = payload.body;
          }
          if (payload.k !== 'record') throw refuseClassSlot('constructor allocation does not contain the class record');
          payloads.add(payload);
          for (const field of payload.entries) {
            if (!closureFields.has(field.name)) continue;
            const stub = field.value.k === 'var' && locals.get(field.value.name);
            const name = placeholderName(stub, program.declarations);
            if (!name || uses.get(field.value.name) !== 1) throw refuseClassSlot('invalid callable slot placeholder');
            placeholders.set(stub, name);
          }
        } else if (allocation !== null && installed.size < closureFields.size) {
          const field = step.k === 'method' && readRecordFieldWriteName(step.op);
          const factory = step.args?.[0];
          if (!field || !closureFields.has(field) || installed.has(field) || !acceptFormalClassMethodInstallation(
            readRecordFieldWriteOperator(step.op), step.object.k === 'var' && step.object.name === allocation,
            step.args.length, factory?.k === 'call' && factory.fn === closureFields.get(field),
            factory?.args?.[0]?.k === 'var' && factory.args[0].name === allocation, factory?.args?.length ?? -1)) throw refuseClassSlot('invalid callable slot installation');
          installed.add(field);
          installations.add(step);
        }
        body = body.body;
      }
      if (allocation === null || body.k !== 'var' || body.name !== allocation
        || installed.size !== closureFields.size) throw refuseClassSlot('constructor does not return its fully initialized receiver');
    }
    if (!methods.size) return program;
    const unusedStubs = new Set();
    const rewrite = node => {
      if (!node || typeof node !== 'object') return node;
      if (Array.isArray(node)) return node.map(rewrite);
      if (installations.has(node)) return unit;
      if (placeholders.has(node)) {
        unusedStubs.add(placeholders.get(node));
        return unit;
      }
      if (node.kind === 'record' && methods.has(node.sourceClassOwner)) {
        const fields = methods.get(node.sourceClassOwner);
        return { ...node, fields: node.fields.map(field => ({ ...field, type: fields.has(field.name) ? UNIT : rewrite(field.type) })) };
      }
      if (node.k === 'record' && methods.has(node.type.sourceClassOwner)) {
        if (!payloads.has(node)) throw refuseClassSlot('class payload appears outside its original allocation');
        const fields = methods.get(node.type.sourceClassOwner);
        return { ...node, type: rewrite(node.type), entries: node.entries.map(entry =>
          ({ ...entry, value: fields.has(entry.name) ? unit : rewrite(entry.value) })) };
      }
      if (node.k === 'apply' && node.fn.k === 'recordField') {
        const field = node.fn;
        const helper = methods.get(field.object.type.sourceClassOwner)?.get(field.field);
        if (helper) {
          const closure = helpers.get(helper);
          if (field.object.k !== 'referenceRead' || node.args.length !== closure.params.length
            || typeKey(node.type) !== typeKey(closure.body.type)
            || node.args.some((argument, index) => typeKey(argument.type) !== typeKey(closure.params[index].type))) throw refuseClassSlot('class invocation does not retain its exact receiver and argument types');
          return { k: 'call', fn: helper, args: [rewrite(field.object.reference), ...node.args.map(rewrite)],
            type: rewrite(node.type), span: node.span };
        }
      }
      if (node.k === 'recordField' && methods.get(node.object.type.sourceClassOwner)?.has(node.field)) throw refuseClassSlot('detached class slot observation');
      if ((node.k === 'call' || node.k === 'functionRef') && helpers.has(node.fn)) throw refuseClassSlot('detached class factory observation');
      return Object.fromEntries(Object.entries(node).map(([key, value]) => [key, rewrite(value)]));
    };
    const declarations = new Map([...program.declarations].map(([name, declaration]) => {
      const closure = helpers.get(name);
      if (!closure) return [name, rewrite(declaration)];
      const { annotatedRet, semanticRet, ...entry } = declaration;
      return [name, rewrite({ ...entry, params: [...entry.params, ...closure.params], body: closure.body, ret: closure.body.type })];
    }));
    const main = rewrite(program.main);
    // Only remove the original unusable fallback when no checked consumer
    // can still call or retain it. A self-call is its own uninitialized body.
    const retain = value => {
      if (!value || typeof value !== 'object') return;
      if (value.k === 'call' || value.k === 'functionRef') unusedStubs.delete(value.fn);
      for (const child of Object.values(value)) retain(child);
    };
    for (const [name, entry] of declarations) if (!unusedStubs.has(name)) retain(entry);
    retain(main);
    for (const name of unusedStubs) declarations.delete(name);
    return { ...program, declarations, main };
  } catch (error) {
    if (error !== unsupportedClass) throw error;
    return program;
  }
}
