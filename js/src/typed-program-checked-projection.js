import {checkRustRuntimeShell} from './translation/rust-runtime-shell.js';
import {acceptProgramStringRepresentationFlag} from './translation/program-string-representation-rules.js';
// Rebuild a linked product program through the existing Surface checker.
// The adapter carries declarations and shared projection instructions; it
// does not assign meaning to types, operators, patterns or recursion.
import {checkRustConstFunctionContainers} from './translation/rust-const-functions.js';
import { checkProgram } from './translation/check.js';
import {checkRustOwnedLocalStateRecordShapes} from './translation/rust-owned-local-state.js';
import { validateRustTypeNamespace, checkRustTypeNamespaceRecordShapes } from './translation/rust-type-namespace.js';
import { projectLinkedProgramExpression, projectLinkedProgramDeclarationPath } from './typed-program-expression-projection.js';
import { readProgramDeclarationTypeProjectionField, readProgramExpressionProjection, readProgramNeutralFieldDefault, readProgramPropositionOperandFields, readProgramPropositionBindingForm } from './typed-program-link-rules.js';

export function recheckLinkedTypedProgram(program, {wholeProgram=false}={}) {
  if(checkRustConstFunctionContainers(program))wholeProgram=true;
  if(checkRustRuntimeShell(program))return program;
  const rootedNamespace = checkRustTypeNamespaceRecordShapes(program);
  if (rootedNamespace) { validateRustTypeNamespace(program); wholeProgram = true; }
  if(checkRustOwnedLocalStateRecordShapes(program))wholeProgram=true;
  const visited = new WeakSet();
  const active = new WeakSet();
  let hasTuple = false;
  let explicitImmutability = false;
  let unprojected = false;
  let visits = 0;
  function inspectProgramRecords(value, depth = 0) {
    if (!value || typeof value !== 'object') return;
    if (active.has(value)) throw new TypeError('the linked program contains a structural cycle during recheck');
    if (visited.has(value)) return;
    if (++visits > 100_000 || depth > 256) throw new TypeError('the linked program exceeds its recheck resource limit');
    visited.add(value);
    active.add(value);
    if (value.k && value.type && !value.fullName) {
      if (value.k === 'tuple' || value.k === 'tupleIndex') hasTuple = true;
      if (value.k === 'tuple' && (Object.hasOwn(value, 'frozen') || Object.hasOwn(value, 'productModel'))) explicitImmutability = true;
      if (!readProgramExpressionProjection(value.k)) unprojected = true;
    }
    for (const child of value instanceof Map ? value.values() : Object.values(value)) inspectProgramRecords(child, depth + 1);
    active.delete(value);
  }
  inspectProgramRecords(program);
  // Legacy mutable tuples may occur inside collection operations outside the
  // shared native projection boundary. Explicit product qualifiers always
  // require whole-program rechecking, including after a true-to-false edit.
  if (!wholeProgram && (!hasTuple || (!explicitImmutability && unprojected))) return program;
  const projectExpression = value => projectLinkedProgramExpression(value, program.declarations, {rootedNamespace, referenceSemantics: program.referenceSemantics === true, sourceLanguage: program.sourceLanguage});
  function projectType(value) {
    if (value.kind === 'data') return { kind: 'named', path: projectLinkedProgramDeclarationPath(value.name, program.declarations, rootedNamespace) };
    if (['array', 'option', 'set', 'cell'].includes(value.kind)) return { ...value, element: projectType(value.element) };
    if (value.kind === 'tuple') return { ...value, elements: value.elements.map(projectType) };
    if (value.kind === 'union') return { ...value, alternatives: value.alternatives.map(projectType) };
    if (value.kind === 'record') return { ...value, fields: value.fields.map(field => ({ ...field, type: projectType(field.type) })) };
    if (value.kind === 'map') return { ...value, key: projectType(value.key), value: projectType(value.value) };
    if (value.kind === 'function') return { ...value, params: value.params.map(projectType), ret: projectType(value.ret) };
    return value;
  }
  function projectProposition(value) {
    const form = readProgramPropositionBindingForm(value.p ?? '');
    if (!form) throw new TypeError('the linked proposition has no checked surface projection');
    const result = { ...value };
    if (form === 'binders') result.binders = value.binders.map(binder => ({ ...binder, type: projectType(binder.type) }));
    for (const field of readProgramPropositionOperandFields(value.p).split(' ').filter(Boolean)) {
      result[field] = form === 'expressions' ? projectExpression(value[field]) : projectProposition(value[field]);
    }
    return result;
  }
  function projectItem(entry) {
    if (entry.k === 'module') return { k: 'module', name: entry.name, items: entry.items.map(projectItem), span: entry.span };
    if (entry.k === 'data') return {
      k: 'data', name: entry.name,
      ctors: entry.ctors.map(constructor => ({ name: constructor.name,
        fields: constructor.fields.map(field => ({ name: field.name, type: projectType(field.type) })),
      })), span: entry.span, ...(entry.generated !== undefined ? { generated: entry.generated } : {}),
      ...(entry.sourceUnitCloneDerivation!==undefined?{sourceUnitCloneDerivation:entry.sourceUnitCloneDerivation}:{}),
    };
    if (entry.k !== 'fn') throw new TypeError(`the linked declaration ${entry.k} has no checked surface projection`);
    const annotation = readProgramDeclarationTypeProjectionField(entry.k, 'annotation');
    const semantic = readProgramDeclarationTypeProjectionField(entry.k, 'semantic');
    const result = readProgramDeclarationTypeProjectionField(entry.k, 'result');
    const retained = annotation && Object.hasOwn(entry, annotation);
    return { k: 'fn', name: entry.name,
      params: entry.params.map(parameter => ({ ...parameter, type: projectType(parameter.type) })),
      ret: projectType(retained ? entry[annotation] : entry.ret),
      ...(retained ? {[semantic]: projectType(entry[result])} : {}),
      body: projectExpression(entry.body), span: entry.span,
      ...(entry.generated !== undefined ? { generated: entry.generated } : {}),
      ...(entry.sourceConstFunction!==undefined?{sourceConstFunction:entry.sourceConstFunction}:{}),
      ...(entry.sourceDiagnosticAttributes!==undefined?{sourceDiagnosticAttributes:entry.sourceDiagnosticAttributes}:{}),
      ...(entry.sourceSignature !== undefined ? { sourceSignature: entry.sourceSignature } : {}),
      ...(entry.sourceTypeNamespace !== undefined ? { sourceTypeNamespace: entry.sourceTypeNamespace } : {}),
      ...(entry.sourceLoop !== undefined ? { sourceLoop: entry.sourceLoop } : {}),
      ...(entry.sourcePrimitiveOwners!==undefined?{sourcePrimitiveOwners:entry.sourcePrimitiveOwners}:{}),
      ...(entry.sourceEmptyOption!==undefined?{sourceEmptyOption:entry.sourceEmptyOption}:{}),
      ...(entry.sourcePureUnit!==undefined?{sourcePureUnit:entry.sourcePureUnit}:{}),
      ...(entry.sourceLocalState!==undefined?{sourceLocalState:entry.sourceLocalState}:{}),
    };
  }
  const surface = { language: program.sourceLanguage, items: program.items.map(projectItem),
    main: program.main == null ? null : { span: program.main.span,
      ...(program.main.sequentialAsync !== undefined ? { sequentialAsync: program.main.sequentialAsync } : {}),
      effects: program.main.effects.map(effect => {
        if (effect.k === 'print') return { k: 'print', expr: projectExpression(effect.expr), style: 'js-console', span: effect.span };
        if (effect.k === 'assert') return { k: 'assert', prop: projectProposition(effect.prop), span: effect.span };
        if (effect.k === 'let') return { k: 'let', name: effect.name, type: projectType(effect.value.type),
          value: projectExpression(effect.value), span: effect.span, ...(effect.constant !== undefined ? { constant: effect.constant } : {}),
          ...(effect.sourcePureUnitCall!==undefined?{sourcePureUnitCall:effect.sourcePureUnitCall}:{}),
        };
        throw new TypeError(`the linked effect ${effect.k} has no checked surface projection`);
      }),
    },
    ...(program.imports !== undefined ? { imports: program.imports } : {}),
    ...(program.externals !== undefined ? { externals: program.externals } : {}),
    ...(program.referenceSemantics !== undefined ? { referenceSemantics: program.referenceSemantics } : {}),
    ...(program.sourcePureUnitCalls!==undefined?{sourcePureUnitCalls:program.sourcePureUnitCalls}:{}),
  };
  const checked = checkProgram(surface);
  if (Object.hasOwn(program, 'utf16Strings')) {
    if (!acceptProgramStringRepresentationFlag(typeof program.utf16Strings, true)) throw new TypeError('a program string representation flag must be an explicit boolean');
    checked.utf16Strings = program.utf16Strings;
  }
  const compared = new WeakMap();
  let comparisons = 0;
  function readRecordContext(value) {
    if (Object.hasOwn(value, 'schemaVersion')) return 'program';
    if (Object.hasOwn(value, 'effects')) return 'main';
    if (value.k && value.fullName) return `declaration:${value.k}`;
    if (value.k && value.type) return `expression:${value.k}`;
    if (['let', 'print'].includes(value.k)) return `effect:${value.k}`;
    if (Object.hasOwn(value, 'name') && Object.hasOwn(value, 'type')) return 'parameter';
    return '';
  }
  function acceptNeutralField(owner, field, value) {
    const role = readRecordContext(owner);
    const policy = readProgramNeutralFieldDefault(role, field);
    if (policy === 'false') return value === false || value === undefined;
    if (policy === 'null') return value == null;
    if (policy === 'empty-array') return value === undefined || (Array.isArray(value) && !value.length);
    return false;
  }
  function compareProgramData(left, right) {
    if (++comparisons > 100_000) throw new TypeError('the linked program exceeds its comparison resource limit');
    if (Object.is(left, right)) return true;
    if (!left || !right || typeof left !== 'object' || typeof right !== 'object') return false;
    const previous = compared.get(left) ?? new WeakSet();
    if (previous.has(right)) return true;
    previous.add(right); compared.set(left, previous);
    if (left instanceof Map || right instanceof Map) {
      if (!(left instanceof Map && right instanceof Map) || left.size !== right.size) return false;
      return compareProgramData([...left], [...right]);
    }
    if (Array.isArray(left) || Array.isArray(right)) return Array.isArray(left) && Array.isArray(right)
      && left.length === right.length && left.every((value, index) => compareProgramData(value, right[index]));
    const keys = new Set([...Object.keys(left), ...Object.keys(right)]);
    for (const field of keys) {
      if (acceptNeutralField(left, field, left[field]) && acceptNeutralField(right, field, right[field])) continue;
      if (!Object.hasOwn(left, field) || !Object.hasOwn(right, field) || !compareProgramData(left[field], right[field])) return false;
    }
    return true;
  }
  if (!compareProgramData(program, checked)) throw new TypeError('the linked program has inconsistent types, bindings or operation semantics after checked surface projection');
  return program;
}
